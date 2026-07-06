# syntax=docker/dockerfile:1.7
# Vaelvet — production container (multi-stage).
#
# Stage 1 (builder)  — toolchain: Rust stable + wasm32 target + binaryen wasm-opt
# Stage 2 (wasm-builder) — compile WASM + server from source, post-opt with wasm-opt
# Stage 3 (runtime)  — distroless/cc-debian12 non-root image; smallest possible attack surface
#
# Build:  podman build -f Containerfile -t localhost/velvet:latest .
# Run:    podman run --rm -p 8080:8080 localhost/velvet:latest
#
# Supply-chain security: all base images are digest-pinned so a mutated/re-pushed
# tag cannot silently change what gets shipped.

ARG RUST_VERSION=1.88.0
ARG DIOXUS_CLI_VERSION=0.7.6
# Digests pinned at: rust:1.88.0-slim-trixie and gcr.io/distroless/cc-debian12:nonroot
ARG RUST_BASE_DIGEST=sha256:9a7159329166b45f453351a077367f501aa3e98378f7e327530e7966a139d05f
ARG DISTROLESS_DIGEST=sha256:b0ae8e989418b458e0f25489bc3be523718938a2b70864cc0f6a00af1ddbd985

# Build-time metadata — stamped into OCI labels.
ARG BUILD_DATE=""
ARG GIT_SHA=""
ARG VERSION="0.2.0"

# ── Stage 1: Toolchain ───────────────────────────────────────────────────────
FROM docker.io/library/rust@${RUST_BASE_DIGEST} AS builder

ARG DIOXUS_CLI_VERSION
ARG TARGETARCH

ENV PATH=/usr/local/cargo/bin:${PATH} \
    CARGO_TERM_COLOR=never \
    # CARGO_INCREMENTAL=0 → deterministic, cache-friendly builds in CI.
    # (Incremental compilation saves disk per-crate on dev machines but
    # produces non-reproducible artifacts in ephemeral CI environments.)
    CARGO_INCREMENTAL=0 \
    # Reduce LLVM codegen threads to 1 so LTO is not split across CGUs.
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

# Install binaryen (wasm-opt) — single-layer, no residual lists.
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl binaryen \
 && rm -rf /var/lib/apt/lists/*

# Install dx CLI + wasm32 target — arch-aware download.
RUN case "${TARGETARCH:-amd64}" in \
        arm64) DX_ARCH="aarch64-unknown-linux-gnu" ;; \
        amd64|*) DX_ARCH="x86_64-unknown-linux-gnu" ;; \
    esac \
 && mkdir -p /usr/local/cargo/bin \
 && curl -fsSL "https://github.com/DioxusLabs/dioxus/releases/download/v${DIOXUS_CLI_VERSION}/dx-${DX_ARCH}.tar.gz" \
         | tar -xz -C /usr/local/cargo/bin \
 && chmod +x /usr/local/cargo/bin/dx \
 && rustup target add wasm32-unknown-unknown

WORKDIR /app

# ── Stage 2: Build workspace ─────────────────────────────────────────────────
FROM builder AS wasm-builder

# Copy manifests first — changes to source won't bust the registry cache.
COPY Cargo.toml Cargo.lock rust-toolchain.toml Dioxus.toml ./
COPY velvet-ui/Cargo.toml ./velvet-ui/
COPY server/Cargo.toml    ./server/

# Stub sources so `cargo fetch` and the dep-resolution layer can be cached
# independently of actual application code.
RUN mkdir -p velvet-ui/src server/src \
 && printf 'pub fn _stub() {}\n' > velvet-ui/src/lib.rs \
 && printf 'fn main() {}\n'      > velvet-ui/src/main.rs \
 && printf 'pub fn _stub() {}\n' > server/src/lib.rs \
 && printf 'fn main() {}\n'      > server/src/main.rs

# Warm the registry cache — only re-runs when Cargo.lock changes.
RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    cargo fetch --locked

# Copy full application source (invalidates the build cache layer above).
COPY velvet-ui/src      ./velvet-ui/src
COPY velvet-ui/build.rs ./velvet-ui/build.rs
COPY velvet-ui/assets   ./velvet-ui/assets
COPY velvet-ui/index.html ./velvet-ui/index.html
COPY server/src         ./server/src
COPY content            ./content
COPY docs/cse_studies   ./docs/cse_studies

# ── WASM production build ────────────────────────────────────────────────────
# RUSTFLAGS:
#   -C debuginfo=0   → strip all debug sections from the WASM binary
#   -C opt-level=z   → size-optimized (already set in [profile.release] but
#                      explicit here so any toolchain profile change is visible)
#   -C target-feature=+bulk-memory,+mutable-globals
#                    → unlock WASM post-MVP features for smaller generated code
RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=velvet-target,target=/app/target,sharing=locked \
    # Remove any stale hashed artifacts from a previous cached run.
    rm -rf /app/target/dx/vaelvet-ui/release/web/public \
 && RUSTFLAGS="-C debuginfo=0 -C opt-level=z -C target-feature=+bulk-memory,+mutable-globals" \
    cd velvet-ui && dx build --release --platform web \
 && mkdir -p /out \
 && cp -a /app/target/dx/vaelvet-ui/release/web/public/. /out/

# Post-opt: run wasm-opt over the generated binary for an extra size reduction
# on top of what `dx build --release` already does via Dioxus.toml [web.wasm_opt].
RUN WASM=$(find /out -name '*.wasm' 2>/dev/null | head -1) \
 && if [ -n "$WASM" ]; then \
      echo "→ Pre-opt WASM:  $(du -sh $WASM | cut -f1)"; \
      wasm-opt -Oz \
               --strip-debug \
               --strip-producers \
               --vacuum \
               --enable-bulk-memory \
               --enable-mutable-globals \
               "$WASM" -o "$WASM"; \
      echo "→ Post-opt WASM: $(du -sh $WASM | cut -f1)"; \
    else \
      echo "WARN: no .wasm found in /out — skipping wasm-opt post-pass"; \
    fi

# Copy raw images (some paths are hardcoded in Rust via `site.hero.logo` etc.)
RUN mkdir -p /out/assets/images \
 && cp /app/velvet-ui/assets/images/* /out/assets/images/ \
 && cp /app/velvet-ui/assets/images/* /out/assets/ 2>/dev/null || true

# SEO / crawler root files — served at "/" by the server.
RUN cp /app/velvet-ui/assets/robots.txt  /out/robots.txt \
 && cp /app/velvet-ui/assets/sitemap.xml /out/sitemap.xml \
 # AI-crawler discovery file (llms.txt) — copy if present.
 && ([ -f /app/velvet-ui/assets/llms.txt ]   && cp /app/velvet-ui/assets/llms.txt   /out/llms.txt   || true) \
 # Human credits file.
 && ([ -f /app/velvet-ui/assets/humans.txt ] && cp /app/velvet-ui/assets/humans.txt /out/humans.txt || true)

# ── Server binary ────────────────────────────────────────────────────────────
RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-target,target=/app/target,sharing=locked \
    cargo build --release -p velvet-server \
 && cp /app/target/release/velvet-server /out/velvet-server

# ── Stage 3: Runtime (distroless, non-root) ──────────────────────────────────
# gcr.io/distroless/cc-debian12:nonroot — no shell, no package manager,
# just the C runtime libraries needed by a statically-linked Rust binary.
FROM gcr.io/distroless/cc-debian12@${DISTROLESS_DIGEST}

ARG BUILD_DATE
ARG GIT_SHA
ARG VERSION

# OCI image labels — discoverable by orchestrators and registries.
LABEL org.opencontainers.image.title="Vaelvet" \
      org.opencontainers.image.description="Cinematic PR house — Dioxus WASM + Rust server." \
      org.opencontainers.image.source="https://github.com/velvt/velvet" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${GIT_SHA}" \
      org.opencontainers.image.vendor="Velvt"

WORKDIR /srv

# Single COPY — the server binary is already inside /out so the redundant
# second COPY that previously existed is removed.
COPY --from=wasm-builder --chown=nonroot:nonroot /out/ /srv/

USER nonroot:nonroot

# Distroless has no shell so HEALTHCHECK CMD cannot use shell builtins.
# We document NONE here and rely on orchestrator-level HTTP probes to /health.
# (A future healthcheck binary could be added as a dedicated scratch stage.)
HEALTHCHECK NONE

# Tokio handles SIGTERM for graceful shutdown.
STOPSIGNAL SIGTERM

EXPOSE 8080
ENTRYPOINT ["/srv/velvet-server"]
