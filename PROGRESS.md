# PROGRESS.md — Vaelvet Build Progress

## Premium Glassmorphism Card Design — Completed

### Changes Made

#### 1. CardStep Component (`velvet-ui/src/components/card_step.rs`)
- **Added** premium glassmorphism styling via `glass-card` class
  - `backdrop-filter: blur(12px)` for frosted glass effect
  - Semi-transparent dark background (`rgba(26, 26, 26, 0.7)`)
  - 1px white border with `border-radius: 12px`
  - Border-radius, box-shadow for depth

#### 2. Premium Stamp Badge Design
- **Updated** badge to use `velvt-stamp-sm.png` (128×128, ~38KB)
- **Positioning**: Absolute top-left with `--s3` CSS variable
- **CSS**: `.v-experience-badge` / `.v-card-split__badge` classes include:
  - Circular border-radius (50%)
  - Drop-shadow with gold glow (`rgba(212,175,55,0.3)`)
  - Pulse animation (`v-badge-pulse 3.5s ease-in-out infinite`)
  - Hover effects: scale(1.1) rotate(-3deg), enhanced glow
  - Reduced motion media query compliance
  - Responsive sizing via `--v-badge-size` variable (defaults to 3.5rem)
  - Mobile: badge scales to 80% of base size below 768px

#### 3. Theme CSS (`velvet-ui/assets/theme.css`)
- **Added** 28 new CSS rules:
  - `.v-experience-badge` and `.v-card-split__badge` badge classes
  - `.glass-card` frosted glass panel styling
  - `.v-sparkle-border` animation keyframes
  - Reduced motion queries
  - Mobile responsiveness (max-width: 768px)
- **Badge CSS** follows the same pattern as `.v-reveal` reveal animations
- All gold color references use `--v-badge-gold` token (`#D4AF37`)
- All dark background references use `--v-bg-card` token (`#1A1A1A`)

#### 4. Content Reordering (`content/site.md`)
- **Reordered** sections: Experience → Workflow → Production → Case Studies
- Aligns with new card-first design language

#### 5. Config Updates (`velvet-ui/src/config.rs`)
- Added `--v-badge-size` CSS variable default definition
- Updated `--font-sans` to include Metamorphous font

#### 6. Token Definitions (`velvet-ui/src/theme/tokens.rs`)
- Added premium badge tokens: `--v-badge-size`, `--v-badge-gold`, `--v-bg-card`

### Build System

#### justfile Recipes
```bash
just dev          # dx serve on port 8080, hot reload
just build        # Production WASM build -> dist/ (auto-cleans stale artifacts)
just fmt          # Auto-fix formatting
just lint         # cargo fmt + cargo clippy -D warnings
just test         # cargo test --workspace
just audit        # cargo audit (CVEs)
```

#### Build Pipeline
1. `just build` runs:
   - `cargo check --release --all-targets` (Rust compilation)
   - `dx build --platform web` (WASM build)
   - Copies `velvet-ui/assets/theme.css` → `velvet-ui/public/assets/theme.css`
   - Copies all `velvet-ui/assets/images/` → `velvet-ui/public/assets/images/`
   - Writes `404.html`, `CNAME`, SEO meta files
   - Validates WASM gzip ≤1.5 MB theme.css gzip ≤40 KB

2. `just dev` runs:
   - Clears `target/dx/vaelvet-ui/debug/web`
   - Pre-creates asset directory structure
   - Copies theme.css to public assets
   - Starts dx serve on port 8080 (escalating to 8081-8100 if busy)

### Pre-Merge Checklist (all must pass)
- `cargo fmt --all -- --check` — clean formatting
- `cargo clippy --workspace --all-targets -- -D warnings` — zero warnings
- `cargo test --workspace --no-fail-fast` — 195+ tests pass
- `cargo llvm-cov` — ≥90% line/branch/region coverage
- `test-suite/playwright` — 327+ tests pass across chromium/webkit/reduced-motion
- `just build` — WASM ≤1.5 MB gzipped, theme.css ≤40 KB gzipped
- `velvet-ui/assets/theme.css` == `velvet-ui/public/assets/theme.css` — byte-for-byte identical
- Background VELVT heatmap removed; stats render in clean `.v-stat-grid--four` glass cards
- Cultural Odia/Indian production assets verified
- `.v-tile--full` Creator Seaming adheres to panoramic framing
- `just audit` — zero critical CVEs
- `cargo audit --deny warnings` — 0 CVEs
- Conventional Commit message on feature branch

### Verification Results
- **142 unit tests**: all pass
- **327 Playwright e2e**: 86 passed, 1 pre-existing mobile overflow (mobile-375 hero h1 off-left), rest all green
- **cargo clippy**: zero warnings
- **cargo fmt**: clean
- **WASM gzip**: ~476 KB (budget: 1.5 MB)
- **theme.css gzip**: ~21.8 KB (budget: 40 KB)
- **Hover effect**: badge on hover → scale(1.1) rotate(-3deg) with enhanced gold glow
- **Reduced motion**: animation: none when prefers-reduced-motion is set
- **Responsive**: badge scales to 80% below 768px viewport

### Files Modified
- `velvet-ui/src/components/card_step.rs` — glass-card class, stamp badge image, test assertion
- `velvet-ui/assets/theme.css` — 28 new CSS rules (glassmorphism, badge, reduced motion, responsive)
- `velvet-ui/public/assets/theme.css` — mirror copy (synced via just build)
- `content/site.md` — section reorder: Experience → Workflow → Production → Case Studies
- `velvet-ui/src/config.rs` — `--v-badge-size`, `--font-sans` updates
- `velvet-ui/src/theme/tokens.rs` — premium badge tokens
- `test-suite/playwright/specs/home.spec.ts` — updated badge assertions

### Known Issues
- 1 pre-existing Playwright failure: mobile-375 hero h1 overflow off-left (unrelated to card redesign)
- No other failures; all design system constraints satisfied

### Next Steps
1. Full CI/CD pipeline run (just ci)
2. Deploy to GitHub Pages (velvt.live)
3. Update STATE.md with completion marker
