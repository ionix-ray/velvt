---
name: samir-product-philosophy
description: Root principles, hard rules, identity, and boot sequence for every session on the Velvet project. Read FIRST on every fresh session or context resume.
---

# Samir Product Philosophy — Velvet Project

Boot sequence for the Velvet / Vaelvet project. Read BEFORE any other file.

## Identity

- **Author**: `cyfen-code <cyfen-code@users.noreply.github.com>`
- **Project**: Vaelvet — cinematic PR agency site
- **Stack**: Dioxus 0.7 → WASM, single crate `vaelvet-ui`, distroless container
- **Workspace root**: `/Volumes/hex/current-work/velvet/`

## Boot Sequence (every session)

1. Read this file
2. Read `.skills/INDEX.md`
3. Read `STATE.md`
4. Read `PROGRESS.md` (last 3 entries)
5. Run `cargo test --workspace` — confirm green before any edit
6. State current task + next 3 actions

## Hard Rules (non-negotiable)

| # | Rule |
|---|------|
| 1 | Commits as `cyfen-code`. No AI co-author tags. |
| 2 | No hardcoded secrets. `.gitignore` gates. |
| 3 | TDD — failing test before code, every time. |
| 4 | WASM builds must pass before any UI merge: `dx build --platform web`. |
| 5 | Config-driven TOML. Zero hardcoded values. `#[serde(default)]` on all fields. |
| 6 | Design, UI, and animations are **frozen** — code optimisation only. |
| 7 | CSS changes: compact dead code only — never remove a visually-active rule. |
| 8 | OpenTofu state: `tofu import` before any `tofu apply`. Never re-provision. |
| 9 | `js-sys` must be removed; only `wasm-bindgen` glue (≤50 LOC JS) is acceptable. |
| 10 | `cargo clippy -- -D warnings` must be clean on every commit. |

## Cross-References

- `.skills/INDEX.md` — skill catalog and triggers
- `.skills/wasm-build-gate/SKILL.md` — WASM compatibility rules
- `.skills/tdd-discipline/SKILL.md` — TDD iron law
- `.skills/code-review-gate/SKILL.md` — PR review checklist
- `.skills/tofu-module-registry/SKILL.md` — Tofu module conventions
