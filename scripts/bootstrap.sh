#!/usr/bin/env bash
# bootstrap.sh — Velvet PR Agency Bootstrap & Dev Server
# Usage: ./scripts/bootstrap.sh [--dev|--build|--serve|--container]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check prerequisites
check_prereqs() {
    local missing=0

    if ! command -v cargo &>/dev/null; then
        log_error "cargo not found. Install Rust: https://rustup.rs"
        missing=1
    fi

    if ! command -v dx &>/dev/null; then
        log_warn "dx CLI not found. Installing..."
        cargo install dioxus-cli --locked
    fi

    if ! command -v wasm32-unknown-unknown &>/dev/null; then
        log_info "Adding wasm32-unknown-unknown target..."
        rustup target add wasm32-unknown-unknown
    fi

    if [ $missing -eq 1 ]; then
        exit 1
    fi

    log_ok "Prerequisites check passed"
}

# Find free port in range 8080-8100
find_free_port() {
    local port=8080
    while lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; do
        port=$((port + 1))
        if [ $port -gt 8100 ]; then
            log_error "No free port in range 8080-8100"
            exit 1
        fi
    done
    echo $port
}

# Kill existing process on port
kill_port() {
    local port=$1
    local pid
    pid=$(lsof -ti :$port 2>/dev/null || true)
    if [ -n "$pid" ]; then
        log_warn "Killing existing process on port $port (PID: $pid)"
        kill -9 $pid 2>/dev/null || true
        sleep 1
    fi
}

# Open URL in Safari and Chrome
open_in_browsers() {
    local url=$1
    log_info "Opening $url in Safari and Chrome..."

    if [[ "$(uname)" == "Darwin" ]]; then
        open -a "Safari" "$url" 2>/dev/null || log_warn "Safari not found"
        open -a "Google Chrome" "$url" 2>/dev/null || log_warn "Chrome not found"
        log_ok "Opened in Safari and Chrome"
    elif command -v xdg-open &>/dev/null; then
        xdg-open "$url" 2>/dev/null || log_warn "xdg-open failed"
        log_ok "Opened in default browser"
    else
        log_warn "No browser opener available. Visit: $url"
    fi
}

# Dev mode: hot reload + auto-open
run_dev() {
    log_info "Starting dev server..."
    check_prereqs

    local port
    port=$(find_free_port)
    kill_port 8080

    log_info "Building in dev mode..."
    dx serve --port $port --hot-reload &
    local dx_pid=$!

    log_info "Waiting for server to be ready..."
    local retries=30
    while [ $retries -gt 0 ]; do
        if curl -s "http://localhost:$port" >/dev/null 2>&1; then
            break
        fi
        sleep 1
        retries=$((retries - 1))
    done

    if [ $retries -eq 0 ]; then
        log_error "Server failed to start"
        kill $dx_pid 2>/dev/null || true
        exit 1
    fi

    log_ok "Dev server ready on http://localhost:$port"
    open_in_browsers "http://localhost:$port"

    log_info "Press Ctrl+C to stop"
    wait $dx_pid
}

# Build mode: production WASM build
run_build() {
    log_info "Starting production build..."
    check_prereqs

    # Clean old build artifacts
    log_info "Cleaning old build artifacts..."
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.wasm
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.js
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.css
    rm -f target/dx/velvet-ui/release/web/public/index.html
    log_ok "Clean complete"

    log_info "Running format check..."
    cargo fmt --all -- --check || { log_error "Format check failed. Run 'cargo fmt --all'"; exit 1; }
    log_ok "Format check passed"

    log_info "Running clippy..."
    cargo clippy --workspace -- -D warnings || { log_error "Clippy failed"; exit 1; }
    log_ok "Clippy passed"

    log_info "Running tests..."
    cargo test --package velvet-ui || { log_error "Tests failed"; exit 1; }
    log_ok "All tests passed"

    log_info "Running security audit..."
    cargo audit || log_warn "Audit found issues (non-blocking)"

    log_info "Building WASM release..."
    dx build --release --package velvet-ui || { log_error "Build failed"; exit 1; }

    log_info "Fixing asset references..."
    bash "$PROJECT_ROOT/scripts/post_build.sh" "$PROJECT_ROOT/target/dx/velvet-ui/release/web/public"

    local wasm_path="target/dx/velvet-ui/release/web/public/assets"
    local wasm_file
    wasm_file=$(ls -1 "$wasm_path"/velvet-ui_bg-*.wasm 2>/dev/null | head -1)

    if [ -n "$wasm_file" ]; then
        local wasm_size
        wasm_size=$(du -h "$wasm_file" | cut -f1)
        log_ok "WASM built: $wasm_size"
    fi

    log_ok "Build complete! Output: target/dx/velvet-ui/release/web/public/"
}

# Serve mode: build + serve + open browsers
run_serve() {
    log_info "Building and serving..."
    run_build

    local port
    port=$(find_free_port)
    kill_port $port

    local build_dir="$PROJECT_ROOT/target/dx/velvet-ui/release/web/public"

    log_info "Serving from $build_dir on port $port..."

    python3 "$PROJECT_ROOT/scripts/spa_server.py" "$build_dir" $port &

    local server_pid=$!

    sleep 2
    log_ok "Server ready on http://localhost:$port"
    open_in_browsers "http://localhost:$port"

    log_info "Press Ctrl+C to stop"
    trap "kill $server_pid 2>/dev/null" EXIT
    wait $server_pid
}

# Container mode: build + Docker + port forward + open
run_container() {
    log_info "Starting container mode..."

    if ! command -v docker &>/dev/null; then
        log_error "Docker not found. Install Docker Desktop: https://docker.com"
        exit 1
    fi

    local port
    port=$(find_free_port)
    kill_port $port

    log_info "Building production WASM..."
    run_build

    log_info "Building Docker image..."
    docker build -t velvet-pr:latest -f Containerfile . || { log_error "Docker build failed"; exit 1; }

    log_info "Starting container on port $port..."
    docker run -d --name velvet-pr \
        -p $port:8080 \
        --read-only \
        --cap-drop ALL \
        --security-opt no-new-privileges:true \
        velvet-pr:latest || {
        log_warn "Container name conflict. Removing old container..."
        docker rm -f velvet-pr 2>/dev/null || true
        docker run -d --name velvet-pr \
            -p $port:8080 \
            --read-only \
            --cap-drop ALL \
            --security-opt no-new-privileges:true \
            velvet-pr:latest
    }

    sleep 3

    if docker ps | grep -q velvet-pr; then
        log_ok "Container running on http://localhost:$port"
        open_in_browsers "http://localhost:$port"
        log_info "Stop container: docker stop velvet-pr && docker rm velvet-pr"
    else
        log_error "Container failed to start"
        docker logs velvet-pr 2>&1 | tail -20
        exit 1
    fi
}

# Full release: build + test + audit + serve + open
run_release_full() {
    log_info "=========================================="
    log_info "  Velvet PR Agency — Full Release Build"
    log_info "=========================================="

    check_prereqs

    log_info "Step 1/6: Format check..."
    cargo fmt --all -- --check || { log_error "Format check failed"; exit 1; }
    log_ok "Format check passed"

    log_info "Step 2/6: Clippy lint..."
    cargo clippy --workspace -- -D warnings || { log_error "Clippy failed"; exit 1; }
    log_ok "Clippy passed"

    log_info "Step 3/6: Tests..."
    cargo test --package velvet-ui || { log_error "Tests failed"; exit 1; }
    log_ok "All tests passed"

    log_info "Step 4/6: Security audit..."
    cargo audit || log_warn "Audit found issues (non-blocking)"
    log_ok "Audit complete"

    log_info "Step 5/6: Production build..."

    # Clean old build artifacts
    log_info "Cleaning old build artifacts..."
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.wasm
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.js
    rm -rf target/dx/velvet-ui/release/web/public/assets/*.css
    rm -f target/dx/velvet-ui/release/web/public/index.html
    log_ok "Clean complete"

    dx build --release --package velvet-ui || { log_error "Build failed"; exit 1; }

    log_info "Fixing asset references..."
    bash "$PROJECT_ROOT/scripts/post_build.sh" "$PROJECT_ROOT/target/dx/velvet-ui/release/web/public"
    log_ok "Production build complete"

    log_info "Step 6/6: Serving and opening browsers..."
    local port
    port=$(find_free_port)
    kill_port $port

    local build_dir="$PROJECT_ROOT/target/dx/velvet-ui/release/web/public"

    python3 "$PROJECT_ROOT/scripts/spa_server.py" "$build_dir" $port &
    local server_pid=$!

    sleep 2
    log_ok "Server ready on http://localhost:$port"

    local wasm_file
    wasm_file=$(ls -1 "$build_dir/wasm"/*.wasm 2>/dev/null | head -1)
    if [ -n "$wasm_file" ]; then
        local wasm_size
        wasm_size=$(du -h "$wasm_file" | cut -f1)
        log_info "WASM size: $wasm_size"
    fi

    open_in_browsers "http://localhost:$port"

    log_info "=========================================="
    log_ok "  Release complete! Press Ctrl+C to stop"
    log_info "=========================================="

    trap "kill $server_pid 2>/dev/null" EXIT
    wait $server_pid
}

# Usage
usage() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  dev          Start dev server with hot reload (default)"
    echo "  build        Production WASM build (no server)"
    echo "  serve        Build + serve + open browsers"
    echo "  container    Build + Docker container + port forward + open browsers"
    echo "  release:full Full pipeline: format → clippy → test → audit → build → serve → open"
    echo "  help         Show this help"
    echo ""
    echo "Examples:"
    echo "  $0                  # Start dev server"
    echo "  $0 build            # Production build only"
    echo "  $0 release:full     # Full release pipeline"
    echo "  $0 container        # Docker container mode"
}

# Main
case "${1:-dev}" in
    dev)
        run_dev
        ;;
    build)
        run_build
        ;;
    serve)
        run_serve
        ;;
    container)
        run_container
        ;;
    release:full)
        run_release_full
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        log_error "Unknown command: $1"
        usage
        exit 1
        ;;
esac
