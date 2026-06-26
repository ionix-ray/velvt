# syntax=docker/dockerfile:1.7
# Vaelvet — CI/CD production container.
# Builds the full site from source inside the container.
# Stage 1: Build Rust workspace (WASM + server) inside Linux.
# Stage 2: Copy binaries + assets to distroless runtime.
#
# Build: podman build -f Containerfile -t localhost/velvet:latest .
# Run:   podman run --rm -p 8080:8080 localhost/velvet:latest

ARG RUST_VERSION=1.88.0
ARG DIOXUS_CLI_VERSION=0.7.6
# Digest-pinned base images: a mutated/republished tag can't silently swap
# what gets built or shipped (supply-chain tag-mutation defense).
ARG RUST_BASE_DIGEST=sha256:9a7159329166b45f453351a077367f501aa3e98378f7e327530e7966a139d05f
ARG DISTROLESS_DIGEST=sha256:b0ae8e989418b458e0f25489bc3be523718938a2b70864cc0f6a00af1ddbd985

# ── Stage 1: Build toolchain ─────────────────────────────────────────────────
# Digest above resolved from rust:${RUST_VERSION}-slim-trixie at pin time.
FROM docker.io/library/rust@${RUST_BASE_DIGEST} AS builder

ARG DIOXUS_CLI_VERSION
ARG TARGETARCH

ENV PATH=/usr/local/cargo/bin:${PATH} \
    CARGO_TERM_COLOR=never \
    CARGO_INCREMENTAL=1

# Install dx CLI, wasm32 target, and binaryen (wasm-opt).
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl binaryen \
 && rm -rf /var/lib/apt/lists/*

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

# Copy dependency manifests first (layer cache hits on dep changes only).
COPY Cargo.toml Cargo.lock rust-toolchain.toml Dioxus.toml ./
COPY velvet-ui/Cargo.toml ./velvet-ui/
COPY server/Cargo.toml ./server/

# Warm cargo registry cache mounts.
RUN mkdir -p velvet-ui/src \
 && printf 'pub fn _stub() {}\n' > velvet-ui/src/lib.rs \
 && printf 'fn main() {}\n' > velvet-ui/src/main.rs \
 && mkdir -p server/src \
 && printf 'pub fn _stub() {}\n' > server/src/lib.rs \
 && printf 'fn main() {}\n' > server/src/main.rs

RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    cargo fetch --locked

# Copy full source and build.
COPY velvet-ui/src ./velvet-ui/src
COPY velvet-ui/build.rs ./velvet-ui/build.rs
COPY velvet-ui/assets ./velvet-ui/assets
COPY velvet-ui/index.html ./velvet-ui/index.html
COPY server/src ./server/src
COPY content ./content
COPY docs/cse_studies ./docs/cse_studies

# Build WASM release + copy output to persistent /out dir.
RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-cargo-git,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=velvet-target,target=/app/target,sharing=locked \
    rm -rf /app/target/dx/vaelvet-ui/release/web/public \
 && cd velvet-ui && dx build --release --platform web \
 && mkdir -p /out \
 && cp -a /app/target/dx/vaelvet-ui/release/web/public/. /out/

# Copy raw assets (images referenced by hardcoded paths in HTML) alongside hashed ones.
RUN mkdir -p /out/assets/images \
 && cp /app/velvet-ui/assets/images/* /out/assets/images/ \
 && cp /app/velvet-ui/assets/images/* /out/assets/ 2>/dev/null || true

# SEO root files served at "/" — robots.txt + sitemap.xml.
RUN cp /app/velvet-ui/assets/robots.txt /out/robots.txt \
 && cp /app/velvet-ui/assets/sitemap.xml /out/sitemap.xml

# Build server binary.
RUN --mount=type=cache,id=velvet-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=velvet-target,target=/app/target,sharing=locked \
    cargo build --release -p velvet-server \
 && cp /app/target/release/velvet-server /out/velvet-server

# ── Stage 3: Runtime (distroless, non-root) ─────────────────────────────────
# Digest above resolved from gcr.io/distroless/cc-debian12:nonroot at pin time.
FROM gcr.io/distroless/cc-debian12@${DISTROLESS_DIGEST}

LABEL org.opencontainers.image.title="Vaelvet" \
      org.opencontainers.image.description="Cinematic PR house — Dioxus WASM + Rust server." \
      org.opencontainers.image.source="https://github.com/velvt/velvet" \
      org.opencontainers.image.licenses="Apache-2.0"

WORKDIR /srv
COPY --from=wasm-builder --chown=nonroot:nonroot /out/ /srv/
COPY --from=wasm-builder --chown=nonroot:nonroot /out/velvet-server /srv/velvet-server

USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/srv/velvet-server"]
