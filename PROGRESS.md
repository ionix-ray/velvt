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

## 2026-06-21

### Completed Today

1. **Case study pages (S3-01)**, branch `feature/case-study-pages`. Replicated
   shalgo's `/blog` content pipeline — TOML frontmatter + `build.rs`
   directory-scan codegen (`include_str!` per file, slug-keyed loader) +
   pulldown-cmark GFM rendering — but on Vaelvet's own theme, not shalgo's
   Tailwind card design. New routes: `/cases` (index), `/cases/tag/:tag`
   (filter), `/cases/:slug` (detail). Content source of truth:
   `docs/cse_studies/*.md`.
2. **Backward-compatible wiring** — `CaseItem` gained `slug: String`
   (`#[serde(default)]`). `cases_panel.rs`'s "View Case Study" button branches
   on `case.slug.is_empty()`: empty keeps the old external `button_link`/
   `target=_blank` link, non-empty links to `/cases/{slug}`. The 3 existing
   `content/site.md` case items (TechNova, Luxe Beauty, GreenFuture) got
   matching `slug =` lines plus 3 new markdown files under
   `docs/cse_studies/` mirroring their client/metric/tags.
3. **TDD**: every new module written test-first — `case_studies.rs` (TOML
   frontmatter parsing, slug lookup, date-sort), `markdown_renderer.rs`
   (pulldown-cmark GFM conversion + component wrapper), `routes/case_study.rs`
   (known/unknown slug), `routes/case_studies_index.rs` (index/tag-filter/
   empty-state), plus new SSR tests on `cases_panel.rs` for both link
   branches. All via the existing `VirtualDom` + `dioxus_ssr::render` SSR-test
   pattern (no `Router` context needed since internal nav uses plain `<a>`,
   relying on the server's existing SPA fallback).
4. **Gates, all green**: `just lint` (fmt + clippy `-D warnings`, including
   the new `build.rs` target — required dropping `.unwrap()/.expect()` from
   it in favour of `fn main() -> Result<...>` with `?`), `just test` (96
   tests, 0 failed), `just coverage` (94.97% lines workspace-wide, gate ≥90%;
   every new file at 100%), `just audit` (0 advisories), `just build`
   (release WASM: 328 KB gz, budget 1.5 MB; `theme.css` 11.6 KB gz, budget
   40 KB). Smoke-tested the built server with curl against all 4 new/old
   paths — all 200, SPA fallback intact, no server-side breakage.
5. **Not yet done**: no in-browser click-through (no browser-automation tool
   available this session) and `just e2e` (Playwright) was not run for this
   feature — existing specs don't touch "View Case Study" so no known
   regression, but a manual check is recommended before merge. Nothing
   committed; work sits uncommitted on `feature/case-study-pages`.

### Bug-fix round — same day, branch `feature/case-study-pages`

User clicked through in a real browser and found exactly the class of bug
the note above flagged as untested: SSR string-assertions don't catch real
rendering/CSS/theme-attribute state.

1. **Bug 1 — brand was text, not the logo image.** Header markup on the
   case-study pages was a literal `"VAELVET"` string, not `brand_mark()`.
2. **Bug 2 — theme broken.** `data-theme` is set by `Home`'s own
   `Signal`/`use_effect`; case-study pages mount a separate component tree
   (full page nav, fresh WASM boot) that never runs `Home`'s effect, so
   `theme.css`'s `:root` (light) defaults applied with no way to switch.
3. **Bug 3 — layout didn't match the reference app.** Pages were single-column;
   the reference (`shalgo/src/blog.rs`) uses a left sticky-sidebar + right
   main-content two-column structure on both the listing and detail pages.

Root-caused all three, then delegated the fix to `code-writer` (Sonnet) with
a fully-specified prompt referencing the exact reference file/line ranges,
to fan work out to the model pinned for implementation rather than burning
orchestrator tokens on the edits directly. Fix:
- New `components/case_header.rs` — shared header owning a real `brand_mark()`
  `<img>` and its own theme signal/effect/toggle (same pattern as `home.rs`),
  reusing `topbar.rs`'s `theme_icon_for`/`toggle_theme` (promoted to
  `pub(crate)`) instead of duplicating the logic. Fixes bugs 1+2 for both
  case-study routes in one place.
- `routes/case_study.rs` and `routes/case_studies_index.rs` rebuilt around
  `.v-case-layout` / `.v-case-layout__sidebar` / `.v-case-layout__main`:
  detail page sidebar carries Published/Topics/Read-time cards (new
  `read_time_minutes` helper, 200wpm/ceil/min-1-minute), index page sidebar
  carries the tag filter; both match the reference's structural shape while
  using only Vaelvet's own CSS tokens (no Tailwind classes carried over).
- `theme.css` gained the `.v-case-*` layout/sidebar/breadcrumb/article rules
  plus a `max-width: 1024px` collapse breakpoint (sidebar goes static/full
  width, column stacks) — same breakpoint the reference app uses, fixes the
  responsiveness complaint.
- Playwright spec extended from 4 to 8 tests, adding real-browser checks
  for exactly what SSR tests can't catch: brand `<img>` visible with alt
  text, `.v-theme-toggle` click flips `html[data-theme]` dark↔light, sidebar
  contains Published/Topics with tag links, and a 375px-viewport bounding-box
  check that the main content starts below (not beside) the sidebar.

**Independently re-verified** (not just the agent's self-report): killed
stray `dx serve` processes from earlier in the session to rule out testing
against a stale build, then re-ran every gate myself —
`just lint` clean, `just test` 126/126, `just coverage` 91.64% lines
(`case_header.rs` 69.77%, `case_studies_index.rs` 98.67%, `case_study.rs`
99.23%), `just audit` 0 advisories, `just build` + manual gzip check (WASM
441,037 B, `theme.css` 12,396 B, both within budget) — all numbers matched
the agent's report exactly. Additionally ran the Playwright suite myself
against a freshly booted `dx serve` (`--project=chromium`): 8/8 passed (one
run had a single `page.goto("/")` timeout from a dev-server cold-start
hot-reload compile, confirmed non-reproducing on isolated re-run — not a
regression). Dispatched a `security-reviewer` (Sonnet) pass over the full
markdown-rendering trust boundary (`dangerous_inner_html` fed only by
compile-time `include_str!` content, slug only ever used as a fixed `match`
key, never a path or interpolated string) — **verdict pass-with-notes**, no
blockers; one stylistic note that a marker type (e.g. `TrustedMarkdown`)
would make the trust boundary self-enforcing instead of convention-only if
this pattern is reused elsewhere later (not required for this PR, only two
call sites today and both first-party).

All three user-reported bugs (logo, theme, layout) confirmed fixed and
gated. Still uncommitted on `feature/case-study-pages`, awaiting user
review/merge decision.
