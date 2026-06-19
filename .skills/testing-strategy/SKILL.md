---
name: testing-strategy
description: Test pyramid for vaelvet-ui — unit tests in-module, integration SSR smoke in velvet-ui/tests/, property tests for parsers. Coverage targets and CI structure. Trigger on 'test', 'coverage', 'smoke', 'integration', 'SSR'.
---

# Testing Strategy — Velvet

Test pyramid adapted for a WASM-only Dioxus project.

## Test Pyramid

```
           ┌─────────────┐
           │  (future)   │  E2E: Playwright on dist/ (not yet wired)
           │  Playwright │
           ├─────────────┤
           │ Integration │  SSR smoke — velvet-ui/tests/integration.rs
           ├─────────────┤
           │    Unit     │  #[cfg(test)] in config.rs, tokens.rs, scroll.rs
           └─────────────┘
  Cross: property (proptest for config parsers)
```

## Coverage Targets

| Module | Line | Notes |
|---|---|---|
| `config.rs` | ≥ 90% | All struct fields tested |
| `theme/tokens.rs` | 100% | Constants — trivial |
| `scroll.rs` | ≥ 80% | Web API calls skip in test env |
| Components | ≥ 80% | SSR renders without panic |

## Unit Tests (in-module)

```rust
// config.rs — existing + expand
#[test] fn site_toml_parses() { ... }
#[test] fn site_toml_all_required_fields_non_empty() { ... }
#[test] fn site_toml_nav_has_entries() { ... }
#[test] fn site_toml_footer_columns_non_empty() { ... }
#[test] fn site_load_is_idempotent() { ... }

// tokens.rs — existing + expand
#[test] fn palette_hex_values_are_well_formed() { ... }
#[test] fn motion_constants_are_non_empty() { ... }
#[test] fn spacing_constants_end_in_rem_or_zero() { ... }
```

## Integration Tests (SSR smoke)

```rust
// velvet-ui/tests/integration.rs
use dioxus_ssr::render;
use vaelvet_ui::{components::hero_panel::HeroPanel, config::Site};

#[test]
fn hero_panel_renders_without_panic() {
    let site = Site::load().clone();
    let html = render(rsx! { HeroPanel { site } });
    assert!(html.contains("v-panel"), "panel class missing");
}

// One test per panel: hero, about, process, studio, cases, cta, footer
```

## Commands

```bash
cargo test --workspace --all-targets         # all tiers
cargo test -p vaelvet-ui --test integration  # SSR smoke only
cargo test -p vaelvet-ui config::tests       # unit only
cargo tarpaulin --workspace --out Html       # coverage report
```

## CI Mapping

```yaml
# In pr-gate.yml
- run: cargo test --workspace --all-targets
# In deploy-pages.yml (already present)
- run: cargo test --workspace --all-targets
```

## Cross-References

- `.skills/tdd-discipline/SKILL.md` — test-first discipline
- `.skills/dioxus-carbon-frontend/SKILL.md` — component patterns
- `.skills/code-review-gate/SKILL.md` — coverage gate in PR review
