---
name: token-optimization
description: Claude Code context budget for Velvet sessions — progressive disclosure, skill trigger map, what to load vs what to skip. Trigger on 'context', 'resume', 'state', 'memory', 'token budget'.
---

# Token Optimization — Velvet

Keep context lean. Load only what the current task needs.

## Progressive Disclosure for Velvet

```
Level 0 (always in context — ~400 tokens):
  - .skills/samir-product-philosophy/SKILL.md   (root rules)
  - STATE.md                                    (current status)
  - PROGRESS.md (last 3 entries)               (what changed)

Level 1 (load on trigger — ~300 tokens each):
  - .skills/INDEX.md                            (trigger map)
  - The ONE skill matching current task
  - The specific source file being edited

Level 2 (load on demand — expensive):
  - Full theme.css (49KB — load only when editing CSS)
  - Full AGENT_DIRECTIVE.md (22KB — load only on boot)
  - Full ops/tofu/*.tf files (load only on infra work)
  - velvet-ui/tests/integration.rs (load only when writing tests)
```

## Context Budget

| Component | Tokens | Rule |
|---|---|---|
| Level 0 files | ~400 | Always |
| Skill (1) | ~300 | Trigger-matched only |
| Source file(s) | ~500 | Current edit target only |
| Conversation history | sliding | Summarize after 8 turns |
| Test output | ~200 | Paste last run only |

## Compression Rules

- State files: use table format, not prose
- `cargo test` output: `[N tests passed, M failed]` — not full output
- `tofu plan`: paste only the diff section, not the full header
- Error messages: first 20 lines only

## Anti-Patterns for Velvet

| Do NOT | Cost |
|---|---|
| Load full theme.css every turn | 49KB = ~12K tokens wasted |
| Load all 29 component files | ~30K tokens wasted |
| Paste full Cargo.lock | 90KB = waste |
| Re-read AGENT_DIRECTIVE.md mid-session | 22KB = load once only |

## Cross-References

- `.skills/token-frugal/SKILL.md` — per-call minimization
- `.skills/graphify/SKILL.md` — module dependency graph for selective loading
- `.skills/samir-product-philosophy/SKILL.md` — boot sequence
