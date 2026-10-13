# MyBrigitte

Time management project with a Rust backend and web and mobile clients.

## Project structure

- [backend/](backend/): Rust API using Axum. SeaORM is the database layer; the API does not query Postgres yet.
- [migration/](migration/): SeaORM migrations. They define the whole Postgres schema (9 tables, 2 views).
- [docs/](docs/): data model, design decisions, SeaORM migration guide, and backend test layout.
- [web/](web/): web client placeholder.
- [mobile/](mobile/): React Native client placeholder.
- [Workflow documentation](.github/workflows/README.md): CI checks and release artifacts.
- [Contributing](CONTRIBUTING.md) and [Code of conduct](CODE_OF_CONDUCT.md).
- [Project specification](time_manager-project.pdf).

Microsoft is the OAuth provider. Authentication is not implemented yet; configuration names are documented in [backend/.env.example](backend/.env.example).

## Backend development

Install stable Rust with the rustfmt and Clippy components, then run from the repository root:

```bash
cargo run -p backend
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The API listens on `http://localhost:3001`. `GET /health` returns its status.
The `/users` routes are demonstration endpoints and do not use a database.

The root Cargo workspace includes `backend/` and `migration/`, and shares [Cargo.lock](Cargo.lock).
The server loads a `.env` file from the current directory and requires `DATABASE_URL`, `PORT`, and `JWT_SECRET` to start. It does not open a database connection.

## Database

Postgres 18 runs as the `postgres` service in [docker-compose.yml](docker-compose.yml). Start it from the repository root:

```bash
docker compose up -d postgres
```

The login, password, and database name are the `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB` variables in that file. Data is kept in the `postgres_data` volume, mounted at `/var/lib/postgresql`.

Schema changes are SeaORM migrations in [migration/](migration/). The migrator reads `DATABASE_URL` from a `.env` file in the current directory, then from the environment. From the repository root, point it at the Compose database (replace the password with `POSTGRES_PASSWORD`):

```bash
export DATABASE_URL=postgresql://brigitte:<POSTGRES_PASSWORD>@localhost:5432/mybrigittedb
cargo run -p migration -- up
```

The URL in [backend/.env.example](backend/.env.example) does not match this Compose service. Use the URL above for local migrations.

Useful commands, still from the repository root:

```bash
cargo run -p migration -- status
cargo run -p migration -- down
cargo run -p migration -- generate <name>
```

`up` applies pending migrations. `down` reverts the last one. `status` lists them. Applied versions are stored in the `seaql_migrations` table. The full command list is in [docs/sea-orm.md](docs/sea-orm.md).

The migrations create the 9 tables and 2 views described in [docs/](docs/). List them with:

```bash
docker compose exec -T postgres psql -U brigitte -d mybrigittedb -c '\d'
```

[docs/schema.sql](docs/schema.sql) is a readable reference only. Do not execute it: every schema change is a new migration. The migration list and conventions are in [docs/sea-orm.md](docs/sea-orm.md).

## Client setup

The client directories are reserved for implementation. Initialize the web client in `web/` and the React Native app in `mobile/`. Commit each bun `package-lock.json` with its `package.json`. CI uses `bun ci` and runs available `lint`, `format:check`, `typecheck`, `test`, and `build` scripts. Test scripts must run once and exit in CI.

For a native Android build, commit the React Native Android project and Gradle wrapper under `mobile/android/`. CI builds a debug APK with Java 17; signed release builds require a separate signing setup.
