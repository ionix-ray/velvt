<!-- Master ecosystem rules — read first -->
@/Volumes/hex/skills/MASTER_PROMPT.md

# CLAUDE.md — Vaelvet PR Agency

Auto-loaded by Claude Code. On resume read in order: `STATE.md`, `TASKS.md` (top), `PROGRESS.md` (tail 3), `memory/MEMORY.md`.

## Brand
- **Name**: Vaelvet
- **Tagline**: *elevate your Presence.*
- **Voice**: cinematic, restrained, premium. Avatar-grade glamour, never neon.
- **Palette** (extracted from logo): see `velvet-ui/src/theme/tokens.rs`. Single source of truth.

## Stack — Locked
- Dioxus 0.7.6 web (WASM), `dioxus-router` 0.7.6
- Rust 1.88 / edition 2024, `wasm32-unknown-unknown`
- `dx` CLI for build/serve; **no Node, no pnpm, no Tailwind, no JS framework**
- CSS: hand-written `velvet-ui/assets/theme.css` (cinematic) + design tokens in `tokens.rs`
- Content: **single source**, `content/site.md` — markdown, one `## Section` heading per content field, each followed by a fenced ` ```toml ` block. Edit it, `just build`. No code changes for copy/links/case studies.
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
- WASM bundle <=1.5 MB gzipped; theme.css <=40 KB gzipped
- Lighthouse: Perf >=90, A11y >=95, Best Practices >=95, SEO >=95
- Logic coverage >=95%
- Conventional Commits: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`, `chore:`, `style:`

## Commands

```bash
# Development
just dev              # dx serve on free port 8080-8100, hot reload
just build            # production WASM build -> dist/ (auto-cleans stale artifacts)
just test             # cargo test --workspace
just lint             # cargo fmt --check + cargo clippy --all-targets -- -D warnings
just fmt              # auto-fix formatting
just audit            # cargo audit (CVEs)
just e2e              # Playwright suite (requires test-suite deps)
just clean            # cargo clean + dist/ + deployment/

# Security tooling
just gitleaks         # gitleaks secret scan (history + working tree)
just sbom             # CycloneDX SBOM (requires cargo-cyclonedx)

# Coverage
just coverage         # llvm-cov summary (>=90% lines)
just coverage-html    # full HTML report -> target/llvm-cov-html/

# Container (Podman)
just container-up          # WASM build -> stage -> container image -> run -> browser
just container-build-fresh # clean rebuild (no cache, for production)
just container-run         # start existing image on :8080
just container-stop        # stop running container
just container-tag         # tag :latest with semver + git SHA
just container-size        # image size + WASM bundle size report

# CI pipeline
just ci                      # full local pipeline (lint -> test -> build -> container -> e2e)
just ci-local                # mirrors GitHub Actions pipeline exactly
just ci-local --skip-wasm    # Rust-only changes (faster)
```

## Code layout
| What | Where |
|---|---|
| App entry + router | `velvet-ui/src/main.rs` |
| Config loader | `velvet-ui/src/config.rs` (parses `content/site.md` via `include_str!`) |
| Route components | `velvet-ui/src/routes/*.rs` |
| UI components | `velvet-ui/src/components/<name>.rs` |
| **Cinematic card** | `velvet-ui/src/components/card_step.rs` <- single source of truth |
| Design tokens | `velvet-ui/src/theme/tokens.rs` |
| Cinematic CSS | `velvet-ui/assets/theme.css` |
| Brand images | `velvet-ui/assets/images/` |
| Content (single source) | `content/site.md` |
| E2E specs | `test-suite/playwright/specs/` |
| CI pipeline | `.github/workflows/deploy-pages.yml` |
| Local CI script | `scripts/ci-local.sh` |

## CardStep Component
`CardStep` is the **single reusable card** for all grid sections (How We Work, Studio Showcase, etc.).

```rust
CardStep {
    logo: site.hero.logo.to_string(),  // badge in top-left corner
    title: step.title.to_string(),
    body: step.body.to_string(),
    num: Some(step.num.to_string()),   // optional step number
    tag: None,                          // optional eyebrow label (use instead of num)
    class_extra: "".to_string(),        // optional extra BEM modifier class
    delay_ms: 120,                      // entrance animation stagger (ms)
}
```

**Design**: transparent background (adapts dark/light), sparkling running border animation,
logo badge top-left, text colour driven by `prefers-color-scheme`.

**Important**: Config fields in `config.rs` are `Box<str>`. Always call `.to_string()` when
passing them as `CardStep` props.

## CI/CD Pipeline
- **GitHub Actions**: `.github/workflows/deploy-pages.yml` -- 5-job pipeline
  - Security audit (cargo-audit + cargo-deny)
  - Test & Lint (fmt, clippy -D warnings, unit + integration tests)
  - Build (WASM + .nojekyll + CNAME + 404.html + SEO files)
  - Deploy to GitHub Pages (`velvt.live`)
  - Post-deploy verification
- **PR gate**: `.github/workflows/pr-gate.yml` -- runs on every PR to main
- **deny.toml**: license + advisory policy (deny GPL/AGPL, allow MIT/Apache-2.0/BSD)
- **Deployment URL**: https://velvt.live/

## Maintainer workflow
**To change site content**: edit `content/site.md`, `just build`. Done. No code changes needed for copy/links/case studies.

## Pre-merge checklist
1. `just ci-local --skip-wasm` green (fast Rust checks)
2. `just lint` green
3. `just test` green (all 142 tests)
4. `just audit` zero critical CVEs
5. `just build` succeeds; WASM bundle <=1.5 MB
6. WASM bundle <=1.5 MB gzipped: `ls -lh dist/assets/*.wasm`
7. `STATE.md`/`TASKS.md`/`PROGRESS.md` updated
8. Conventional Commit message
