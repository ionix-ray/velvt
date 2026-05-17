# PRD.md — Velvet PR Agency Website

## Product Vision
Editorial-minimal, story-driven website for a premium PR agency. Cinematic luxury aesthetic with velvet depth, purposeful whitespace, and scroll-triggered narrative progression.

## Target Users
- High-net-worth individuals seeking PR representation
- Corporate communications directors
- Event producers & talent agencies
- Podcast listeners (future)

## Design System

### Typography
| Role | Font | Weight | Usage |
|------|------|--------|-------|
| H1–H3 | Playfair Display | 400/700 | Headings, hero text |
| Body | Inter | 400/500 | Paragraphs, navigation |
| Accent | Manrope | 600 | CTAs, labels |

### Color Palette
| Token | Value | Usage |
|-------|-------|-------|
| `--bg-primary` | `#0A0A0A` | Page background |
| `--bg-secondary` | `#111111` | Cards, sections |
| `--velvet-crimson` | `#8B0000 → #A52A2A` | Primary accent, gradients |
| `--deep-plum` | `#4B0082 → #6A0DAD` | Secondary accent |
| `--warm-gold` | `#D4AF37` | Micro-accents, highlights |
| `--text-primary` | `#F5F5F5` | Headings, body |
| `--text-secondary` | `#A0A0A0` | Subtitles, captions |

### Layout
- 12-column responsive grid
- Max content width: 1280px
- Breakpoints: 320px, 768px, 1024px, 1440px
- Generous vertical rhythm (8px base unit)

### Animation Budget
- Target: 60fps, <16ms per frame
- Scroll-triggered reveals (IntersectionObserver)
- Parallax overlays (CSS transforms only)
- Hover micro-interactions (<200ms)
- Page transitions (fade + slide, 300ms)
- `prefers-reduced-motion`: disable all animations

## Routing Map

| Route | Component | Description | SEO |
|-------|-----------|-------------|-----|
| `/` | Home (Hero) | Cinematic hero with CTA | Primary landing page |
| `/services` | Services | PR, media relations, crisis comms | Service schema JSON-LD |
| `/talent` | Talent & Events | Roster, event production | FAQ schema |
| `/portfolio` | Portfolio | Case studies, client stories | Article schema |
| `/podcast` | Podcast (Coming Soon) | Teaser + email capture | ComingSoon schema |
| `/contact` | Contact | Form + social links | LocalBusiness schema |

## Sections (Story Progression)
1. **Hero** — Full-viewport cinematic intro, tagline, primary CTA
2. **Services** — 3-column card grid (PR, Media Relations, Crisis)
3. **Talent & Events** — Roster showcase, event highlights
4. **Portfolio** — Case study carousel with parallax
5. **Podcast** — Coming soon teaser with email capture
6. **Contact** — Form, social links, office info
7. **Footer** — Minimal, persistent CTA, legal

## SEO Strategy
- Semantic HTML5 landmarks (`<header>`, `<main>`, `<nav>`, `<section>`, `<footer>`)
- Dynamic `<title>` + `<meta description>` per route
- OpenGraph + Twitter Card meta
- JSON-LD: Organization, Service, FAQ, Article, ComingSoon, LocalBusiness
- `sitemap.xml` + `robots.txt`
- `rel="canonical"` on all pages
- WASM-rendered content must be crawlable (pre-render fallback)

## Acceptance Criteria
| ID | Criterion | Verification |
|----|-----------|-------------|
| AC-01 | All 6 routes render correctly | E2E Playwright tests |
| AC-02 | Lighthouse Perf ≥90 | CI Lighthouse audit |
| AC-03 | Lighthouse A11y ≥95 | CI Lighthouse audit |
| AC-04 | Lighthouse SEO ≥95 | CI Lighthouse audit |
| AC-05 | WASM bundle <1.5MB | `ls -la target/release/` |
| AC-06 | Responsive 320px–1440px | Playwright viewport tests |
| AC-07 | Keyboard navigable | E2E tab-navigation tests |
| AC-08 | `prefers-reduced-motion` respected | CSS media query tests |
| AC-09 | JSON-LD valid on all routes | Structured Data Testing Tool |
| AC-10 | CSP headers present | HTTP header inspection |
| AC-11 | Logic coverage ≥95% | `cargo tarpaulin` |
| AC-12 | Zero critical CVEs | `cargo audit` clean |
