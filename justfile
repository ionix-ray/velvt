# Vaelvet — single source of dev commands. Run `just` for the menu.

set shell := ["bash", "-cu"]
set positional-arguments

default:
    @just --list --unsorted

# ── Dev ──────────────────────────────────────────────────────────────────────
# Dev server: pick first free port 8080..8100, hot reload, open browser.
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

# Production WASM build — outputs to target/dx/.../release/web/public.
build:
    cd velvet-ui && dx build --release --platform web

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

# Logic coverage — excludes irreducible bootstrap/glue (process entrypoints,
# wasm32 web_sys/DOM bindings) that native `cargo test` cannot execute and
# that Playwright e2e already exercises against the running app.
# Requires: cargo install cargo-llvm-cov (not a crate dep — a dev CLI tool,
# same category as `dx`/`podman`, intentionally not pinned in Cargo.toml).
coverage:
    cargo llvm-cov --workspace --all-targets \
        --ignore-filename-regex '(velvet-ui/src/main\.rs|velvet-ui/src/scroll\.rs|velvet-ui/src/routes/home\.rs|server/src/main\.rs)' \
        --fail-under-lines 90 \
        --summary-only

# Full HTML coverage report (includes excluded files, for inspection).
coverage-html:
    cargo llvm-cov --workspace --all-targets --html --output-dir target/llvm-cov-html
    @echo "→ target/llvm-cov-html/html/index.html"

# Security audit.
audit:
    cargo audit --deny warnings

# Secret scan (history + working tree).
gitleaks:
    gitleaks detect --source . --no-banner

# SBOM (CycloneDX) for both crates.
# Requires: cargo install cargo-cyclonedx.
sbom:
    cargo cyclonedx --all --format json
    @echo "→ velvet-ui/vaelvet-ui.cdx.json"
    @echo "→ server/velvet-server.cdx.json"

# Playwright e2e (Node only inside test-suite/).
e2e:
    cd test-suite/playwright && npm ci && npx playwright test

# Wipe build artifacts.
clean:
    cargo clean
    rm -rf target/dx dist deployment/*
    podman stop velvet 2>/dev/null || true
    podman rm -f velvet 2>/dev/null || true

# ── Container (fast path: pre-built assets) ──────────────────────────────────
# Single command: build WASM → container → run → open browser.
container-up:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Vaelvet Container Build =="

    # 1. Build WASM locally (cargo incremental = fast).
    echo "→ Building WASM..."
    cd velvet-ui && dx build --release --platform web
    cd ..

    # 2. Build Rust server locally (fast, cached).
    echo "→ Building Rust server..."
    cargo build --release -p velvet-server 2>&1 | tail -3

    # 3. Stage deployment folder (static assets + Rust server binary).
    echo "→ Staging deployment/..."
    rm -rf deployment/*
    cp -a target/dx/vaelvet-ui/release/web/public/. deployment/
    cp target/release/velvet-server deployment/

    # 4. Find free port.
    PORT=8080
    while lsof -iTCP:$PORT -sTCP:LISTEN -P -n >/dev/null 2>&1; do
        PORT=$((PORT + 1))
        if [ $PORT -gt 8100 ]; then echo "no free port 8080-8100"; exit 1; fi
    done

    # 5. Build + run container.
    echo "→ Building container..."
    podman build -t localhost/velvet:latest . 2>&1 | grep -E "COMMIT|Successfully"

    echo "→ Starting container on port $PORT..."
    podman stop velvet 2>/dev/null || true
    podman run -d --rm -p ${PORT}:8080 --name velvet \
        --read-only \
        --tmpfs /tmp:rw,noexec,nosuid,size=16m \
        --cap-drop=ALL \
        --security-opt=no-new-privileges \
        --pids-limit=128 \
        --memory=128m \
        --cpus=0.5 \
        localhost/velvet:latest

    sleep 1

    # 6. Verify server responding.
    if curl -s http://localhost:${PORT}/health | grep -q ok; then
        echo "→ Health check OK"
    else
        echo "⚠️  Health check failed"
        exit 1
    fi

    # 6. Open browser for manual validation.
    echo "→ Opening http://localhost:${PORT}/ in browser"
    open "http://localhost:${PORT}/"

    echo ""
    echo "✅ Vaelvet running at http://localhost:${PORT}/"
    echo "   Stop: just container-stop"
    echo "   Logs: podman logs velvet"

# Run existing container image on :8080.
container-run:
    podman run --rm -p 8080:8080 --name velvet \
        --read-only \
        --tmpfs /tmp:rw,noexec,nosuid,size=16m \
        --cap-drop=ALL \
        --security-opt=no-new-privileges \
        --pids-limit=128 \
        --memory=128m \
        --cpus=0.5 \
        localhost/velvet:latest

# Stop running container.
container-stop:
    podman stop velvet 2>/dev/null || true

# Container image size + bundle size.
container-size:
    echo "Image:"
    podman image inspect localhost/velvet:latest --format '{{{{.Size}}}}' \
        | numfmt --to=iec 2>/dev/null || true
    echo ""
    echo "WASM bundle:"
    ls -lh deployment/assets/*.wasm 2>/dev/null || echo "Run 'just container-up' first"
