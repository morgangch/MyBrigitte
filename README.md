# MyBrigitte

Time management project with a Rust backend and web and mobile clients.

## Project structure

- [backend/](backend/): Rust API using Axum. SeaORM is the planned database layer; persistence is not implemented yet.
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

The root Cargo workspace includes `backend/` and shares [Cargo.lock](Cargo.lock).
The environment example describes future database and Microsoft OAuth settings; the current server does not load `.env` or consume those settings.

## Client setup

The client directories are reserved for implementation. Initialize the web client in `web/` and the React Native app in `mobile/`. Commit each npm `package-lock.json` with its `package.json`. CI uses `npm ci` and runs available `lint`, `format:check`, `typecheck`, `test`, and `build` scripts. Test scripts must run once and exit in CI.

For a native Android build, commit the React Native Android project and Gradle wrapper under `mobile/android/`. CI builds a debug APK with Java 17; signed release builds require a separate signing setup.
