# language: en
@P0 @S7 @hygiene
Feature: Zero-junk repository and gating CI
  As a maintainer
  I want no stale build artifacts tracked in git and CI that actually fails on security regressions
  So that every build is fresh, small, and enterprise-auditable

  @S7-01
  Scenario: Tracked build junk is removed and stays out
    Given `test-dist/` (18 MB stale hashed dx output), `test-results/.last-run.json`, and `.codegraph/intelligence.db` are tracked
    When the cleanup runs
    Then `git ls-files | grep -E 'test-dist|test-results|codegraph'` returns nothing
    And `.gitignore` gains `/test-dist`, `/test-results`, `/.codegraph`
    And no justfile/CI/script reference to the removed paths exists

  @S7-02
  Scenario: Single source per test spec
    Given `test-suite/playwright/screenshot.spec.ts` duplicates `specs/screenshot.spec.ts`
    When both are diffed
    Then the stale duplicate is removed (or both kept with documented reason if they differ)

  @S7-03
  Scenario: Security audit gates the build
    Given `pr-gate.yml` and `deploy-pages.yml` run `cargo audit ... || true`
    When the workflows are updated
    Then `cargo audit` and `cargo deny` fail the pipeline on findings
    And the stale `refactor/production-grade` branch trigger is removed
    And the redundant triple test invocation collapses to one `cargo test --workspace --no-fail-fast`

  @S7-04
  Scenario: Coverage gate runs in CI
    Given house gate is >= 90% lines (cargo-llvm-cov)
    When a PR is opened
    Then CI runs `cargo llvm-cov --workspace --all-targets --fail-under-lines 90`
    And the job fails below 90%

  @S7-05
  Scenario: Container publish path is hardened
    Given `publish-container.yml` lacks harden-runner
    When updated
    Then `step-security/harden-runner@v2` runs first and permissions are least-privilege
    And `cloudbuild.yaml` no longer passes `--target runtime` (stage does not exist)

  @S7-06
  Scenario: Fresh, small, deterministic build
    Given `just build`
    When it runs
    Then old dx output is wiped before compiling (`clean-wasm`)
    And `assets/theme.css` is synced to `public/assets/` so containers never ship stale CSS
    And the result is a single WASM + single JS bundle (no lazy loading, no code splitting, no duplicate assets)
    And budgets hold: WASM <= 1.5 MB gz, theme.css <= 40 KB gz
