# Progress Log — Vaelvet Container + UI/UX Refactor

## 2026-06-20

### Completed Today

1. **Server crate lint-inheritance fix** — `server/Cargo.toml` hardcoded
   `edition = "2021"` / `rust-version = "1.75"` and had no `[lints]` table,
   so the workspace's `deny(unwrap_used, expect_used, panic, ...)` lints were
   silently not enforced on shipping server code. Switched to
   `{ workspace = true }`, added `[lints] workspace = true`, removed a dead
   `[profile.release]` block cargo already ignored for non-root packages.
2. **Removed `.unwrap()`/`.expect()`** from `handlers.rs`, `error.rs`,
   `main.rs` now that the lint actually fires — replaced with infallible
   `Response::new` + header-mutation, and proper error/exit-code handling
   in `main()`.
3. **Coverage**: 25.83% → 70.90% workspace-wide (`cargo llvm-cov --workspace`).
   - `server/`: added 30 tests across `handlers`, `error`, `main`, `lib`,
     `middleware`, plus a new `server/tests/integration.rs` (8 router-level
     tests via `tower::ServiceExt::oneshot`, including a path-traversal
     security test).
   - `velvet-ui/`: extracted pure logic out of Dioxus closures into free,
     directly-testable functions (`keyboard_nav_index`, `wheel_nav_index`,
     `scroll_sync_index`, `progress_for` in `home.rs`; `validate_inquiry`,
     `build_mailto` in `cta_panel.rs`; class-builder helpers in `topbar.rs`,
     `next_hint.rs`, `stacked_nav.rs`, `loader.rs`, `mobile_nav.rs`). Added
     direct SSR render tests for `MobileNav`/`Loader` conditional branches.
   - Remaining low-coverage files (`home.rs` 38%, `scroll.rs` 0%, both
     `main.rs` entry points) are structural: Dioxus event-handler/`use_effect`
     closures and the WASM/socket entry points never execute under native
     `cargo test` without a real browser or bound socket. Documented, not
     hidden.
4. **Dead code removal** — `ServicesPanel`, `WorkWithUs` components (never
   rendered, confirmed via grep) and `velvet_ui::prelude` module (unused;
   its `window()` helper's doc comment falsely claimed `web_sys::window()`
   "returns `None` outside WASM" — it actually **panics** on native targets,
   confirmed by hitting that exact panic this session. Removed rather than
   fixed, since nothing used it).
5. **SPA anchor routing** — `home.rs` now maps each panel index to a URL
   hash (`#home`, `#about`, `#stories`, `#showcase`, `#cases`, `#contact`,
   `#footer`) via `history.replaceState` (no page jump/reload), updated from
   every navigation path (spindle clicks, keyboard, wheel, manual scroll).
   Loading the page with a hash present jumps to that panel on first paint.
6. **Showcase masonry fix** — the 5-item grid in a 3-column layout was
   leaving an empty trailing cell, and card heights were set by guessing
   `nth-child(2)`/`nth-child(5)`. Replaced with `aspect-ratio`-based uniform
   cards and a Rust-computed `--wide`/`--full` span modifier on the last
   item of an incomplete row, so the layout self-corrects if the item count
   in `content/site.md` ever changes.

### Verified
- `just lint` (fmt + clippy -D warnings): clean
- `cargo test --workspace`: 79 passed, 0 failed
- `cargo llvm-cov --workspace --summary-only`: 70.90% lines

### To Do — Next Session
- `just audit`, WASM bundle size re-check after this session's changes
- Decide whether to invest in `wasm-bindgen-test`/headless-browser coverage
  for the documented structural gaps, or accept them as the floor
- Close Task #8 (final pre-merge checklist) and #14 (code review write-up)

## 2026-06-15

### ✅ Completed Today

1. **CI/CD Containerfile** — builds from source inside container (no pre-copy)
   - Multi-stage: Builder (Rust + dx CLI) → Wasm-builder → Distroless runtime
   - Cache mounts for cargo registry, git, and target (incremental)
   - Clean old dx output before each build to avoid stale hashes
   - Copies raw image assets alongside hashed ones (DX flattens paths)

2. **Production Rust server** (`server/`)
   - Axum + Tokio static file server
   - Health check endpoint `/health`
   - SPA fallback (index.html for non-file routes)
   - Security headers: CSP, HSTS, X-Frame-Options, X-Content-Type-Options
   - CSP allows Google Fonts: `font-src 'self' https://fonts.gstatic.com`
   - Content-Type detection for all assets
   - Immutable asset caching for hashed bundles
   - Error handling with typed `ServerError`
   - Config-driven (addr, static-root via CLI flags)

3. **Previous button** — added alongside Next button
   - `NextHint` component now accepts `label`, `direction` props
   - CSS: `.v-next-hint--left` and `.v-next-hint--right` positioned at bottom
   - Keyboard navigation already supported (ArrowLeft/ArrowUp)

4. **`just container-up`** — single command pipeline
   - Builds WASM locally (fast incremental)
   - Builds Rust server locally
   - Stages deployment/ folder
   - Builds container
   - Runs on first free port 8080-8100
   - Verifies health check
   - Opens browser

5. **Deep cleanup** — removed stale targets, old images, caches

6. **Updated README** with new container commands

### ⚠️ Verified Working

| Check | Status |
|-------|--------|
| Health check | ✅ `{"status":"ok"}` |
| JS hashed asset | ✅ 41,975 bytes |
| WASM hashed asset | ✅ 876,786 bytes |
| CSS hashed asset | ✅ 36,827 bytes |
| Raw images (logo.jpg, mark.jpg, etc.) | ✅ |
| SPA fallback (/services, /manifesto, etc.) | ✅ |
| Security headers (CSP, HSTS, X-Frame) | ✅ |
| Fonts load via Google Fonts CSP | ✅ |

### 🔧 To Do — Next Session

**P0 (Critical):**
- Self-host fonts (download woff2, add @font-face, remove Google Fonts import)
- Responsive design — media queries for mobile/tablet (currently horizontal-scroll only)
- Playwright E2E tests for Previous button, navigation, responsiveness

**P1 (Important):**
- Retro loading animation with progress bar (replace logo-fade loader)
  - Theme-matching colors (crimson/deep/black)
  - Creative text: "Curating presence...", "Composing entrances..."
  - Smooth percentage-based progress with CSS animation
- Forms — contact form with validation and submission handling

**P2 (Nice to have):**
- Container image size optimization (currently ~40MB)
- Lighthouse audit target (Perf ≥90, A11y ≥95)
- Full responsive CSS overhaul
