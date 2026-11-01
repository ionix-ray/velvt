# MEMORY.md — auto-managed memory index

## Project Context
- **Name**: Velvet PR Agency Website
- **Stack**: Dioxus 0.7.6 (Rust → WASM), dioxus-router 0.7.6
- **Design**: Editorial minimalism, cinematic luxury, velvet depth
- **Status**: ALL SPRINTS COMPLETE — Production ready

## Design Rationale
- Dioxus 0.7.6 chosen (dx CLI 0.7.6 requires matching dioxus version)
- Single WASM bundle (no chunking) — site is content-heavy, not logic-heavy
- Self-hosted fonts (`IBM Plex Sans` and `Kalnia Glaze`) eliminate CDN dependency and improve privacy.
- CSS-only animations (no Three.js) — keeps bundle under 1.5MB
- IntersectionObserver for scroll triggers (native, no JS library)
- Navbar uses `<a>` tags instead of dioxus-router `Link` for SSR testability
- Strict viewport responsiveness enforced from 320px ultra-narrow up to 1440px desktop utilizing CSS `clamp()` and media queries for fluid scaling.

## Rollback Points
- If WASM bundle exceeds 1.5MB: strip unused Dioxus features, enable `opt-level = "z"`
- If Lighthouse Perf <90: defer non-critical JS, optimize images, reduce CSS

## Token Optimization Notes
- All state externalized to markdown files (CLAUDE.md auto-loads)
- TASKS.md is single source of truth for task status
- PROGRESS.md is append-only (never edit past entries)

## Key Decisions
- HTML entity encoding (`&amp;`) in SSR output — tests check for partial strings
- `#[allow(clippy::module_inception)]` on component mod.rs files (Dioxus convention)
- `#[allow(clippy::cast_possible_truncation)]` on enumerate loops (small item counts)
- `document::Link` for CSP meta tag (static site, no server headers)

## Architecture Audit (2026-10-25)
- **Single source for links**: `content/site.md` — `[[footer.socials]]` (X/LinkedIn/Instagram), `[[team.handles]]`, `[[footer.columns]]`, `[contact]`. No other file may hold a social URL (was 4x duplicated incl. hardcoded `social_strip.rs` + `index.html` JSON-LD).
- **No RPC/broker/queue in this repo** — by design. Static delivery only. Don't add messaging infra without a real async workload.
- **Backlog spine**: `docs/ROADMAP.md` (priority P0-P3 + stage S4-S8 + status) → Gherkin AC in `docs/features/*.feature` → sprint rows in `TASKS.md`. Keep all three in sync; PROGRESS.md stays append-only.
- **Known bug catalog** (fix status tracked in ROADMAP): reveal observer class `.in` unconsumed; heatmap `.is-revealed` never applied (opacity:0 forever); nav labels triplicated (home.rs/mobile_nav.rs/site.md); theme.css not synced by `just build` (only `just dev`).
- **Tracked junk removed**: `test-dist/` (18 MB), `test-results/`, `.codegraph/` must stay gitignored.
- **Compliance matrix**: `docs/SECURITY-COMPLIANCE.md` is the OWASP/LLM/CIS source of truth; residual risks carry `S6-xx` IDs.

## Model / Skill Routing
- Server/backend Rust work → `tdd-cycle` + `rust-testing`, model Sonnet 4.6.
- UI/cinematic design → `frontend-design` + `impeccable` + `ui-ux-pro-max` + `high-end-visual-design` (installed under `.agents/skills/`).
- Doc/compliance/bulk writing → Haiku 4.5 or Gemini via `agy` (present at `~/.local/bin/agy`).
- Orientation without burning context: `graft ask` against `graft/` graph (~82K tok saved/pass).
