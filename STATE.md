# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S3 — Case study pages
**Current task**: S3-01 merged to `main` (2026-06-23). Pending GitHub Pages deploy verification.
**Last action**: Force-replaced `origin/main` (was `ac48174`, the pre-rebrand-to-Velvt history) with `feature/case-study-pages` head `04332d4` via `--force-with-lease`. Authorized by user after disjoint-history audit. Two pre-merge cleanup commits added:
  - `3b44cf1 style(ui): restore alphabetic import order in stacked_nav` — rustfmt drift caught by `just lint`.
  - `04332d4 chore(deps): bump quinn-proto + memmap2 for RUSTSEC advisories` — quinn-proto 0.11.14→0.11.15 (RUSTSEC-2026-0185), memmap2 0.9.10→0.9.11 (RUSTSEC-2026-0186). `just audit` now clean.
**Pre-merge gates** (all green on `04332d4`):
  - `just lint` clean (fmt + clippy `-D warnings`)
  - `just test` 43 cargo tests pass
  - `just audit` 0 advisories
  - `just build` 441 KB gz WASM (budget 1.5 MB)
  - `just e2e` 144/144 pass (chromium 48, webkit 48, reduced-motion 48)
**Discarded**: `origin/main`'s 11 commits up to `ac48174` (rebrand-to-Velvt, binaryen install, Rust 1.88 bump, gh-pages deploy fixes). The "Velvt" rebrand conflicted with the Vaelvet brand of record; user chose force-replace.
**Next action**: confirm GitHub Pages workflow runs on the new `main`. If the dropped CI fixes (binaryen install, dx out_dir) are still needed, port them as fresh commits on top of `04332d4`.
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
