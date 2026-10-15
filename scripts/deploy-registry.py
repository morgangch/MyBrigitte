#!/usr/bin/env python3
"""Deploy a GHCR release manifest, never rebuild or delete database volumes."""
import fcntl
import json
import os
from pathlib import Path
import re
import subprocess
import sys


def image_environment(manifest, channel):
    if channel not in ('prod', 'dev') or manifest.get('channel') != channel:
        raise ValueError('Release channel must match the requested deployment environment')
    images = manifest.get('images', [])
    if len(images) != 3 or {i.get('service') for i in images} != {'backend', 'web', 'proxy'}:
        raise ValueError('Manifest must contain exactly backend, web and proxy')
    env = {}
    for image in images:
        suffix = '-dev' if channel == 'dev' else ''
        if (image.get('channel') != channel or
                not re.fullmatch(r'ghcr\.io/[a-z0-9_.-]+/[a-z0-9_./-]+', image.get('image', '')) or
                not image['image'].endswith('/' + image['service'] + suffix) or
                not re.fullmatch(r'sha256:[0-9a-f]{64}', image.get('digest', ''))):
            raise ValueError('Invalid registry reference, digest or channel in manifest')
        env[image['service'].upper() + '_IMAGE'] = image['image'] + '@' + image['digest']
    return env


def main():
    if len(sys.argv) != 4:
        raise SystemExit('Usage: deploy-registry.py <prod|dev> <env-file> <release-images.json>')
    channel, env_path, manifest_path = sys.argv[1:]
    env_path = str(Path(env_path).resolve(strict=True))
    manifest = json.loads(Path(manifest_path).read_text())
    try:
        images = image_environment(manifest, channel)
    except ValueError as error:
        raise SystemExit(str(error)) from error
    project = 'mybrigitte-prod' if channel == 'prod' else 'mybrigitte-dev-hosted'
    root = Path(__file__).resolve().parent.parent
    env = {**os.environ, **images}
    compose = ['docker', 'compose', '-p', project, '--env-file', env_path, '-f', str(root / 'docker-compose.yml')]
    # Linux host lock prevents two deployments from racing migrations/startup.
    lock_dir = Path.home() / '.local' / 'state' / 'mybrigitte'
    lock_dir.mkdir(parents=True, exist_ok=True, mode=0o700)
    with (lock_dir / (project + '.lock')).open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        def run(*args):
            subprocess.run(compose + list(args), cwd=root, env=env, check=True)
        config = json.loads(subprocess.check_output(compose + ['config', '--format', 'json'], cwd=root, env=env, text=True))
        database = config['services']['postgres']['environment']['POSTGRES_DB']
        if channel == 'dev' and not database.endswith('_dev'):
            raise SystemExit('Hosted development database name must end in _dev')
        if channel == 'prod' and database.endswith('_dev'):
            raise SystemExit('Production must not use a development database')
        secret = config['services']['backend']['environment']['JWT_SECRET']
        password = config['services']['postgres']['environment']['POSTGRES_PASSWORD']
        if len(secret) < 32 or 'replace_with' in secret or 'replace_with' in password or password == 'local_dev_only':
            raise SystemExit('Configure distinct hosted credentials before deployment')
        run('pull')
        run('up', '-d', '--no-build', 'postgres')
        run('run', '--rm', 'migrate')
        run('up', '-d', '--no-build', '--wait', '--wait-timeout', '120')
        print('Deployment healthy; database volume preserved.')


if __name__ == '__main__':
    main()
