# Vaelvet Project - Agent Customizations & Rules

## Design & UI/UX Principles (Cinematic UI)

When modifying or generating frontend components (HTML, CSS, Rust/Dioxus), strictly adhere to the following UI/UX guidelines to maintain a "Cinematic and Immersive" aesthetic:

1. **Transparent & Glassmorphic Backgrounds**: Avoid opaque blocks of colors. Always use varying levels of `backdrop-filter: blur()` combined with semi-transparent `rgba` backgrounds (`--bg-nav`, `--glass-bg`) to ensure the underlying dynamic or dark themes bleed through, creating a spatial depth effect.
2. **Sparkling / Dynamic Outlines**: Cards, Tiles, and Process panels must feature dynamic, animated running borders (e.g. `v-border-sparkle`). This gives the page an active, energetic vibe instead of remaining static.
3. **Space-Like / Deep Aesthetics**: Adhere heavily to the `--bg-dark`, `--crimson`, and `--off-white` theme variables. Ensure high contrast while making it look premium.
4. **Fluid Motion & Micro-Animations**: Rely on CSS transitions (`var(--ease)`) for hovers, reveals, and scale transformations. Ensure `prefers-reduced-motion` is always respected, but for users allowing motion, give a 3D gravity or parallax feel.
5. **High-Contrast Clean Glass Stat Grid (Zero Heatmap Animation Clutter)**: The background VELVT animated heatmap grid behind stats has been permanently eliminated. All metrics, stats, and achievements (e.g., 'By the Numbers') must render using clean, transparent 4-column glass stat cards (`.v-stat-grid--four` and `.v-about-stat`). No noisy background grid animations, letterforms, or flashing cell matrices behind text.

## Card Transparency (Permanent Rule — Do NOT Regress)

ALL card backgrounds MUST be fully transparent so the page background bleeds through. This applies in BOTH dark mode and light mode.

- Card root: `background: transparent` always
- Inner pseudo-element: `background: var(--glass-card-fill)` (defined as ≤6% alpha via CSS variable)
- NEVER use: `background: var(--bg-card)`, `background: var(--bg-primary)`, or any hardcoded `rgba(10,4,5,0.6)` type fills
- Test: Playwright checks `computedStyle.backgroundColor` alpha ≤ 0.2 on every card root

## Authentic Cultural Production Photography (Permanent Rule)

All imagery for Workflow (`workflow-01` through `05`) and Experience Studio (`studio-01` through `07`) must feature authentic, culturally grounded Odia and Indian production sets and artistry:
- **`workflow-03-production.webp`**: Must depict an authentic Odissi classical dance performance filmed on an open stage with the Sun Temple of Konark in the background, captured from behind a professional cinema production camera set.
- **`studio-07-creators.webp`**: Must depict a real Indian creator/model filming authentic content from a smartphone on location in nature, capturing natural light and cinematic composition.
- All visual assets must be production-grade WebP images (< 250 KB), responsive, preloaded or styled with clean object-fit and position.

## Full-Tile Panoramic Framing (Permanent Rule — `.v-tile--full`)

For full-width highlight cards such as Creator Seeding (`.v-tile--full`):
- Maximum card height must remain strictly bounded (< 500px on desktop) to prevent dominating the viewport.
- Photo frame height must follow `clamp(170px, 18vw, 235px)` with `border-radius: var(--radius-sm)` and `object-position: center 36%` (optimized for facial/phone subject framing).
- Layout sequence: the image frame MUST be positioned above the title and description text.
- Both `.v-tile--full` and `.v-tile--showcase` modifier classes must be present.

## Unique Peek-In Animations Per Card Type (Permanent Rule)

Each card section uses a DIFFERENT CSS animation keyframe for cinematic variety:
- Process steps: `peek-up` (slide up), `peek-left`, `peek-right` per nth-child position
- Showcase tiles: `peek-left`/`peek-right` alternating by even/odd
- Team cards: `peek-tilt` (slight rotation entrance)
- Case study cards: `peek-scale` (scale from center)
- About stats: `peek-down` (drop from top)

All carry `.v-reveal` class. `prefers-reduced-motion` MUST collapse all animations to 0.001ms.

## Test-Driven Frontend (Playwright) — Updated Workflow

1. **100% Playwright Coverage**: Any change in visual design, card layouts, animations, or DOM structure MUST have an accompanying Playwright test.
2. **Multi-Project Passing Gate**: Run `VAELVET_URL=http://localhost:8080 npx playwright test` across all projects (`chromium`, `webkit`, and `reduced-motion`) and ensure 100% pass before deciding a task is complete. No regressions are acceptable (327+ tests passing).
3. **WebKit Scroll-Snap & Reveal Compatibility**: Reveal assertions in Playwright MUST use coordinate-based window scrolling:
   `container.evaluate((el) => { window.scrollTo(0, window.scrollY + el.getBoundingClientRect().top - window.innerHeight / 3); })`
   rather than `scrollIntoViewIfNeeded()`, to prevent WebKit scroll-snap jumping past the target.
4. **Strict > 90% Rust Logic Coverage**: Code coverage must be verified using `cargo llvm-cov`. Unit logic coverage across the workspace must remain strictly above 90% lines (current: >91%), 92% functions, and 93% regions.
5. **Dual CSS Synchronization**: `velvet-ui/assets/theme.css` and `velvet-ui/public/assets/theme.css` must remain 100% byte-for-byte identical.
6. **Use Browser Tool**: Take screenshots with the browser subagent to visually verify every section in BOTH dark mode and light mode, and at mobile (375px) and desktop (1280px) viewports.

## Project Workflow (Permanent Rule)

1. **Branching & Pull Requests**: All future modifications, new features, bug fixes, or structural changes MUST occur on a separate feature branch. 
2. **No Direct Commits**: Do not commit directly to `main` or the current stable deployment branch (`refactor/production-grade`).
3. **Pull Request Protocol**: Changes must be submitted via a pull request (or a completely separate task request) so they can be reviewed and tested in isolation before being merged into the stable base.
