# ARCHITECTURE.md — Velvet PR Agency Website

## Tech Stack (Locked)
| Layer | Technology | Version |
|-------|-----------|---------|
| Framework | Dioxus (web) | 0.7.6 |
| Router | dioxus-router | 0.7.6 |
| Build | dx CLI | latest |
| Language | Rust | 1.85+ |
| Target | wasm32-unknown-unknown | — |
| CSS | Inline + assets/theme.css | — |
| Testing | `#[test]`, dioxus::testing, Playwright E2E | — |
| CI | GitHub Actions | — |

## Folder Structure
```
velvet/
├── Cargo.toml                    # Workspace config
├── dx.toml                       # Dioxus build config
├── justfile                      # Dev automation
├── velvet-ui/
│   ├── Cargo.toml                # UI crate deps
│   ├── Dioxus.toml               # Dioxus app config (metadata, assets)
│   ├── assets/
│   │   ├── theme.css             # Design tokens, animations, responsive
│   │   ├── fonts/                # Self-hosted woff2 (Playfair, Inter, Manrope)
│   │   └── images/               # Optimized assets (webp/avif)
│   └── src/
│       ├── main.rs               # App entry, router, context provider
│       ├── routes.rs             # Route definitions + layout
│       ├── theme/
│       │   └── tokens.rs         # Rust constants for design tokens
│       ├── components/
│       │   ├── navbar/           # Sticky nav, scroll-aware, keyboard nav
│       │   ├── hero/             # Full-viewport cinematic, CTA
│       │   ├── services/         # 3-col card grid, hover interactions
│       │   ├── talent/           # Roster showcase, parallax
│       │   ├── portfolio/        # Case study carousel
│       │   ├── podcast/          # Coming soon + email capture
│       │   ├── contact/          # Form with validation
│       │   ├── footer/           # Persistent CTA, legal
│       │   └── shared/           # Button, Card, Section, FadeIn wrapper
│       └── routes/
│           ├── home.rs
│           ├── services.rs
│           ├── talent.rs
│           ├── portfolio.rs
│           ├── podcast.rs
│           └── contact.rs
├── tests/                        # Integration + E2E specs
├── test-suite/                   # Playwright fixtures + data
├── config/                       # TOML configs (theme, routes, seo)
├── security/                     # Threat models, findings, SBOM
├── docs/                         # ADRs, runbooks
└── memory/                       # Context tracker
```

## Data Flow
```
User Action → Dioxus Event Handler → Signal Update → Re-render
     ↓
Router Match → Route Component → Layout Wrapper → SEO Meta Injection
     ↓
CSS Theme Tokens → Inline Styles + theme.css → Browser Render
```

## WASM Strategy
- Single WASM bundle (no chunking needed for site this size)
- Lazy-load images via `loading="lazy"`
- Fonts: self-hosted woff2, `font-display: swap`, subsetted
- Bundle budget: <1.5MB gzipped
- Fallback: `<noscript>` with static HTML for crawlers

## Testing Strategy
| Type | Tool | Location | Target |
|------|------|----------|--------|
| Unit | `#[test]` | Inline `#[cfg(test)]` | Logic ≥95% |
| Component | `dioxus::testing` | velvet-ui/src/ | Render + props |
| E2E | Playwright | test-suite/playwright/ | All routes, interactions |
| Visual | Snapshot | test-suite/playwright/ | Key breakpoints |
| Security | `cargo-audit` | CI | Zero critical CVEs |
| Performance | Lighthouse CI | CI | Perf ≥90, A11y ≥95 |

## SEO Architecture
- `Dioxus.toml` → `<head>` metadata injection
- Per-route `use_effect` for dynamic `<title>`/`<meta>`
- JSON-LD injected via `<script type="application/ld+json">`
- `sitemap.xml` generated at build time
- `robots.txt` served from assets

## Security Controls
- CSP: `default-src 'self'; script-src 'self' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; font-src 'self'; img-src 'self' data:;`
- Input sanitization on contact form (no HTML injection)
- CORS: same-origin only (static site)
- WASM sandbox: no filesystem/network access beyond fetch

## Performance Budget
| Metric | Target |
|--------|--------|
| WASM bundle (gzipped) | <1.5MB |
| First Contentful Paint | <1.5s |
| Largest Contentful Paint | <2.5s |
| Time to Interactive | <3.0s |
| Cumulative Layout Shift | 0 |
| Animation frame time | <16ms (60fps) |
