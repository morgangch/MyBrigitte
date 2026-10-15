import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class DockerHookTests(unittest.TestCase):
    def setUp(self):
        self.work = tempfile.TemporaryDirectory()
        self.addCleanup(self.work.cleanup)
        self.root = Path(self.work.name)
        (self.root / "scripts").mkdir()
        shutil.copyfile(Path(__file__).with_name("check-docker.sh"), self.root / "scripts/check-docker.sh")
        self.tools = self.root / "tools"
        self.tools.mkdir()
        docker = self.tools / "docker"
        docker.write_text(f'''#!{sys.executable}
import io, json, os, sys, tarfile
if 'run' in sys.argv:
    with tarfile.open(fileobj=io.BytesIO(sys.stdin.buffer.read())) as archive:
        names = archive.getnames()
    with open(os.environ['CHECKS_TEST_LOG'], 'w') as output:
        json.dump(names, output)
    sys.exit(int(os.environ.get('CHECKS_TEST_EXIT', '0')))
''')
        docker.chmod(0o755)
        self.env = dict(os.environ, PATH=str(self.tools) + os.pathsep + os.environ["PATH"], CHECKS_TEST_LOG=str(self.root / "archive.json"))
        for key in subprocess.check_output(["git", "rev-parse", "--local-env-vars"], text=True).splitlines():
            self.env.pop(key, None)
        subprocess.run(["git", "init", "-q", str(self.root)], env=self.env, check=True)
        (self.root / "tracked.txt").write_text("tracked")
        (self.root / ".env").write_text("private")
        (self.root / ".env.dev").write_text("private")
        subprocess.run(["git", "add", "tracked.txt", ".env", ".env.dev"], cwd=self.root, env=self.env, check=True)
        (self.root / "new source with spaces.txt").write_text("unstaged")
        (self.root / "deleted.txt").write_text("removed")
        subprocess.run(["git", "add", "deleted.txt"], cwd=self.root, env=self.env, check=True)
        (self.root / "deleted.txt").unlink()

    def run_hook(self, mode="quick", exit_code="0"):
        env = dict(self.env, CHECKS_TEST_EXIT=exit_code)
        return subprocess.run(["bash", str(self.root / "scripts/check-docker.sh"), mode], cwd="/", env=env, capture_output=True)

    def test_archive_includes_working_changes_without_secrets_or_git_metadata(self):
        result = self.run_hook()
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        names = json.loads((self.root / "archive.json").read_text())
        self.assertIn("tracked.txt", names)
        self.assertIn("new source with spaces.txt", names)
        self.assertNotIn(".env", names)
        self.assertNotIn(".env.dev", names)
        self.assertNotIn(".git", names)
        self.assertNotIn("deleted.txt", names)

    def test_container_failure_blocks_hook(self):
        self.assertNotEqual(self.run_hook(exit_code="7").returncode, 0)

    def test_invalid_mode_never_invokes_container(self):
        self.assertEqual(self.run_hook(mode="invalid").returncode, 2)
        self.assertFalse((self.root / "archive.json").exists())


if __name__ == "__main__":
    unittest.main()
