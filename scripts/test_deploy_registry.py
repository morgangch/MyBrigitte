import copy
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('deploy_registry', Path(__file__).with_name('deploy-registry.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class DeploymentManifestTest(unittest.TestCase):
    def manifest(self, channel='prod'):
        suffix = '-dev' if channel == 'dev' else ''
        return {'channel': channel, 'images': [
            {'service': service, 'channel': channel, 'image': f'ghcr.io/example/mybrigitte/{service}{suffix}', 'digest': 'sha256:' + 'a' * 64}
            for service in ('backend', 'web', 'proxy')
        ]}

    def test_production_references_are_digest_pinned(self):
        env = module.image_environment(self.manifest(), 'prod')
        self.assertEqual(set(env), {'BACKEND_IMAGE', 'WEB_IMAGE', 'PROXY_IMAGE'})
        self.assertTrue(all('@sha256:' in value for value in env.values()))

    def test_dev_requires_its_own_packages(self):
        manifest = self.manifest('dev')
        module.image_environment(manifest, 'dev')
        manifest['images'][0]['image'] = 'ghcr.io/example/mybrigitte/backend'
        with self.assertRaises(ValueError):
            module.image_environment(manifest, 'dev')

    def test_reject_channel_mismatch(self):
        with self.assertRaises(ValueError):
            module.image_environment(self.manifest('dev'), 'prod')

    def test_reject_missing_duplicate_or_untrusted_image(self):
        for mutate in ('missing', 'duplicate', 'registry', 'digest'):
            with self.subTest(mutate=mutate):
                manifest = copy.deepcopy(self.manifest())
                if mutate == 'missing':
                    manifest['images'].pop()
                elif mutate == 'duplicate':
                    manifest['images'][0] = manifest['images'][1]
                elif mutate == 'registry':
                    manifest['images'][0]['image'] = 'docker.io/other/backend'
                else:
                    manifest['images'][0]['digest'] = 'latest'
                with self.assertRaises(ValueError):
                    module.image_environment(manifest, 'prod')


if __name__ == '__main__':
    unittest.main()
