# MyBrigitte

Time management project with an Axum/SeaORM Rust API, React/Vite web client,
SeaORM migrations, and a reserved mobile directory. The backend opens a PostgreSQL
connection on startup. The web dashboard currently uses demonstration data;
its `/api` proxy is ready for API integration. Microsoft OAuth is planned.

## Docker development

Requires Docker Engine with Compose 2.23+ (Compose Watch `sync+restart`).
Run all commands from this directory. The default stack is **production**:
`docker compose up` reads the root `.env` and uses production runtime images.
See the production setup below. Development requires its explicit override.

`make dev` creates `.env.dev` from its example if absent and starts development
with hot reload. `make dev-down` stops it while preserving data.
`make prod` uses the production `.env`; `make prod-down` preserves its data.

Copy the development configuration:

```bash
cp .env.dev.example .env.dev
docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml up --build --watch
```

Open http://localhost:5173. Vite reloads mounted frontend sources; Compose Watch
syncs Rust sources and restarts `cargo run`, recompiling the backend. Changes to
workspace/dependency/migration files rebuild the dev backend. Migration changes
also need the migration job rebuilt and rerun before the backend:

```bash
docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml build migrate
docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml run --rm migrate
```

The first backend build downloads and compiles dependencies. Rust build artifacts,
Cargo registry and frontend dependencies use development-only named volumes.
After changing `web/bun.lock`, rebuild web and refresh its dependency volume:

```bash
docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml build web
docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml run --rm --no-deps web bun install --frozen-lockfile
```

## Disposable development fixtures

The dev override adds a one-shot `seed` service after migrations and before the
backend. It imports [web/src/lib/fakeData.ts](web/src/lib/fakeData.ts) directly at
execution time, with `TZ=Europe/Paris`, and inserts 9 users (including Léa), 5
teams, 8 memberships, Léa's work policy, 5 schedule days and 7 work sessions.
Text IDs such as `user-1` are converted into stable UUIDs using
`md5('mybrigitte-dev:user-1')::uuid`.

```bash
make dev          # migrations, seed, then app with hot reload
make dev-seed     # rebuild and replay fixtures without overwriting existing rows
make dev-reset-db # DELETE the development database, then recreate and seed it
```

`dev-reset-db` stops only the `mybrigitte-dev` project and removes only
`mybrigitte-dev_postgres_data`. Production and development dependency/build
cache volumes remain intact. Fixture dates are relative to the seed execution
date; replay preserves existing records, while reset creates a fresh dataset.
Dev `POSTGRES_DB` must end in `_dev`: the seed rejects other database names
before writing. No seed service is present in the default production stack.

The seed runs in one transaction, with an advisory lock and conflict handling
for replays. Password hashes remain NULL because fakeData contains no passwords;
no genuine Microsoft OAuth identity is fabricated from its display email.
Summary/KPI values and `today_status` are derived display values and are not
inserted into separate tables. The demo RTT day maps to a day with no expected
work because the schema does not store leave categories. The frontend and demo
API still need to be connected to the persisted records.

## Docker production

```bash
# On a fresh checkout only; preserve any existing .env values.
cp .env.example .env
# Replace both example secrets with distinct random URL-safe values first.
docker compose up --build -d --wait
# Subsequent launches can simply use: docker compose up
```

If `.env` already exists (for example from host Rust development), add the
`POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`, `JWT_SECRET` and optional
`WEB_PORT` / `WEB_BIND_ADDRESS` keys from `.env.example` without overwriting your
other values. The `.env` defines `DATABASE_URL` using `$POSTGRES_USER`, `$POSTGRES_PASSWORD`,
`$POSTGRES_HOST`, `$POSTGRES_PORT` and `$POSTGRES_DB`. Compose expands these
references and passes the resulting URL to the backend and migration job.
`PORT` controls the backend listener, healthcheck and proxy upstream; `WEB_PORT`
controls the public proxy port. Docker uses `POSTGRES_HOST=postgres`, while the
host Rust example uses `POSTGRES_HOST=localhost`.
There are two runtime env files: `.env` for production and `.env.dev` for local
development. Their tracked templates are `.env.example` and `.env.dev.example`.
The optional explicit production Compose override uses the same root `.env`:

```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d --build --wait
```

Open http://localhost. For deployment, set `BACKEND_IMAGE`, `WEB_IMAGE` and `PROXY_IMAGE`
to release image digests in `.env`, then use `pull` followed by
`up -d --no-build --wait` with the same project, env and Compose arguments.
The backend release image contains both `backend` and `migration` executables.
Production runs compiled Rust and Nginx static files without source mounts.

Only the dedicated Nginx `proxy` service publishes a port, bound to loopback by default in both environments.
For hosting, put an HTTPS ingress in front and configure its access restrictions;
TLS, OAuth and hosted deployment automation are separate work. `WEB_BIND_ADDRESS`
can change the production bind address. `WEB_PORT` can change either host port.
Dev and prod use distinct project names, credentials, networks and data volumes,
so both can run simultaneously. Keep `.env.dev` and `.env` outside Git and
never reuse production secrets or data in development. Compose isolation on one
host does not isolate host/daemon failures.

## Networking and database lifecycle

Browser requests use relative URLs: `/api/health` forwards to backend `/health`,
and `/api/users` to `/users`. The dedicated proxy forwards `/api` directly to the backend in both stacks,
and other requests to Vite (development) or the static web server (production).
It supports Vite WebSockets and resolves upstreams after container replacement.
Vite also retains its API proxy for development outside Docker. The
browser needs only the web origin. No CORS configuration or Compose DNS names
are needed in browser code. SPA routes fall back to `index.html`.

Proxy, web and backend share `application`; only backend, migrator and PostgreSQL join
the internal `database` network. PostgreSQL, backend and web publish no host ports.
Startup waits for PostgreSQL readiness, then a successful `migration up`, then
backend/web health, then proxy. An unsuccessful migration blocks the application.
Production migrations never reset or seed the database. The dev override
explicitly runs the separate seed job after migrations.

PostgreSQL 18 stores data at `/var/lib/postgresql` in the project-scoped
`postgres_data` volume. Stop a stack with the same arguments and `down`; its data
survives. Never use `down --volumes` for production. Back up before schema changes;
rolling back an image does not roll back the schema. Migration conventions are in
[docs/db/sea-orm.md](docs/db/sea-orm.md), with database references in [docs/db/](docs/db/).
Containers use Docker's `local` logging driver, rotating three 10 MB files per
container. Logs survive restarts but are removed with containers; durable archives
still require an external logging service.

## Registry releases and deployment

Push `vX.Y.Z` for a commit reachable from **main**, or `dev-vX.Y.Z` for a commit
reachable from **dev**. CI rejects mismatched tags, runs checks and Compose smoke
tests, then publishes three images to GHCR. Production packages are `backend`,
`web`, `proxy`; development packages are `backend-dev`, `web-dev`, `proxy-dev`.
The tag prefix makes commits shared by both branches unambiguous. Tagged releases
attach `release-images.json` with immutable image digests.

Set `BACKEND_IMAGE`, `WEB_IMAGE` and `PROXY_IMAGE` to the GHCR references and
immutable digests in the release manifest, then use `docker compose pull` and
`docker compose up -d --no-build --wait` with the environment-specific env file
and project. The release builds and publishes images; it does not configure a
remote server or automatic deployment. See [release details](.github/workflows/README.md#ghcr-release-channels).

## Security and conformity limits

The proxy sets a production CSP that blocks inline scripts and external script
sources, frame embedding protection, nosniff, referrer/permission policies, a 1 MB
request-body limit and API rate limiting. Local dev allows inline scripts and
WebSockets for Vite. Inline styles remain allowed for the current chart/UI
components. React text rendering is used; no raw HTML injection sink was found
in the reviewed application sources. Frontend page behavior is outside this Docker branch.

These measures do **not** make the application production-ready: API routes still
lack authentication/authorization, Microsoft and password/session flows remain
unfinished, and business endpoints/UI integration are incomplete. CSRF protection
must be implemented with the real session design (secure HttpOnly/SameSite cookies,
Origin/CSRF-token checks as appropriate). Keep HTTPS at the hosting ingress;
no secure cookie/session handling is claimed for the current demo. Rotate the
previously hard-coded database password if it was used outside a disposable DB.

The Compose/Docker foundation meets the school requirements for separate services,
production config and persistent DB. The Linear achievements ask about a port for
each service, while MYB-76 explicitly requires only a public reverse proxy: the
implemented configuration follows MYB-76 and keeps other ports internal.
Local Docker logs survive stops/restarts/crashes within rotation limits; container
removal deletes them. Off-container archives/backups remain deployment work.
Keycloak is still absent despite MYB-79/MYB-29, and Watchtower is absent despite
the bonus MYB-78. No complete school/achievement compliance is claimed. Additional security/coverage CI work is separated into the `cicd/security-coverage`
branch. Remote validation and accessibility CI remain to be completed. Requirements were reviewed against the school PDF and
[Linear achievements](https://linear.app/mybrigitte/document/achievements-59d8fc479d26).

## Verification

```bash
bash scripts/docker-smoke.sh
```

This builds and starts both stacks concurrently under unique test project names,
checks web/SPA/API proxies and migrations, checks network/port configuration,
verifies database isolation, and recreates dev to verify persistence while prod
stays available. It uses synthetic example credentials and deletes only its own
test volumes on exit. Override `SMOKE_DEV_PORT` / `SMOKE_PROD_PORT` if needed.
Release CI runs this check before publishing Docker images, including on tags.

## Development outside Docker

Use Rust 1.99 and Bun 1.4.3. Provide a local PostgreSQL instance. Use
[backend/.env.example](backend/.env.example) as a reference and export
`DATABASE_URL`, `PORT` and `JWT_SECRET` for the host process. Preserve the root
production `.env`; explicit process environment values take precedence.
Run migrations before starting the API:

```bash
cargo run --locked -p migration -- up
cargo run --locked -p backend
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cd web
bun ci
bun run dev
```

Vite defaults to proxying `/api` to localhost:3001 outside Docker.
`API_PROXY_TARGET` overrides that server-side target when needed.
The API exposes `/health` and demonstration `/users` routes.

See [workflow documentation](.github/workflows/README.md),
[contributing](CONTRIBUTING.md) and [code of conduct](CODE_OF_CONDUCT.md).
