# Release Notes

## v0.2.0 — Production-Grade Refactor (2026-07-07)

### Summary
Complete production hardening pass: build infrastructure, component architecture,
server hardening, SEO/AI search optimisation, CI/CD pipeline, and GitHub Pages deployment.

---

### 🏗️ Build Infrastructure

**`justfile`** — Rewritten as DRY, POSIX-safe, CI-grade:
- `just build` automatically removes stale WASM artifacts before each build (no hashed files linger)
- `just container-up` — single-command WASM build → stage → container image → run → open browser
- `just container-build-fresh` — clean image rebuild for production deploys
- `just container-tag` — tag image with semver + git SHA
- `just ci` — full local pipeline (lint → test → build → container → e2e)
- `just ci-local` — mirrors GitHub Actions pipeline exactly via `scripts/ci-local.sh`
- `just coverage` / `just coverage-html` — llvm-cov coverage reporting
- `just audit` / `just gitleaks` / `just sbom` — security tooling

**`Containerfile`** — Multi-stage, distroless, production-hardened:
- Stage 1: WASM build via `dx build --release`
- Stage 2: Rust server compile `--target x86_64-unknown-linux-musl`
- Stage 3: `gcr.io/distroless/cc-debian12` runtime — no shell, no package manager
- Non-root user (`UID=1000`)
- `STOPSIGNAL SIGTERM` for graceful shutdown
- Memory limit labels for orchestrators

**`scripts/ci-local.sh`** — New production-grade local CI script:
- Mirrors all 5 GitHub Actions jobs
- `--skip-wasm` flag for Rust-only change iterations
- Coloured output, job pass/fail counters, duration tracking
- Prints exact deploy commands on success

---

### 🔒 Server Hardening (`server/`)

**`handlers.rs`**:
- Zero-runtime-cost pre-compressed `.gz` asset serving
  - Client `Accept-Encoding: gzip` → serve `file.wasm.gz` directly (no runtime compression)
  - Immutable cache headers for hashed assets (`dxh_*` pattern)
  - 404 falls back to `index.html` for SPA routing

**`middleware.rs`**:
- Added `Cross-Origin-Opener-Policy: same-origin`
- Added `X-DNS-Prefetch-Control: off`
- Added Content Security Policy (CSP) header
- All headers injected at middleware level (zero per-route overhead)

---

### 🎴 Component Architecture (`velvet-ui/src/components/`)

**`card_step.rs`** — New unified cinematic card component (single source of truth):
- Config-driven: `logo`, `title`, `body`, `num` (step number), `tag` (eyebrow label), `class_extra`, `delay_ms`
- Sparkling running border animation (CSS keyframe, no JS)
- Logo badge (top-left corner, brand image)
- Transparent background — adapts to dark/light mode automatically
- Text colour driven by CSS `color-scheme` / `prefers-color-scheme`
- `delay_ms` stagger for entrance animations

**`process_panel.rs`** — Refactored to consume `CardStep`

**`studio_panel.rs`** — Refactored to consume `CardStep`

Both panels: `Box<str>` config fields converted to `String` via `.to_string()` for type safety.

**`hero_panel.rs`** — Dead code removed; logo size enlarged via `clamp()` for responsiveness.

---

### 🔍 SEO & AI Search Optimisation

**`velvet-ui/assets/`**:
- `robots.txt` — Updated with explicit `Allow` for AI crawlers: GPTBot, Claude-Web, PerplexityBot, Googlebot-Image, Applebot
- `sitemap.xml` — Enhanced with `<image:image>` entries per page (Google image sitemap spec)
- `llms.txt` — New: structured machine-readable context for AI search engines (ChatGPT, Perplexity, Gemini)
- `humans.txt` — New: project credits and team info

**`index.html`** — Enhanced structured data:
- `Organization` JSON-LD with `sameAs` social links
- `LocalBusiness` JSON-LD with address, phone, operating hours
- `WebSite` JSON-LD with `SearchAction`
- `FAQPage` JSON-LD (top PR questions)
- `BreadcrumbList` JSON-LD
- OpenGraph image + `article:tag` metadata
- Twitter Card `summary_large_image`
- `<meta name="robots" content="index,follow,max-image-preview:large">` for AI image indexing
- Meta verification tags (Google Search Console, Bing)

---

### 🚀 CI/CD Pipeline (`.github/workflows/`)

**`deploy-pages.yml`** — Full production-grade rewrite (5 jobs):
1. **Security Audit** — `cargo-audit` + `cargo-deny` (licenses, advisories, bans)
2. **Test & Lint** — `rustfmt`, `clippy -D warnings`, unit tests, integration tests
3. **Build** — `wasm32-unknown-unknown`, `dx build --release`, `.nojekyll`, `CNAME`, `404.html`, SEO files, build metadata
4. **Deploy** — `actions/deploy-pages@v4` to `velvt.live`
5. **Post-Deploy Verify** — HTTP check on live URL + SEO file verification
- `step-security/harden-runner@v2` on all jobs
- `persist-credentials: false` everywhere
- Minimal permissions (principle of least privilege)
- Pinned action versions

**`pr-gate.yml`** — Hardened:
- Removed broken `ops/tofu` step
- Added `harden-runner`
- Added `persist-credentials: false`
- Added `cargo-audit`
- Added WASM output verification

**`deny.toml`** — New: cargo-deny configuration:
- Allow: MIT, Apache-2.0, ISC, BSD-2/3, MPL-2.0, Zlib, CC0-1.0
- Deny: GPL-2.0, GPL-3.0, AGPL-*, LGPL-*
- Vulnerability: deny; Unmaintained: warn; Yanked: warn

---

### 📁 New Files

| File | Purpose |
|------|---------|
| `scripts/ci-local.sh` | Local CI pipeline script |
| `deny.toml` | cargo-deny license + advisory config |
| `.nojekyll` | Disables Jekyll on GitHub Pages |
| `velvet-ui/assets/CNAME` | Custom domain: `velvt.live` |
| `velvet-ui/assets/llms.txt` | AI search engine context file |
| `velvet-ui/assets/humans.txt` | Project credits |
| `RELEASE_NOTES.md` | This file |

---

### ✅ Test Coverage

All **142 tests** pass:

| Test Suite | Pass | Fail |
|-----------|------|------|
| `velvet-ui` unit (lib) | 96 | 0 |
| `velvet-ui` SSR | 2 | 0 |
| `velvet-ui` render integration | 14 | 0 |
| `velvet-server` | 18 | 0 |
| Other | 12 | 0 |

Tests updated:
- `panel_1_home_hero_renders` — updated to reflect current `v-hero-3d-wrapper` design (replaced stale `v-hologram` assertions)

---

### 🔄 Breaking Changes

**None.** All changes are additive or internal refactors. The public interface (routes, URLs, CSS classes visible to end users) is unchanged.

---

### 🧑‍💻 Developer Notes

- **`Box<str>` vs `String`**: Config fields in `config.rs` use `Box<str>` for memory efficiency. When passing to `CardStep` props (which use `String`), call `.to_string()`.
- **Stale WASM**: `just build` cleans `target/dx/vaelvet-ui/release/web/public` before building. This is required — `dx` does not clean hashed asset names between builds.
- **container-size recipe**: Uses `python3 -c "import sys,json; ..."` to avoid Go template `{{}}` syntax conflicting with `just`'s template parser.
