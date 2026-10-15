import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location("ci_plan", Path(__file__).with_name("ci-plan.py"))
CI = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CI)
DEV = {"pull_request": {"base": {"ref": "dev"}}}


class PlanTests(unittest.TestCase):
    def test_unavailable_git_base_falls_back_to_full_checks(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "event.json").write_text(json.dumps({"pull_request": {"base": {"ref": "dev", "sha": "missing-base"}}}))
            env = dict(os.environ, GITHUB_EVENT_NAME="pull_request", GITHUB_EVENT_PATH=str(root / "event.json"), GITHUB_OUTPUT=str(root / "output"))
            result = subprocess.run([sys.executable, str(Path(__file__).with_name("ci-plan.py").resolve())], cwd=root, env=env, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr.decode())
            outputs = dict(line.split("=", 1) for line in (root / "output").read_text().splitlines())
            self.assertTrue(all(value == "true" for value in outputs.values()))

    def test_web_pr_runs_only_web_and_audits(self):
        self.assertEqual(CI.plan("pull_request", DEV, ["web/src/App.tsx"]), {"full": "false", "backend": "false", "web": "true", "mobile": "false", "scan_actions": "false"})

    def test_docs_pr_does_not_compile_components(self):
        plan = CI.plan("pull_request", DEV, ["README.md", "docs/api.md"])
        self.assertTrue(all(plan[key] == "false" for key in plan))

    def test_cargo_and_migrations_run_backend(self):
        for path in ("Cargo.lock", "migration/src/lib.rs", "backend/src/main.rs", "backend/src/embedded.md", ".cargo/config.toml"):
            with self.subTest(path=path):
                self.assertEqual(CI.plan("pull_request", DEV, [path])["backend"], "true")

    def test_shared_and_unknown_changes_run_every_component(self):
        for path in ("scripts/check.py", "docker-compose.yml", "new-component/file"):
            plan = CI.plan("pull_request", DEV, [path])
            self.assertTrue(all(plan[key] == "true" for key in ("backend", "web", "mobile")))

    def test_workflow_pr_keeps_actions_scanning(self):
        self.assertEqual(CI.plan("pull_request", DEV, [".github/workflows/all_checks.yml"])["scan_actions"], "true")

    def test_main_manual_and_release_always_run_full(self):
        for event_name, event in (("pull_request", {"pull_request": {"base": {"ref": "main"}}}), ("workflow_dispatch", {}), ("push", {}), ("workflow_call", {})):
            self.assertTrue(all(value == "true" for value in CI.plan(event_name, event, []).values()))

    def results(self, full=False):
        outputs = CI.plan("workflow_dispatch", {}, []) if full else CI.plan("pull_request", DEV, ["web/src/App.tsx"])
        return {"repository_validation": {"result": "success", "outputs": outputs}, "security": {"result": "success"}, "coverage": {"result": "success" if full else "skipped"}, **{f"{client}_{check}": {"result": "success" if full or client == "web" else "skipped"} for client in ("backend", "web", "mobile") for check in ("code_quality", "build_test")}}

    def test_dev_gate_accepts_only_intended_skips(self):
        self.assertTrue(CI.gate(self.results()))
        for result in ("skipped", "failure", "cancelled"):
            results = self.results()
            results["web_build_test"]["result"] = result
            self.assertFalse(CI.gate(results))

    def test_full_gate_rejects_skipped_coverage(self):
        results = self.results(full=True)
        self.assertTrue(CI.gate(results))
        results["coverage"]["result"] = "skipped"
        self.assertFalse(CI.gate(results))

    def test_missing_plan_failed_validation_or_failed_audit_blocks(self):
        for job in ("security", "repository_validation"):
            results = self.results()
            results[job]["result"] = "failure"
            self.assertFalse(CI.gate(results))
        results = self.results()
        results["repository_validation"]["outputs"] = {}
        self.assertFalse(CI.gate(results))


if __name__ == "__main__":
    unittest.main()
