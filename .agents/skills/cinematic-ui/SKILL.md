---
name: cinematic-ui
description: Enforces the transparent, glassmorphic cinematic UI design system and mandatory Playwright test-driven development for all frontend changes.
---

# Cinematic UI Design & Testing Skill

This skill enforces our cinematic design aesthetic for the frontend (`velvet-ui`) and mandates a strict test-first progression for any visual modifications. **Always read and follow this skill when modifying the user interface.**

## Design Aesthetics
1. **Cinematic Glassmorphism:** All cards, panels, and stats must use `backdrop-filter: blur(10px)` and **fully transparent** backgrounds (i.e. `background: transparent` on the card root). Utilize `.v-card-modern`.
2. **Sparkle Borders:** Any highlighted card (stats, process step, team member, case card) MUST include the animated border effect. Add the `<div class="v-sparkle-border"></div>` element inside the container, and ensure the container has `position: relative`.
3. **Dynamic Backgrounds:** Sections should feel alive. Use `.v-cinematic-bg` for subtle particles/mesh gradients behind content instead of flat backgrounds.
4. **Heatmap & Data Viz:** For statistical representation, favor creative animated elements (like `.v-heatmap-grid`) over static blocks.

## Transparency Rules (Non-Negotiable)
**Every card MUST be see-through.** The page background bleeds through all cards in every theme.

| Rule | Correct | Forbidden |
|------|---------|-----------|
| Card root background | `background: transparent` | `background: var(--bg-card)` |
| Inner pseudo-element fill | `background: var(--glass-card-fill)` (<=6% alpha) | `background: rgba(10,4,5,0.6)` or `var(--bg-primary)` |
| Border | `outline: 1px solid var(--glass-border-subtle)` | Hardcoded rgba border |
| Light mode text | `color: var(--text-primary)` | Hardcoded `color: var(--off-white)` |

### CSS Variables for Glass (defined in `theme.css`)
```css
/* Light mode (:root) */
--glass-card-rgb: 240, 232, 220;
--glass-card-alpha: 0.06;
--glass-card-fill: rgba(var(--glass-card-rgb), var(--glass-card-alpha));
--glass-border-rgb: 181, 42, 42;
--glass-border-subtle: rgba(var(--glass-border-rgb), 0.15);

/* Dark mode ([data-theme="dark"]) */
--glass-card-rgb: 10, 4, 5;
--glass-card-alpha: 0.06;
--glass-card-fill: rgba(var(--glass-card-rgb), var(--glass-card-alpha));
--glass-border-rgb: 212, 62, 62;
--glass-border-subtle: rgba(var(--glass-border-rgb), 0.18);
```

## Unique Scroll-Reveal Peek-In Animations

Each card TYPE uses a different entrance animation:

| Card | Animation | Keyframe |
|------|-----------|---------|
| `.v-process__step:nth-child(3n+1)` | Slide up + scale | `peek-up` |
| `.v-process__step:nth-child(3n+2)` | Slide from left | `peek-left` |
| `.v-process__step:nth-child(3n+3)` | Slide from right | `peek-right` |
| `.v-showcase__grid .v-process__step:nth-child(even)` | From left | `peek-left` |
| `.v-showcase__grid .v-process__step:nth-child(odd)` | From right | `peek-right` |
| `.v-team-card` | Tilt + slide (4deg rotate) | `peek-tilt` |
| `.v-case-card` | Scale from center | `peek-scale` |
| `.v-about-stat` | Drop from top | `peek-down` |

### Animation Rules
- Class: `.v-reveal` (required on ALL card elements)
- Duration: 0.65s-0.75s with `var(--ease-out)` timing
- Fill: `both` — card stays invisible until animation runs
- Stagger: via `animation-delay` inline style (`delay_ms` prop in `CardStep`)
- `prefers-reduced-motion`: MUST set `animation-duration: 0.001ms !important`

### Sparkle Border Speed Variation (rotation rhythm per section)
- `.v-process__step .v-sparkle-border::before` -> `3.5s`
- `.v-about-stat .v-sparkle-border::before` -> `5s`
- `.v-case-card .v-sparkle-border::before` -> `6s`
- `.v-team-card .v-sparkle-border::before` -> `4s` (default)

## Testing & Validation Rules (Mandatory)
Before starting any design implementation:
1. **Check Existing Tests First:** All UI components have assertions in `test-suite/playwright/specs/home.spec.ts`.
2. **Never Break Layout Checks:** Tests explicitly measure `.boundingBox()`. Ensure responsive layouts for `mobile-375` and `tablet-768` pass.
3. **100% Coverage:** Any new visual effect MUST have a corresponding Playwright test.
4. **Containerized Testing:** `VAELVET_URL=http://localhost:8080 npx playwright test --project=chromium` from `test-suite/playwright/`.

### Key Transparency Tests (must always pass)
- `"cards: v-process__step has transparent background (no opaque fill)"` -- alpha <= 0.2
- `"cards: v-team-card transparent in dark and light mode"` -- alpha <= 0.2 in both themes
- `"cards: v-process__step h4 text is readable in light mode (dark enough)"` -- luminance < 200
- `"animations: reduced-motion disables peek-in card animations"` -- duration <= 0.01s
- `"cards: all v-process__step and v-team-card have v-sparkle-border attached"`

**Do not present the work to the user until all tests pass.**
