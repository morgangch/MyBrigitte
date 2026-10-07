# MyBrigitte workflows

The repository keeps a Rust workspace at its root, with an Axum backend in
[backend/](../../backend/). SeaORM is the planned persistence layer and Microsoft
is the OAuth provider; neither persistence nor authentication is implemented yet.
The [web/](../../web/) and [mobile/](../../mobile/) directories are placeholders;
the mobile client will use React Native.

## Workflow files

| Workflow | Purpose |
| --- | --- |
| [all_checks.yml](all_checks.yml) | Runs repository validation and every component check on pull requests, manual runs, or calls from the release workflow. |
| [repository_validation.yml](repository_validation.yml) | Validates the Cargo workspace and required files; initialized clients must have Bun lockfiles. |
| [backend_code_quality.yml](backend_code_quality.yml) | Runs rustfmt and Clippy with warnings treated as errors. |
| [backend_build_test.yml](backend_build_test.yml) | Builds and tests the Rust workspace with the committed lockfile. |
| [web_code_quality.yml](web_code_quality.yml) | Runs available lint, format:check, and typecheck scripts. |
| [web_build_test.yml](web_build_test.yml) | Runs available test and build scripts. |
| [mobile_code_quality.yml](mobile_code_quality.yml) | Runs available React Native lint, format:check, and typecheck scripts. |
| [mobile_build_test.yml](mobile_build_test.yml) | Runs available test and build scripts; builds a debug APK when the Android Gradle wrapper exists. |
| [release.yml](release.yml) | Checks, packages backend and web, publishes GHCR images, and creates GitHub Releases on version tags; supports manual development or production builds. |
| [mirror.yml](mirror.yml) | Mirrors the repository after a successful release build or on manual runs, using MIRROR_URL and SSH_PRIVATE_KEY. |

Client workflows report placeholders and skip client commands until a package.json
exists. Once initialized, bun ci requires a committed bun.lock. Available
scripts must fail on errors, and test scripts must run once and exit. Missing
scripts are skipped; no client checks are claimed for unimplemented clients.
Strict validation requires both clients to be initialized. The quality gate requires
all component workflows to succeed, including build and test workflows.

Branch pushes do not trigger standalone checks. Opening or updating a pull request
triggers all checks, including pull requests from dev to main. Manual runs remain
available through all_checks.yml, release.yml, and mirror.yml; component workflows
run through all_checks.yml. A manual mirror run does not run checks first.
To enforce the PR gate, configure branch protection or a ruleset to require pull
requests and the Quality Gate Decision status check before merging into main
(and dev if desired). Workflow configuration alone does not prevent merging.

The release workflow packages the backend binary in a tar.gz archive (preserving
executable permissions) and the web dist directory in web-dist.tar.gz. Version
tags beginning with v create a GitHub Release with these assets and generated
release notes. Any Android debug APK produced by the checks is also attached and
identified as a development build requiring Metro. Native iOS builds and signed
Android releases require additional setup.

Docker images for backend and web are published to ghcr.io/<owner>/<repo>/<service>.
Images receive the Git tag, semver tags when applicable, and a commit SHA tag.
Stable production builds update latest; development builds update latest-dev.
Manual runs retain release_type and skip_docker options, and test the Compose
stack before publishing images unless skip_docker is enabled. Manual runs on
branches do not create GitHub Releases. The mirror runs after successful packaging
and publication (allowing explicitly skipped Docker or release jobs). No remote
service deployment is performed.

## Local checks

From the [repository root](../../):

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --workspace --locked
cargo test --workspace --locked
```

From an initialized client directory:

```bash
bun ci
bun run --if-present lint
bun run --if-present format:check
bun run --if-present typecheck
if jq -e '.scripts.test | type == "string"' package.json > /dev/null; then bun run test; fi
bun run --if-present build
```

See [the project README](../../README.md), [contribution guidelines](../../CONTRIBUTING.md),
and [Microsoft configuration names](../../backend/.env.example).
