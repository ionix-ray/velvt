# Velvet Skills Index

> In-repo skills. Loaded progressively — **never** load all at once. See `graphify/SKILL.md` for protocol.

## Boot Sequence

1. `samir-product-philosophy/SKILL.md` — hard rules + boot sequence
2. This file — find the matching skill
3. ONE skill matching the current task

## Trigger Map

| If the task involves… | Load skill |
|---|---|
| Session start, boot, context resume | `samir-product-philosophy` |
| `Cargo.toml`, workspace, crate layout, build profile | `rust-workspace-scaffold` |
| Dioxus component, `rsx!`, Signal, `use_effect`, `web-sys` | `dioxus-carbon-frontend` |
| `dx build`, WASM compat, `wasm32-unknown-unknown`, JS glue LOC | `wasm-build-gate` |
| `Containerfile`, `compose.yml`, distroless, `podman`, `docker` | `container-distroless` |
| Writing a test, TDD cycle, RED-GREEN-REFACTOR | `tdd-discipline` |
| Test pyramid, coverage, SSR smoke, integration tests | `testing-strategy` |
| Context window, what files to load, token budget, resume | `token-optimization` + `graphify` |
| LLM cost, verbose output, terse format | `token-frugal` |
| CI workflow, GitHub Actions, deploy, publish | `autonomous-deploy-cicd` |
| GKE, K8s, resource limits, node pool, scaling | `infrastructure-scalability` |
| `cargo audit`, CVE, CSP headers, secrets, supply chain | `defensive-security` |
| PR review, merge checklist, gate | `code-review-gate` |
| `tofu`, `terraform`, `ops/tofu/`, `tofu import`, `tofu plan` | `tofu-module-registry` |
| Context graph, selective loading, which files to load | `graphify` |

## Skill Catalog

| Skill | Category | Status |
|---|---|---|
| `samir-product-philosophy` | Process | Adapted for Velvet |
| `rust-workspace-scaffold` | Build | Adapted for Velvet |
| `dioxus-carbon-frontend` | Build | Adapted for Velvet |
| `wasm-build-gate` | Build | Adapted for Velvet |
| `container-distroless` | Build | Adapted for Velvet |
| `tdd-discipline` | Testing | Adapted for Velvet |
| `testing-strategy` | Testing | Adapted for Velvet |
| `token-optimization` | Cost | Adapted for Velvet |
| `token-frugal` | Cost | Adapted for Velvet |
| `autonomous-deploy-cicd` | Delivery | Adapted for Velvet |
| `infrastructure-scalability` | Reliability | Adapted for Velvet |
| `defensive-security` | Security | Adapted for Velvet |
| `code-review-gate` | Process | **New — Velvet-specific** |
| `graphify` | Cost | **New — Velvet-specific** |
| `tofu-module-registry` | Delivery | **New — Velvet-specific** |

## Source

Original 27 skills: `~/Library/Mobile Documents/com~apple~CloudDocs/skills/`
