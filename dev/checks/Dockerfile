# syntax=docker/dockerfile:1
FROM oven/bun:1.4.3 AS bun
FROM rhysd/actionlint:1.7.12@sha256:b1934ee5f1c509618f2508e6eb47ee0d3520686341fec936f3b79331f9315667 AS actionlint

FROM rust:1.99-bookworm AS audit
RUN apt-get update && apt-get install -y --no-install-recommends cmake \
    && rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build \
    CARGO_TARGET_DIR=/build cargo install --locked --version 0.22.2 cargo-audit

FROM rust:1.99-bookworm
RUN apt-get update && apt-get install -y --no-install-recommends python3 git util-linux shellcheck \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rustfmt clippy
COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
COPY --from=audit /usr/local/cargo/bin/cargo-audit /usr/local/bin/cargo-audit
COPY --from=actionlint /usr/local/bin/actionlint /usr/local/bin/actionlint
COPY entrypoint.sh /usr/local/bin/check-entrypoint
WORKDIR /workspace
ENV CARGO_HOME=/cache/cargo CARGO_TARGET_DIR=/cache/target \
    BUN_INSTALL_CACHE_DIR=/cache/bun PYTHONDONTWRITEBYTECODE=1
# No repository bind mount or Docker socket; only the selected source archive enters.
ENTRYPOINT ["flock", "/cache/checks.lock", "sh", "/usr/local/bin/check-entrypoint"]
