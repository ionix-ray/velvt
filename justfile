# Velvet PR Agency — Dev Automation
# Usage: just <command>

# Find free port 8080-8100, kill existing, build WASM, serve & auto-open
dev:
    @bash scripts/bootstrap.sh dev

# Production WASM build (format → clippy → test → build)
build:
    @bash scripts/bootstrap.sh build

# Build + serve + open in Safari + Chrome
serve:
    @bash scripts/bootstrap.sh serve

# Full release pipeline: format → clippy → test → audit → build → serve → open
release:full:
    @bash scripts/bootstrap.sh release:full

# Docker container mode: build → docker → port forward → open browsers
container:
    @bash scripts/bootstrap.sh container

# Run all tests
test:
    cargo test --package velvet-ui

# Lint + format
lint:
    cargo fmt --all && cargo clippy --workspace -- -D warnings

# Security audit
audit:
    cargo audit

# Clean build artifacts
clean:
    dx clean && cargo clean
    @docker rm -f velvet-pr 2>/dev/null || true

# Generate static output for CDN deploy
static:
    dx build --release --package velvet-ui
    @echo "Static output in target/dx/velvet-ui/release/web/public/"

# Dev with hot reload (debug)
dev-debug:
    dx serve --hot-reload

# Build Docker image only
docker-build:
    docker build -t velvet-pr:latest -f Containerfile .

# Stop and remove container
docker-stop:
    docker stop velvet-pr 2>/dev/null || true
    docker rm velvet-pr 2>/dev/null || true

# Show logs from container
docker-logs:
    docker logs -f velvet-pr

# Run Playwright E2E tests (requires running server)
e2e:
    @cd test-suite/playwright && npx playwright test

# Run Playwright E2E tests with UI
e2e-ui:
    @cd test-suite/playwright && npx playwright test --ui

# Quick health check
health:
    @curl -s -o /dev/null -w "HTTP %{http_code}" http://localhost:8080/ || echo "Server not running"

# Show current port
port:
    @lsof -i :8080 -sTCP:LISTEN -P -n 2>/dev/null | grep LISTEN || echo "No process on port 8080"

# Help
.DEFAULT_GOAL := help
help:
    @echo "Velvet PR Agency — Available Commands"
    @echo "======================================"
    @echo ""
    @echo "  just dev            Start dev server with hot reload"
    @echo "  just build          Production WASM build (format + clippy + test + build)"
    @echo "  just serve          Build + serve + open in Safari + Chrome"
    @echo "  just release:full   Full release pipeline with quality gates"
    @echo "  just container      Docker container with port forward + open browsers"
    @echo "  just test           Run all Rust tests"
    @echo "  just lint           Format + clippy"
    @echo "  just audit          Security audit"
    @echo "  just clean          Clean build artifacts"
    @echo "  just static         Generate static output"
    @echo "  just e2e            Run Playwright E2E tests"
    @echo "  just e2e-ui         Run Playwright E2E with UI"
    @echo "  just docker-build   Build Docker image only"
    @echo "  just docker-stop    Stop and remove container"
    @echo "  just docker-logs    Show container logs"
    @echo "  just health         Quick health check"
    @echo "  just port           Show current port status"
    @echo "  just help           Show this help"
