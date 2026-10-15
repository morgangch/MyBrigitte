#!/usr/bin/env python3
"""Select the dev PR checks and validate explicitly permitted job skips."""

import json
import os
from pathlib import Path
import subprocess
import sys


def plan(event_name, event, paths):
    full = not (event_name == "pull_request" and event.get("pull_request", {}).get("base", {}).get("ref") == "dev")
    selected = {"backend": full, "web": full, "mobile": full}
    scan_actions = full
    for path in paths:
        if ("/" not in path and path.endswith(".md")) or path.startswith("docs/") or path == ".github/workflows/README.md":
            continue
        if path.startswith(("backend/", "migration/", ".cargo/")) or path in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "rust-toolchain"):
            selected["backend"] = True
        elif path.startswith("web/"):
            selected["web"] = True
        elif path.startswith("mobile/"):
            selected["mobile"] = True
        else:
            # Shared tooling, Docker, workflow changes and unknown paths are conservative.
            selected = dict.fromkeys(selected, True)
        if path.startswith(".github/workflows/"):
            scan_actions = True
    return {key: str(value).lower() for key, value in {"full": full, **selected, "scan_actions": scan_actions}.items()}


def gate(results):
    validation = results.get("repository_validation", {})
    if validation.get("result") != "success":
        return False
    outputs = validation.get("outputs", {})
    if any(outputs.get(key) not in ("true", "false") for key in ("full", "backend", "web", "mobile")):
        return False
    required = {"security": True, "coverage": outputs["full"] == "true"}
    for client in ("backend", "web", "mobile"):
        for check in ("code_quality", "build_test"):
            required[f"{client}_{check}"] = outputs["full"] == "true" or outputs[client] == "true"
    return all(results.get(job, {}).get("result") in (("success",) if enabled else ("success", "skipped")) for job, enabled in required.items())


def main():
    if sys.argv[1:] == ["gate"]:
        results = json.loads(os.environ["RESULTS"])
        summary = "\n".join(f"{name}: {job['result']}" for name, job in results.items()) + "\n"
        print(summary)
        if os.environ.get("GITHUB_STEP_SUMMARY"):
            with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as output:
                output.write(summary)
        return 0 if gate(results) else 1
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    event_name = os.environ["GITHUB_EVENT_NAME"]
    paths = []
    if os.environ["GITHUB_EVENT_NAME"] == "pull_request":
        base = event["pull_request"]["base"]["sha"]
        result = subprocess.run(["git", "diff", "--name-only", "--no-renames", "-z", base, "HEAD"], capture_output=True, check=False)
        paths = result.stdout.decode().strip("\0").split("\0") if result.stdout else []
        if result.returncode != 0:
            print("Cannot compare PR base; selecting the full check profile.")
            event_name = "diff-failed"
    values = plan(event_name, event, paths)
    print(json.dumps(values, indent=2))
    with open(os.environ["GITHUB_OUTPUT"], "a") as output:
        output.writelines(f"{key}={value}\n" for key, value in values.items())
    return 0


if __name__ == "__main__":
    sys.exit(main())
