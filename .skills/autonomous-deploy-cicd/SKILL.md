---
name: autonomous-deploy-cicd
description: CI/CD for vaelvet-ui — GitHub Actions workflows (pr-gate, deploy-pages, publish-container), security scans, WASM build gate, container signing. Trigger on 'CI/CD', 'workflow', 'deploy', 'GitHub Actions', 'container push'.
---

# Autonomous Deploy CI/CD — Velvet

Three workflows: `pr-gate.yml` (fast feedback), `deploy-pages.yml` (GitHub Pages), `publish-container.yml` (GHCR).

## Workflow Architecture

| Workflow | Trigger | Duration | Purpose |
|---|---|---|---|
| `pr-gate.yml` | Every PR | ~4 min | fmt + clippy + test + WASM build + audit |
| `deploy-pages.yml` | Push to main | ~8 min | test → WASM build → GitHub Pages |
| `publish-container.yml` | Push to main | ~12 min | Containerfile build → GHCR push |

## pr-gate.yml (must be fast)

```yaml
name: PR Gate
on: [pull_request]
jobs:
  gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { toolchain: "1.88.0", components: "clippy,rustfmt", targets: "wasm32-unknown-unknown" }
      - uses: actions/cache@v4
        with: { path: "~/.cargo\ntarget", key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}" }
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace --all-targets
      - run: cargo install dioxus-cli --version "=0.7.6" --locked
      - run: sudo apt-get install -y binaryen
      - run: dx build --platform web --package vaelvet-ui --release
      - run: cargo install cargo-audit --locked
      - run: cargo audit
```

## Security Gates

```bash
cargo audit                              # CVE deps
cargo deny check licenses bans advisories  # supply chain (add deny.toml)
# gitleaks is pre-commit, not CI for this project (static site)
```

## Container Signing (future)

```bash
cosign sign --keyless --yes ghcr.io/velvt/velvet/velvt-web@sha256:...
```

## Cross-References

- `.skills/wasm-build-gate/SKILL.md` — WASM build step
- `.skills/container-distroless/SKILL.md` — Containerfile
- `.skills/defensive-security/SKILL.md` — security scan tools
