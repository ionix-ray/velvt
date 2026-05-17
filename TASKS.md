# TASKS.md — Velvet PR Agency Website

## Sprint 1: Foundation (P0 — Week 1)
| ID | Task | Priority | Depends | Effort | Status |
|----|------|----------|---------|--------|--------|
| T-001 | Scaffold workspace, Cargo.toml, dx.toml, justfile | P0 | — | S | DONE |
| T-002 | Write design tokens (theme.css + tokens.rs) | P0 | T-001 | S | DONE |
| T-003 | Set up router + route skeleton (6 routes) | P0 | T-001 | M | DONE |
| T-004 | Implement shared components (Button, Card, Section, FadeIn) | P0 | T-002 | M | DONE |
| T-005 | Write unit tests for shared components | P0 | T-004 | M | DONE |
| T-006 | Configure Dioxus.toml metadata + SEO base | P0 | T-003 | S | DONE |
| T-007 | Set up Playwright E2E harness | P0 | T-003 | S | DONE |

## Sprint 2: Core Pages (P0 — Week 2)
| ID | Task | Priority | Depends | Effort | Status |
|----|------|----------|---------|--------|--------|
| T-008 | Hero component (full-viewport, CTA, parallax bg) | P0 | T-004 | M | DONE |
| T-009 | Navbar (sticky, scroll-aware, keyboard nav) | P0 | T-004 | M | DONE |
| T-010 | Services section (3-col grid, hover states) | P0 | T-004 | M | DONE |
| T-011 | Talent & Events section (roster, parallax) | P0 | T-004 | M | DONE |
| T-012 | Portfolio section (carousel, case studies) | P0 | T-004 | M | DONE |
| T-013 | Podcast section (coming soon, email capture) | P1 | T-004 | S | DONE |
| T-014 | Contact section (form + validation) | P0 | T-004 | M | DONE |
| T-015 | Footer (persistent CTA, legal) | P0 | T-004 | S | DONE |
| T-016 | Write tests for all page components | P0 | T-008–T-015 | L | DONE |

## Sprint 3: Polish & SEO (P1 — Week 3) — COMPLETE ✅
| ID | Task | Priority | Depends | Effort | Status |
|----|------|----------|---------|--------|--------|
| T-017 | Scroll-triggered animations (IntersectionObserver) | P1 | T-008–T-015 | M | DONE |
| T-018 | Page transitions (fade + slide) | P1 | T-003 | S | DONE |
| T-019 | `prefers-reduced-motion` fallbacks | P1 | T-017 | S | DONE |
| T-020 | JSON-LD structured data (all schema types) | P1 | T-006 | M | DONE |
| T-021 | sitemap.xml + robots.txt | P1 | T-006 | S | DONE |
| T-022 | OpenGraph + Twitter Card meta per route | P1 | T-006 | S | DONE |
| T-023 | CSP headers + security hardening | P1 | T-014 | S | DONE |
| T-024 | WASM bundle size optimization | P1 | T-008–T-015 | M | DONE |

## Sprint 4: QA & Ship (P1 — Week 4)
| ID | Task | Priority | Depends | Effort | Status |
|----|------|----------|---------|--------|--------|
| T-025 | Run Lighthouse CI, fix bottlenecks | P1 | T-020–T-024 | M | DONE |
| T-026 | Playwright E2E: all routes + interactions | P1 | T-016 | L | DONE |
| T-027 | Visual regression snapshots (3 breakpoints) | P1 | T-026 | M | DONE |
| T-028 | Keyboard navigation audit | P1 | T-009 | S | DONE |
| T-029 | Responsive audit (320px–1440px) | P1 | T-026 | S | DONE |
| T-030 | cargo-audit + cargo-deny clean | P1 | T-024 | S | DONE |
| T-031 | Generate static output, DEPLOY.md | P1 | T-025–T-030 | S | DONE |
| T-032 | Final coverage report (≥95% logic) | P1 | T-016 | S | DONE |

## Coverage Targets
| Module | Target |
|--------|--------|
| Shared components | ≥95% |
| Route logic | ≥95% |
| Theme tokens | 100% |
| Form validation | 100% |
| SEO meta injection | ≥95% |
