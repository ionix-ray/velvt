---
name: code-review-gate
description: TDD-enforced PR review checklist for Velvet. Verifies RED-GREEN-REFACTOR visible in git log, Clippy clean, WASM builds, tests pass, design frozen. Trigger on 'review', 'PR', 'merge', 'checklist', 'gate'.
---

# Code Review Gate — Velvet

Every PR must pass ALL items before merge. No exceptions.

## Automated Gates (CI enforces)

```bash
cargo fmt --all -- --check          # G1: format clean
cargo clippy --workspace -- -D warnings  # G2: zero warnings
cargo test --workspace --all-targets     # G3: all tests pass
dx build --platform web --package vaelvet-ui --release  # G4: WASM builds
cargo audit                         # G5: 0 CRITICAL/HIGH CVEs
```

## Manual Review Checklist

### TDD Verification
- [ ] `test:` commit exists BEFORE `feat:`/`fix:` commit in git log
- [ ] Failing test was confirmed failing (comment or CI failure screenshot)
- [ ] Coverage on changed module: no regression > 2pp

### Design Freeze Verification
- [ ] `velvet-ui/assets/theme.css`: no visual rules added or removed (only compaction allowed)
- [ ] Component `.rsx!` structure unchanged — only Rust-side optimisations
- [ ] Animations: CSS classes still applied the same way
- [ ] No new `<script>` tags or JS files in `dist/`

### Rust Quality
- [ ] No `String` where `Box<str>` fits (TOML-parsed static fields)
- [ ] No `unwrap()`/`expect()` outside `#[cfg(test)]`
- [ ] No `js-sys` imports (use `web-sys` + `wasm-bindgen::JsValue`)
- [ ] `#[serde(default)]` on all new config fields
- [ ] New components use `crate::prelude::*`

### Infrastructure
- [ ] Any `ops/tofu/` change: `tofu validate` passes
- [ ] Any `ops/tofu/` change: `tofu plan` shows expected diff (no surprise destroys)
- [ ] `terraform.tfvars` NOT committed
- [ ] `*.tfstate` NOT committed

### Security
- [ ] No secrets in diff (`git log -p | grep -i "secret\|password\|token\|key"`)
- [ ] CSP headers in `sws.toml` unchanged (or tightened, never relaxed)

## Red Flags — Request Changes Immediately

- Code added without a test commit preceding it
- `js-sys` re-introduced
- `.tfstate` in diff
- CSS visual rule removed
- `unwrap()` outside test code
- Coverage decrease > 2pp

## Cross-References

- `.skills/tdd-discipline/SKILL.md` — TDD cycle
- `.skills/wasm-build-gate/SKILL.md` — WASM gate
- `.skills/testing-strategy/SKILL.md` — coverage targets
- `.skills/tofu-module-registry/SKILL.md` — tofu validation
