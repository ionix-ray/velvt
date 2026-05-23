# CLAUDE.md — Vaelvet PR Agency

Auto-loaded by Claude Code. On resume read in order: `STATE.md`, `TASKS.md` (top), `PROGRESS.md` (tail 3), `memory/MEMORY.md`.

## Brand
- **Name**: Vaelvet
- **Tagline**: *elevate your Presence.*
- **Voice**: cinematic, restrained, premium. Avatar-grade glamour, never neon.
- **Palette** (extracted from logo): see `velvet-ui/src/theme/tokens.rs`. Single source of truth.

## Stack — Locked
- Dioxus 0.7.6 web (WASM), `dioxus-router` 0.7.6
- Rust 1.87 / edition 2024, `wasm32-unknown-unknown`
- `dx` CLI for build/serve; **no Node, no pnpm, no Tailwind, no JS framework**
- CSS: hand-written `velvet-ui/assets/theme.css` (cinematic) + design tokens in `tokens.rs`
- Content: **config-driven** via `content/site.toml` (machine-readable) ⇆ `README.md` (human-readable). Editing either changes the site.
- Tests: `#[test]`, `dioxus-ssr`, Playwright e2e (Node only inside `test-suite/`, never in app)

## Model policy
- Default conversational model: **Sonnet 4.6** (`claude-sonnet-4-6`).
- Routine ops (doc-keeping, file search, lint-fix, simple refactors): **Haiku 4.5** (`claude-haiku-4-5`).
- **No Opus by default.** Only escalate to Opus when explicitly invoked.
- Subagents in `.claude/agents/` pin their model in frontmatter.

## Hard rules
- No `unwrap()` / `expect()` / `panic!()` / `todo!()` / `unimplemented!()`
- No `println!` — use `tracing`
- No lazy loading; all hero assets preloaded in `index.html`
- TDD-first: failing test → implementation → refactor. No prod code before a failing test commit.
- WASM bundle ≤1.5 MB gzipped; theme.css ≤40 KB gzipped
- Lighthouse: Perf ≥90, A11y ≥95, Best Practices ≥95, SEO ≥95
- Logic coverage ≥95%
- Conventional Commits: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`, `chore:`, `style:`

## Commands
```bash
just dev      # dx serve on free port 8080–8100, hot reload
just build    # production WASM build → dist/
just test     # cargo test --workspace
just lint     # cargo fmt --check + cargo clippy --all-targets -- -D warnings
just audit    # cargo audit
just static   # static export to dist/ (post-build hash-fix)
just e2e      # Playwright suite (requires test-suite deps)
just clean    # cargo clean + dist/
```

## Code layout
| What | Where |
|---|---|
| App entry + router | `velvet-ui/src/main.rs` |
| Config loader | `velvet-ui/src/config.rs` (parses `content/site.toml` via `include_str!`) |
| Route components | `velvet-ui/src/routes/*.rs` |
| UI components | `velvet-ui/src/components/<name>.rs` |
| Design tokens | `velvet-ui/src/theme/tokens.rs` |
| Cinematic CSS | `velvet-ui/assets/theme.css` |
| Brand images | `velvet-ui/assets/images/` |
| Content (machine) | `content/site.toml` |
| Content (human) | `README.md` |
| E2E specs | `test-suite/playwright/specs/` |

## Maintainer workflow
**To change site content**: edit `content/site.toml` (or `README.md`), `just build`. Done. No code changes needed for copy/links/case studies.

## Pre-merge checklist
1. `just lint` green
2. `just test` green
3. `just audit` zero critical CVEs
4. WASM bundle ≤1.5 MB
5. STATE.md/TASKS.md/PROGRESS.md updated
6. Conventional Commit message
