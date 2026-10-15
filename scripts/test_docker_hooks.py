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
        for helper in ("check.py", "ci-plan.py"):
            shutil.copyfile(Path(__file__).with_name(helper), self.root / "scripts" / helper)
        self.tools = self.root / "tools"
        self.tools.mkdir()
        docker = self.tools / "docker"
        docker.write_text(f'''#!{sys.executable}
import io, json, os, sys, tarfile
if 'run' in sys.argv:
    with tarfile.open(fileobj=io.BytesIO(sys.stdin.buffer.read())) as archive:
        names = archive.getnames()
        contents = {{member.name.removeprefix('./'): archive.extractfile(member).read().decode() for member in archive.getmembers() if member.isfile()}}

    with open(os.environ['CHECKS_TEST_LOG'], 'w') as output:
        json.dump(names, output)
    with open(os.environ['CHECKS_TEST_LOG'] + '.contents', 'w') as output:
        json.dump(contents, output)
    with open(os.environ['CHECKS_TEST_LOG'] + '.runs', 'a') as output:
        output.write(json.dumps(contents) + '\\n')
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
        (self.root / "backend").mkdir()
        (self.root / "backend/.env").write_text("nested private")
        subprocess.run(["git", "add", "tracked.txt", ".env", ".env.dev", "backend/.env"], cwd=self.root, env=self.env, check=True)
        (self.root / "new source with spaces.txt").write_text("unstaged")
        (self.root / "deleted.txt").write_text("removed")
        subprocess.run(["git", "add", "deleted.txt"], cwd=self.root, env=self.env, check=True)
        subprocess.run(["git", "config", "user.name", "Hook Test"], cwd=self.root, env=self.env, check=True)
        subprocess.run(["git", "config", "user.email", "hook-test@example.invalid"], cwd=self.root, env=self.env, check=True)
        subprocess.run(["git", "-c", "core.hooksPath=/dev/null", "commit", "-qm", "base"], cwd=self.root, env=self.env, check=True)
        (self.root / "deleted.txt").unlink()

    def run_hook(self, mode="quick", exit_code="0", extra=(), stdin=None, env_extra=None):
        env = dict(self.env, CHECKS_TEST_EXIT=exit_code, **(env_extra or {}))
        return subprocess.run(["bash", str(self.root / "scripts/check-docker.sh"), mode, *extra], cwd="/", env=env, input=stdin, text=True, capture_output=True)

    def test_archive_includes_working_changes_without_secrets_or_git_metadata(self):
        result = self.run_hook()
        self.assertEqual(result.returncode, 0, result.stderr)
        names = json.loads((self.root / "archive.json").read_text())
        self.assertIn("tracked.txt", names)
        self.assertIn("new source with spaces.txt", names)
        self.assertNotIn(".env", names)
        self.assertNotIn(".env.dev", names)
        self.assertNotIn(".git", names)
        self.assertNotIn("deleted.txt", names)

    def git(self, *args, env=None):
        return subprocess.check_output(["git", *args], cwd=self.root, env=env or self.env, text=True).strip()

    def contents(self):
        return json.loads((self.root / "archive.json.contents").read_text())

    def commit_web(self, content):
        path = self.root / "web/App.tsx"
        path.parent.mkdir(exist_ok=True)
        path.write_text(content)
        self.git("add", "web/App.tsx")
        self.git("-c", "core.hooksPath=/dev/null", "commit", "-qm", content)
        return self.git("rev-parse", "HEAD")

    def test_staged_snapshot_excludes_unstaged_edits_and_untracked_files(self):
        self.commit_web("base code")
        (self.root / "web/App.tsx").write_text("staged code")
        self.git("add", "web/App.tsx")
        (self.root / "web/App.tsx").write_text("broken unstaged code")
        (self.root / "web/untracked.ts").write_text("unrelated")
        result = self.run_hook(extra=("--staged",))
        self.assertEqual(result.returncode, 0, result.stderr)
        contents = self.contents()
        self.assertEqual(contents["web/App.tsx"], "staged code")
        self.assertEqual(contents[".checks-runner/paths"], "web/App.tsx\0")
        self.assertNotIn("web/untracked.ts", contents)
        self.assertNotIn(".env", contents)
        self.assertNotIn("backend/.env", contents)

    def test_git_partial_commit_uses_its_temporary_index(self):
        (self.root / "README.md").write_text("old docs")
        self.git("add", "README.md")
        self.git("-c", "core.hooksPath=/dev/null", "commit", "-qm", "initial docs")
        (self.root / "web").mkdir()
        (self.root / "web/App.tsx").write_text("unrelated staged code")
        self.git("add", "web/App.tsx")
        (self.root / "README.md").write_text("new docs")
        hooks = self.root / "hooks"
        hooks.mkdir()
        hook = hooks / "pre-commit"
        shutil.copyfile(Path(__file__).resolve().parents[1] / ".githooks/pre-commit", hook)
        hook.chmod(0o755)
        self.git("-c", f"core.hooksPath={hooks}", "commit", "-qm", "partial docs", "README.md")
        self.assertFalse((self.root / "archive.json").exists())
        self.assertEqual(self.git("diff", "--cached", "--name-only"), "web/App.tsx")

    def test_docs_only_staged_commit_never_starts_docker(self):
        (self.root / "README.md").write_text("docs")
        self.git("add", "README.md")
        self.assertEqual(self.run_hook(extra=("--staged",)).returncode, 0)
        self.assertFalse((self.root / "archive.json").exists())

    def test_alternate_index_is_honored_without_modifying_primary_index(self):
        (self.root / "web").mkdir()
        (self.root / "web/App.tsx").write_text("primary index code")
        self.git("add", "web/App.tsx")
        alternate = str(self.root / "alternate-index")
        env = dict(self.env, GIT_INDEX_FILE=alternate)
        self.git("read-tree", "HEAD", env=env)
        (self.root / "README.md").write_text("alternate index docs")
        self.git("add", "README.md", env=env)
        result = self.run_hook(extra=("--staged",), env_extra={"CHECKS_INDEX_FILE": alternate})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((self.root / "archive.json").exists())
        self.assertEqual(self.git("diff", "--cached", "--name-only"), "web/App.tsx")

    def test_push_checks_outgoing_commit_instead_of_index_or_worktree(self):
        base = self.git("rev-parse", "HEAD")
        head = self.commit_web("outgoing code")
        (self.root / "web/App.tsx").write_text("dirty index")
        self.git("add", "web/App.tsx")
        result = self.run_hook("full", extra=("--push", "origin"), stdin=f"refs/heads/feature {head} refs/heads/feature {base}\n")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.contents()["web/App.tsx"], "outgoing code")
        self.assertEqual(self.contents()[".checks-runner/paths"], "web/App.tsx\0")

    def test_new_branch_compares_to_dev_while_tags_require_all_checks(self):
        base = self.git("rev-parse", "HEAD")
        self.git("update-ref", "refs/remotes/origin/dev", base)
        head = self.commit_web("new branch")
        zero = "0" * 40
        result = self.run_hook("full", extra=("--push", "origin"), stdin=f"refs/heads/new {head} refs/heads/new {zero}\n")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.contents()[".checks-runner/paths"], "web/App.tsx\0")
        result = self.run_hook("full", extra=("--push", "origin"), stdin=f"refs/tags/v1.0.0 {head} refs/tags/v1.0.0 {zero}\n")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.contents()[".checks-runner/paths"], "unknown-base\0")

    def test_deleted_refs_do_not_start_checks(self):
        result = self.run_hook("full", extra=("--push", "origin"), stdin=f"refs/heads/old {'0' * 40} refs/heads/old {self.git('rev-parse', 'HEAD')}\n")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((self.root / "archive.json").exists())

    def test_multiple_outgoing_refs_have_separate_snapshots(self):
        base = self.git("rev-parse", "HEAD")
        first = self.commit_web("first ref")
        second = self.commit_web("second ref")
        records = f"refs/heads/one {first} refs/heads/one {base}\nrefs/heads/two {second} refs/heads/two {base}\n"
        result = self.run_hook("full", extra=("--push", "origin"), stdin=records)
        self.assertEqual(result.returncode, 0, result.stderr)
        runs = [json.loads(line) for line in (self.root / "archive.json.runs").read_text().splitlines()]
        self.assertEqual([run["web/App.tsx"] for run in runs], ["first ref", "second ref"])

    def test_container_failure_blocks_hook(self):
        self.assertNotEqual(self.run_hook(exit_code="7").returncode, 0)

    def test_invalid_mode_never_invokes_container(self):
        self.assertEqual(self.run_hook(mode="invalid").returncode, 2)
        self.assertFalse((self.root / "archive.json").exists())


if __name__ == "__main__":
    unittest.main()
