# language: en
@P0 @S6 @security
Feature: OWASP, CIS and AI-agent security posture
  As an enterprise operator
  I want every OWASP Top 10 category, OWASP LLM Top 10 risk, and CIS Docker L1 control mapped to real code, real tests, and an honest status
  So that compliance is auditable and residual risk is tracked

  @S6-01
  Scenario: Compliance matrix exists and is code-grounded
    Given `docs/SECURITY-COMPLIANCE.md`
    When reviewed
    Then every OWASP Top 10 2021 row cites control + file:line + verification test + status
    Then every OWASP LLM Top 10 2025 row is N/A, mitigated, or backlog with `S6-xx` ID
    Then every applicable CIS Docker L1 control cites the Containerfile/compose.yml line

  @S6-02
  Scenario: Existing server controls stay verified
    Given security headers (CSP, HSTS, X-Frame-Options, nosniff, Referrer-Policy, Permissions-Policy, COOP) in `server/src/middleware.rs`
    Given path-traversal guard in `server/src/handlers.rs` `is_within_root`
    When integration tests run
    Then header assertions and traversal/`%2f` escape tests pass
    And a 400 is returned for any escape attempt

  @S6-03
  Scenario: Residual risks are tracked, not hidden
    Given known gaps: no in-server TLS (edge-terminated), CSP `'unsafe-inline'` styles, no CSP report endpoint, no rate limiting, actions tag-pinned not SHA-pinned, no cosign image signing
    When documented
    Then each appears in the residual risk register with severity, status, and backlog ID
    And none is silently omitted

  @S6-04
  Scenario: Secret and supply-chain hygiene
    Given gitleaks pre-commit/push, `deny.toml`, `just audit`, `just sbom`
    When gates run locally and in CI
    Then zero secrets in history, zero advisories, license policy enforced (deny GPL/AGPL)
    And CI audit steps are gating (no `|| true`)

  @S6-05
  Scenario: AI-agent repo surface is governed
    Given `.claude/`, `.agents/`, `.skills/`, `AGENT_DIRECTIVE.md` shape agent behavior
    When an agent operates on the repo
    Then prompt-injection exposure via content files (`content/site.md`, `docs/cse_studies/*.md`) is documented
    And `dangerous_inner_html` trust boundary (first-party compile-time content only) is documented
    And excessive-agency controls (file scope, no-secret rules, TDD + gate discipline) are cited in the matrix
