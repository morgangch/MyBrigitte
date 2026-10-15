#!/usr/bin/env python3
"""Run the repository's local quality gates without loading application secrets."""

import argparse
import importlib.util
import os
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import shlex
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def run_group(name, commands):
    for cwd, command in commands:
        print(f"[{name}] {shlex.join(command)} ({cwd.relative_to(ROOT)})", flush=True)
        if shutil.which(command[0]) is None:
            print(f"[{name}] Missing executable: {command[0]}", file=sys.stderr)
            return False
        if command[:2] == ["cargo", "audit"] and shutil.which("cargo-audit") is None:
            print("Install cargo-audit: cargo install --locked cargo-audit", file=sys.stderr)
            return False
        result = subprocess.run(command, cwd=cwd, check=False)
        if result.returncode:
            print(f"[{name}] FAILED (exit {result.returncode})", file=sys.stderr)
            return False
    print(f"[{name}] OK", flush=True)
    return True


def groups(mode, paths=None):
    selected = dict.fromkeys(("backend", "web", "mobile", "scripts", "workflows"), True)
    if paths is not None:
        spec = importlib.util.spec_from_file_location("check_policy", Path(__file__).with_name("ci-plan.py"))
        policy = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(policy)
        selected = policy.local_plan(paths)
        print("Selected components: " + ", ".join(name for name, enabled in selected.items() if enabled), flush=True)
    checks = {}
    clients = []
    for name in ("web", "mobile"):
        directory = ROOT / name
        package = directory / "package.json"
        if package.is_file() and selected[name]:
            clients.append((name, directory, json.loads(package.read_text()).get("scripts", {})))

    if mode in ("audit", "full"):
        if selected["backend"]:
            checks["Rust audit"] = [(ROOT, ["cargo", "audit"])]
        for name, directory, _ in clients:
            checks[f"{name} audit"] = [(directory, ["bun", "audit", "--audit-level=high"])]

    if mode in ("quick", "full"):
        backend = [(ROOT, ["cargo", "fmt", "--all", "--check"])]
        if mode == "full":
            backend.extend([
                (ROOT, ["cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"]),
                (ROOT, ["cargo", "test", "--workspace", "--locked"]),
            ])
        if selected["backend"]:
            checks["Backend"] = backend
        for name, directory, scripts in clients:
            commands = [(directory, ["bun", "ci"])]
            names = ["lint", "format:check", "typecheck"]
            if mode == "full":
                names.extend(["test", "build"])
            commands.extend((directory, ["bun", "run", task]) for task in names if task in scripts)
            checks[name] = commands
        if (mode == "full" or paths is not None) and selected["scripts"]:
            checks["Scripts"] = [(ROOT, [sys.executable, "-m", "unittest", "discover", "-s", "scripts", "-p", "test_*.py"])]
        if (mode == "full" or paths is not None) and selected["workflows"]:
            if shutil.which("actionlint"):
                checks["Workflows"] = [(ROOT, ["actionlint"])]
            else:
                print("[Workflows] actionlint not installed; YAML checks skipped locally.", flush=True)
    return checks


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("audit", "quick", "full"), default="full", nargs="?")
    parser.add_argument("--changed-paths", type=Path, help="NUL-delimited Git diff paths; absent means all components")
    args = parser.parse_args()
    paths = None
    if args.changed_paths is not None:
        paths = [os.fsdecode(path) for path in args.changed_paths.read_bytes().split(b"\0") if path]
    checks = groups(args.mode) if paths is None else groups(args.mode, paths)
    # Keep Cargo commands sequential; independent clients/audits can run together.
    with ThreadPoolExecutor(max_workers=4) as pool:
        futures = {name: pool.submit(run_group, name, commands) for name, commands in checks.items()}
        failed = [name for name, future in futures.items() if not future.result()]
    if failed:
        print("Failed checks: " + ", ".join(failed), file=sys.stderr)
        return 1
    print("All requested checks passed.", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
