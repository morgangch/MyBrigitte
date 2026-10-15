import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location('release_channel', Path(__file__).with_name('release-channel.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ReleaseChannelTest(unittest.TestCase):
    def setUp(self):
        self.work = tempfile.TemporaryDirectory()
        self.addCleanup(self.work.cleanup)
        self.root = Path(self.work.name)
        self.git('init', '-q', '-b', 'main')
        self.git('config', 'user.name', 'Release Test')
        self.git('config', 'user.email', 'release-test@example.invalid')
        self.git('commit', '-q', '--allow-empty', '-m', 'main')
        self.main = self.git('rev-parse', 'HEAD')
        self.git('update-ref', 'refs/remotes/origin/main', self.main)
        self.git('checkout', '-q', '-b', 'dev')
        self.git('commit', '-q', '--allow-empty', '-m', 'dev')
        self.dev = self.git('rev-parse', 'HEAD')
        self.git('update-ref', 'refs/remotes/origin/dev', self.dev)
        self.git('checkout', '-q', '-b', 'feature')
        self.git('commit', '-q', '--allow-empty', '-m', 'feature')
        self.feature = self.git('rev-parse', 'HEAD')

    def git(self, *args):
        return subprocess.check_output(['git', *args], cwd=self.root, text=True).strip()

    def classify(self, tag, sha, **kwargs):
        return module.classify(tag, sha, cwd=self.root, **kwargs)

    def test_main_tag_is_production(self):
        result = self.classify('v1.2.3', self.main)
        self.assertEqual(result['channel'], 'prod')
        self.assertEqual(result['image_suffix'], '')
        self.assertEqual(result['prerelease'], 'false')

    def test_dev_tag_is_development(self):
        result = self.classify('dev-v1.2.3', self.dev)
        self.assertEqual(result['channel'], 'dev')
        self.assertEqual(result['image_suffix'], '-dev')

    def test_shared_commit_uses_explicit_tag_prefix(self):
        self.assertEqual(self.classify('dev-v1.2.3', self.main)['channel'], 'dev')

    def test_production_tag_rejects_unmerged_dev_commit(self):
        with self.assertRaises(ValueError):
            self.classify('v1.2.3', self.dev)

    def test_dev_tag_rejects_feature_commit(self):
        with self.assertRaises(ValueError):
            self.classify('dev-v1.2.3', self.feature)

    def test_invalid_tag_is_rejected(self):
        with self.assertRaises(ValueError):
            self.classify('vlatest;echo bad', self.main)

    def test_release_candidate_does_not_promote_latest(self):
        self.assertEqual(self.classify('v1.2.3-rc.1', self.main)['prerelease'], 'true')

    def test_manual_publication_is_branch_scoped(self):
        self.assertEqual(self.classify('dev', self.dev, event='workflow_dispatch', branch='dev')['channel'], 'dev')
        with self.assertRaises(ValueError):
            self.classify('feature', self.feature, event='workflow_dispatch', branch='feature')


if __name__ == '__main__':
    unittest.main()
