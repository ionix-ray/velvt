# MEMORY.md — auto-managed memory index

## Project Context
- **Name**: Velvet PR Agency Website
- **Stack**: Dioxus 0.7.6 (Rust → WASM), dioxus-router 0.7.6
- **Design**: Editorial minimalism, cinematic luxury, velvet depth
- **Status**: ALL SPRINTS COMPLETE — Production ready

## Design Rationale
- Dioxus 0.7.6 chosen (dx CLI 0.7.6 requires matching dioxus version)
- Single WASM bundle (no chunking) — site is content-heavy, not logic-heavy
- Self-hosted fonts to eliminate CDN dependency and improve privacy
- CSS-only animations (no Three.js) — keeps bundle under 1.5MB
- IntersectionObserver for scroll triggers (native, no JS library)
- Navbar uses `<a>` tags instead of dioxus-router `Link` for SSR testability

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
