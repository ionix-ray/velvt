# Velvt Release Notes: Stable V1

**Date:** July 2026
**Branch:** `refactor/production-grade`
**Status:** **STABLE & PRODUCTION-READY**

## Overview
This release marks a fully stabilized, production-grade milestone for the Velvt PR agency platform. We have resolved critical deployment, aesthetic, and security issues without introducing breaking changes to the core cinematic UI.

## Key Accomplishments

### 1. UI & Visual Integrity (Zero-FOUC)
- **FOUC Remediation:** Fixed the "Flash of Unstyled Content" that occurred on first load. We prevented Dioxus from injecting a redundant, dynamic stylesheet that stalled the browser, and instead implemented a static HTML `<link>` to load `theme.css` instantly.
- **V-Curtain Fallback:** Added a pre-paint structural fallback curtain that completely masks initial rendering calculations, with proper `prefers-reduced-motion` compliance.
- **Cinematic Experience:** Restored the `v-border-sparkle` dynamic outlines, glassmorphic semi-transparent backgrounds, and `peek-scale`/`peek-tilt` viewport intersection animations.

### 2. Testing & Quality Assurance
- **100% E2E Playwright Coverage:** Ensured zero regressions on layout sizing and alignment.
- **Custom SPA Server:** Built `spa_server.py` to allow headless browsers to accurately hit deep-linked routing (e.g., `/achivements/...`) preventing 404 test failures.
- **Responsive Layouts:** Finalized mobile vs desktop viewport checks, ensuring sidebar layouts stack gracefully at 375px widths.

### 3. Pipeline & Deployment (CI/CD)
- **Asset Pipeline Stability:** Manually implemented a fallback tracking copy in the GitHub actions pipeline (`deploy-pages.yml`) to ensure unhashed fonts and the core `theme.css` files are always shipped to the GitHub Pages environment.
- **Security Audit:** Successfully passed all Rust CVE, advisory, and dependency licensing checks via `cargo-audit` and `cargo-deny`.

### 4. Defensive Security (Anti-Scraping)
- Locked down right-click functionality via native browser APIs.
- Explicitly blocked common inspection shortcut keys (F12, Ctrl+Shift+I, Ctrl+Shift+J, Ctrl+Shift+C, Ctrl+U).
- *Implemented in a headless-friendly way to ensure no hanging or freezing occurs during CI Playwright automated testing.*

## Next Steps & Future Operations
As established in our `AGENTS.md` project rules:
- **This state is locked.**
- Any future modifications or additions must occur on a separate feature branch.
- Changes must be brought in via a pull request and validated across the entire test suite before returning to the stable base.
