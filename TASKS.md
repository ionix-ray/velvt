# TASKS.md — Vaelvet backlog

Picked up by `autonomous-iterate`. One row = one PR. AC must be Given/When/Then before implementation.

## Sprint S0 — Scaffold (this session)
| ID | Task | Status |
|----|------|--------|
| S0-01 | Archive `archive/pre-vaelvet` + tag | DONE |
| S0-02 | Wipe old code, preserve `.git/.claude/.github/memory` | DONE |
| S0-03 | Optimise logo + favicon from iCloud | DONE |
| S0-04 | CLAUDE.md, README.md, STATE/TASKS/PROGRESS | DONE |
| S0-05 | `.claude/settings.json` + agents (Sonnet/Haiku, no Opus) | DONE |
| S0-06 | Workspace Cargo.toml, Dioxus.toml, justfile, Containerfile | DONE |
| S0-07 | velvet-ui Cargo.toml + index.html (preload, no lazy) | DONE |
| S0-08 | theme/tokens.rs + assets/theme.css (cinematic, ≤40KB) | DONE |
| S0-09 | main.rs + config.rs + components + route | DONE |
| S0-10 | Per-component tests + Playwright smoke | DONE |
| S0-11 | Memory entries | DONE |

## Sprint S1 — First production build (next)
| ID | Task | AC sketch | Effort |
|----|------|-----------|--------|
| S1-01 | Install `dx` CLI; verify `just dev` boots on a free port | When dev server runs, GET `/` returns 200 with hero text | S |
| S1-02 | Verify `just build` produces WASM ≤1.5 MB gzipped | Build artifact under budget; report `du -h dist/*.wasm` | S |
| S1-03 | Wire IntersectionObserver scroll triggers via `web-sys` | Sections fade-in on first visibility; reduced-motion path skips | M |
| S1-04 | Animated SVG `V`-mark hero overlay (CSS keyframes, no JS) | Vine grows from baseline over 1.2s, GPU-composited only | M |
| S1-05 | Carbon-style 12-col fluid grid via CSS custom properties | Grid responds at 320 / 768 / 1024 / 1440; no media-query bloat | S |
| S1-06 | Three-plane parallax hero (CSS transform-3d) | Three depth layers; perspective set; scroll updates `--scroll-y` once per frame | M |
| S1-07 | Carbon icon set inline (top 8: arrow-right, email, phone, location, play, close, menu, chevron-down) | All inline SVG, no font icons, currentColor stroke | S |
| S1-08 | Lighthouse audit: Perf ≥90, A11y ≥95 | Report exported to `reports/lighthouse-<date>.html` | M |
| S1-09 | Threat model for the public site | `security/threat-model/components/web.md` filled, STRIDE per element | M |

## Sprint S2 — Cinematic motion + content polish
| ID | Task | AC sketch | Effort |
|----|------|-----------|--------|
| S2-01 | Hero "stage-curtain reveal" on first load (1.4s, prefers-reduced-motion respected) | First paint shows curtain; reveal triggers exactly once per session | M |
| S2-02 | Case-study cards: hover = rim-light + z-translate | GPU-composited, no layout shift on hover | S |
| S2-03 | Manifesto block: split-letter typography animation | Letters settle on scroll-in, deterministic, no JS RNG | M |
| S2-04 | Contact form (no-JS-first, progressive enhancement) | Submits to mailto: fallback; client validation enhances | M |
| S2-05 | OG + Twitter card meta + JSON-LD `Organization` | Meta passes Twitter card validator | S |

## Sprint S3 — Case study pages
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S3-01 | Markdown-backed case study pages (`/cases`, `/cases/tag/:tag`, `/cases/:slug`) replicating shalgo's `/blog` content-loading logic (TOML frontmatter + `build.rs` codegen + pulldown-cmark) on Vaelvet's own theme | Clicking "View Case Study" on a tagged `CaseItem` navigates to `/cases/{slug}` and renders the matching markdown from `docs/cse_studies/`; untagged items keep the legacy external-link behaviour; tag chips link to `/cases/tag/:tag` | M | DONE |
