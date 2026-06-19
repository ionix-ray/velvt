# Progress Log — Vaelvet Container + UI/UX Refactor

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
