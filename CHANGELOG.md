# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2026-06-28

### Added
- **Content:** Added a new service entry for "Gift Card Solution" in `content/site.md` to support end-to-end generation and lifecycle management of custom coupons and gift cards.
- **Testing:** Added 21 new Playwright end-to-end tests covering responsive layouts across 4 viewport sizes (320px, 375px, 768px, 1280px), color accuracy for the brand red (`#B52A2A`), and explicit checks for `Kalnia Glaze` typography. Total E2E test suite now sits at 222 passing tests.

### Changed
- **Styling:** Unified the design language across all card components (`.v-process__step`, `.v-case-card`, and `.v-tile`). Cards now feature a consistent glassmorphism/pillar style with a 10px border-radius, replacing flat borders with sleek backgrounds.
- **Typography:** Removed all external Google Fonts CDN links. All fonts (`IBM Plex Sans` and `Kalnia Glaze`) are now self-hosted locally from `/assets/fonts/` for strict privacy, better performance, and offline-capability. `Kalnia Glaze` specifically applied to `.v-founder__name` with the brand crimson color (`var(--accent)`).
- **Animations:** Introduced subtle hover micro-interactions (e.g., `transform: translateY(-4px)`) and enhanced box-shadows across all cards for a more premium feel.
- **Founder Profile:** Refined `.v-founder__photo` with `aspect-ratio: 1/1` and `object-fit: cover` to ensure any founder portrait renders as a perfect square. Responsive stacking specifically tuned for ultra-narrow 375px viewports.
- **Responsive Layout:** Added explicit `@media (max-width: 375px)` and `380px` breakpoints for ultra-narrow screens to reduce side-panel padding, ensuring edge-to-edge readability for contact forms, statistics grids, and footer alignment without horizontal scrolling.

### Fixed
- **Testing & Containerization:** Fixed Playwright color assertions that previously failed under different themes (dark vs light mode). Now accurately validating the entire crimson family dynamically. Verified local deployment environment stability with successful Podman container builds utilizing Distroless runtime. 
- **Footer Spacing:** Fixed blank space issues at the bottom of the page, ensuring the giant Velvt text aligns flawlessly to the extreme end of the layout.
