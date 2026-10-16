import base64
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock

SPEC = importlib.util.spec_from_file_location("stack", Path(__file__).with_name("stack.py"))
STACK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STACK)


def vertex(digest, name, **state):
    return json.dumps({"vertexes": [{"digest": digest, "name": name, **state}]})


def log(digest, text):
    return json.dumps({"logs": [{"vertex": digest, "stream": 2, "data": base64.b64encode(text.encode()).decode()}]})


class BuildTrackerTests(unittest.TestCase):
    def tracker(self):
        tracker = STACK.BuildTracker({"backend": 4}, fallback=100)
        tracker.feed(vertex("a", "[backend builder 1/3] FROM docker.io/library/rust:1.99-bookworm", cached=True))
        tracker.feed(vertex("b", "[backend builder 2/3] COPY backend ./backend", started="t", completed="t"))
        tracker.feed(vertex("c", "[backend builder 3/3] RUN cargo build --release --locked -p backend", started="t"))
        tracker.feed(vertex("d", "[web 1/1] RUN bun ci", cached=True))
        tracker.feed(vertex("e", "[internal] load build definition from Dockerfile", started="t", completed="t"))
        return tracker

    def test_cargo_step_progress_follows_compiled_crates_across_chunks(self):
        tracker = self.tracker()
        before = tracker.progress()
        tracker.feed(log("c", "   Compiling serde v1.0.229\n   Compil"))
        tracker.feed(log("c", "ing tokio v1.53.2\n"))
        self.assertGreater(tracker.progress(), before)
        self.assertEqual(tracker.compiled(), {"backend": 2})
        rows = dict((target, (kind, detail)) for target, kind, detail in tracker.targets())
        self.assertEqual(rows["backend"], ("run", "compiling 2/≈4 · tokio"))
        self.assertEqual(rows["web"], ("done", "cached"))

    def test_progress_never_reports_completion_before_the_step_finishes(self):
        tracker = self.tracker()
        tracker.feed(log("c", "".join(f"   Compiling crate{i} v1.0.0\n" for i in range(50))))
        self.assertLess(tracker.progress(), 1.0)
        tracker.feed(vertex("c", "[backend builder 3/3] RUN cargo build --release --locked -p backend", started="t", completed="t"))
        self.assertEqual(tracker.progress(), 1.0)
        self.assertEqual(tracker.targets()[0], ("backend", "done", "built"))

    def test_single_image_builds_without_target_prefix_and_cached_rebuilds(self):
        tracker = STACK.BuildTracker({}, fallback=100)
        tracker.feed(vertex("a", "[1/2] FROM docker.io/library/nginx:1.31.0-alpine", started="t", completed="t"))
        tracker.feed(vertex("b", "[2/2] COPY default.conf.template /etc/nginx/templates/", started="t", completed="t", cached=True))
        self.assertEqual(tracker.targets(), [("image", "done", "cached")])
        self.assertEqual(tracker.progress(), 1.0)

    def test_failure_reports_the_failing_step_its_log_tail_and_plain_messages(self):
        tracker = self.tracker()
        tracker.feed(log("c", "error[E0425]: cannot find value `x`\n"))
        tracker.feed(vertex("c", "[backend builder 3/3] RUN cargo build --release --locked -p backend", started="t", completed="t", error="exit code: 101"))
        tracker.feed("failed to solve: process did not complete successfully\n")
        failure = "\n".join(tracker.failure())
        self.assertIn("cannot find value", failure)
        self.assertIn("exit code: 101", failure)
        self.assertIn("failed to solve", failure)
        self.assertEqual(tracker.targets()[0][1], "fail")
        self.assertLess(tracker.progress(), 1.0)

    def test_step_labels_hide_cache_mount_flags(self):
        tracker = STACK.BuildTracker({}, fallback=1)
        tracker.feed(vertex("a", "[web 2/2] RUN --mount=type=cache,id=bun,target=/root/.bun/install/cache     bun ci", started="t"))
        self.assertEqual(tracker.targets(), [("web", "run", "RUN bun ci")])


class UpTrackerTests(unittest.TestCase):
    def test_statuses_cover_one_shot_jobs_health_and_failures(self):
        tracker = STACK.UpTracker(["postgres", "migrate", "seed", "backend", "web"], {"backend": 10})
        tracker.follow = lambda service, row: None
        tracker.update([
            {"Service": "postgres", "State": "running", "Health": "healthy"},
            {"Service": "migrate", "State": "exited", "ExitCode": 0},
            {"Service": "seed", "State": "exited", "ExitCode": 1},
            {"Service": "backend", "State": "running", "Health": "starting"},
            {"Service": "unknown", "State": "running"},
        ], now=1.0)
        self.assertEqual(tracker.status("postgres"), ("done", "healthy"))
        self.assertEqual(tracker.status("migrate"), ("done", "done"))
        self.assertEqual(tracker.status("seed"), ("fail", "exited with code 1"))
        self.assertEqual(tracker.status("backend"), ("run", "starting"))
        self.assertEqual(tracker.status("web"), ("wait", "waiting"))
        tracker.logs["backend"] = STACK.LineCounter()
        tracker.logs["backend"].feed("   Compiling backend v0.1.0 (/app/backend)\n")
        self.assertEqual(tracker.status("backend"), ("run", "compiling 1/≈10 · backend"))
        self.assertAlmostEqual(tracker.progress(), (2 + 0.09) / 5)


class HelperTests(unittest.TestCase):
    def test_clip_counts_visible_columns_only(self):
        line = "\x1b[32m✔\x1b[0m backend compiling"
        clipped = STACK.clip(line, 9)
        self.assertEqual(STACK.ANSI.sub("", clipped), "✔ backend")
        self.assertTrue(clipped.startswith("\x1b[32m"))

    def test_bar_has_fixed_visible_width(self):
        for fraction in (0, 0.013, 0.5, 0.999, 1, 2):
            self.assertEqual(len(STACK.ANSI.sub("", STACK.bar(fraction, 20))), 20)

    def test_crate_estimate_ignores_small_incremental_builds(self):
        stats = {}
        STACK.remember_crates(stats, "k", 1, fallback=345)
        self.assertNotIn("k", stats)
        STACK.remember_crates(stats, "k", 300, fallback=345)
        STACK.remember_crates(stats, "k", 2, fallback=345)
        self.assertEqual(stats["k"], 300)
        STACK.remember_crates(stats, "k", 280, fallback=345)
        self.assertEqual(stats["k"], 280)
        STACK.remember_crates(stats, "k", 0, fallback=345)
        self.assertEqual(stats["k"], 280)

    def test_app_url_reads_the_compose_env_file_and_environment_overrides(self):
        with tempfile.TemporaryDirectory() as directory:
            env = Path(directory) / ".env.dev"
            env.write_text("# comment\nWEB_PORT=5173\nWEB_BIND_ADDRESS=127.0.0.1\n")
            compose = ["docker", "compose", "--env-file", str(env), "-f", "docker-compose.yml"]
            with mock.patch.dict(os.environ, {}, clear=True):
                self.assertEqual(STACK.app_url(compose), "http://localhost:5173")
            with mock.patch.dict(os.environ, {"WEB_PORT": "80", "WEB_BIND_ADDRESS": "10.0.0.2"}, clear=True):
                self.assertEqual(STACK.app_url(compose), "http://10.0.0.2")

    def test_plain_mode_off_terminal_or_on_request(self):
        with mock.patch.object(STACK.sys, "stdout", io.StringIO()), mock.patch.dict(os.environ, {}, clear=True):
            self.assertTrue(STACK.wants_plain())
        with mock.patch.object(STACK.sys.stdout, "isatty", return_value=True):
            with mock.patch.dict(os.environ, {"PLAIN": "1"}, clear=True):
                self.assertTrue(STACK.wants_plain())
            with mock.patch.dict(os.environ, {}, clear=True):
                self.assertFalse(STACK.wants_plain())

    def test_plain_mode_keeps_the_original_compose_commands(self):
        compose = ["docker", "compose", "-p", "x"]
        with mock.patch.object(STACK, "foreground", return_value=0) as run:
            STACK.plain("dev", False, compose)
            STACK.plain("prod", False, compose)
        self.assertEqual(run.call_args_list[0].args[0], [*compose, "up", "--build", "--watch"])
        self.assertEqual(run.call_args_list[1].args[0][:6], [*compose, "up", "--detach"])

    def test_ctrl_c_waits_for_compose_to_stop_instead_of_killing_it(self):
        process = mock.Mock()
        process.wait.side_effect = [KeyboardInterrupt, KeyboardInterrupt, 0]
        with mock.patch.object(STACK.subprocess, "Popen", return_value=process):
            self.assertEqual(STACK.foreground(["docker", "compose", "up"]), 0)
        process.kill.assert_not_called()
        process.terminate.assert_not_called()

    def test_ascii_glyphs_for_legacy_windows_consoles(self):
        with mock.patch.object(STACK, "GLYPHS", STACK.ASCII):
            for fraction in (0, 0.37, 1):
                rendered = STACK.ANSI.sub("", STACK.bar(fraction, 20))
                self.assertEqual(len(rendered), 20)
                self.assertTrue(set(rendered) <= {"#", "-"})
            counter = STACK.LineCounter()
            counter.feed("   Compiling tokio v1.53.2\n")
            tracker = STACK.UpTracker(["backend"], {"backend": 10})
            tracker.rows["backend"] = {"Service": "backend", "State": "running", "Health": "starting"}
            tracker.logs["backend"] = counter
            self.assertEqual(tracker.status("backend"), ("run", "compiling 1/~10 - tokio"))
        for glyphs in (STACK.UNICODE, STACK.ASCII):
            self.assertEqual(set(glyphs), set(STACK.UNICODE))
        self.assertTrue(all(ord(char) < 128 for value in STACK.ASCII.values() for char in value))

    def test_help_lists_sections_and_documented_targets_only(self):
        with tempfile.TemporaryDirectory() as directory:
            makefile = Path(directory) / "Makefile"
            makefile.write_text("##@ Development\n\ndev: .env.dev ## Start dev\n.env.dev:\n\t@cp a b\nhidden:\n", encoding="utf-8")
            with mock.patch.object(STACK.sys, "stdout", io.StringIO()) as out:
                self.assertEqual(STACK.main(["help", str(makefile)]), 0)
        text = STACK.ANSI.sub("", out.getvalue())
        self.assertIn("Development", text)
        self.assertIn("dev            Start dev", text)
        self.assertNotIn("hidden", text)
        self.assertNotIn(".env.dev:", text)

    def test_usage_errors(self):
        with mock.patch.object(STACK.sys, "stderr", io.StringIO()):
            self.assertEqual(STACK.main([]), 2)
            self.assertEqual(STACK.main(["staging", "docker"]), 2)


if __name__ == "__main__":
    unittest.main()
