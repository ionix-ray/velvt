# language: en
@P1 @S5 @frontend-lead @design-system
Feature: Cinematic, config-driven, brand-faithful UI
  As a visitor of velvt.live
  I want every section visible, every motion intentional, and every link driven by one config file
  So that the experience feels premium, responsive, and never broken

  @S5-01
  Scenario: Scroll reveal actually reveals
    Given `scroll.rs` adds class `in` on intersection
    When an element with `.v-reveal` enters the viewport
    Then CSS consumes the reveal state (one consistent mechanism, observer -> DOM)
    And the VELVT heatmap cells (currently stuck at `opacity: 0` because `.is-revealed` is never applied) become visible after scroll-in
    And a Playwright test asserts cell opacity > 0 after entering the viewport

  @S5-02
  Scenario: Single source of truth for all social and external links
    Given `content/site.md` is the content single source (house rule)
    When footer socials change
    Then `footer_panel`, `social_strip`, and any other link surface render from `site.footer.socials`
    And no component hardcodes a social URL
    And an HTML comment in `index.html` records `content/site.md` as source of truth for JSON-LD `sameAs`
    And an SSR test asserts strip hrefs equal config hrefs

  @S5-03
  Scenario: No dead code, no stubs
    Given `next_hint.rs` is never imported by any route
    When cleanup runs
    Then component, export, tests, and stale comments are removed
    And dead CSS blocks (`.v-case-card*`, `.v-banner*`, `.v-dash-card*`, `.v-pr-grid*`, `.v-ai__*`, `.v-awards`, `.v-social-grid`, `.v-services`, `.v-glass-card`, `.v-delay-*`) are purged after grep-verifying zero usage
    And the devtools script duplicated at two spots in `index.html` exists exactly once

  @S5-04
  Scenario: Futuristic card design ported from hex-ui, brand stays Vaelvet
    Given the agency card system at `/Volumes/hex/current-work/hex-ui/.../cards/` (same Dioxus 0.7 stack)
    When ported
    Then case cards gain a hover-revealed inline-SVG sparkline (pathLength draw-in) and oversized stat (signal-board)
    And entrance motion adopts `cubic-bezier(.22,1,.36,1)` fade-up with per-card stagger, hover saturate(0)->saturate(1) spring (press-proof)
    And integration is via a `variant` prop on `CardStep` or CSS classes — no copy-pasted markup
    And palette/voice stay crimson + Kalnia/IBM (avatar-grade glamour, never neon)
    And theme.css stays <= 40 KB gzipped
    And a Playwright test asserts sparkline presence and hover state change

  @S5-05
  Scenario: Every animation respects reduced motion
    Given infinite/pulsing animations `rotate-border`, `cinematic-pan`, `pulse-dot`, heatmap keyframes are currently ungated
    When a user prefers reduced motion
    Then all infinite animations stop at final state
    And no element is left invisible (heatmap must be visible)
    And a Playwright reduced-motion run passes

  @S5-06
  Scenario: One nav label source
    Given `PANEL_LABELS` (home.rs), `LABELS` (mobile_nav.rs) and `[[nav]]` (site.md) disagree at the same indices
    When unified
    Then both component lists derive from `site.nav`
    And index mapping (7 panels) is preserved exactly
    And tests assert equality with config

  @S5-07
  Scenario: Untested components gain SSR coverage
    Given `social_strip`, `footer_panel`, `hero_panel`, `process_panel`, `section_dots`, `scroll_progress` have zero tests
    When tests are added
    Then each has meaningful assertions (config content, hrefs, aria labels)
    And workspace coverage remains >= 90%
