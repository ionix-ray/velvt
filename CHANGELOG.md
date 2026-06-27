# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2026-06-26

### Added
- **Typography:** Integrated Google Fonts for "Kalnia Glaze" into `index.html`.
- **Content:** Added a new service entry for "Gift Card Solution" in `content/site.md` to support end-to-end generation and lifecycle management of custom coupons and gift cards.

### Changed
- **Styling:** Unified the design language across all card components (`.v-process__step`, `.v-case-card`, and `.v-tile`). Cards now feature a consistent glassmorphism/pillar style with a 10px border-radius, replacing flat borders with sleek backgrounds.
- **Typography:** Updated CSS variables (`--font-display`) and global heading styles to default to Kalnia Glaze.
- **Animations:** Introduced subtle hover micro-interactions (e.g., `transform: translateY(-4px)`) and enhanced box-shadows across all cards for a more premium feel.
- **Founder Profile:** Refined `.v-founder__photo` with `aspect-ratio: 1/1` and `object-fit: cover` to ensure any founder portrait renders as a perfect square, regardless of the original image dimensions. Updated `content/site.md` to use `/assets/images/velvt-logo.png` as the active placeholder.

### Fixed
- **Testing & Containerization:** Re-verified all 125 test cases with the new styles/content modifications passing successfully. Verified local deployment environment stability with successful Podman container builds utilizing Distroless runtime.
