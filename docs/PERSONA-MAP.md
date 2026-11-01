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
| `frontend-lead` | Wordmark lockup, ambient glyph field, `theme.css`, hero tile, cinematic cards | `cinematic-ui`, `frontend-design`, `high-end-visual-design`, `impeccable`, `ui-ux-pro-max`, `web-design-guidelines`, `modern-web-guidance` |
| `visual-asset-artist` | Cultural Odia/Indian photography briefs, WebP generation, responsive framing | `image-generator` subagent, visual media compression |
| `qa-lead` | Playwright multi-project suite, WebKit reveal compatibility, llvm-cov logic coverage | `cinematic-ui`, `web-design-guidelines`, `chrome-devtools`, `a11y-debugging` |
| `prompt-lead` | Loop design, context budget, prompt shape, multimodal fan-out | `loop-engineering`, `token-economy`, `context-preservation` |
| `principal-architect` | Knowledge graph, `[background]` config surface, multimodal fan-out system | `system-design-architecture`, `improve-codebase-architecture` |
| `backend-lead` | `config.rs` clamps, SSR render tests, axum server, security headers | `tdd-cycle`, `rust-testing`, `modern-web-guidance` |

See `docs/MULTIMODAL_FANOUT_SYSTEM.md` for full fan-out architecture specification.

## Loop design (loop-engineering)

- Primitive: `ScheduleWakeup`-equivalent — re-run validation only after a file write.
- Cadence: under 270s (warm cache), never 300s dead zone.
- Caps: iteration cap 10, deadline 24h, user-stop sentinel = user says "stop".
- Order per iteration: `fmt` → `clippy -D warnings` → `cargo test` → `cargo llvm-cov (≥90%)` → `dx build` → `playwright (3 projects)`.
  First non-zero exit aborts the iteration (secure-sdlc gate discipline).

## Context engineering

- Graft graph at `graft/` (638 nodes, 413 edges, 56 cards) — query with `graft ask` instead
  of reading files whole. Saves ~82K tokens per orientation pass.
- Design skills load only at the polish phase, never during the red phase, to keep the
  red/green loop lean.

## Model routing (task → model → agent)

| Task class | Antigravity / AGY Model | Claude Model | Agent / persona | Notes |
|---|---|---|---|---|
| Architecture, threat model, gate decisions | `pro` / `inherit` | `claude-sonnet-4-6` | `principal-architect`, `prompt-lead` | Deep reasoning lane |
| Rust feature/bugfix (TDD red-green) | `pro` / `inherit` | `claude-sonnet-4-6` | `backend-lead` | failing test first, every time |
| UI / cinematic design & card port | `pro` / `inherit` | `claude-sonnet-4-6` | `frontend-lead` | `cinematic-ui` + design skills |
| Visual asset generation | `image-generator` | Midjourney / Imagen | `visual-asset-artist` | Authentic cultural Odia/Indian photography |
| QA / Playwright multi-project specs | `pro` / `inherit` | `claude-sonnet-4-6` | `qa-lead` | Chromium, WebKit, Reduced Motion |
| Fast targeted refactors, regex, queries | `flash` | `claude-haiku-4-5` | `code-writer` / fast lane | High token efficiency |
| Doc-keeping, compliance matrix, changelogs | `flash_lite` | `claude-haiku-4-5` | `doc-keeper` | Cheapest routine ops lane |

## Token economy (fan-out discipline)

- Scope-limited subagent prompts: each fan-out agent gets an explicit file-scope
  whitelist and a "report, don't touch out of scope" rule — prevents merge churn
  and keeps orchestrator context small.
- Subagent reports are structured markdown summaries, not file dumps.
- Read via `graft ask` / targeted `grep` first; full-file reads only for files under edit.
- Design/verification skills stay unloaded during red phase (see Loop design).
- Tracking docs are the durable memory: STATE/TASKS/PROGRESS/MEMORY carry state so
  sessions resume without replaying conversation (context-and-memory protocol).
