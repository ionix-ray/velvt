# Persona Map — Velvt immersive branch

Reference framework: `/Volumes/tripd/spf` (personas/, skills/, governance/mappings.toml).
Design skills sourced from skills.sh (verified: install count + GitHub stars + source).

## Skill sourcing evidence

| Skill | Source | Installs | Stars | Verdict |
|---|---|---|---|---|
| `frontend-design` | anthropics/skills | 930.6K | 178,795 | official — installed |
| `web-design-guidelines` | vercel-labs/agent-skills | 675.6K | 31,673 | official — installed |
| `high-end-visual-design` | leonxlnx/taste-skill | 388.2K | 90,881 | high adoption — installed |
| `ui-ux-pro-max` | nextlevelbuilder/ui-ux-pro-max-skill | 374.5K | 131,226 | high adoption — installed |
| `impeccable` | pbakaus/impeccable | 298.8K | 72,072 | Paul Bakaus (ex-Google) — installed |
| `SVG Logo Designer` | rknall/claude-skills | 7.2K | 78 | <100 stars — **rejected** |
| `superdesign` | superdesigndev/superdesign-skill | 9.9K | 611 | low adoption — **rejected** |

Rejected below the 1K-install / 100-star floor per `find-skills-discovery` Steps 2–4.

## Persona → task → skill mapping

| Persona | Owns | Skills loaded |
|---|---|---|
| `frontend-lead` | Wordmark lockup, ambient glyph field, `theme.css`, hero tile | `frontend-design`, `high-end-visual-design`, `impeccable`, `ui-ux-pro-max`, `web-design-guidelines` |
| `qa-lead` | Playwright spec migration to the SVG lockup, Rust test suite | `playwright-best-practices`, `testing-strategy` |
| `prompt-lead` | Loop design, context budget, prompt shape | `loop-engineering`, `token-economy`, `context-preservation` |
| `principal-architect` | `graft/` knowledge graph, `[background]` config surface | `system-design-architecture`, `improve-codebase-architecture` |
| `backend-lead` | `config.rs` clamps, SSR render tests, axum server | `tdd-cycle`, `rust-testing` |

## Loop design (loop-engineering)

- Primitive: `ScheduleWakeup`-equivalent — re-run validation only after a file write.
- Cadence: under 270s (warm cache), never 300s dead zone.
- Caps: iteration cap 10, deadline 24h, user-stop sentinel = user says "stop".
- Order per iteration: `fmt` → `clippy -D warnings` → `cargo test` → `dx build` → `playwright`.
  First non-zero exit aborts the iteration (secure-sdlc gate discipline).

## Context engineering

- Graft graph at `graft/` (638 nodes, 413 edges, 56 cards) — query with `graft ask` instead
  of reading files whole. Saves ~82K tokens per orientation pass.
- Design skills load only at the polish phase, never during the red phase, to keep the
  red/green loop lean.

## Model routing (task → model → agent)

| Task class | Model | Agent / persona | Notes |
|---|---|---|---|
| Architecture, threat model, gate decisions | Sonnet 4.6 (default); Opus explicit-only | `principal-architect`, `prompt-lead` | No Opus by default (CLAUDE.md) |
| Rust feature/bugfix (TDD red-green) | Sonnet 4.6 | `backend-lead` + `tdd-cycle`, `rust-testing` | failing test first, every time |
| UI / cinematic design & card port | Sonnet 4.6 | `frontend-lead` + `frontend-design`, `impeccable`, `ui-ux-pro-max`, `high-end-visual-design`, `web-design-guidelines` | skills load at polish phase |
| QA / Playwright specs | Sonnet 4.6 | `qa-lead` + `playwright-best-practices`, `testing-strategy` | |
| Doc-keeping, compliance matrix, changelogs, bulk copy | Haiku 4.5 | routine ops lane | cheapest lane per CLAUDE.md policy |
| Bulk doc/codegen, long-context review | Gemini via `agy` (`~/.local/bin/agy`) | secondary lane | use for high-volume, low-judgment passes |

## Token economy (fan-out discipline)

- Scope-limited subagent prompts: each fan-out agent gets an explicit file-scope
  whitelist and a "report, don't touch out of scope" rule — prevents merge churn
  and keeps orchestrator context small.
- Subagent reports are structured markdown summaries, not file dumps.
- Read via `graft ask` / targeted `grep` first; full-file reads only for files under edit.
- Design/verification skills stay unloaded during red phase (see Loop design).
- Tracking docs are the durable memory: STATE/TASKS/PROGRESS/MEMORY carry state so
  sessions resume without replaying conversation (context-and-memory protocol).
