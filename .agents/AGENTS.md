# Vaelvet Project - Agent Customizations & Rules

## Design & UI/UX Principles (Cinematic UI)

When modifying or generating frontend components (HTML, CSS, Rust/Dioxus), strictly adhere to the following UI/UX guidelines to maintain a "Cinematic and Immersive" aesthetic:

1. **Transparent & Glassmorphic Backgrounds**: Avoid opaque blocks of colors. Always use varying levels of `backdrop-filter: blur()` combined with semi-transparent `rgba` backgrounds (`--bg-nav`, `--glass-bg`) to ensure the underlying dynamic or dark themes bleed through, creating a spatial depth effect.
2. **Sparkling / Dynamic Outlines**: Cards, Tiles, and Process panels must feature dynamic, animated running borders (e.g. `v-border-sparkle`). This gives the page an active, energetic vibe instead of remaining static.
3. **Space-Like / Deep Aesthetics**: Adhere heavily to the `--bg-dark`, `--crimson`, and `--off-white` theme variables. Ensure high contrast while making it look premium.
4. **Fluid Motion & Micro-Animations**: Rely on CSS transitions (`var(--ease)`) for hovers, reveals, and scale transformations. Ensure `prefers-reduced-motion` is always respected, but for users allowing motion, give a 3D gravity or parallax feel.
5. **No Clutter & Heatmap Visualizations**: When dealing with metrics or stats (like "By the Numbers"), use compact, GitHub-heatmap-style layouts rather than huge singular card blocks. It provides a technical, data-driven, yet beautiful footprint on the screen.

## Test-Driven Frontend (Playwright)

1. **100% Playwright Coverage**: Any change in visual design, card layouts, animations, or DOM structure MUST have an accompanying Playwright test checking bounds, computed styles, visibility, and layout constraints.
2. **Visual Checks before Progress**: Always run the E2E test suite locally using `VAELVET_URL=http://localhost:8087 npx playwright test` after a UI change and ensure ALL tests pass before deciding a task is complete. No regressions are acceptable.
6. **Native Full-Color Assets**: When a full-color transparent asset (like a primary logo) is provided, render it using a standard `<img>` tag without any `mask-image`, CSS color overlays, or distorting 3D/hover animations unless explicitly instructed. Let the native brand colors show.
7. **Wider Layout Spacing**: To avoid a compacted, cramped view, container `max-width` (e.g. `.v-container`) must be generous (e.g., `90rem` or `1440px`) to utilize screen space effectively and create a zoom-out effect rather than squishing content into the center.
