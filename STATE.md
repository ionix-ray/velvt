# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S2 — Production-ready container + UI/UX polish
**Current task**: Container + UI refactor (paused for today, resume tomorrow)
**Last action**: CI/CD Containerfile builds from source; Rust server serves with security headers; Previous button added; raw image assets copied to container; all asset loading verified
**Next action**: Self-hosted fonts, responsive CSS, retro loader animation, contact forms, Playwright E2E tests
**Files touched**:
  - Containerfile (full CI/CD multi-stage build)
  - server/ (new Rust Axum server with security headers)
  - velvet-ui/src/routes/home.rs (Previous button)
  - velvet-ui/src/components/next_hint.rs (direction prop)
  - velvet-ui/assets/theme.css (Previous button positioning)
  - justfile (container-up unified command)
  - README.md (updated commands)
  - PROGRESS.md (session log)
**Open questions**: none
**Rollback**: `git reset --hard v0.1.0-pre-vaelvet`

## Today's deliverables
- Container builds fully from source (no pre-copy) with cache mounts
- Rust server: health check, SPA fallback, CSP, HSTS, X-Frame-Options
- Previous/Next navigation buttons at bottom
- `just container-up` single command: build → container → browser
- All assets verified: JS (41KB), WASM (876KB), CSS (36KB), images (all sizes)
