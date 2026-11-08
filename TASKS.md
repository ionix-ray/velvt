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
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S1-01 | Install `dx` CLI; verify `just dev` boots on a free port | When dev server runs, GET `/` returns 200 with hero text | S | DONE |
| S1-02 | Verify `just build` produces WASM ≤1.5 MB gzipped | Build artifact under budget; report `du -h dist/*.wasm` | S | DONE |
| S1-03 | Wire IntersectionObserver scroll triggers via `web-sys` | Sections fade-in on first visibility; reduced-motion path skips | M | DONE |
| S1-04 | Animated SVG `V`-mark hero overlay (CSS keyframes, no JS) | Vine grows from baseline over 1.2s, GPU-composited only | M | DONE |
| S1-05 | Carbon-style 12-col fluid grid via CSS custom properties | Grid responds at 320 / 768 / 1024 / 1440; no media-query bloat | S | DONE |
| S1-06 | Three-plane parallax hero (CSS transform-3d) | Three depth layers; perspective set; scroll updates `--scroll-y` once per frame | M | DONE |
| S1-07 | Carbon icon set inline (top 8: arrow-right, email, phone, location, play, close, menu, chevron-down) | All inline SVG, no font icons, currentColor stroke | S | DONE |
| S1-08 | Lighthouse audit: Perf ≥90, A11y ≥95 | Report exported to `reports/lighthouse-<date>.html` | M | DONE |
| S1-09 | Threat model for the public site | `security/threat-model/components/web.md` filled, STRIDE per element | M | DONE |

## Sprint S2 — Cinematic motion + content polish
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S2-01 | Hero "stage-curtain reveal" on first load (1.4s, prefers-reduced-motion respected) | First paint shows curtain; reveal triggers exactly once per session | M | DONE |
| S2-02 | Case-study cards: hover = rim-light + z-translate | GPU-composited, no layout shift on hover | S | DONE |
| S2-03 | Manifesto block: split-letter typography animation | Letters settle on scroll-in, deterministic, no JS RNG | M | DONE |
| S2-04 | Contact form (no-JS-first, progressive enhancement) | Submits to mailto: fallback; client validation enhances | M | DONE |
| S2-05 | OG + Twitter card meta + JSON-LD `Organization` | Meta passes Twitter card validator | S | DONE |

## Sprint S3 — Case study pages
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S3-01 | Markdown-backed case study pages (`/cases`, `/cases/tag/:tag`, `/cases/:slug`) replicating shalgo's `/blog` content-loading logic (TOML frontmatter + `build.rs` codegen + pulldown-cmark) on Vaelvet's own theme | Clicking "View Case Study" on a tagged `CaseItem` navigates to `/cases/{slug}` and renders the matching markdown from `docs/cse_studies/`; untagged items keep the legacy external-link behaviour; tag chips link to `/cases/tag/:tag` | M | DONE |

## Sprint S4 — Server resilience & config-driven hardening (P0)
Feature file: `docs/features/server-resilience.feature` · Priorities/stages: `docs/ROADMAP.md`
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S4-01 | TOML config loading with CLI > env > file > defaults precedence | Given malformed TOML, server falls back to defaults with `tracing::error`, no panic | M | IN-PROGRESS |
| S4-02 | Graceful shutdown (SIGTERM/SIGINT, injectable signal source) | In-flight requests complete on container stop; shutdown testable without real signals | M | IN-PROGRESS |
| S4-03 | Request timeout + concurrency + body limits (config-driven layers) | Slow handler → 408; layer stack asserted by test | S | IN-PROGRESS |
| S4-04 | `spawn_blocking` fs + TOCTOU closed (single canonical check+read) | All 9 traversal/SPA/gzip integration tests pass unchanged | M | IN-PROGRESS |
| S4-05 | Dead `ServerError` variants wired or removed | No test-only variant remains in prod enum | S | IN-PROGRESS |
| S4-06 | k6 load testing | — | — | OOS (user decision 2026-10-25) |

## Sprint S5 — UI bugs, single-source config, cinematic cards (P1)
Feature file: `docs/features/ui-cinematic-experience.feature`
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S5-01 | Fix dead reveal observer + invisible VELVT heatmap | Heatmap cells reach opacity > 0 on scroll-in (Playwright) | M | DONE |
| S5-02 | Social links single-sourced from `content/site.md` | Strip + footer render identical config-driven href set; SSR test | M | IN-PROGRESS |
| S5-03 | Dead code purge (`next_hint.rs`, dead CSS blocks, dup devtools script) | Grep shows zero references; theme.css shrinks, stays ≤40 KB gz | M | IN-PROGRESS |
| S5-04 | hex-ui card port: sparkline signal-board + saturate-spring hover, `variant` on `CardStep` | Backward-compatible default; Playwright asserts sparkline + hover state | L | IN-PROGRESS |
| S5-05 | Full reduced-motion gating (rotate-border, cinematic-pan, pulse-dot, heatmap) | Reduced-motion run green; no element left invisible | S | IN-PROGRESS |
| S5-06 | Nav labels derive from `site.nav` (kill 3-way drift) | home/mobile/site.md labels identical; index map preserved | S | IN-PROGRESS |
| S5-07 | SSR tests for 7 untested components | social_strip, footer, hero, process, section_dots, scroll_progress covered | M | IN-PROGRESS |
| S5-08 | `just build` syncs theme.css → `public/assets/` | Container never ships stale CSS | S | TODO (after S4 justfile merge) |
| S5-09 | Font optimization: `@font-face font-display: swap` → `optional` (12 rules) | Eliminates FOUC/FOUT swap delay; theme.css ≤40 KB gz budget maintained | S | DONE |
| S5-10 | Self-host Metamorphous Google font (weight 400) | TTF → woff2 at `velvet-ui/assets/fonts/metamorphous-400.ttf`; `--font-sans` updated | S | DONE |
| S5-11 | Fix critical lazy-load violation in `about_aggregated_panel.rs` | `loading: "lazy"` → `"eager"` per product rule (no lazy loading, hero assets preloaded) | S | DONE |

## Sprint S6 — Security & compliance (P0)
Feature file: `docs/features/security-compliance.feature` · Matrix: `docs/SECURITY-COMPLIANCE.md`
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S6-01 | Code-grounded compliance matrix (OWASP 10 + LLM 10 + CIS L1) | Every row cites file:line + test + status | M | IN-PROGRESS |
| S6-02 | Header + traversal controls green | Integration tests pass | S | DONE |
| S6-03 | Residual risk register (TLS@edge, CSP styles, reporting, rate limit, SHA pins, cosign) | Each gap tracked with severity + S6-xx ID, none hidden | S | IN-PROGRESS |
| S6-04 | Gating audit/deny + gitleaks + SBOM | CI fails on CVE; no `\|\| true` | M | IN-PROGRESS |
| S6-05 | AI-agent surface governance documented | Prompt-injection + inner_html boundary + agency controls cited | M | IN-PROGRESS |
| S6-06 | Rate limiting (per-IP, config-driven) | 429 over threshold; config toggle; TDD | M | TODO |
| S6-07 | Cosign signing + SLSA provenance | Image verifiable on pull | M | TODO |

## Sprint S7 — Repo hygiene & gating CI (P0)
Feature file: `docs/features/build-hygiene.feature`
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S7-01 | Remove tracked junk (test-dist 18 MB, test-results, .codegraph db) + gitignore | `git ls-files` grep empty for all three | S | DONE |
| S7-02 | Dedupe `screenshot.spec.ts` | One spec source | S | DONE |
| S7-03 | CI audit gating, stale trigger removal, single test run | audit fails pipeline on findings | S | DONE |
| S7-04 | Coverage job in CI (`--fail-under-lines 90`) | PR red below 90% | S | DONE |
| S7-05 | harden-runner on publish + fix `cloudbuild.yaml --target runtime` | GCB build no longer references missing stage | S | DONE |
| S7-06 | Fresh small deterministic build (clean-wasm, theme.css sync, no lazy, no dup) | Budgets: WASM ≤1.5 MB gz, css ≤40 KB gz | M | DONE |
| S7-07 | SHA-pin GitHub Actions | Every action pinned to verified SHA | S | TODO |

## Sprint S8 — Agent / model / skill / context engineering (P2)
| ID | Task | AC sketch | Effort | Status |
|----|------|-----------|--------|--------|
| S8-01 | Persona→task→skill→model mapping maintained | `docs/PERSONA-MAP.md` current | S | DONE |
| S8-02 | Context engineering via graft graph | `graft ask` used for orientation; ~82K tok saved/pass | S | DONE |
| S8-03 | Token-economy rules for fan-out agents | Scope-limited prompts, progressive disclosure | S | DONE |
| S8-04 | Gemini via `agy` wired as secondary model | `agy` present, routing documented | S | DONE |