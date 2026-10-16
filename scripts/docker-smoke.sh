#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# These unique projects contain synthetic data only; cleanup deletes their volumes.
suffix="${GITHUB_RUN_ID:-$$}"
dev_project="mybrigitte-smoke-dev-$suffix"
prod_project="mybrigitte-smoke-prod-$suffix"
dev_port="${SMOKE_DEV_PORT:-15173}"
prod_port="${SMOKE_PROD_PORT:-18081}"
dev() { PORT="${SMOKE_DEV_API_PORT:-3901}" WEB_PORT="$dev_port" docker compose -p "$dev_project" --env-file .env.dev.example -f docker-compose.yml -f docker-compose.dev.yml "$@"; }
prod() { PORT="${SMOKE_PROD_API_PORT:-3902}" WEB_PORT="$prod_port" docker compose -p "$prod_project" --env-file .env.example -f docker-compose.yml -f docker-compose.prod.yml "$@"; }
cleanup() {
  result=$?
  if [ "$result" -ne 0 ]; then dev logs; prod logs; fi
  dev down --volumes --remove-orphans
  prod down --volumes --remove-orphans
  exit "$result"
}
trap cleanup EXIT
prod up -d --build --wait --wait-timeout 600
dev up -d --build --wait --wait-timeout 600
for port in "$dev_port" "$prod_port"; do
  curl --fail --silent --show-error "http://127.0.0.1:$port/" >/dev/null
  curl --fail --silent --show-error "http://127.0.0.1:$port/dashboard" >/dev/null
  curl --fail --silent --show-error "http://127.0.0.1:$port/api/health" | python3 -c 'import json,sys; assert json.load(sys.stdin)["status"] == "ok"'
  curl --fail --silent --show-error "http://127.0.0.1:$port/api/users/1" >/dev/null
done
# Seed contains the frontend fixture records and can be replayed without duplicates.
check_seed() {
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM users;')" = 9
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM teams;')" = 6
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM team_members;')" = 9
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM work_sessions;')" = 7
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM work_schedules;')" = 5
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM work_sessions WHERE end_time IS NULL;')" = 1
  test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT sum(extract(epoch FROM end_time - start_time)/60)::integer FROM work_sessions WHERE end_time IS NOT NULL;')" = 1302
}
check_seed
dev run --rm --no-deps seed
check_seed
test "$(prod exec -T postgres psql -U mybrigitte_prod -d mybrigitte_prod -Atc 'SELECT count(*) FROM users;')" = 0
# Production blocks inline scripts; both stacks emit browser security headers.
for port in "$dev_port" "$prod_port"; do
  curl --fail --silent --show-error -D - -o /dev/null "http://127.0.0.1:$port/" | python3 -c 'import sys; h=sys.stdin.read().lower(); assert "content-security-policy:" in h; assert "x-content-type-options: nosniff" in h; assert "x-frame-options: deny" in h'
done
curl --fail --silent --show-error -D - -o /dev/null "http://127.0.0.1:$prod_port/" | python3 -c 'import sys; h=sys.stdin.read(); policy=next(x for x in h.splitlines() if x.lower().startswith("content-security-policy:")); scripts=next(x for x in policy.split(";") if "script-src" in x); assert "unsafe-inline" not in scripts and "unsafe-eval" not in scripts'
# Replacing the API container must not require restarting Nginx.
prod up -d --no-deps --force-recreate --wait backend
curl --fail --silent --show-error --retry 10 --retry-delay 2 --retry-all-errors "http://127.0.0.1:$prod_port/api/health" | python3 -c 'import json,sys; assert json.load(sys.stdin)["status"] == "ok"'
# Both migration jobs completed and schema is present.
dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -v ON_ERROR_STOP=1 -c 'SELECT count(*) FROM seaql_migrations;'
prod exec -T postgres psql -U mybrigitte_prod -d mybrigitte_prod -v ON_ERROR_STOP=1 -c 'SELECT count(*) FROM seaql_migrations;'
# The web has no database network; web/backend/DB have no published ports.
for project in "$dev_project" "$prod_project"; do
  docker inspect "$(docker ps -q --filter "label=com.docker.compose.project=$project" --filter 'label=com.docker.compose.service=web')" | python3 -c 'import json,sys; n=json.load(sys.stdin)[0]["NetworkSettings"]["Networks"]; assert len(n)==1; assert next(iter(n)).endswith("_application")'
  for service in backend postgres web; do
    docker inspect "$(docker ps -q --filter "label=com.docker.compose.project=$project" --filter "label=com.docker.compose.service=$service")" | python3 -c 'import json,sys; assert not json.load(sys.stdin)[0]["HostConfig"]["PortBindings"]'
  done
done
dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -v ON_ERROR_STOP=1 -c 'CREATE TABLE docker_isolation_probe (id integer); INSERT INTO docker_isolation_probe VALUES (1);'
test "$(prod exec -T postgres psql -U mybrigitte_prod -d mybrigitte_prod -Atc "SELECT to_regclass('public.docker_isolation_probe') IS NULL;")" = t
dev down
after_stop=$(curl --fail --silent --show-error "http://127.0.0.1:$prod_port/api/health")
python3 -c 'import json,sys; assert json.loads(sys.argv[1])["status"] == "ok"' "$after_stop"
dev up -d --wait --wait-timeout 600
test "$(dev exec -T postgres psql -U mybrigitte_dev -d mybrigitte_dev -Atc 'SELECT count(*) FROM docker_isolation_probe;')" = 1
echo 'Docker smoke checks passed: builds, proxies, migrations, isolation and persistence.'
