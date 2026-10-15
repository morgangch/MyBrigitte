#!/bin/sh
set -eu
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
current=$(git config --get core.hooksPath || true)
if [ -n "$current" ] && [ "$current" != .githooks ]; then
    echo "Existing core.hooksPath=$current; refusing to replace custom hooks." >&2
    exit 1
fi
for hook in pre-commit pre-push; do
    existing=$(git rev-parse --git-path "hooks/$hook")
    if [ -f "$existing" ] && [ "$current" != .githooks ]; then
        echo "Existing $existing; integrate it before enabling repository hooks." >&2
        exit 1
    fi
done
git config --local core.hooksPath .githooks
echo 'Docker hooks enabled locally: pre-commit (quick), pre-push (full checks + audits).'
