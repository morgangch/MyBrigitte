# MyBrigitte workflows

The repository keeps a Rust workspace at its root, with an Axum backend in
[backend/](../../backend/). SeaORM connects to PostgreSQL at startup. Microsoft OAuth is planned.
The [web/](../../web/) client uses React/Vite and demonstration data;
[mobile/](../../mobile/) is reserved for React Native.

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
| [release.yml](release.yml) | Verifies tag ancestry, publishes channel-specific runtime images to GHCR, and creates digest-based GitHub Releases. |
| [mirror.yml](mirror.yml) | Mirrors the repository after a successful release build or on manual runs, using MIRROR_URL and SSH_PRIVATE_KEY. |

Client workflows report placeholders and skip client commands until a package.json
exists. Once initialized, bun ci requires a committed bun.lock. Available
scripts must fail on errors, and test scripts must run once and exit. Missing
scripts are skipped; no client checks are claimed for unimplemented clients.
Strict validation requires both clients to be initialized. The quality gate requires
every job required by the selected profile to succeed; only planned skips pass.

Branch pushes do not trigger standalone checks. Opening or updating a pull request
selects the checks for its target branch, including full checks from dev to main. Manual runs remain
available through all_checks.yml, release.yml, and mirror.yml; component workflows
run through all_checks.yml. A manual mirror run does not run checks first.
To enforce the PR gate, configure branch protection or a ruleset to require pull
requests and the Quality Gate Decision status check before merging into main
(and dev if desired). Workflow configuration alone does not prevent merging.

## Check profiles and caching

| Event | Component checks | Security | Coverage / production builds |
| --- | --- | --- | --- |
| PR targeting dev | Changed components: lint/types and unit tests | RustSec + Bun; Actions CodeQL when workflows change | Skipped |
| PR targeting main (or other branches) | All workspace/client tests including integration and doc tests | Full CodeQL + audits | Enabled |
| Manual checks / release workflow calls | Full checks regardless of branch/tag | Full CodeQL + audits | Enabled |

`scripts/ci-plan.py` computes the profile and validates Quality Gate Decision.
Dev PR file selection uses a two-commit checkout and the event's base SHA. Shared
files, unknown paths or a failed diff select all components. Markdown at the
repository root and docs changes do not compile components. Validation always
runs actionlint and the Python tests; dependency audits always run. A skipped,
failed or cancelled required job cannot produce a green gate.

Rust 1.99.0 keeps cache keys stable and matches the Docker toolchain. Rust caches
include workspace artifacts and remain separate for build/tests, Clippy and LLVM
coverage. The new prefix starts caches that retain workspace artifacts. Cargo
fingerprints invalidate changed source; Cargo.lock/compiler versions contribute
to the keys. Cargo test already builds its targets, so an extra cargo build runs
only when tests are explicitly disabled. Bun's existing download caches remain
keyed by OS, runtime and lockfile. Advisory databases are refreshed on every audit.
A new test runner would add installation overhead here; the current tests take
about a second once compiled, so reducing compilation/scanning work matters more.

## GHCR release channels

Git tags have no branch of their own. The tag convention selects a channel and
`scripts/release-channel.py` verifies that the tagged commit belongs to its
expected remote branch before checks or publication:

| Trigger | Required ancestry | Published runtime images |
| --- | --- | --- |
| `v1.2.3` / `v1.2.3-rc.1` | `origin/main` | `backend`, `web`, `proxy` |
| `dev-v1.2.3` | `origin/dev` | `backend-dev`, `web-dev`, `proxy-dev` |
| Manual run on main/dev | Selected branch | Matching channel, version `sha-<commit>` |

Feature-branch tags, wrong-channel commits and malformed versions are rejected.
A commit present on both branches is disambiguated by the prefix. All packages
live below `ghcr.io/<owner>/<repo>/`; tags include the requested version and full
commit SHA. Stable prod and dev packages each receive their own `latest` alias;
production release candidates do not advance `latest`.

Hosted dev images are optimized runtime images built from dev code, not the
local Vite/Cargo hot-reload images. Both channels pass the full checks and Docker
smoke test before the three image builds are pushed. There is no skip-publication
path that can produce a successful release with missing images.

GitHub Releases on tags contain `release-images.json`, with the three published
GHCR digests and deployment commands; binary/frontend tarballs are no longer the
release output. BuildKit also publishes provenance and SBOM attestations. The
mirror runs after a successful tagged release. Manual publication creates images
but no GitHub Release or mirror push.

Deploy published digests by setting `BACKEND_IMAGE`, `WEB_IMAGE` and
`PROXY_IMAGE` in the environment-specific env file and running `pull`, then
`up -d --no-build --wait` with the same project. PostgreSQL data is preserved.
Configure GHCR read access for private packages on the target. No automatic SSH
server deployment is configured. Watchtower remains absent; migrations and
rollback must be coordinated before enabling automatic container replacement.

## Additional CI security and coverage

This branch adds `security.yml`: RustSec Cargo dependency audit, Bun high/critical
advisory audit and CodeQL security-extended reports for Rust, TS/JS and Actions.
Dependency findings fail CI; CodeQL findings need repository code-scanning merge
rules to enforce severity thresholds. Dependabot checks Cargo, Bun, Docker and
GitHub Actions weekly, grouping version updates by ecosystem and targeting `dev`.
GitHub reads its configuration from the default branch, so promote the config to
`main` to activate this policy. Security results join Quality Gate Decision.

`coverage.yml` runs backend tests under LLVM instrumentation and uploads LCOV,
HTML and a percentage summary. Coverage joins the quality gate but has no arbitrary
percentage threshold while business endpoints remain under construction. The
additional jobs require a real GitHub run; local YAML validation is not a remote
success certificate. Accessibility/browser CI is still pending. Mirroring summary
shell quoting is corrected without changing its target or credentials.

## Local checks

From the [repository root](../../), Docker runs the same gates as the hooks:

```bash
make check-quick
make audit
make check
```

Optional native equivalents for an already installed toolchain:

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

## Registry deployment helper

Download `release-images.json` from a matching release, configure environment
secrets/GHCR read access, then run:

```bash
python3 scripts/deploy-registry.py prod .env release-images.json
python3 scripts/deploy-registry.py dev .env.dev release-images.json
```

The helper validates the channel and digest-pinned image references, rejects
placeholder hosted secrets, serializes deployments on the server, pulls images,
runs migrations and starts Compose with `--no-build --wait`. Development hosting
uses a separate `mybrigitte-dev-hosted` project, runtime images and no fixture seed.
DB volumes are preserved. No SSH target or remote deployment workflow is enabled.
A previous image manifest can be used for application rollback only when the
schema remains compatible; this does not replace backups or reverse migrations.
Tests cover channel separation, missing/duplicate services, image origins and
invalid digests. Run `python3 -m unittest discover -s scripts -p 'test_*.py'`.
