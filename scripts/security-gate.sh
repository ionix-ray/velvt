#!/usr/bin/env bash
# Vaelvet security gate - single source of truth for SAST/SCA/Secrets/FS scans
# Must be run from repo root. Fails fast on HIGH/CRITICAL.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/security-reports"
mkdir -p "$REPORT_DIR"

# Pinned tool versions (bump deliberately after review)
CARGO_AUDIT_VER="0.21.2"
CARGO_DENY_VER="0.18.4"
GITLEAKS_VER="8.24.3"
TRIVY_VER="0.57.1"

log() { echo -e "\033[1;34m[SECURITY-GATE]\033[0m $*"; }
ok()  { echo -e "\033[1;32m[OK]\033[0m $*"; }
warn(){ echo -e "\033[1;33m[WARN]\033[0m $*"; }
err() { echo -e "\033[1;31m[FAIL]\033[0m $*"; exit 1; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || err "missing required command: $1"
}

detect_os_arch() {
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  ARCH="$(uname -m)"
  case "$ARCH" in
    x86_64|amd64) ARCH="amd64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) err "unsupported arch: $ARCH" ;;
  esac
  case "$OS" in
    linux|darwin) ;;
    *) err "unsupported os: $OS" ;;
  esac
  echo "$OS/$ARCH"
}

install_rust_bin() {
  local crate="$1" ver="$2" bin="$3"
  if command -v "$bin" >/dev/null 2>&1; then
    ok "$bin already present"
    return
  fi
  log "installing $crate@$ver"
  cargo install "$crate" --version "=$ver" --locked >/dev/null
}

install_gitleaks() {
  if command -v gitleaks >/dev/null 2>&1; then ok "gitleaks already present"; return; fi
  local osarch; osarch="$(detect_os_arch)"
  local os=${osarch%/*} arch=${osarch#*/}
  local tmp; tmp="$(mktemp -d)"
  local url="https://github.com/gitleaks/gitleaks/releases/download/v${GITLEAKS_VER}/gitleaks_${GITLEAKS_VER}_${os}_${arch}.tar.gz"
  log "downloading gitleaks v${GITLEAKS_VER} from official release"
  curl -fsSL "$url" -o "$tmp/gitleaks.tgz"
  tar -xzf "$tmp/gitleaks.tgz" -C "$tmp" gitleaks
  install -m 0755 "$tmp/gitleaks" /usr/local/bin/gitleaks 2>/dev/null || { mkdir -p "$HOME/.local/bin" && install -m 0755 "$tmp/gitleaks" "$HOME/.local/bin/gitleaks" && export PATH="$HOME/.local/bin:$PATH"; }
  rm -rf "$tmp"
}

install_trivy() {
  if command -v trivy >/dev/null 2>&1; then ok "trivy already present"; return; fi
  local osarch; osarch="$(detect_os_arch)"
  local os=${osarch%/*} arch=${osarch#*/}
  local tmp; tmp="$(mktemp -d)"
  local url="https://github.com/aquasecurity/trivy/releases/download/v${TRIVY_VER}/trivy_${TRIVY_VER}_${os}-${arch}.tar.gz"
  log "downloading trivy v${TRIVY_VER} from official release"
  curl -fsSL "$url" -o "$tmp/trivy.tgz"
  tar -xzf "$tmp/trivy.tgz" -C "$tmp" trivy
  install -m 0755 "$tmp/trivy" /usr/local/bin/trivy 2>/dev/null || { mkdir -p "$HOME/.local/bin" && install -m 0755 "$tmp/trivy" "$HOME/.local/bin/trivy" && export PATH="$HOME/.local/bin:$PATH"; }
  rm -rf "$tmp"
}

run_sast() {
  log "SAST: cargo fmt"
  cargo fmt --all -- --check
  log "SAST: cargo clippy (deny warnings)"
  cargo clippy --workspace --all-targets -- -D warnings
  ok "SAST passed"
}

run_deps() {
  log "SCA: cargo audit"
  cargo audit --deny warnings
  log "SCA: cargo deny (licenses/sources/bans/advisories)"
  cargo deny check
  ok "Dependency policy passed"
}

run_secrets() {
  log "Secrets: gitleaks (working tree + history)"
  gitleaks detect --source . --no-banner --log-opts="--all" \
    --report-format sarif --report-path "$REPORT_DIR/gitleaks.sarif" \
    --report-format json  --report-path "$REPORT_DIR/gitleaks.json" 2>&1 | tail -3
  ok "gitleaks scan complete (reports in $REPORT_DIR)"
}

run_trivy_fs() {
  log "FS Scan: trivy (vuln + secret + misconfig) HIGH/CRITICAL"
  trivy fs \
    --scanners vuln,secret,config \
    --severity HIGH,CRITICAL \
    --ignore-unfixed \
    --exit-code 1 \
    --format table \
    --output "$REPORT_DIR/trivy-fs-table.txt" \
    .
  trivy fs \
    --scanners vuln,secret,config \
    --severity HIGH,CRITICAL \
    --ignore-unfixed \
    --format sarif \
    --output "$REPORT_DIR/trivy-fs.sarif" \
    .
  ok "trivy FS scan passed (no unfixable HIGH/CRITICAL)"
}

main() {
  need_cmd cargo
  need_cmd curl
  need_cmd tar
  need_cmd install

  install_rust_bin "cargo-audit" "$CARGO_AUDIT_VER" "cargo-audit"
  install_rust_bin "cargo-deny"   "$CARGO_DENY_VER"   "cargo-deny"
  install_gitleaks
  install_trivy

  run_sast
  run_deps
  run_secrets
  run_trivy_fs

  echo
  ok "ALL SECURITY CHECKS PASSED"
  echo "Reports: $REPORT_DIR"
}

main "$@"
