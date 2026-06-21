# Vaelvet — coverage goals

| Layer       | Floor   | Tooling                                                   |
|-------------|---------|-----------------------------------------------------------|
| Unit (Rust) | ≥ 90 %  | `cargo llvm-cov --workspace --html`                      |
| SSR render  | 100 %   | `velvet-ui/tests/render.rs` — every section asserted     |
| E2E         | Golden path + reduced-motion + LCP budget | Playwright |
| WASM bundle | ≤ 1.5 MB gz | `just size`                                          |
| Lighthouse  | Perf ≥ 90, A11y ≥ 95, BP ≥ 95, SEO ≥ 95 | manual / CI    |

## Critical assertions (must not regress)
1. `site.md` parses (fenced TOML blocks extract + deserialize) and all sections populate.
2. Hero headline visible within 2.5 s LCP.
3. CSP header present; no inline scripts beyond curtain CSS.
4. Reduced-motion media query disables parallax.
5. No `unwrap`/`expect`/`panic` in shipping binary (`just lint`).
