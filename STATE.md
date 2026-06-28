# STATE.md — Vaelvet · live checkpoint

**Current sprint**: COMPLETE — All sprints (S0-S3) finished.
**Current status**: PRODUCTION READY.
**Last action**: Finalized E2E test suite (222/222 passing) verifying typography constraints (Kalnia Glaze locally hosted, no CDNs), brand colors (`#B52A2A`), and extreme viewport responsiveness (down to 320px). Container build (distroless) is passing cleanly.
**Pre-merge gates** (all green on current HEAD):
  - `just lint` clean (fmt + clippy `-D warnings`)
  - `just test` 43 cargo tests pass
  - `just audit` 0 advisories
  - `just build` ~440 KB gz WASM (budget 1.5 MB)
  - `just e2e` 222/222 pass (chromium, webkit, reduced-motion)
**Next action**: Merge `feature/case-study-pages` and structural layout fixes to `main`, push, and initiate deployment. Observe GitHub Pages / production container deploy workflow.
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
