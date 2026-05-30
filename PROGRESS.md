# PROGRESS.md — Vaelvet · append-only ledger

## 2026-05-21 — Sprint S0: Scaffold (this session)
- Archive: `archive/pre-vaelvet` branch + `v0.1.0-pre-vaelvet` tag created (rollback point)
- Wipe: removed old `velvet-ui/src`, `docs/`, `Cargo.lock`, `target/`, old state files; kept `.git`, `.claude`, `.github`, `.cargo`, `memory/`, `rust-toolchain.toml`
- Brand assets: logo PNG (32 MB) → `logo.jpg` 494 KB @ 1280px; mark `Untitled-1-3.png` (30 MB) → `mark.jpg` 131 KB @ 768px; favicon.png 9 KB @ 64px; logo-nav.jpg 122 KB @ 640px
- Anchors written: `CLAUDE.md` (brand + model policy + stack lock), `README.md` (config-driven content), `STATE.md`, `TASKS.md`, `PROGRESS.md`
- Harness: `.claude/settings.json` with model defaults (Sonnet+Haiku, no Opus), permission allowlist for cargo/dx/just/sips; subagents in `.claude/agents/` pinned to specific models
- Workspace configs: root `Cargo.toml`, `Dioxus.toml`, `justfile`, `Containerfile`, `rust-toolchain.toml` (kept)
- App code: `velvet-ui/Cargo.toml`, `index.html` (preload), `theme/tokens.rs`, `theme/theme.css`, `main.rs`, `config.rs`, components (nav, hero, services, case_studies, manifesto, contact, footer), one route (home)
- Tests: per-component SSR tests; Playwright spec for LCP + reduced-motion
- Memory: project (Vaelvet brand), feedback (model policy, stack lock to Dioxus)

## 2026-05-28 — Container + WASM debug
- Containerfile: 5-stage build (dx-base → wasm-builder → wasm-optimizer → runtime) with static-web-server, no Caddyfile
- Bugfix: `gloo_timers::Timeout` dropped immediately via `let _` at `home.rs:56` — stored in `use_signal` to keep alive
- Bugfix: Dioxus auto-injects `<link rel="preload" crossorigin>` for the main JS — server CORS allowlist had hardcoded ports; changed to `--cors-allow-origins=*`
- Container: 11.3 MB distroless image; WASM 875 KB; verified full site renders (13 panels, loader disappears)

## 2026-05-29 — Build verification + justfile fix
- All 20 tests pass (3 unit + 17 SSR render)
- `just lint` clean (cargo fmt + clippy -D warnings)
- `just build` fixed: runs `dx build --release --package vaelvet-ui` from root
- WASM 887 KB raw / 317 KB gzipped (budget ≤1.5 MB) ✅
- theme.css 28 KB raw (budget ≤40 KB) ✅
- Dev server boots on port 8099, returns HTTP 200

## 2026-05-29 — Container hardening + JS inline + OUT_DIR fix
- **Bugfix**: removed build.rs (used `env!("OUT_DIR")` which broke under `dx` WASM cross-compilation). config.rs now uses `include_str!("../../content/site.toml")` directly
- **New**: `sws.toml` — static-web-server config with all security headers:
  - `strict-transport-security` (2yr preload), `x-frame-options: DENY`, `x-content-type-options: nosniff`
  - `content-security-policy` (self-only, wasm-unsafe-eval, frame-ancestors 'none')
  - `permissions-policy` (camera/mic/geo all denied), `referrer-policy` (strict)
  - `cross-origin-embedder-policy: require-corp`, `cross-origin-opener-policy: same-origin`, `cross-origin-resource-policy: same-origin`
- **New**: `scripts/inline-js.sh` — post-processes the build to inline Dioxus JS glue into `index.html`, eliminating the external JS file
- **New**: `just build` simplified (no dist/ copy)
- Container: **11.5 MB distroless** image; binds `0.0.0.0` inside container but port-mapped via `-p 127.0.0.1:...` for localhost-only access
- Verified all headers via curl; health endpoint returns OK

## 2026-05-29 — Build verification + justfile fix
- All 20 tests pass (3 unit + 17 SSR render)
- `just lint` clean (cargo fmt + clippy -D warnings)
- `just build` fixed: runs `dx build --release --package vaelvet-ui` from root, then copies output to `dist/`
- WASM 887 KB raw / 317 KB gzipped (budget ≤1.5 MB) ✅
- theme.css 28 KB raw (budget ≤40 KB) ✅
- Dev server boots on port 8099, returns HTTP 200
- Updated STATE.md to reflect S1 readiness
