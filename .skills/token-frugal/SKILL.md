---
name: token-frugal
description: Per-call LLM token discipline for Velvet tasks. Terse output, no ceremony, structured formats. Trigger on 'cost', 'tokens', 'context window', 'budget', 'reduce'.
---

# Token Frugal — Velvet

Minimize tokens on every interaction. The cheapest token is the one not sent.

## Output Style Rules (when acting on this skill)

1. No preamble — answer directly
2. Code blocks only for code — no prose wrappers
3. Diffs over full file pastes (unless file is <50 lines)
4. `[test: 14 passed, 0 failed]` not full test output
5. `[tofu plan: 0 to add, 0 to change, 0 to destroy]` not full plan
6. Table > bullet list > prose (in that order of preference)

## Velvet-Specific Savings

| Task | Terse approach |
|---|---|
| "Show me config.rs changes" | Diff of changed lines only |
| "Run tests" | Summary line + any failures only |
| "Check tofu" | `tofu validate: OK \| tofu plan: 0 changes` |
| "Build status" | `dx build: OK (dist/ 3.2MB)` or error line |
| "CSS compaction" | Before/after byte count + list of removed rules |

## When Verbosity IS Justified

- New skill creation (full SKILL.md needed)
- First-time explanation of a pattern
- Error with non-obvious root cause (paste full trace)
- Security finding (full detail required)

## Cross-References

- `.skills/token-optimization/SKILL.md` — context budget strategy
- `.skills/graphify/SKILL.md` — selective module loading
