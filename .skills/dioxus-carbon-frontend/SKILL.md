---
name: dioxus-carbon-frontend
description: Dioxus 0.7.x patterns for vaelvet-ui — WASM-only, Signals state, config-driven components, web-sys DOM access, no js-sys. Trigger on 'Dioxus', 'UI', 'frontend', 'component', 'rsx', 'Signal'. Pairs with wasm-build-gate.
---

# Dioxus Carbon Frontend — Velvet

WASM-only Dioxus 0.7 patterns for `vaelvet-ui`. Design and animations frozen — optimisation only.

## State Management

```rust
// In routes/home.rs — all navigation state lives here
let mut current_panel = use_signal(|| 0usize);
let mut menu_open     = use_signal(|| false);
let theme             = use_signal(|| "dark".to_string());
let loader_hidden     = use_signal(|| false);

// Derived: no Memo needed — simple bool expressions in rsx!
let hint_hidden     = *current_panel.read() >= PANEL_LABELS.len() - 1;
let is_footer_panel = *current_panel.read() >= PANEL_LABELS.len() - 1;
```

## Component Patterns

```rust
// Pure reactive — no logic in components, all from Site
#[component]
pub fn HeroPanel(site: Site) -> Element {
    rsx! {
        section { class: "v-panel", id: "home",
            // ... reads site.hero.* only, no signals
        }
    }
}

// Avoid: inline closures that capture Signal — prefer passing callbacks
// Good: on_navigate: impl Fn(usize) + 'static
// Bad:  onclick: move |_| current_panel.set(x)  (in child)
```

## web-sys DOM Access (no js-sys)

```rust
// Scroll panel — pure web-sys
fn scroll_to_panel(idx: usize) {
    let Some(win) = web_sys::window() else { return; };
    let Some(doc) = win.document() else { return; };
    let Some(panels) = doc.query_selector(".v-panels").ok().flatten() else { return; };
    let target = idx as f64 * panels.client_width() as f64;
    let opts = web_sys::ScrollToOptions::new();
    opts.set_left(target);
    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
    panels.scroll_to_with_scroll_to_options(&opts);
}

// IntersectionObserver threshold — Float64Array, not js-sys::Array
use wasm_bindgen::JsValue;
let thresh_arr = js_sys::Array::new(); // If js-sys still needed
// PREFERRED: use web_sys::js_sys alias, or pass as JsValue directly
```

## CSS Convention

- All animation classes stay in `assets/theme.css` — do NOT move to Rust
- Rust only applies/removes classes via `web-sys::Element::class_list()`
- Design tokens in `theme/tokens.rs` are Rust constants mirroring CSS vars — for compile-time validation only

## Build Commands

```bash
dx serve --platform web --package vaelvet-ui --hot-reload true  # dev
dx build --platform web --package vaelvet-ui --release          # prod
dx build --platform web --package vaelvet-ui                    # WASM compat check
```

## WASM Compatibility Rules

| Do NOT use | Use instead |
|---|---|
| `std::fs` | `web_sys` storage |
| `tokio::time` | `gloo-timers` |
| `js-sys` (where possible) | `web-sys` + `wasm-bindgen::JsValue` |
| `Closure::forget()` in loops | Static closures or scoped lifetimes |

## Cross-References

- `.skills/wasm-build-gate/SKILL.md` — build gate
- `.skills/rust-workspace-scaffold/SKILL.md` — crate layout
- `.skills/tdd-discipline/SKILL.md` — SSR smoke tests via dioxus-ssr
