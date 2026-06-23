# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S3 — Case study pages
**Current task**: S3-01, markdown-backed case study pages on branch `feature/case-study-pages` — pending merge to `main` + GitHub Pages deploy
**Last action**: On top of yesterday's S3-01 data layer + sample case studies (commit `4795f78`), shipped a brand-consistency + showcase-grid visual refactor and a CI-build fix in two clean commits:
  - `3e3fc7c chore(ci): include build.rs + case-study docs in container` — `Containerfile` now `COPY`s `velvet-ui/build.rs` and `docs/cse_studies/`; `.containerignore` switched from blanket `docs/` to `docs/*` + `!docs/cse_studies/` so the markdown bundled at compile time via `include_str!` actually reaches the build context.
  - `d9df0aa feat(ui): inline topbar logo, redesign showcase grid` — topbar logo moved from floating-badge to inline-in-bar (`clamp(2.4rem, 4.2vw, 3.25rem)`, mobile 2.35rem); scrollbars switched from hidden to themed-thin so tall panels stay reachable; `footer_panel.rs` + `stacked_nav.rs` now resolve their logo through the shared `brand_mark()` helper (single source of truth, same `velvet-square.png` as topbar/loader/case-header/index.html preload); full rename `.v-masonry__*` → `.v-showcase__*` with new visual treatment (red radial gradient backdrop, 260px min-height per card, gradient media overlay behind text, hover lift); two new brand-consistency Playwright tests; one fixed `loader: hides via clip-path collapse` test that was matched by the pre-mount empty-DOM window on slow engines (webkit / reduced-motion), now asserts on the `.hidden` class directly.
**Next action**: squash-merge `feature/case-study-pages` → `main`, push, GitHub Pages workflow auto-deploys. Awaiting user confirmation before the irreversible push.
**Files touched this session**:
  - `velvet-ui/build.rs`, `velvet-ui/src/case_studies.rs`, `velvet-ui/src/generated_case_studies.rs` (data layer — untouched by the layout fix pass)
  - `velvet-ui/src/components/case_header.rs` (new) — shared brand-image + theme-toggle header for all case-study pages
  - `velvet-ui/src/components/markdown_renderer.rs` (new) — pulldown-cmark GFM → `dangerous_inner_html`
  - `velvet-ui/src/routes/case_study.rs` — two-column detail page (sidebar: Published/Topics/Read time; main: article + back link), `read_time_minutes` helper
  - `velvet-ui/src/routes/case_studies_index.rs` — two-column index/tag-filter page (sidebar: tag filter list; main: results count + grid), `all_distinct_tags` helper
  - `velvet-ui/src/components/cases_panel.rs` — conditional internal-vs-external "View Case Study" link
  - `velvet-ui/src/components/topbar.rs` — `theme_icon_for`/`toggle_theme` promoted to `pub(crate)` so `case_header.rs` can reuse them
  - `velvet-ui/src/config.rs` — `CaseItem.slug: String` (`#[serde(default)]`, non-breaking)
  - `velvet-ui/src/{lib,main}.rs`, `routes/mod.rs`, `components/mod.rs` — module/route wiring
  - `velvet-ui/assets/theme.css` — `.v-case-*` layout/sidebar/breadcrumb/article rule set, 1024px collapse breakpoint
  - `content/site.md` — `slug =` added to the 3 existing `[[cases.items]]`
  - `docs/cse_studies/{technova-full-funnel-growth,luxe-beauty-celebrity-launch,greenfuture-immersive-storytelling}.md` (new)
  - `Cargo.toml`, `velvet-ui/Cargo.toml` — `pulldown-cmark` workspace dependency, `build.rs` wired in
  - `test-suite/playwright/specs/case-studies.spec.ts` (new spec file, 8 tests: nav, frontmatter render, 404, index/filter, brand image, theme toggle, sidebar, responsive stacking)
**Open questions**: none
**Rollback**: `git reset --hard v0.1.0-pre-vaelvet`

## This session's deliverables
- Workspace coverage 25.83% → 70.90% (`cargo llvm-cov --workspace`)
- 79 tests passing, `just lint` clean, zero `unwrap`/`expect`/`panic` outside documented browser-only glue
- SPA anchor routing: panel index ⇄ URL hash, shareable/bookmarkable, no JS beyond existing wasm-bindgen
- Showcase masonry: deterministic, content-length-agnostic card sizing

## S3 deliverables (case study pages)
- `just lint` clean, `just test` 126 tests passing (0 failed)
- `just audit` clean, 0 advisories
- `just build` (release WASM) succeeds; bundle **441 KB gz** (budget 1.5 MB), `theme.css` **11.8 KB gz** (budget 40 KB)
- Playwright e2e against live `dx serve`: **chromium 48/48, webkit 48/48, reduced-motion 48/48** — covers logo unification (topbar / stacked-nav / footer / loader / case-header all use the shared brand mark), responsive grid collapse (3-col → 2-col @ 1024 px → 1-col @ 640 px), masonry-bounded heights, mobile social strip, theme toggle, case-study routing/frontmatter/tag-filter
- Loader test (`loader: hides via clip-path collapse`) fixed: was matching the pre-mount empty-DOM window on slow engines, now asserts on the `.hidden` class directly
