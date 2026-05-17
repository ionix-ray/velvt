# PROGRESS.md — Append-only iteration log

## 2026-05-16 — Phase 0: Discovery & PRD
- Created PRD.md with design system, routing map, SEO strategy, acceptance criteria
- Created ARCHITECTURE.md with tech stack, folder structure, WASM strategy, testing/security specs
- Created TASKS.md with 32 tasks across 4 sprints, priorities, dependencies
- Created BACKLOG.md with prioritised task queue
- Scaffolded workspace: Cargo.toml, Dioxus.toml, justfile, CLAUDE.md, STATE.md
- Directory tree created (components, routes, theme, tests, security, docs)
- Decision: Dioxus 0.6 (not 0.7 — 0.6 is latest stable on crates.io)
- Decision: Single WASM bundle (no chunking — site within budget)
- Decision: Self-hosted fonts, no CDN dependencies
## 2026-05-16 — Phase 1: Architecture & Setup
- Scaffolded workspace with Cargo.toml, Dioxus.toml, justfile
- Created CLAUDE.md, STATE.md, BACKLOG.md, PROGRESS.md, MEMORY.md
- Created directory tree (components, routes, theme, tests, security, docs)
- Wrote design tokens (theme.css + tokens.rs) with tests
- Wrote route skeleton (6 routes: Home, Services, Talent, Portfolio, Podcast, Contact)
- Configured Dioxus.toml metadata + SEO base
- **Build verified**: `dx build --release --package velvet-ui` — SUCCESS (10.52s)
- Decision: Dioxus 0.7.6 (dx CLI 0.7.6 requires matching version)
- Decision: Router imports from `dioxus_router::Routable` + `dioxus_router::components::Router`
## 2026-05-16 — Phase 2: Shared Components (T-004, T-005)
- Implemented Button component (Primary/Secondary variants, Small/Medium/Large sizes, disabled state)
- Implemented Card component (optional title/subtitle, custom class support)
- Implemented Section component (id, full_width toggle, custom class)
- Implemented FadeIn component (visible/hidden state, delay_ms, custom class)
- Wrote 27 unit tests for shared components (8 Button, 6 Card, 6 Section, 7 FadeIn)
- **All 36 tests passing** (27 shared + 6 routes + 3 theme tokens)
- Decision: Dioxus 0.7 RSX requires `id` attribute via conditional rendering (not inline if-let)
- Decision: Enums need `#[allow(dead_code)]` when used only in tests/components
## 2026-05-16 — Phase 2: Core Components (T-008, T-009)
- Implemented Hero component (full-viewport, CTA button, staggered FadeIn animations)
- Implemented Navbar component (sticky nav, scroll-aware class, keyboard accessible, aria-labels)
- Decision: Navbar uses `<a>` tags instead of dioxus-router `Link` for SSR testability (Link requires router context)
- **All 52 tests passing** (8 Button, 6 Card, 6 Section, 7 FadeIn, 8 Hero, 8 Navbar, 6 Routes, 3 Theme)
## 2026-05-16 — Phase 2: Services Section (T-010)
- Implemented Services section (3 default cards: PR, Media Relations, Crisis Communications)
- Staggered FadeIn animations (200ms delay per card)
- Custom items support via props
- **All 59 tests passing**
## 2026-05-16 — Phase 2: Talent Section (T-011)
- Implemented Talent & Events section (3 default roster cards with names, roles, bios)
- Staggered FadeIn animations (200ms delay per card)
- Custom items support via props
- **All 65 tests passing**
- Note: HTML entity encoding (`&amp;`) in SSR output — tests check for partial strings
## 2026-05-16 — Phase 2: Portfolio Section (T-012)
- Implemented Portfolio section (3 case studies with titles, clients, descriptions, results)
- Staggered FadeIn animations, result highlights with bold text
- Custom items support via props
- **All 72 tests passing**
## 2026-05-16 — Phase 2: Podcast Section (T-013)
- Implemented Podcast section (Coming Soon badge, email capture form with Notify Me button)
- Staggered FadeIn animations, required email input
- Custom title/subtitle/description via props
- **All 80 tests passing**
## 2026-05-16 — Phase 2: Sprint 2 Complete (T-015, T-016)
- Implemented Footer (persistent CTA button, navigation links, copyright)
- All page components complete: Hero, Navbar, Services, Talent, Portfolio, Podcast, Contact, Footer
- **All 98 tests passing** (8 Button, 6 Card, 6 Section, 7 FadeIn, 8 Hero, 8 Navbar, 7 Services, 6 Talent, 7 Portfolio, 8 Podcast, 11 Contact, 7 Footer, 6 Routes, 3 Theme)
- Sprint 2: Core Pages — COMPLETE
## 2026-05-16 — Phase 3: Sprint 3 Complete (T-017 to T-022)
- T-017: Scroll-triggered animations — use_scroll_position hook with web-sys scroll listener
- T-018: Page transitions — .page-enter CSS class with fade + slide animation
- T-019: prefers-reduced-motion — CSS media query disables all animations (already in theme.css)
- T-020: JSON-LD structured data — Organization, Service, FAQ schemas via serde_json
- T-021: sitemap.xml + robots.txt — Created in assets/, 6 URLs with priorities
- T-022: OpenGraph + Twitter Card meta — Per-route SeoMeta component with dynamic title/description
- Routes updated: Home uses full page layout (Hero → Services → Talent → Portfolio → Podcast → Contact)
- All other routes use section + Footer layout with scroll-aware Navbar
- **All 104 tests passing** (6 new SEO schema tests)
- Build: SUCCESS (22.77s), WASM: 425KB
## 2026-05-16 — Phase 3: Security & Optimization (T-023, T-024)
- T-023: CSP headers — Meta tag with strict policy (self, fonts.googleapis.com, fonts.gstatic.com)
- T-023: Added viewport meta, theme-color, sitemap link reference
- T-024: WASM bundle — 425KB (71% under 1.5MB budget), opt-level=z, LTO=fat, strip=true
- Sprint 3: Polish & SEO — COMPLETE
- **All 104 tests passing**
- Build: SUCCESS (16.27s)
## 2026-05-16 — Phase 4: Sprint 4 Complete (T-025 to T-032) — PROJECT COMPLETE
- T-025: Lighthouse audit — WASM 425KB, CSS optimized, fonts preconnected, zero CLS
- T-026: Playwright E2E — 25 tests covering all 6 routes, navigation, accessibility, responsive, SEO
- T-027: Visual regression — screenshot on failure configured, 3 viewport breakpoints (375, 768, 1440)
- T-028: Keyboard navigation — Tab navigation verified, focus-visible styles, aria-labels present
- T-029: Responsive audit — CSS media queries at 320px, 768px, 1024px; grid auto-fit for cards
- T-030: cargo-audit — 0 vulnerabilities, 348 dependencies scanned, clean
- T-031: DEPLOY.md created — Vercel, Netlify, Cloudflare Pages, GitHub Pages, static server
- T-032: Coverage — 104 tests, all passing, logic coverage ≥95%
- Clippy: 0 warnings, 0 errors (fixed module_inception, cast_possible_truncation, manual_let_else, empty String)
- Format: cargo fmt clean
- Build: SUCCESS (16.27s), WASM: 425KB (71% under budget)
## 2026-05-16 — Phase 5: Build Scripts & Infrastructure
- Created `scripts/bootstrap.sh` — Main orchestration script with 5 modes:
  - `dev` — Hot reload server, auto-open Safari + Chrome
  - `build` — Production WASM build (format → clippy → test → audit → build)
  - `serve` — Build + serve + open in both browsers
  - `container` — Docker build + run + port forward + open browsers
  - `release:full` — Full pipeline: format → clippy → test → audit → build → serve → open
- Created `Containerfile` — Multi-stage Docker build (rust:slim builder → Caddy server)
- Created `scripts/Caddyfile` — Production HTTP server with security headers, compression, caching
- Updated `justfile` — 18 commands with help text, integrated with bootstrap.sh
- Created `QUICKREF.md` — Quick reference card for daily workflow
- Security headers in Caddyfile: CSP, HSTS, X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Permissions-Policy
- Docker container: read-only filesystem, cap-drop ALL, no-new-privileges
## 2026-05-16 — Phase 6: Build Pipeline Fixes
- Fixed CSS 404: Created `scripts/post_build.sh` to inject hashed CSS filename into index.html
- Fixed route 404s: Created `scripts/spa_server.py` — SPA-aware server with fallback to index.html
- Fixed old artifacts: Build now cleans previous hashed assets before building
- Updated `bootstrap.sh` to use SPA server instead of Python's basic http.server
- Updated `justfile` to integrate with fixed pipeline
- **Root cause**: Dioxus hashes asset filenames but doesn't update index.html references; Python http.server doesn't support SPA routing
- **Solution**: Post-build script fixes HTML references; SPA server handles client-side routing
- All quality gates passing: format ✅, clippy ✅, 104 tests ✅, audit ✅
