#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Local CI Pipeline — Velvt PR Agency
# Mirrors the GitHub Actions pipeline exactly.
# Run this before pushing to avoid wasting CI minutes.
#
# Usage:
#   ./scripts/ci-local.sh            # full pipeline
#   ./scripts/ci-local.sh --skip-wasm  # skip WASM build (faster for Rust-only changes)
# ═══════════════════════════════════════════════════════════════════════

set -euo pipefail

# ── Colours ───────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# ── Args ──────────────────────────────────────────────────────────────
SKIP_WASM=false
for arg in "$@"; do
    case "$arg" in
        --skip-wasm) SKIP_WASM=true ;;
        *) echo "Unknown argument: $arg" && exit 1 ;;
    esac
done

# ── Helpers ───────────────────────────────────────────────────────────
CHECK="✅"
CROSS="❌"
INFO="ℹ️ "
JOBS_PASSED=0
JOBS_FAILED=0
START_TIME=$(date +%s)

print_stage() {
    echo ""
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo ""
}

pass() { echo -e "${GREEN}${CHECK} $1 PASSED${NC}"; ((JOBS_PASSED++)); }
fail() { echo -e "${RED}${CROSS} $1 FAILED${NC}"; ((JOBS_FAILED++)); }

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# ── Banner ────────────────────────────────────────────────────────────
echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🎬 VELVT — LOCAL CI PIPELINE${NC}"
echo -e "${PURPLE}  Repo: $REPO_ROOT${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"

# ── Job 1: Security Audit ─────────────────────────────────────────────
print_stage "🔒 JOB 1/5: Security Audit"

echo -e "${BLUE}${INFO} Checking cargo-audit...${NC}"
if ! command -v cargo-audit &>/dev/null; then
    echo -e "${YELLOW}⚠️  Installing cargo-audit...${NC}"
    cargo install cargo-audit --locked
fi
cargo audit --deny warnings && pass "cargo-audit" || { fail "cargo-audit (CVEs found — check above)"; }

if [ -f deny.toml ]; then
    echo -e "${BLUE}${INFO} Running cargo-deny...${NC}"
    if ! command -v cargo-deny &>/dev/null; then
        echo -e "${YELLOW}⚠️  Installing cargo-deny...${NC}"
        cargo install cargo-deny --locked
    fi
    cargo deny check licenses bans advisories && pass "cargo-deny" || fail "cargo-deny"
else
    echo -e "${YELLOW}⚠️  deny.toml not found — skipping cargo-deny${NC}"
fi

# ── Job 2: Code Quality & Tests ───────────────────────────────────────
print_stage "⚙️  JOB 2/5: Code Quality & Tests"

echo -e "${BLUE}${INFO} Checking formatting...${NC}"
cargo fmt --all -- --check && pass "rustfmt" || { fail "rustfmt — run 'just fmt' first"; exit 1; }

echo -e "${BLUE}${INFO} Running clippy (zero warnings)...${NC}"
cargo clippy --workspace --all-targets -- -D warnings && pass "clippy" || { fail "clippy"; exit 1; }

echo -e "${BLUE}${INFO} Running unit tests...${NC}"
cargo test --workspace --lib && pass "unit-tests" || { fail "unit-tests"; exit 1; }

echo -e "${BLUE}${INFO} Running integration tests...${NC}"
cargo test --workspace --test '*' && pass "integration-tests" || { fail "integration-tests"; exit 1; }

echo -e "${BLUE}${INFO} Full test suite...${NC}"
cargo test --workspace --no-fail-fast && pass "full-test-suite" || { fail "full-test-suite"; exit 1; }

# ── Job 3: WASM Build ─────────────────────────────────────────────────
print_stage "📦 JOB 3/5: WASM Build"

if [ "$SKIP_WASM" = "true" ]; then
    echo -e "${YELLOW}⚠️  Skipping WASM build (--skip-wasm)${NC}"
else
    echo -e "${BLUE}${INFO} Checking dx CLI...${NC}"
    if ! command -v dx &>/dev/null; then
        echo -e "${YELLOW}⚠️  Installing dioxus-cli...${NC}"
        cargo install dioxus-cli --locked
    fi

    echo -e "${BLUE}${INFO} Cleaning stale WASM artifacts...${NC}"
    rm -rf target/dx/vaelvet-ui/release/web/public dist

    echo -e "${BLUE}${INFO} Building WASM (release)...${NC}"
    (cd velvet-ui && dx build --release --platform web) \
        && pass "wasm-build" \
        || { fail "wasm-build"; exit 1; }

    echo -e "${BLUE}${INFO} Staging dist/...${NC}"
    mkdir -p dist
    cp -a target/dx/vaelvet-ui/release/web/public/. dist/
    touch dist/.nojekyll
    echo "velvt.live" > dist/CNAME
    cp dist/index.html dist/404.html
    cp velvet-ui/assets/robots.txt  dist/robots.txt  2>/dev/null || true
    cp velvet-ui/assets/sitemap.xml dist/sitemap.xml 2>/dev/null || true
    cp velvet-ui/assets/llms.txt    dist/llms.txt    2>/dev/null || true
    cp velvet-ui/assets/humans.txt  dist/humans.txt  2>/dev/null || true
fi

# ── Job 4: Artifact Verification ──────────────────────────────────────
print_stage "🔍 JOB 4/5: Artifact Verification"

if [ "$SKIP_WASM" = "false" ]; then
    CHECKS_OK=true

    check_file() {
        if test -f "dist/$1"; then
            echo -e "  ${GREEN}${CHECK}${NC} dist/$1"
        else
            echo -e "  ${RED}${CROSS}${NC} dist/$1 MISSING"
            CHECKS_OK=false
        fi
    }

    check_file "index.html"
    check_file "404.html"
    check_file ".nojekyll"
    check_file "CNAME"
    check_file "robots.txt"
    check_file "sitemap.xml"
    check_file "llms.txt"

    if find dist -name "*.rs" -o -name "Cargo.toml" | grep -q .; then
        echo -e "  ${RED}${CROSS}${NC} SOURCE FILES LEAKED INTO dist/"
        CHECKS_OK=false
    else
        echo -e "  ${GREEN}${CHECK}${NC} No source files in dist/"
    fi

    echo ""
    echo -e "${BLUE}${INFO} WASM bundle size:${NC}"
    find dist -name "*.wasm" -exec ls -lh {} \; 2>/dev/null || echo "  (no .wasm found)"

    if [ "$CHECKS_OK" = "true" ]; then
        pass "artifact-verification"
    else
        fail "artifact-verification"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  Skipped (--skip-wasm mode)${NC}"
fi

# ── Job 5: Summary ────────────────────────────────────────────────────
print_stage "🏁 JOB 5/5: Summary"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))
MINS=$((DURATION / 60))
SECS=$((DURATION % 60))

echo ""
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${PURPLE}  🎉 CI PIPELINE COMPLETE${NC}"
echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${GREEN}${CHECK} Jobs Passed: ${JOBS_PASSED}${NC}"
echo -e "  ${RED}${CROSS} Jobs Failed: ${JOBS_FAILED}${NC}"
echo -e "  ${CYAN}⏱  Duration:  ${MINS}m ${SECS}s${NC}"
echo ""

if [ "$JOBS_FAILED" -eq 0 ]; then
    echo -e "${GREEN}✅ ALL CHECKS PASSED — READY TO PUSH${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "  1. ${CYAN}git add -A && git commit -m 'feat: ...'${NC}"
    echo -e "  2. ${CYAN}git push origin main${NC}"
    echo -e "  3. GitHub Actions auto-deploys to ${CYAN}https://velvt.live/${NC}"
    echo ""
    echo -e "  Or trigger manually:"
    echo -e "  ${CYAN}gh workflow run deploy-pages.yml${NC}"
    exit 0
else
    echo -e "${RED}❌ CHECKS FAILED — FIX BEFORE PUSHING${NC}"
    exit 1
fi
