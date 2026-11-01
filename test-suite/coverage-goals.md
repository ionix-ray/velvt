# Vaelvet — coverage goals

| Layer       | Floor   | Tooling                                                   |
|-------------|---------|-----------------------------------------------------------|
| Unit (Rust) | ≥ 90 %  | `cargo llvm-cov --workspace --html`                      |
| SSR render  | 100 %   | `velvet-ui/tests/render.rs` — every section asserted     |
| E2E         | Golden path + reduced-motion + LCP budget | Playwright |
| WASM bundle | ≤ 1.5 MB gz | `just container-size`                         |
| Lighthouse  | Perf ≥ 90, A11y ≥ 95, BP ≥ 95, SEO ≥ 95 | manual / CI    |

## Critical assertions (must not regress)
1. `site.md` parses (fenced TOML blocks extract + deserialize) and all sections populate.
2. Hero headline visible within 2.5 s LCP.
3. CSP header present; no inline scripts beyond curtain CSS.
4. Reduced-motion media query disables parallax.
5. No `unwrap`/`expect`/`panic` in shipping binary (`just lint`).
6. All cards maintain transparent background (alpha <= 0.2) in both dark and light modes.
7. Background VELVT animated heatmap grid is deleted; stats render in clean 4-column glass cards (`.v-stat-grid--four`).
8. Workflow (`workflow-01` to `05`) and Experience Studio (`studio-01` to `07`) feature authentic Odia/Indian cultural production photography.
9. Panoramic `.v-tile--full` cards maintain bounded height (< 500px), photo frame height `clamp(170px, 18vw, 235px)`, `object-position: center 36%`.
10. WebKit scroll-snap reveal testing uses window-level coordinate scrolling to avoid oversnap.
11. Multi-project Playwright suite: 100% passing across chromium, webkit, and reduced-motion (327+ tests).
