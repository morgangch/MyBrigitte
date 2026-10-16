#!/usr/bin/env python3
"""Progress UI for the Compose stacks: image builds, then a startup checklist.

Usage: stack.py {dev,prod} [--detach] docker compose <global options...>

The Makefile owns the Compose arguments; this only adds the presentation.
Compose's own output is used unchanged off a terminal, in CI or with PLAIN=1.
"""
import base64
from collections import deque
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import threading
import time

ROOT = Path(__file__).resolve().parent.parent
# Bake prefixes steps with the target only when it builds several images.
STEP = re.compile(r"^\[(?:(?P<target>[\w.-]+) )?(?:(?P<stage>[\w.-]+) )?(?P<step>\d+)/(?P<total>\d+)\] (?P<command>.*)")
COMPILING = re.compile(r"^\s*Compiling (?P<crate>\S+)")
MOUNT = re.compile(r"--mount=\S+")
ANSI = re.compile(r"\x1b\[[0-9;?]*[A-Za-z]")
SPINNER = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
PARTIAL = " ▏▎▍▌▋▊▉"
WAIT_TIMEOUT = 900
COLOR = "NO_COLOR" not in os.environ


def style(text, code):
    return f"\x1b[{code}m{text}\x1b[0m" if COLOR else text


def dim(text):
    return style(text, "2")


def clip(line, width):
    """Truncates to `width` visible columns, keeping escape sequences intact."""
    out, visible, index = [], 0, 0
    while index < len(line):
        match = ANSI.match(line, index)
        if match:
            out.append(match.group())
            index = match.end()
            continue
        if visible == width:
            break
        out.append(line[index])
        visible += 1
        index += 1
    return "".join(out) + ("\x1b[0m" if COLOR and visible == width else "")


def bar(fraction, width=30):
    fraction = min(max(fraction, 0.0), 1.0)
    cells = fraction * width
    full = int(cells)
    partial = PARTIAL[int((cells - full) * len(PARTIAL))] if full < width else ""
    filled = "█" * full + partial.strip()
    return style(filled, "36") + dim("░" * (width - len(filled)))


def clock(seconds):
    seconds = int(seconds)
    return f"{seconds // 60}:{seconds % 60:02d}"


def stats_path():
    base = os.environ.get("XDG_CACHE_HOME") or Path.home() / ".cache"
    return Path(base) / "mybrigitte" / "stack-stats.json"


def load_stats():
    try:
        return json.loads(stats_path().read_text())
    except (OSError, ValueError):
        return {}


def save_stats(stats):
    try:
        stats_path().parent.mkdir(parents=True, exist_ok=True)
        stats_path().write_text(json.dumps(stats, indent=2, sort_keys=True))
    except OSError:
        pass


def remember_crates(stats, key, compiled, fallback):
    """Full compiles define the estimate; small incremental ones must not shrink it."""
    if compiled and compiled >= stats.get(key, fallback) / 2:
        stats[key] = compiled


def default_crates():
    try:
        return (ROOT / "Cargo.lock").read_text().count("[[package]]")
    except OSError:
        return 300


def env_value(compose, key, default):
    if key in os.environ:
        return os.environ[key]
    path = next((compose[i + 1] for i, arg in enumerate(compose[:-1]) if arg == "--env-file"), None)
    try:
        lines = Path(path).read_text().splitlines() if path else []
    except OSError:
        lines = []
    for line in lines:
        name, sep, value = line.partition("=")
        if sep and name.strip() == key:
            return value.strip().strip("'\"")
    return default


def app_url(compose):
    host = env_value(compose, "WEB_BIND_ADDRESS", "127.0.0.1")
    port = env_value(compose, "WEB_PORT", "80")
    host = "localhost" if host in ("127.0.0.1", "0.0.0.0", "") else host
    return f"http://{host}" + ("" if port == "80" else f":{port}")


def step_weight(command):
    if "cargo build" in command:
        return 40
    if command.startswith("RUN"):
        return 4
    if command.startswith("FROM"):
        return 2
    return 1


class LineCounter:
    """Splits a byte stream into lines, counts `Compiling` and keeps a tail."""

    def __init__(self, tail=40):
        self.partial = ""
        self.tail = deque(maxlen=tail)
        self.compiled = 0
        self.crate = None

    def feed(self, text):
        lines = (self.partial + text).split("\n")
        self.partial = lines.pop()
        for line in lines:
            line = ANSI.sub("", line).rstrip("\r")
            self.tail.append(line)
            match = COMPILING.match(line)
            if match:
                self.compiled += 1
                self.crate = match.group("crate")


class BuildTracker:
    """Folds BuildKit's JSON progress events into per-target progress."""

    def __init__(self, expected, fallback):
        self.expected = expected
        self.fallback = fallback
        self.vertices = {}
        self.logs = {}
        self.messages = deque(maxlen=20)

    def feed(self, line):
        try:
            event = json.loads(line)
        except ValueError:
            event = None
        if not isinstance(event, dict):
            if line.strip():
                self.messages.append(line.rstrip())
            return
        for vertex in event.get("vertexes") or []:
            state = self.vertices.setdefault(vertex["digest"], {})
            state.update({key: vertex[key] for key in ("name", "started", "completed", "cached", "error") if key in vertex})
            match = STEP.match(state.get("name", ""))
            if match:
                state.update(match.groupdict(), target=match.group("target") or "image")
                state["command"] = " ".join(MOUNT.sub("", state["command"]).split())
        for log in event.get("logs") or []:
            data = base64.b64decode(log.get("data") or "").decode("utf-8", "replace")
            self.logs.setdefault(log.get("vertex"), LineCounter()).feed(data)

    def steps(self):
        return [vertex for vertex in self.vertices.values() if "target" in vertex]

    def compile_fraction(self, digest, vertex):
        counter = self.logs.get(digest)
        if counter is None or "cargo build" not in vertex["command"]:
            return 0.0
        return min(counter.compiled / max(self.estimate(vertex["target"]), 1), 0.99)

    def estimate(self, target):
        return self.expected.get(target, self.fallback)

    def progress(self):
        total = done = 0.0
        for digest, vertex in self.vertices.items():
            if "target" not in vertex:
                continue
            weight = step_weight(vertex["command"])
            total += weight
            if vertex.get("error"):
                continue
            if vertex.get("completed") or vertex.get("cached"):
                done += weight
            elif vertex.get("started"):
                done += weight * self.compile_fraction(digest, vertex)
        return done / total if total else 0.0

    def targets(self):
        """Ordered (target, symbol, detail) rows."""
        rows = {}
        for digest, vertex in self.vertices.items():
            if "target" in vertex:
                rows.setdefault(vertex["target"], []).append((digest, vertex))
        result = []
        for target, steps in rows.items():
            failed = [vertex for _, vertex in steps if vertex.get("error")]
            running = [(digest, vertex) for digest, vertex in steps if vertex.get("started") and not vertex.get("completed")]
            if failed:
                result.append((target, "fail", failed[0]["command"]))
            elif all(vertex.get("completed") or vertex.get("cached") for _, vertex in steps):
                # Base images are resolved, never reported as cached.
                cached = all(vertex.get("cached") for _, vertex in steps if not vertex["command"].startswith("FROM"))
                result.append((target, "done", "cached" if cached else "built"))
            elif running:
                digest, vertex = running[-1]
                counter = self.logs.get(digest)
                if counter and counter.compiled and "cargo build" in vertex["command"]:
                    expected = max(self.estimate(target), counter.compiled)
                    result.append((target, "run", f"compiling {counter.compiled}/≈{expected} · {counter.crate}"))
                else:
                    result.append((target, "run", vertex["command"]))
            else:
                result.append((target, "wait", "waiting"))
        return result

    def compiled(self):
        counts = {}
        for digest, vertex in self.vertices.items():
            counter = self.logs.get(digest)
            if "target" in vertex and counter and "cargo build" in vertex["command"]:
                counts[vertex["target"]] = counts.get(vertex["target"], 0) + counter.compiled
        return counts

    def failure(self):
        lines = []
        for digest, vertex in self.vertices.items():
            if vertex.get("error"):
                lines.append(style(f"✘ {vertex.get('name', '')}", "31"))
                lines.extend(f"  {line}" for line in (self.logs.get(digest) or LineCounter()).tail)
                lines.append(style(f"  {vertex['error']}", "31"))
        return lines + list(self.messages)


class UpTracker:
    """Service states from `compose ps`, plus log tails of starting containers."""

    def __init__(self, services, expected):
        self.services = services
        self.expected = expected
        self.rows = {}
        self.logs = {}
        self.since = {}
        self.finished = {}
        self.followers = []

    def update(self, rows, now):
        for row in rows:
            service = row.get("Service")
            if service not in self.services:
                continue
            self.rows[service] = row
            if row.get("State") in ("running", "exited", "restarting"):
                self.since.setdefault(service, now)
                self.follow(service, row)
            symbol, _ = self.status(service)
            if symbol in ("done", "fail"):
                self.finished.setdefault(service, now)
            else:
                self.finished.pop(service, None)

    def follow(self, service, row):
        if service in self.logs or not row.get("ID"):
            return
        counter = self.logs[service] = LineCounter()

        def pump():
            process = subprocess.Popen(["docker", "logs", "--follow", row["ID"]], stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            self.followers.append(process)
            for chunk in iter(lambda: process.stdout.read1(4096), b""):
                counter.feed(chunk.decode("utf-8", "replace"))

        threading.Thread(target=pump, daemon=True).start()

    def stop(self):
        for process in self.followers:
            if process.poll() is None:
                process.terminate()

    def status(self, service):
        row = self.rows.get(service)
        if row is None or row.get("State") == "created":
            return "wait", "waiting"
        state, health, code = row.get("State"), row.get("Health"), row.get("ExitCode", 0)
        if state == "exited":
            return ("done", "done") if code == 0 else ("fail", f"exited with code {code}")
        if health == "unhealthy":
            return "fail", "unhealthy"
        if health == "healthy":
            return "done", "healthy"
        if state == "restarting":
            return "run", "restarting"
        counter = self.logs.get(service)
        if counter and counter.compiled:
            expected = max(self.expected.get(service, counter.compiled), counter.compiled)
            return "run", f"compiling {counter.compiled}/≈{expected} · {counter.crate}"
        return "run", "starting" if health == "starting" else "running"

    def progress(self):
        done = 0.0
        for service in self.services:
            symbol, _ = self.status(service)
            if symbol == "done":
                done += 1
            counter = self.logs.get(service)
            if symbol == "run" and counter and counter.compiled:
                done += 0.9 * min(counter.compiled / max(self.expected.get(service, 1), 1), 1.0)
        return done / len(self.services) if self.services else 1.0

    def compiled(self):
        return {service: counter.compiled for service, counter in self.logs.items() if counter.compiled}


class Screen:
    """Redraws a block of lines in place."""

    def __init__(self, stream=sys.stdout):
        self.stream = stream
        self.height = 0
        self.tick = 0

    def spinner(self):
        return SPINNER[self.tick % len(SPINNER)]

    def symbol(self, kind):
        return {
            "done": style("✔", "32"),
            "fail": style("✘", "31"),
            "run": style(self.spinner(), "33"),
            "wait": dim("·"),
        }[kind]

    def draw(self, lines):
        width = max(shutil.get_terminal_size((100, 20)).columns - 1, 20)
        text = (f"\x1b[{self.height}F" if self.height else "") + "\x1b[J"
        text += "".join(clip(line, width) + "\n" for line in lines)
        self.stream.write(text)
        self.stream.flush()
        self.height = len(lines)
        self.tick += 1

    def commit(self):
        """Keeps the current block on screen; the next draw starts below it."""
        self.height = 0


def compose_lines(command, sink):
    """Runs `command`, feeding each merged stdout/stderr line to `sink`."""
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, encoding="utf-8", errors="replace", bufsize=1)

    def pump():
        for line in process.stdout:
            sink(line)

    reader = threading.Thread(target=pump, daemon=True)
    reader.start()
    return process, reader


def phase_header(title, fraction, started):
    return f"{style(f'{title:<16}', '1')} {bar(fraction)} {int(fraction * 100):>3}%  {dim(clock(time.monotonic() - started))}"


def build_phase(screen, compose, mode, stats):
    expected = {key.split(":", 2)[2]: value for key, value in stats.items() if key.startswith(f"{mode}:build:")}
    tracker = BuildTracker(expected, default_crates())
    started = time.monotonic()
    process, reader = compose_lines([*compose, "--progress", "json", "build"], tracker.feed)
    shown = 0.0
    while process.poll() is None:
        shown = max(shown, tracker.progress())
        render_build(screen, tracker, shown, started)
        time.sleep(0.1)
    reader.join()
    for target, compiled in tracker.compiled().items():
        remember_crates(stats, f"{mode}:build:{target}", compiled, tracker.fallback)
    if process.returncode:
        render_build(screen, tracker, shown, started)
        print("\n".join(tracker.failure()))
        return process.returncode
    render_build(screen, tracker, 1.0, started)
    screen.commit()
    return 0


def render_build(screen, tracker, shown, started):
    lines = [phase_header("Build images", shown, started)]
    for target, kind, detail in tracker.targets():
        lines.append(f"  {screen.symbol(kind)} {target:<10} {detail if kind == 'run' else dim(detail)}")
    screen.draw(lines)


def compose_ps(compose):
    result = subprocess.run([*compose, "ps", "--all", "--format", "json"], capture_output=True, text=True)
    rows = []
    for line in result.stdout.splitlines():
        try:
            parsed = json.loads(line)
        except ValueError:
            continue
        rows.extend(parsed if isinstance(parsed, list) else [parsed])
    return rows


def up_phase(screen, compose, mode, stats):
    services = subprocess.run([*compose, "config", "--services"], capture_output=True, text=True, check=True).stdout.split()
    expected = {service: stats.get(f"{mode}:run:{service}", default_crates()) for service in services}
    tracker = UpTracker(services, expected)
    output = deque(maxlen=30)
    started = time.monotonic()
    process, reader = compose_lines([*compose, "up", "--detach", "--no-build", "--wait", "--wait-timeout", str(WAIT_TIMEOUT)], output.append)
    shown = 0.0
    try:
        while process.poll() is None:
            tracker.update(compose_ps(compose), time.monotonic())
            shown = max(shown, tracker.progress())
            render_up(screen, tracker, shown, started)
            time.sleep(0.25)
        reader.join()
        tracker.update(compose_ps(compose), time.monotonic())
        for service, compiled in tracker.compiled().items():
            remember_crates(stats, f"{mode}:run:{service}", compiled, default_crates())
        if process.returncode:
            render_up(screen, tracker, shown, started)
            for service in services:
                kind, detail = tracker.status(service)
                if kind == "fail":
                    print(style(f"\n✘ {service}: {detail}", "31"))
                    print("\n".join(f"  {line}" for line in tracker.logs.get(service, LineCounter()).tail))
            print("\n".join(line.rstrip() for line in output))
            return process.returncode
        render_up(screen, tracker, 1.0, started, ready=True)
        screen.commit()
        return 0
    finally:
        tracker.stop()


def render_up(screen, tracker, shown, started, ready=False):
    lines = [phase_header("Start services", shown, started)]
    now = time.monotonic()
    for service in tracker.services:
        kind, detail = ("done", tracker.status(service)[1]) if ready else tracker.status(service)
        since = tracker.since.get(service)
        elapsed = dim(clock(tracker.finished.get(service, now) - since)) if since else ""
        text = f"{detail:<40}"
        lines.append(f"  {screen.symbol(kind)} {service:<10} {dim(text) if kind in ('done', 'wait') else text} {elapsed}")
    screen.draw(lines)


def summary(compose, mode, detach, started):
    url = app_url(compose)
    logs = "logs" if mode == "dev" else "prod-logs"
    lines = [
        "",
        style(f"✔ MyBrigitte {mode} is ready", "1;32") + dim(f" in {clock(time.monotonic() - started)}"),
        f"  {dim('App ')}  {style(url, '4;36')}",
        f"  {dim('Logs')}  make {logs} {dim(f'(or make {logs} s=backend)')}",
    ]
    if mode == "dev" and not detach:
        lines.append(f"  {dim('Stop')}  Ctrl+C {dim('(data is kept; make dev-down removes the containers)')}")
        lines.append(dim("\n  Following backend and web logs; source changes reload automatically.\n"))
    else:
        lines.append(f"  {dim('Stop')}  make {mode}-down")
    print("\n".join(lines))


def plain(mode, detach, compose):
    if mode == "dev" and not detach:
        command = [*compose, "up", "--build", "--watch"]
    else:
        command = [*compose, "up", "--detach", "--build", "--wait", "--wait-timeout", str(WAIT_TIMEOUT)]
    return subprocess.call(command)


def wants_plain():
    return bool(os.environ.get("PLAIN") or os.environ.get("CI")) or not sys.stdout.isatty()


def main(argv):
    if len(argv) < 2 or argv[0] not in ("dev", "prod"):
        print(__doc__, file=sys.stderr)
        return 2
    mode, rest = argv[0], argv[1:]
    detach = rest[:1] == ["--detach"]
    compose = rest[1:] if detach else rest
    if wants_plain():
        return plain(mode, detach, compose)
    if os.name == "nt":
        os.system("")  # Enables VT escape sequences in legacy Windows consoles.
    stats = load_stats()
    screen = Screen()
    started = time.monotonic()
    print(style(f"\n MyBrigitte · {mode} ", "1;7") + "\n")
    sys.stdout.write("\x1b[?25l")
    try:
        code = build_phase(screen, compose, mode, stats) or up_phase(screen, compose, mode, stats)
    except KeyboardInterrupt:
        print(style("\nInterrupted.", "33") + f" Containers already started keep running: make {mode}-down")
        return 130
    finally:
        sys.stdout.write("\x1b[?25h")
        sys.stdout.flush()
        save_stats(stats)
    if code:
        print(style(f"\n✘ {mode} failed.", "1;31") + " Full Compose output: PLAIN=1 make " + mode)
        return code
    summary(compose, mode, detach, started)
    if mode == "dev" and not detach:
        # Images are fresh, so this only attaches logs and starts Compose Watch.
        command = [*compose, "up", "--watch", "--attach", "backend", "--attach", "web"]
        sys.stdout.flush()
        os.execvp(command[0], command)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
