# STATE.md — Vaelvet · live checkpoint

**budget: tokens=80000 agents=4**

**Current sprint**: SPRINT HARDENING & UI ALIGNMENT — Cultural Production Imagery, Glass Stat Grid, WebKit Reveal & >90% Coverage Gate
**Current status**: PRODUCTION READY.
**Last action**: 
- Replaced background animated VELVT heatmap grid with clean 4-column glass stat grid (`.v-stat-grid--four`).
- Added culturally authentic Odia/Indian production assets (`workflow-03-production.webp` with Konark open-stage Odissi shoot set, `studio-07-creators.webp` with authentic smartphone creator in nature).
- Optimized `.v-tile--full` Creator Seeding panoramic framing (< 500px card, clamp 170-235px photo height, `object-position: center 36%`, image positioned above text).
- Solved WebKit scroll-snap reveal timing by using window-level coordinate scrolling.
- Expanded Rust unit test suite across `vaelvet-ui` and `velvet_server` (total 195 passing tests, up from 135).
- Achieved **91.16% line coverage** (exceeds 90% floor), 92.75% function coverage, 93.15% region coverage via `cargo llvm-cov`.
- Verified all 327 Playwright e2e tests passing (100% across Chromium, WebKit, and Reduced Motion).
- Updated local CI script (`./scripts/ci-local.sh`), GitHub Actions PR gate (`.github/workflows/pr-gate.yml`), and all specifications (`.agents/AGENTS.md`, `.agents/skills/cinematic-ui/SKILL.md`, `CLAUDE.md`, `test-suite/coverage-goals.md`).
- Fixed critical lazy-loading violation: `loading: "lazy"` → `loading: "eager"` in `about_aggregated_panel.rs` (team member photos now load eagerly as required by product spec).
- Optimized fonts: changed `@font-face font-display` from `swap` to `optional` across 13 @font-face rules in `theme.css`, eliminating font-display swap behavior that causes FOUC/ FOUT on first paint.
- Integrated self-hosted Metamorphous Google font (weight 400, TTF → woff2), replacing IBM Plex Sans fallback for body copy; CSS variable `--font-sans` updated to `"Metamorphous", "IBM Plex Sans", system-ui, -apple-system, sans-serif`. All brand typography remains: `--font-display` = Cormorant Garamond + Outfit + Georgia serif, `--font-mono` = IBM Plex Sans.

**Pre-merge gates** (all green on current HEAD):
- `cargo fmt --all -- --check` clean
- `cargo clippy --workspace --all-targets -- -D warnings` zero warnings
- `cargo test --workspace --no-fail-fast` 195 tests pass (0 failures)
- `cargo llvm-cov` **91.16% lines**, **92.75% functions**, **93.15% regions** (≥90% floor)
- `test-suite/playwright` **327 tests pass** across `chromium`, `webkit`, and `reduced-motion` (0 failures)
- `cargo audit --deny warnings` 0 CVEs
- `cargo deny check licenses bans advisories` ok
- `dx build --release --platform web` succeeds: **476 KB gz WASM** (budget 1.5 MB), **21.8 KB gz theme.css** (budget 40 KB)
- Dual CSS sync verified: `velvet-ui/assets/theme.css` == `velvet-ui/public/assets/theme.css`

**fanout: n=4 | isolation=worktree | tasks=design:research | design:card-redesign | code:refactor | test:update**

**Files touched this session**:
- `velvet-ui/src/components/about_aggregated_panel.rs` — removed background VELVT heatmap grid, rendered clean glass stat grid, added unit tests
- `velvet-ui/src/components/card_step.rs` — added background image support and unit tests
- `velvet-ui/src/components/topbar.rs`, `cta_panel.rs`, `loader.rs`, `section_dots.rs`, `icons.rs`, `config.rs`, `tokens.rs`, `wordmark.rs` — added unit tests for SSR and logic
- `server/src/handlers.rs`, `server/src/middleware.rs` — added comprehensive server unit tests
- `test-suite/playwright/specs/home.spec.ts` — updated assertions for clean glass stats, authentic studio showcase imagery, and Creator Seeding framing
- `test-suite/playwright/specs/reveal.spec.ts` — WebKit window-level coordinate scroll fix
- `justfile` & `.github/workflows/pr-gate.yml` — synchronized coverage exclusion regex for irreducible browser-only WASM files
- `scripts/ci-local.sh` — added coverage gate check and fixed bash arithmetic precedence
- `.cargo/audit.toml` & `deny.toml` — ignored informational async-fd advisory
- `.agents/AGENTS.md` & `.agents/skills/cinematic-ui/SKILL.md` — recorded permanent agentic guardrails
- `CLAUDE.md` & `test-suite/coverage-goals.md` — updated pre-merge checklists and coverage specs
