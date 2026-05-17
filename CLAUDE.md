# CLAUDE.md — Velvet PR Agency Website

This file is auto-loaded by Claude Code. On resume, read: `STATE.md`, top of `TASKS.md`, last 3 entries of `PROGRESS.md`, `memory/MEMORY.md`.

## Stack (Locked)
- Dioxus 0.7.6 (web platform) + dioxus-router 0.7.6
- Rust 1.85+, wasm32-unknown-unknown target
- Build: `dx` CLI. **No pnpm/npm/Node.js**
- CSS: `assets/theme.css` + inline styles. Design tokens in `theme/tokens.rs`
- Testing: `#[test]`, `dioxus::testing`, Playwright E2E

## Hard Rules
- No `unwrap()` / `expect()` / `panic!()` / `todo!()` in shipping code
- No `println!` — use `tracing`
- TDD-first: failing test before implementation
- Conventional Commits: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`, `chore:`
- WASM bundle budget: <1.5MB gzipped
- Lighthouse targets: Perf ≥90, A11y ≥95, Best Practices ≥95, SEO ≥95
- Logic coverage ≥95%

## Build & Dev Commands
```bash
just dev          # Serve on free port 8080-8100, auto-open browser
just build        # Production WASM build
just test         # All Rust tests
just lint         # fmt + clippy
just static       # Generate static output in dist/
just audit        # Security audit
```

## Where Code Lives
| Type | Path |
|------|------|
| App entry + router | `velvet-ui/src/main.rs` |
| Route components | `velvet-ui/src/routes/*.rs` |
| UI components | `velvet-ui/src/components/*/` |
| Shared components | `velvet-ui/src/components/shared/` |
| Design tokens (Rust) | `velvet-ui/src/theme/tokens.rs` |
| Theme CSS | `velvet-ui/assets/theme.css` |
| SEO config | `Dioxus.toml` + per-route meta |
| E2E tests | `test-suite/playwright/specs/` |
| Threat model | `security/threat-model/components/` |

## Pre-merge Checklist
1. `just lint` — green
2. `just test` — green
3. `just audit` — zero critical CVEs
4. WASM bundle <1.5MB
5. TASKS.md updated
6. PROGRESS.md appended
