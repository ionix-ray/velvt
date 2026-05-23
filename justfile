# Vaelvet — single source of dev commands. Run `just` for the menu.

set shell := ["bash", "-cu"]
set positional-arguments

default:
    @just --list --unsorted

# Dev server: pick first free port in 8080..8100, hot reload, open browser.
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    PORT=8080
    while lsof -iTCP:$PORT -sTCP:LISTEN -P -n >/dev/null 2>&1; do
        PORT=$((PORT + 1))
        if [ $PORT -gt 8100 ]; then echo "no free port 8080-8100"; exit 1; fi
    done
    echo "→ dx serve --port $PORT"
    cd velvet-ui && dx serve --port $PORT --open

# Production WASM build → dist/
build:
    cd velvet-ui && dx build --release

# Static export — `dist/` ready to drop on any CDN.
static: build
    @ls -lh dist/*.wasm 2>/dev/null || true
    @echo "→ static output: dist/"

# All tests (cargo + dioxus-ssr).
test:
    cargo test --workspace --all-targets

# fmt + clippy (deny warnings).
lint:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings

# Auto-fix formatting.
fmt:
    cargo fmt --all

# Security audit (cargo-audit must be installed: `cargo install cargo-audit`).
audit:
    cargo audit --deny warnings

# Playwright e2e (Node only inside test-suite/).
e2e:
    cd test-suite/playwright && npm ci && npx playwright test

# Bundle size report.
size: build
    @ls -lh dist/*.wasm | awk '{print $5, $9}'
    @gzip -c dist/*.wasm 2>/dev/null | wc -c | awk '{printf "wasm gz: %.1f KB\n", $1/1024}'

# Wipe build artifacts.
clean:
    cargo clean
    rm -rf dist
