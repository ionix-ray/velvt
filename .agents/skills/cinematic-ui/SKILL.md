---
name: cinematic-ui
description: Enforces the transparent, glassmorphic cinematic UI design system and mandatory Playwright test-driven development for all frontend changes.
---

# Cinematic UI Design & Testing Skill

This skill enforces our cinematic design aesthetic for the frontend (`velvet-ui`) and mandates a strict test-first progression for any visual modifications. **Always read and follow this skill when modifying the user interface.**

## Design Aesthetics
1. **Cinematic Glassmorphism:** All cards, panels, and stats must use `backdrop-filter: blur(12px)` and transparent backgrounds (e.g. `rgba(255, 255, 255, 0.05)` or `rgba(10, 4, 5, 0.6)` on dark themes) instead of solid colors. Utilize `.v-card-modern`.
2. **Sparkle Borders:** Any highlighted card (stats, process step, team member) MUST include the animated border effect. Add the `<div class="v-sparkle-border"></div>` element inside the container, and ensure the container has `position: relative`.
3. **Dynamic Backgrounds:** Sections should feel alive. Use `.v-cinematic-bg` for subtle particles/mesh gradients behind content instead of flat backgrounds.
4. **Heatmap & Data Viz:** For any statistical representation, favor creative animated elements (like the `.v-heatmap-grid`) over static blocks. Ensure elements are absolutely positioned with high `z-index` over the background visual where necessary.

## Testing & Validation Rules (Mandatory)
Before starting any design implementation, or when modifying existing UI components:
1. **Check Existing Tests First:** All UI components have visual/structural assertions in `test-suite/playwright/specs/home.spec.ts`. You must review these tests.
2. **Never Break Layout Checks:** Tests explicitly measure `.boundingBox()` or rely on `.toBeVisible()`. Ensure your new changes do not break responsive layouts for `mobile-375` and `tablet-768`.
3. **100% Coverage:** Any new visual effect, animation, or structural change MUST be paired with a corresponding Playwright test checking its presence, size, and specific classes (e.g., asserting `.v-sparkle-border` is visible on a new card).
4. **Containerized Testing:** Execute `VAELVET_URL=http://localhost:8087 npm run test --prefix test-suite/playwright` after rebuilding and launching the podman container to validate your changes.

**Do not present the work to the user until all tests pass.**
