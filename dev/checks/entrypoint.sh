#!/bin/sh
set -eu
tar -xf - -C /workspace
git init -q /workspace
for client in web mobile; do
    if [ -f "/workspace/$client/package.json" ]; then
        mkdir -p "/cache/$client/node_modules" "/cache/$client/dist"
        ln -s "/cache/$client/node_modules" "/workspace/$client/node_modules"
        ln -s "/cache/$client/dist" "/workspace/$client/dist"
    fi
done
runner=/workspace/scripts/check.py
if [ -f /workspace/.checks-runner/check.py ]; then runner=/workspace/.checks-runner/check.py; fi
exec python3 "$runner" "$@"
