# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S3 — Case study pages
**Current task**: S3-01, markdown-backed case study pages on branch `feature/case-study-pages`
**Last action**: Replicated shalgo's `/blog` content-loading logic (TOML frontmatter + `build.rs` codegen + pulldown-cmark GFM rendering) for a new Vaelvet-themed case study feature: `/cases` (index), `/cases/tag/:tag` (filtered), `/cases/:slug` (detail). First pass shipped the data layer but had 3 layout bugs caught in review: (1) brand header was literal text, not the real logo image; (2) dark/light theme never applied on these pages (no `data-theme` wiring outside `Home`); (3) layout didn't match the reference app's two-column (sidebar + main) structure. All three fixed: new shared `components/case_header.rs` owns the real `brand_mark()` image + its own `theme` signal/effect/toggle (mirrors `home.rs`'s mechanism exactly); both pages rebuilt around `.v-case-layout`/`.v-case-layout__sidebar`/`.v-case-layout__main` (sticky sidebar, collapses to stacked column under 1024px) with a breadcrumb, Published/Topics/Read-time sidebar cards on the detail page, and a tag-filter sidebar on the index page — structurally matching shalgo's blog detail/listing pages, restyled entirely with Vaelvet tokens (no Tailwind classes carried over). `CaseItem.slug` (`#[serde(default)]`) keeps the wiring backward-compatible: items with a slug link internally to `/cases/{slug}`; items without one keep the old external `button_link`/`target=_blank` behaviour. 3 sample case studies (TechNova, Luxe Beauty, GreenFuture) mirror the existing `content/site.md` case items.
**Next action**: none — feature complete, gated, and security-reviewed (pass-with-notes, no blockers). Awaiting user review/merge decision. Nothing committed or pushed.
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
- `just lint` clean, `just test` 96 tests passing (0 failed), `just coverage` 91.93%/94.97% lines (gate ≥90%) — every new file at 100% line coverage
- `just audit` clean, 0 advisories
- `just build` (release WASM) succeeds; bundle 328 KB gz (budget 1.5 MB), `theme.css` 11.6 KB gz (budget 40 KB)
- curl smoke test against the built server: `/`, `/cases`, `/cases/:slug`, `/cases/tag/:tag` all 200 via SPA fallback, no server errors logged
- No browser-level visual check performed (no browser-automation tool available this session) — verification is SSR-render-assertion + curl-level routing only; recommend a manual click-through or `just e2e` before merge
