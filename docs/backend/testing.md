# Tests

All tests run with: `cargo test`

- Run a single test file: `cargo test --test api`
- Run tests matching a name: `cargo test health_check`
- Show `println!` output: `cargo test -- --nocapture`

## Crate structure

```
backend/
├── Cargo.toml
├── src/
│   ├── main.rs     # Binary crate: reads the config and starts the server
│   ├── lib.rs      # Library crate: only declares the modules and exposes `create_app`
│   ├── config.rs   # Environment configuration
│   ├── error.rs    # `ApiError` and its HTTP conversion
│   ├── routes/     # URL -> handler mapping (`create_app` lives in `routes/mod.rs`)
│   ├── handlers/   # HTTP layer: extracts request data, calls services, returns JSON
│   ├── services/   # Business logic, independent from HTTP
│   └── models/     # Data structures (e.g. `User`)
└── tests/
    ├── common/
    │   └── mod.rs      # Shared helpers (e.g. `get_json`)
    ├── health_test.rs  # Integration tests (one file = one independent test crate)
    └── user_test.rs
```

`main.rs` must stay minimal: integration tests can only import the **library** (`lib.rs`), never the binary.

## Integration tests (`tests/` folder)

Test the application from the outside, as a client would (e.g. sending HTTP requests to the router).

- Each file in `tests/` is compiled as a **separate crate**.
- Import the code through the crate name: `use backend::create_app;`
- Only `pub` items of `lib.rs` are accessible.
- No `#[cfg(test)]` nor `mod tests { }` wrapper needed: Cargo only compiles this folder during `cargo test`.
- Shared helpers go in `tests/common/mod.rs` and are imported with `mod common;`.

## Database tests

Tests that need Postgres read `TEST_DATABASE_URL`. When it is unset, they return early and pass, so `cargo test` works without a database. CI does not set it: these tests run locally only.

```bash
docker run -d --rm --name mybrigitte-test-pg -p 127.0.0.1:55432:5432 \
  -e POSTGRES_USER=test -e POSTGRES_PASSWORD=test -e POSTGRES_DB=mybrigitte_test postgres:18
TEST_DATABASE_URL=postgresql://test:test@127.0.0.1:55432/mybrigitte_test cargo test -p backend
docker stop mybrigitte-test-pg
```

- `common::test_db()` connects and applies the migrations once per test binary.
- Start a database test with `let Some(db) = common::test_db().await else { return };`.
- Tests never clean up. Each one creates its own team with a unique name (`common::manager_with_team`, `insert_team`, `insert_user`) and filters on it, so tests can run in parallel on the same database.
- Never point `TEST_DATABASE_URL` at the dev or production database.

## Unit tests (inside `src/`)

Test internal logic (private functions, error conversions...).

- Placed at the bottom of the file of the module they test, inside a `#[cfg(test)] mod tests { }` block.
- `use super::*;` gives access to every item of the parent module, **including private ones**: do not make an item `pub` only to test it.
- `#[cfg(test)]` excludes this code from the production build.

## Dependencies

Crates used only by tests (`tower::ServiceExt`, `http-body-util`...) belong in `[dev-dependencies]` in `Cargo.toml`.
