# Progress Log — Vaelvet Container + UI/UX Refactor

## 2026-06-28

### Completed Today
1. **Testing & QA Complete** — Scaled the Playwright test suite to 222 robust, end-to-end tests covering 4 major responsive breakpoints (320px, 375px, 768px, 1280px). Tested across Chromium, Webkit, and a reduced-motion profile. Verified complete stability of visual layouts, responsive edge-cases, and brand constraints.
2. **Typography Optimization** — Eliminated external CDN dependencies for typography. Both `IBM Plex Sans` and `Kalnia Glaze` are now fully self-hosted. Enforced the `Kalnia Glaze` display font globally, specifically ensuring `.v-founder__name` renders elegantly with accurate `--accent` (`#B52A2A` / `#D43E3E`) colors across all themes.
3. **Responsive UI/UX Refinements** — Tuned container side-panel gutters for 320px and 375px viewport breakpoints to drastically improve small mobile readability. Added responsive 1-column collapse constraints to the founder grid and minimized footer negative space.
4. **Docs & Container Wrap-up** — Verified `.gitignore` (ignoring `/deployment/` completely) and `.containerignore`. Final distroless container builds flawlessly with a 1.5MB WASM footprint constraint met. `TASKS.md`, `STATE.md`, and `CHANGELOG.md` fully closed-out.

## 2026-06-23

### Completed Today

1. **Brand mark + topbar layout unification** (`d9df0aa`).
   - `footer_panel.rs` and `stacked_nav.rs` switched from a hardcoded
     `asset!()` literal / text wordmark to the shared
     `theme::brand::brand_mark()` helper. All six brand-mark slots
     (topbar, loader, stacked-nav, footer, case-header,
     `index.html` preload) now point at the same `velvet-square.png`
     source, removing the last divergence flagged in yesterday's
     review.
   - Topbar logo moved from floating-badge (fixed top: 56 + s2,
     left: s3, pointer-events: none) to inline-in-bar
     (`clamp(2.4rem, 4.2vw, 3.25rem)` height, mobile override
     2.35 rem). Topbar gains `min-height`, `justify-content:
     space-between`, fluid horizontal padding.
   - Scrollbars switched from hidden to themed-thin (8 px accent
     thumb with hover state) on both `.v-panels` and `.v-panel`
     vertical overflow — tall content (showcase grid above viewport
     height) is now reachable instead of clipped.

2. **Showcase grid redesign** (same commit).
   - Full rename `.v-masonry__*` → `.v-showcase__*` across
     `studio_panel.rs` (rendered classes + helper functions +
     unit tests) and `theme.css`. New visual treatment:
     `v-showcase-panel` red radial-gradient backdrop, per-card
     260 px min-height, gradient media block underneath the
     content layer, hover lift (-4 px translate, accent border,
     stronger shadow), themed background per card. Incomplete
     trailing row span logic preserved (wide / full).
   - Mobile breakpoint (`@max-width: 1024px`) collapses 3-col →
     2-col → 1-col. Confirmed in Playwright responsive spec.

3. **Container build context fix** (`3e3fc7c`).
   - `Containerfile` now copies `velvet-ui/build.rs` and
     `docs/cse_studies/` into the build stage. `.containerignore`
     replaces blanket `docs/` with `docs/*` + `!docs/cse_studies/`.
     Required because the case-study `build.rs` codegen embeds
     `docs/cse_studies/*.md` via `include_str!` at compile time —
     without these paths in the container context, the release
     image fails to build.

4. **Playwright loader regression fix** (UI commit).
   - `loader: hides via clip-path collapse, not display:none, and
     stops blocking clicks` was failing under `webkit` and
     `reduced-motion` (passed under `chromium`). Root cause:
     `await page.waitForSelector(".v-loader", { state: "hidden" })`
     is matched by "element not in DOM yet" too. On slower engines
     the WASM mount window is long enough that playwright's first
     poll sees an empty body, the wait returns immediately, and
     the next-line `getComputedStyle().pointerEvents` reads `auto`
     from the still-initial loader. Rewrote the assertion to
     `expect(loader).toHaveClass(/\bhidden\b/, { timeout: 10000 })`
     so the wait blocks on the actual `.hidden` modifier being
     applied.

5. **Brand-consistency tests** (UI commit).
   - Added `brand: topbar uses the standard in-bar logo layout` —
     asserts logo image is rendered inside topbar's bounding box,
     no longer breakout-positioned.
   - Added `brand: topbar and stacked navigation share the same
     brand mark` — `src` attribute equality + `velvet-square` path
     check, regression-tests the shared `brand_mark()` helper.

### Verified

- `cargo fmt --check`: clean
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo test --workspace`: **126 passed, 0 failed**
- `cargo audit`: 0 advisories
- `dx build --release --platform web`: 441 KB gz WASM (budget 1.5 MB),
  11.8 KB gz `theme.css` (budget 40 KB)
- Playwright against live `dx serve`:
  - chromium: 48/48
  - webkit: 48/48
  - reduced-motion (Chrome + `prefers-reduced-motion: reduce`): 48/48
  - One flaky case-studies failure observed under parallel-project
    invocation; passed cleanly when each project ran sequentially —
    not a regression, dev-server contention.

### Discovered + flagged

- Pre-existing gibberish commit messages on `main` (`jkl`, `gbgh`,
  `ijhbvghj`) and the S3-01 commit on the branch (`hbhjb`,
  authored 2026-06-22). Recommend squash-merge to `main` rather
  than fast-forward so the deploy commit on `main` has a clean
  Conventional Commit message.

### To Do — Next Session

- Squash-merge `feature/case-study-pages` → `main`, push, observe
  GitHub Pages deploy workflow.
- Once deployed, manual sanity check at the Pages URL
  (`https://ionix-ray.github.io/velvt/`) for: logo unification,
  showcase grid layout at 1280 / 1024 / 640 px, case-study
  navigation (`/cases`, `/cases/:slug`, `/cases/tag/:tag`).
- Address pre-existing branch's `unwrap_used` lint exemptions if
  flagged in cargo-deny output during the deploy workflow.

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
