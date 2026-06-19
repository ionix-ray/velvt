---
name: tdd-discipline
description: Strict Red-Green-Refactor TDD — no implementation without a failing test first. Applies to every feature, bug fix, and refactor in vaelvet-ui. Pairs with testing-strategy and code-review-gate.
---

# TDD Discipline — Velvet

Iron law: **NO IMPLEMENTATION WITHOUT A FAILING TEST FIRST.**

## The RED-GREEN-REFACTOR Cycle

1. **RED** — Write the smallest failing test. Run it. See it fail. Commit.
2. **GREEN** — Write minimum code to pass that one test. Run. Green. Commit.
3. **REFACTOR** — Improve without changing behaviour. All tests still green. Commit.

## Velvet Test Commands

```bash
# Full suite (fastest feedback)
cargo test --workspace --all-targets

# Specific module
cargo test -p vaelvet-ui config::tests
cargo test -p vaelvet-ui theme::tokens::tests

# Integration (SSR smoke)
cargo test -p vaelvet-ui --test integration

# Before merge (full gate)
cargo fmt --all -- --check && \
cargo clippy --workspace -- -D warnings && \
cargo test --workspace --all-targets && \
dx build --platform web --package vaelvet-ui --release
```

## Velvet-Specific Applies To

- Config struct changes in `config.rs` → test new field is non-empty
- Token constant changes in `tokens.rs` → test value format is valid CSS
- New component → SSR smoke test in `velvet-ui/tests/integration.rs`
- Bug in scroll/navigation → regression test in `velvet-ui/tests/`
- Tofu module change → `tofu validate` + `tofu plan` must show 0 changes

## Rationalizations to Refuse

| "But..." | Reality |
|---|---|
| "It's just a CSS compaction" | If it touches Rust code, write a test |
| "SSR tests are hard" | `dioxus-ssr` renders to string — 5 lines per test |
| "The TOML parser handles it" | Add a test for the new field anyway |

## Commit Style

```
test: SSR smoke for ProcessPanel renders without panic

  - Adds velvet-ui/tests/integration.rs::process_panel_renders
  - Currently fails: component not exported from lib.rs
```

```
feat: export ProcessPanel from lib.rs

  - Closes: above test
  - Tests: integration::process_panel_renders now green
```

## Cross-References

- `.skills/testing-strategy/SKILL.md` — test pyramid
- `.skills/code-review-gate/SKILL.md` — RED commit must be visible in git log
