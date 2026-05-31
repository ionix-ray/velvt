# syntax=docker/dockerfile:1.7
# Velvt — multi-stage: build WASM, optimize with wasm-opt,
#           inline JS glue into HTML, serve via static-web-server on localhost.
#
# Build & run:
#   podman build -t velvt-web . && podman run -p 8080:8080 velvt-web
#
# Architecture: arm64 (Apple Silicon) / amd64

ARG DIOXUS_CLI_VERSION=0.7.6

# ── Stage 1: Base — Rust latest + dx-cli via cargo-binstall ─────────────
FROM rust:slim AS dx-base

ARG DIOXUS_CLI_VERSION
ENV CARGO_TERM_COLOR=always \
    CARGO_NET_RETRY=10

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

RUN cargo binstall dioxus-cli --version ${DIOXUS_CLI_VERSION} --no-confirm
RUN rustup target add wasm32-unknown-unknown
RUN dx --version

# ── Stage 2: WASM builder ───────────────────────────────────────────────
FROM dx-base AS wasm-builder

WORKDIR /app

# Cache deps first (separate layer from source)
COPY Cargo.toml Cargo.lock rust-toolchain.toml Dioxus.toml ./
COPY velvet-ui/Cargo.toml ./velvet-ui/
RUN mkdir -p velvet-ui/src && echo 'fn main() {}' > velvet-ui/src/main.rs \
    && cargo fetch --locked

# Full source
COPY content/ ./content/
COPY velvet-ui/src ./velvet-ui/src
COPY velvet-ui/assets ./velvet-ui/assets
COPY velvet-ui/index.html ./velvet-ui/
COPY velvet-ui/tests ./velvet-ui/tests

WORKDIR /app
RUN dx build --release --package vaelvet-ui

# ── Stage 3: WASM optimizer + JS inline ─────────────────────────────────
FROM rust:slim AS wasm-optimizer

RUN apt-get update && apt-get install -y --no-install-recommends binaryen \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /in
COPY --from=wasm-builder /app/target/dx/vaelvet-ui/release/web/public/ /in/
COPY --chmod=755 scripts/inline-js.sh /inline-js.sh

# wasm-opt: aggressive dead-code elimination, strip debug
RUN mkdir -p /out && cp -a /in/. /out/ \
    && find /out -name '*.wasm' -print0 \
       | xargs -0 -I{} wasm-opt -Oz --strip-debug --strip-producers \
                                      --vacuum --dce -o {} {} \
    && du -sh /out

# Inline the Dioxus JS glue into index.html — zero external JS files at runtime.
RUN /inline-js.sh

# ── Stage 4: Production runtime (distroless + static-web-server) ────────
FROM docker.io/joseluisq/static-web-server:2 AS server-source

FROM gcr.io/distroless/static-debian12:nonroot AS runtime

LABEL org.opencontainers.image.title="Velvt" \
      org.opencontainers.image.description="Cinematic PR agency — WASM bundle served distroless, localhost-only." \
      org.opencontainers.image.source="https://github.com/velvt/velvt" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.vendor="Velvt"

WORKDIR /srv

COPY --from=server-source --chown=65532:65532 /static-web-server /usr/local/bin/static-web-server
COPY --from=wasm-optimizer --chown=65532:65532 /out /srv
COPY --chown=65532:65532 sws.toml /srv/sws.toml

USER 65532:65532
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/static-web-server"]
CMD ["--config-file=/srv/sws.toml"]

# ── Stage 5: Development target with hot reload ─────────────────────────
FROM dx-base AS development

WORKDIR /app

EXPOSE 8080

ENTRYPOINT ["dx", "serve", "--host", "0.0.0.0", "--port", "8080", "--hot-reload"]
