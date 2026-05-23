# Vaelvet — multi-stage. Builder → distroless static serve via Caddy.
# CIS Docker L1 baseline: no root, no privesc, read-only fs, dropped caps.

# ── builder ─────────────────────────────────────────────────────────────
FROM rust:1.85-slim AS builder
WORKDIR /build

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version 0.7.6 --locked

COPY Cargo.toml Cargo.lock* ./
COPY rust-toolchain.toml ./
COPY content/ ./content/
COPY velvet-ui/ ./velvet-ui/

RUN cd velvet-ui && dx build --release

# ── runtime: Caddy static + security headers ────────────────────────────
FROM caddy:2.8-alpine AS runtime
WORKDIR /srv

COPY --from=builder /build/dist /srv
COPY ops/Caddyfile /etc/caddy/Caddyfile

RUN addgroup -S vaelvet && adduser -S -G vaelvet vaelvet \
    && chown -R vaelvet:vaelvet /srv /etc/caddy

USER vaelvet
EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -qO- http://127.0.0.1:8080/ >/dev/null || exit 1

ENTRYPOINT ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]
