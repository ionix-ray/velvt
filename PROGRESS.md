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
