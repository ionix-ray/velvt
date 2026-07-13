# Vaelvet — single source of dev commands. Run `just` for the menu.
# All recipes are POSIX-shell safe (set shell := ["bash", "-cu"]).

set shell := ["/bin/bash", "-cu"]
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

# ── Build ─────────────────────────────────────────────────────────────────────
# Wipe any stale hashed WASM/JS artifacts before a fresh production build.
# Called automatically by every recipe that produces a production WASM.
[private]
clean-wasm:
    rm -rf target/dx/vaelvet-ui/release/web/public

# Production WASM build — outputs to target/dx/.../release/web/public.
# Always cleans stale artifacts first so no old hashed files linger.
build: clean-wasm
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

# ── Container (Internal helpers) ─────────────────────────────────────────────
# Shared deployment staging logic — called by container-up and container-build-fresh.
# Builds WASM + server, cleans old stale artifacts, stages the deployment/ folder.
[private]
_stage-deployment:
    #!/usr/bin/env bash
    set -euo pipefail

    # 1. Clean stale WASM artifacts so no hashed files from a previous build linger.
    echo "→ Cleaning stale WASM artifacts..."
    rm -rf target/dx/vaelvet-ui/release/web/public

    # 2. Build WASM (cargo incremental = fast on subsequent builds).
    echo "→ Building WASM..."
    cd velvet-ui && dx build --release --platform web
    cd ..

    # 3. Build Rust server binary.
    echo "→ Building Rust server..."
    cargo build --release -p velvet-server 2>&1 | tail -5

    # 4. Stage deployment/ (wipe old, copy fresh).
    echo "→ Staging deployment/..."
    rm -rf deployment/*
    mkdir -p deployment
    cp -a target/dx/vaelvet-ui/release/web/public/. deployment/
    cp target/release/velvet-server deployment/

    # 5. Copy SEO/crawler root files.
    cp velvet-ui/assets/robots.txt  deployment/robots.txt
    cp velvet-ui/assets/sitemap.xml deployment/sitemap.xml
    # AI-crawler and human discovery files
    [ -f velvet-ui/assets/llms.txt ]   && cp velvet-ui/assets/llms.txt   deployment/llms.txt   || true
    [ -f velvet-ui/assets/humans.txt ] && cp velvet-ui/assets/humans.txt deployment/humans.txt || true

    echo "→ Deployment staged in deployment/"

# ── Container (fast path: pre-built assets) ──────────────────────────────────
# Single command: build WASM → stage → build container → run → open browser.
container-up:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Vaelvet Container Build ==="

    just _stage-deployment

    # Find free port 8080..8100.
    PORT=8080
    while lsof -iTCP:$PORT -sTCP:LISTEN -P -n >/dev/null 2>&1; do
        PORT=$((PORT + 1))
        if [ $PORT -gt 8100 ]; then echo "no free port 8080-8100"; exit 1; fi
    done

    echo "→ Building container image..."
    podman build -t localhost/velvet:latest . 2>&1 | grep -E "COMMIT|Successfully|STEP"

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

    # Health check.
    if curl -sf http://localhost:${PORT}/health | grep -q ok; then
        echo "→ Health check OK"
    else
        echo "⚠️  Health check failed — check: podman logs velvet"
        exit 1
    fi

    echo "→ Opening http://localhost:${PORT}/ in browser"
    open "http://localhost:${PORT}/"

    echo ""
    echo "✅ Vaelvet running at http://localhost:${PORT}/"
    echo "   Stop: just container-stop"
    echo "   Logs: podman logs velvet"

# Full clean + rebuild from zero. No layer reuse, no stale Cargo.lock.
# Use this for production deploys and CI.
container-build-fresh:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Vaelvet Fresh Build (no cache) ==="

    # Wipe everything — Cargo artifacts, dx output, staging folder.
    cargo clean
    rm -rf target/dx dist deployment/*

    just _stage-deployment

    echo "→ Building container image (no-cache)..."
    podman build --no-cache --pull=always -t localhost/velvet:latest .
    echo "✅ Fresh image built — localhost/velvet:latest"

# Run existing container image on :8080 (foreground, logs visible).
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

# Tag the current :latest image with a semver + git SHA label.
# Usage: just container-tag  (reads version from workspace Cargo.toml)
container-tag:
    #!/usr/bin/env bash
    set -euo pipefail
    VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
    SHA=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
    TAG="${VERSION}-${SHA}"
    podman tag localhost/velvet:latest localhost/velvet:${TAG}
    echo "✅ Tagged: localhost/velvet:${TAG}"

# Container image size + bundle size report.
container-size:
    #!/usr/bin/env bash
    echo "=== Image size ==="
    podman image inspect localhost/velvet:latest \
        | python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0].get('Size','?'), 'bytes')" 2>/dev/null \
        || echo "(run 'just container-up' first)"
    echo ""
    echo "=== WASM bundle ==="
    ls -lh deployment/assets/*.wasm 2>/dev/null || echo "Run 'just container-up' first"
    echo ""
    echo "=== CSS bundle ==="
    ls -lh deployment/assets/*.css 2>/dev/null || echo "Run 'just container-up' first"

# ── CI Pipeline ──────────────────────────────────────────────────────────────
# Full CI pipeline: lint → test → build → container → e2e.
# Run this before merging any branch to main.
ci:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "=== Vaelvet CI Pipeline ==="
    echo ""

    echo "[1/5] Lint (fmt + clippy)..."
    just lint

    echo "[2/5] Unit + integration tests..."
    just test

    echo "[3/5] Production WASM build..."
    just build

    echo "[4/5] Container build (fresh)..."
    just container-build-fresh

    echo "[5/5] E2E Playwright tests..."
    VAELVET_URL=http://localhost:8080 just e2e

    echo ""
    echo "✅ CI pipeline complete — all checks passed."

# Mirror of the GitHub Actions deploy pipeline — run locally before pushing.
# Usage: just ci-local            (full pipeline, includes WASM)
#        just ci-local --skip-wasm (Rust-only changes, faster)
ci-local *args:
    bash scripts/ci-local.sh {{ args }}
