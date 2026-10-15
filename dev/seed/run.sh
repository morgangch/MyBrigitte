#!/bin/sh
set -eu
[ "${APP_ENV:-}" = development ] || { echo 'Seed is development-only.' >&2; exit 1; }
bun /app/dev/seed/export.ts > /tmp/dev-seed.sql
psql --no-psqlrc --set=ON_ERROR_STOP=1 --file=/tmp/dev-seed.sql
echo 'Development fixtures loaded (existing records preserved).'
