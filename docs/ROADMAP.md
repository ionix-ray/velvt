# ROADMAP.md — priority-staged backlog (spine for TASKS.md)

Every row: ID · priority (P0-P3) · stage (sprint) · feature file · owner persona/model/skill bundle · status.
One row = one PR. AC lives in the linked `.feature` file (Given/When/Then). `TASKS.md` mirrors these as sprint tables.

**Status legend**: `DONE` · `IN-PROGRESS` (this session, fan-out) · `TODO` · `OOS` (out of scope, user decision)

## Stage S4 — Server resilience & config-driven hardening (P0)

| ID | Item | Feature | Priority | Status |
|----|------|---------|----------|--------|
| S4-01 | TOML config loading, layered precedence (CLI > env > file > defaults) | server-resilience.feature | P0 | IN-PROGRESS |
| S4-02 | Graceful shutdown (SIGTERM/SIGINT, injectable signal source) | server-resilience.feature | P0 | IN-PROGRESS |
| S4-03 | Request timeout + concurrency limit + body limit layers (config-driven) | server-resilience.feature | P0 | IN-PROGRESS |
| S4-04 | `spawn_blocking` fs, TOCTOU closed (single canonical path check+read) | server-resilience.feature | P0 | IN-PROGRESS |
| S4-05 | Dead error variants wired or removed | server-resilience.feature | P1 | IN-PROGRESS |
| S4-06 | k6 / load testing | — | — | OOS (user: not required, 2026-10-25) |

## Stage S5 — UI: bug fixes, single-source config, cinematic cards (P1)

| ID | Item | Feature | Priority | Status |
|----|------|---------|----------|--------|
| S5-01 | Fix dead reveal observer + invisible VELVT heatmap | ui-cinematic-experience.feature | P0 | IN-PROGRESS |
| S5-02 | Social/external links single-sourced from `content/site.md` | ui-cinematic-experience.feature | P1 | IN-PROGRESS |
| S5-03 | Dead code purge (`next_hint.rs`, dead CSS, dup devtools script) | ui-cinematic-experience.feature | P1 | IN-PROGRESS |
| S5-04 | hex-ui futuristic card port (sparkline signal-board + saturate spring, `variant` prop) | ui-cinematic-experience.feature | P1 | IN-PROGRESS |
| S5-05 | Full `prefers-reduced-motion` gating (border rotate, pan, pulse, heatmap) | ui-cinematic-experience.feature | P1 | IN-PROGRESS |
| S5-06 | Nav labels unified onto `site.nav` (kill 3-way drift) | ui-cinematic-experience.feature | P2 | IN-PROGRESS |
| S5-07 | SSR tests for 7 untested components | ui-cinematic-experience.feature | P2 | IN-PROGRESS |
| S5-08 | `just build` syncs `assets/theme.css` → `public/assets/` (stale-CSS ship bug) | build-hygiene.feature | P0 | TODO (justfile owned by S4 agent; apply after merge) |

## Stage S6 — Security & compliance (P0)

| ID | Item | Feature | Priority | Status |
|----|------|---------|----------|--------|
| S6-01 | `docs/SECURITY-COMPLIANCE.md` matrix (OWASP Top 10 + LLM Top 10 + CIS L1, code-grounded) | security-compliance.feature | P0 | IN-PROGRESS |
| S6-02 | Keep header + traversal controls green (integration tests) | security-compliance.feature | P0 | DONE (existing, re-verified) |
| S6-03 | Residual risk register (TLS@edge, CSP unsafe-inline styles, CSP reporting, rate limit, SHA-pinned actions, cosign signing) | security-compliance.feature | P1 | IN-PROGRESS |
| S6-04 | Gating audit/deny in CI + gitleaks + SBOM | security-compliance.feature | P0 | IN-PROGRESS (S7 overlap) |
| S6-05 | AI-agent repo surface governance (prompt-injection, `dangerous_inner_html` boundary, excessive agency) | security-compliance.feature | P1 | IN-PROGRESS |
| S6-06 | Rate limiting (per-IP fixed window, config-driven) | security-compliance.feature | P2 | TODO |
| S6-07 | Cosign image signing + provenance (SLSA) | security-compliance.feature | P2 | TODO |

## Stage S7 — Repo hygiene & CI gating (P0)

| ID | Item | Feature | Priority | Status |
|----|------|---------|----------|--------|
| S7-01 | Remove tracked junk: `test-dist/` (18 MB), `test-results/.last-run.json`, `.codegraph/intelligence.db` + gitignore | build-hygiene.feature | P0 | IN-PROGRESS |
| S7-02 | Dedupe Playwright `screenshot.spec.ts` | build-hygiene.feature | P1 | IN-PROGRESS |
| S7-03 | Audit gating in CI (kill `\|\| true`), stale branch trigger, triple-test collapse | build-hygiene.feature | P0 | IN-PROGRESS |
| S7-04 | Coverage job in CI (`--fail-under-lines 90`) | build-hygiene.feature | P0 | IN-PROGRESS |
| S7-05 | harden-runner on publish workflow + fix `cloudbuild.yaml --target runtime` | build-hygiene.feature | P1 | IN-PROGRESS |
| S7-06 | Fresh small deterministic build (clean-wasm, no lazy, no dup, budgets) | build-hygiene.feature | P0 | IN-PROGRESS |
| S7-07 | SHA-pin GitHub Actions (needs verified SHAs) | — | P2 | TODO |

## Stage S8 — Agent / model / skill / context engineering (P2)

| ID | Item | Where | Priority | Status |
|----|------|-------|----------|--------|
| S8-01 | Persona → task → skill → model mapping maintained | docs/PERSONA-MAP.md | P1 | DONE (extended this session) |
| S8-02 | Context engineering via graft graph (`graft ask`, ~82K tok/orientation saved) | docs/PERSONA-MAP.md §Context | P1 | DONE |
| S8-03 | Token economy rules for agents (progressive disclosure, scope-limited fan-out, no full-file reads) | docs/PERSONA-MAP.md §Token | P1 | DONE |
| S8-04 | Gemini via `agy` available as secondary model for bulk/doc tasks | docs/PERSONA-MAP.md §Model routing | P2 | DONE (`/Users/samirparhi-dev/.local/bin/agy` present) |
| S8-05 | Model tier policy: Sonnet 4.6 default, Haiku 4.5 routine, Opus explicit-only | CLAUDE.md §Model policy | P0 | DONE (existing) |

## Explicitly out of scope

| Item | Reason |
|------|--------|
| k6 / locust / load-test harness | User decision 2026-10-25: "k6 is not required here" |
| In-server RPC (gRPC/tonic) or message broker | Architecture is static-site delivery — no async workload exists; adding a broker would be speculative complexity (deletion test fails). If a dynamic backend lands, re-open as S9 with tonic + NATS per `system-design-architecture`. |
