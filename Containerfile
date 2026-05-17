# syntax=docker/dockerfile:1.7

# ─── Stage 1: Build WASM ───
FROM rust:1.85-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Install dx CLI
RUN cargo install dioxus-cli --version 0.7.6 --locked

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY velvet-ui/Cargo.toml velvet-ui/Cargo.toml

# Fetch dependencies (cached layer)
RUN cargo fetch || true

COPY . .
RUN dx build --release --package velvet-ui

# ─── Stage 2: Serve with Caddy ───
FROM caddy:2.8-alpine

COPY --from=builder /app/target/dx/velvet-ui/release/web/public /srv

COPY scripts/Caddyfile /etc/caddy/Caddyfile

RUN chmod -R 755 /srv

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -q --spider http://localhost:8080 || exit 1

CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]
