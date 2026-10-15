#!/usr/bin/env python3
"""Resolve a tag's channel explicitly; Git tags do not carry a branch name."""
import os
import re
import subprocess


def classify(tag, commit, event='push', branch=None, cwd=None):
    if event == 'workflow_dispatch':
        if branch not in ('main', 'dev'):
            raise ValueError('Manual publication requires the main or dev branch')
        source = branch
        version = f'sha-{commit[:12]}'
        prerelease = source == 'dev'
    else:
        match = re.fullmatch(r'(dev-)?v(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)', tag)
        if not match:
            raise ValueError('Use vX.Y.Z for main or dev-vX.Y.Z for dev')
        source = 'dev' if match[1] else 'main'
        version = tag
        prerelease = source == 'dev' or '-' in match[2]
    ref = f'refs/remotes/origin/{source}'
    result = subprocess.run(['git', 'merge-base', '--is-ancestor', commit, ref], cwd=cwd,
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if result.returncode != 0:
        raise ValueError(f'Tagged commit must belong to origin/{source}')
    return {
        'channel': 'prod' if source == 'main' else 'dev',
        'environment': 'production' if source == 'main' else 'development',
        'image_suffix': '' if source == 'main' else '-dev',
        'source_branch': source,
        'version': version,
        'prerelease': str(prerelease).lower(),
    }


if __name__ == '__main__':
    try:
        values = classify(os.environ['GITHUB_REF_NAME'],
                          subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                          os.environ.get('GITHUB_EVENT_NAME', 'push'),
                          os.environ.get('GITHUB_REF_NAME'))
    except ValueError as error:
        raise SystemExit(str(error)) from error
    values['registry'] = 'ghcr.io/' + os.environ['GITHUB_REPOSITORY'].lower()
    with open(os.environ['GITHUB_OUTPUT'], 'a', encoding='utf-8') as output:
        output.writelines(f'{key}={value}\n' for key, value in values.items())
