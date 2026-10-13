# Migrations

This crate is the SeaORM migrator. Its binary exposes the `migrate` commands of `sea-orm-cli` (`up`, `down`, `status`, `generate`, and the destructive resets). Entity generation (`sea-orm-cli generate entity`) is not part of this binary.

Run every command from the repository root. The migrator reads `DATABASE_URL` from a `.env` file in the current directory, then from the environment. `DATABASE_SCHEMA` defaults to `public`.

```bash
export DATABASE_URL=postgresql://brigitte:<POSTGRES_PASSWORD>@localhost:5432/mybrigittedb
```

With no subcommand, the binary applies every pending migration (`up`).

```bash
cargo run -p migration
cargo run -p migration -- --help
```

Applied versions are stored in `seaql_migrations` (`version`, `applied_at` as Unix seconds). On Postgres, each migration runs in its own transaction: a failure rolls that migration back. Writing a new migration is described in [docs/db/sea-orm.md](../docs/db/sea-orm.md).

## Commands

```bash
cargo run -p migration -- up
cargo run -p migration -- up -n 1          # apply only the next pending migration

cargo run -p migration -- down             # roll back the last applied migration
cargo run -p migration -- down -n 3        # roll back the last 3

cargo run -p migration -- status

cargo run -p migration -- generate add_teams_archived_at
cargo run -p migration -- generate add_teams_archived_at --local-time
```

`generate` creates `migration/src/mYYYYMMDD_HHMMSS_<name>.rs`. The timestamp is UTC unless `--local-time` is set. Register the new module in `src/lib.rs` after the existing ones: the CLI does not edit that file.

`init` scaffolds a new migration crate. Do not run it here; this crate already exists.

## Flags

These flags apply to every command. `-u` and `-s` override the environment for that run.

```bash
cargo run -p migration -- -v status                 # debug logs
cargo run -p migration -- -u "$DATABASE_URL" up
cargo run -p migration -- -s public status          # PostgreSQL schema, default public
```

## Destructive commands

`fresh`, `refresh`, and `reset` delete data. Use them only on a local disposable database.

```bash
cargo run -p migration -- fresh      # drop all tables, then reapply every migration
cargo run -p migration -- refresh    # roll back every migration, then reapply them
cargo run -p migration -- reset      # roll back every migration and leave the database empty
```
