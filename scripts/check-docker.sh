#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
case "${1:-full}" in
    quick|audit|full) mode=${1:-full} ;;
    *) echo 'Usage: scripts/check-docker.sh [quick|audit|full]' >&2; exit 2 ;;
esac
docker compose version >/dev/null
# Explicit file/env selection prevents loading the app's .env or starting its stack.
compose=(docker compose --env-file /dev/null -p mybrigitte-checks -f docker-compose.checks.yml)
"${compose[@]}" build --quiet checks
# Include unstaged/new source files, exclude ignored files and every .env file.
# Streaming an archive also supports linked worktrees and avoids host file ownership changes.
git ls-files --cached --others --exclude-standard --deduplicate -z |
    while IFS= read -r -d '' file; do
        case "$file" in .env|.env.*|*/.env|*/.env.*) continue ;; esac
        if [[ -f "$file" || -L "$file" ]]; then printf '%s\0' "$file"; fi
    done |
    tar --null --verbatim-files-from -T - -cf - |
    "${compose[@]}" run --rm --no-deps -T checks "$mode"
