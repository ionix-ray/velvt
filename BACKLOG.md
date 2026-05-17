# BACKLOG.md — Prioritised task queue

P0 (blockers) → P1 (this sprint) → P2 (next sprint) → P3 (later)

## Sprint 1: Foundation (P0)
- T-002: Write design tokens (theme.css + tokens.rs) — P0 — Depends: T-001
- T-003: Set up router + route skeleton (6 routes) — P0 — Depends: T-001
- T-004: Implement shared components (Button, Card, Section, FadeIn) — P0 — Depends: T-002
- T-005: Write unit tests for shared components — P0 — Depends: T-004
- T-006: Configure Dioxus.toml metadata + SEO base — P0 — Depends: T-003
- T-007: Set up Playwright E2E harness — P0 — Depends: T-003

## Sprint 2: Core Pages (P0)
- T-008: Hero component — P0 — Depends: T-004
- T-009: Navbar component — P0 — Depends: T-004
- T-010: Services section — P0 — Depends: T-004
- T-011: Talent & Events section — P0 — Depends: T-004
- T-012: Portfolio section — P0 — Depends: T-004
- T-014: Contact section with form validation — P0 — Depends: T-004
- T-015: Footer — P0 — Depends: T-004
- T-016: Write tests for all page components — P0 — Depends: T-008–T-015

## Sprint 3: Polish & SEO (P1)
- T-013: Podcast section (coming soon) — P1 — Depends: T-004
- T-017: Scroll-triggered animations — P1 — Depends: T-008–T-015
- T-018: Page transitions — P1 — Depends: T-003
- T-019: prefers-reduced-motion fallbacks — P1 — Depends: T-017
- T-020: JSON-LD structured data — P1 — Depends: T-006
- T-021: sitemap.xml + robots.txt — P1 — Depends: T-006
- T-022: OpenGraph + Twitter Card meta — P1 — Depends: T-006
- T-023: CSP headers + security hardening — P1 — Depends: T-014
- T-024: WASM bundle size optimization — P1 — Depends: T-008–T-015

## Sprint 4: QA & Ship (P1)
- T-025: Lighthouse CI + fix bottlenecks — P1 — Depends: T-020–T-024
- T-026: Playwright E2E: all routes — P1 — Depends: T-016
- T-027: Visual regression snapshots — P1 — Depends: T-026
- T-028: Keyboard navigation audit — P1 — Depends: T-009
- T-029: Responsive audit — P1 — Depends: T-026
- T-030: cargo-audit + cargo-deny clean — P1 — Depends: T-024
- T-031: Static output + DEPLOY.md — P1 — Depends: T-025–T-030
- T-032: Final coverage report — P1 — Depends: T-016
