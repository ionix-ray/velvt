---
name: rust-workspace-scaffold
description: Cargo workspace conventions for vaelvet-ui. Single crate, WASM-only target, config-driven TOML, clippy deny lints, hexagonal pattern inside UI. Use when editing Cargo.toml, adding crates, or changing build profile.
---

# Rust Workspace Scaffold — Velvet

Single-crate workspace: `velvet-ui/` → `vaelvet-ui` crate. Targets `wasm32-unknown-unknown` exclusively.

## Workspace Layout

```
velvet/
├── Cargo.toml               # Workspace root — resolver = "2"
├── Cargo.lock
├── rust-toolchain.toml      # Pinned toolchain
├── Dioxus.toml              # dx build config
├── content/site.toml        # Single source of content truth
└── velvet-ui/
    ├── Cargo.toml           # crate: vaelvet-ui
    ├── index.html           # WASM mount point (minimal JS in <style> only)
    ├── assets/theme.css     # Design tokens + layout (compacted, no dead rules)
    └── src/
        ├── main.rs          # Entry — launches dioxus::launch(App)
        ├── lib.rs           # Crate root — pub mod declarations
        ├── prelude.rs       # Re-exports: Site, dioxus::prelude::*, window()
        ├── config.rs        # Site struct — OnceLock, Box<str> fields
        ├── scroll.rs        # web-sys scroll + IntersectionObserver (no js-sys)
        ├── markdown.rs      # pulldown-cmark renderer
        ├── routes/home.rs   # Sole route: 7-panel horizontal scroll
        ├── components/      # 29 panel/UI components (pure reactive, no logic)
        └── theme/tokens.rs  # Design tokens (Rust constants mirroring CSS vars)
```

## Workspace Cargo.toml Rules

```toml
[workspace.lints.rust]
unsafe_code = "deny"          # Hard deny — no exceptions

[workspace.lints.clippy]
unwrap_used   = "deny"
expect_used   = "deny"
panic         = "deny"
todo          = "deny"
unimplemented = "deny"
print_stdout  = "deny"
print_stderr  = "deny"
```

## Config Patterns

```rust
// All TOML-parsed fields: Box<str> (immutable, smaller than String)
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Brand {
    pub name: Box<str>,
    pub tagline: Box<str>,
}

// OnceLock: parse once, serve forever
static SITE: OnceLock<Site> = OnceLock::new();
pub fn load() -> &'static Self {
    SITE.get_or_init(|| toml::from_str(RAW).unwrap_or_default())
}
```

## Build Gate (run before every commit)

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
dx build --platform web --package vaelvet-ui --release
```

## Cross-References

- `.skills/wasm-build-gate/SKILL.md` — WASM compat rules
- `.skills/dioxus-carbon-frontend/SKILL.md` — UI component patterns
- `.skills/tdd-discipline/SKILL.md` — test-first
