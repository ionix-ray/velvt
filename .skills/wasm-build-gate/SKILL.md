---
name: wasm-build-gate
description: Every build must compile to wasm32-unknown-unknown. Catches platform regressions. Mandatory before any UI merge for vaelvet-ui. Trigger on 'WASM', 'wasm-opt', 'wasm32', 'dx build'.
---

# WASM Build Gate — Velvet

Every UI change must pass `dx build --platform web --package vaelvet-ui` before merge.

## Mandatory Check

```bash
# Before every merge — BOTH must pass
cargo check --workspace                                      # native check
dx build --platform web --package vaelvet-ui --release      # WASM build
```

## Known WASM Incompatibilities in vaelvet-ui

| Pattern | Status | Fix |
|---|---|---|
| `js-sys::Array::of1()` | **Remove** | Use `wasm_bindgen::JsValue` directly |
| `std::time::Instant` | Not used (OK) | Use `web_sys::Performance` if needed |
| `tokio::time` | Not used (OK) | Use `gloo-timers` |
| `reqwest::ClientBuilder::timeout()` | Not used (OK) | N/A |
| `Closure::forget()` | Used in scroll.rs | Acceptable — page lifetime |

## Dioxus.toml Config

```toml
[web.wasm_opt]
level = "z"             # size-optimised, already set

[web.resource]
style  = []             # resources injected from Rust via asset!()
script = []             # NO external scripts
```

## JS Glue Budget: ≤ 50 LOC

The `dx build` output contains exactly **one** `.js` file: the `wasm-bindgen` bootstrap. After build:

```bash
wc -l dist/assets/*.js          # Must be ≤ 50 LOC
grep -c "function\|=>" dist/assets/*.js  # Sanity check — no app logic
```

If LOC exceeds 50: check that no `#[wasm_bindgen]` exports with JS shims have been added.

## Release Profile

```toml
[profile.release]
opt-level       = "z"
lto             = "fat"
codegen-units   = 1
panic           = "abort"
strip           = true
overflow-checks = false   # safe for UI code
```

## CI Gate

```yaml
wasm-build:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with: { toolchain: "1.88.0", targets: "wasm32-unknown-unknown" }
    - run: cargo install dioxus-cli --version "=0.7.6" --locked
    - run: sudo apt-get install -y binaryen
    - run: dx build --platform web --package vaelvet-ui --release
    - run: "wc -l dist/assets/*.js | awk 'END{if($1>50)exit 1}'"
```

## Cross-References

- `.skills/dioxus-carbon-frontend/SKILL.md` — component patterns
- `.skills/rust-workspace-scaffold/SKILL.md` — build profile
- `.skills/container-distroless/SKILL.md` — container for WASM assets
