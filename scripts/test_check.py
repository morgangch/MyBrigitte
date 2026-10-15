import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("local_checks", Path(__file__).with_name("check.py"))
CHECKS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKS)


class CheckTests(unittest.TestCase):
    def setUp(self):
        output = patch("builtins.print")
        output.start()
        self.addCleanup(output.stop)

    def test_command_failure_blocks_group_and_remaining_commands(self):
        with patch.object(CHECKS.shutil, "which", return_value="/bin/tool"), patch.object(
            CHECKS.subprocess, "run", return_value=subprocess.CompletedProcess([], 1)
        ) as run:
            self.assertFalse(CHECKS.run_group("test", [(CHECKS.ROOT, ["tool"]), (CHECKS.ROOT, ["tool"])]))
            self.assertEqual(run.call_count, 1)

    def test_missing_audit_tool_blocks_instead_of_skipping(self):
        with patch.object(CHECKS.shutil, "which", side_effect=lambda name: None if name == "cargo-audit" else "/bin/cargo"), patch.object(
            CHECKS.subprocess, "run"
        ) as run:
            self.assertFalse(CHECKS.run_group("audit", [(CHECKS.ROOT, ["cargo", "audit"])]))
            run.assert_not_called()

    def test_audit_failure_does_not_hide_other_audits(self):
        with patch.object(CHECKS, "groups", return_value={"Rust audit": [], "web audit": []}), patch.object(
            CHECKS, "run_group", side_effect=lambda name, commands: name != "Rust audit"
        ) as run, patch("sys.argv", ["check.py", "audit"]):
            self.assertEqual(CHECKS.main(), 1)
            self.assertEqual({call.args[0] for call in run.call_args_list}, {"Rust audit", "web audit"})

    def test_commands_use_repository_root_from_another_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "repo"
            (root / "scripts").mkdir(parents=True)
            (root / "web").mkdir()
            (root / "web/package.json").write_text('{}')
            shutil.copyfile(Path(__file__).with_name("check.py"), root / "scripts/check.py")
            tools = Path(directory) / "tools"
            tools.mkdir()
            for tool in ("cargo", "cargo-audit", "bun"):
                executable = tools / tool
                executable.write_text('#!/bin/sh\nprintf "%s\\n" "$PWD" > "$CHECK_LOG_DIR/$(basename "$0")"\n')
                executable.chmod(0o755)
            env = dict(os.environ, PATH=str(tools) + os.pathsep + os.environ["PATH"], CHECK_LOG_DIR=directory)
            result = subprocess.run([os.sys.executable, str(root / "scripts/check.py"), "audit"], cwd=directory, env=env, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr.decode())
            self.assertEqual((Path(directory) / "cargo").read_text().strip(), str(root))
            self.assertEqual((Path(directory) / "bun").read_text().strip(), str(root / "web"))

    def test_hook_installation_is_local_idempotent_and_preserves_custom_hooks(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "scripts").mkdir()
            installer = root / "scripts/install-hooks.sh"
            shutil.copyfile(Path(__file__).with_name("install-hooks.sh"), installer)
            subprocess.run(["git", "init", "-q", directory], check=True)
            for _ in range(2):
                subprocess.run(["sh", str(installer)], cwd="/", check=True, capture_output=True)
            configured = subprocess.check_output(["git", "-C", directory, "config", "--local", "--get", "core.hooksPath"], text=True).strip()
            self.assertEqual(configured, ".githooks")
            subprocess.run(["git", "-C", directory, "config", "--local", "core.hooksPath", "custom-hooks"], check=True)
            self.assertNotEqual(subprocess.run(["sh", str(installer)], capture_output=True).returncode, 0)
            self.assertEqual(subprocess.check_output(["git", "-C", directory, "config", "--get", "core.hooksPath"], text=True).strip(), "custom-hooks")

    def test_initialized_client_uses_frozen_install_and_audit_threshold(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "web").mkdir()
            (root / "web/package.json").write_text('{"scripts":{"lint":"lint","test":"tests"}}')
            (root / "mobile").mkdir()
            with patch.object(CHECKS, "ROOT", root), patch.object(CHECKS.shutil, "which", return_value=None):
                full = CHECKS.groups("full")
                quick = CHECKS.groups("quick")
            self.assertNotIn("mobile", full)
            self.assertEqual(full["web"][0][1], ["bun", "ci"])
            self.assertEqual(full["web audit"][0][1], ["bun", "audit", "--audit-level=high"])
            self.assertIn((root / "web", ["bun", "run", "test"]), full["web"])
            self.assertNotIn((root / "web", ["bun", "run", "test"]), quick["web"])
            self.assertNotIn("Rust audit", quick)


if __name__ == "__main__":
    unittest.main()
