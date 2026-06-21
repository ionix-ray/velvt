# STATE.md — Vaelvet · live checkpoint

**Current sprint**: S2 — Production-ready container + UI/UX polish
**Current task**: Coverage raise + system-design code review (Task #7/#14), URL-anchor SPA routing, showcase card sizing
**Last action**: Workspace test coverage raised 25.83% → 70.90%; server crate lint-inheritance bug fixed (was silently allowing unwrap/expect); dead code removed (`ServicesPanel`, `WorkWithUs`, `velvet_ui::prelude`); SPA panel navigation now reflected in the URL hash (`#home`, `#showcase`, …) via `history.replaceState`, with deep-link-to-panel on load; showcase masonry grid no longer leaves a dangling empty cell on incomplete rows — span class computed in Rust from `items.len()`, not guessed via CSS `nth-child`
**Next action**: `just audit` clean (0 advisories), WASM bundle 337 KB gz (budget 1.5 MB), `theme.css` 7.6 KB gz (budget 40 KB) — pre-merge checklist items 1-4 all green; remaining structural coverage gaps (`home.rs` closures, `scroll.rs`, both `main.rs` entry points) accepted as documented floor; Task #8/#14 ready to close
**Files touched this session**:
  - `server/Cargo.toml` (workspace lint/edition inheritance fix)
  - `server/src/{handlers,error,main,lib,middleware}.rs` (unwrap/expect removal, 30 new tests)
  - `server/tests/integration.rs` (new, 8 router-level tests via `tower::oneshot`)
  - `velvet-ui/src/routes/home.rs` (extracted pure nav functions, URL-hash anchors, tests)
  - `velvet-ui/src/components/{cta_panel,topbar,next_hint,stacked_nav,loader,mobile_nav,studio_panel}.rs` (logic extraction + tests; masonry span fix)
  - `velvet-ui/src/components/mod.rs`, `lib.rs` (dead code removal: services_panel, work_with_us, prelude)
  - `velvet-ui/assets/theme.css` (masonry grid rework: aspect-ratio cards, `--wide`/`--full` span modifiers)
**Open questions**: none
**Rollback**: `git reset --hard v0.1.0-pre-vaelvet`

## This session's deliverables
- Workspace coverage 25.83% → 70.90% (`cargo llvm-cov --workspace`)
- 79 tests passing, `just lint` clean, zero `unwrap`/`expect`/`panic` outside documented browser-only glue
- SPA anchor routing: panel index ⇄ URL hash, shareable/bookmarkable, no JS beyond existing wasm-bindgen
- Showcase masonry: deterministic, content-length-agnostic card sizing
