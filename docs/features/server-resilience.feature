# language: en
@P0 @S4 @backend-lead @tdd-cycle @rust-testing
Feature: Server resilience and config-driven hardening
  As a production operator of the Vaelvet static delivery server
  I want the Axum server to be fully config-driven, gracefully shutdown, and immune to slow-client exhaustion
  So that deploys never drop in-flight requests and every tunable lives in TOML

  Background:
    Given the server crate at `server/`
    And all tunables follow house rule 11 (TOML config, `#[serde(default)]`, zero hardcoded values)

  @S4-01
  Scenario: Load configuration from TOML file with layered precedence
    Given a config file `velvet-server.toml` with `addr`, `static_root`, `request_timeout_secs`, `max_concurrent_requests`
    When the server starts with `--config <path>`
    Then the file values are used
    And when only env var `VELVET_SERVER_CONFIG` is set, the env path is used
    And when neither is set but `./velvet-server.toml` exists, that file is used
    And when no file exists, built-in defaults apply
    And a malformed TOML file logs `tracing::error` and falls back to defaults without panicking
    And CLI flags override env, env overrides file, file overrides defaults

  @S4-02
  Scenario: Graceful shutdown on SIGTERM/SIGINT
    Given the server is serving in-flight requests
    When the process receives SIGTERM (container stop) or SIGINT (ctrl-c)
    Then `axum::serve` uses `with_graceful_shutdown`
    And in-flight requests complete before exit
    And the shutdown signal source is injectable so the behavior is unit-testable without real signals

  @S4-03
  Scenario: Request timeout and concurrency limits protect against slow clients
    Given `request_timeout_secs = 10` and `max_concurrent_requests = 256` in TOML
    When a handler exceeds the request timeout
    Then the request fails with HTTP 408 instead of hanging
    And a `DefaultBodyLimit` is applied to all routes
    And the layer stack is asserted by a test

  @S4-04
  Scenario: No blocking filesystem calls on the async runtime
    Given a request for a static asset
    When the handler resolves, containment-checks, and reads the file
    Then the entire resolve-check-read sequence runs inside a single `spawn_blocking` closure
    And the containment check (`is_within_root`) canonicalizes and reads the SAME path object (no TOCTOU window)
    And path traversal attempts still return HTTP 400 (existing integration tests unchanged)

  @S4-05
  Scenario: No dead error surface
    Given `ServerError` variants
    When a variant is never constructed outside tests
    Then it is either wired into a real flow or removed with tests updated
    And `clippy` dead-code analysis reports no unreachable error paths

  @out-of-scope
  Scenario: Load testing
    # Deliberately excluded — k6/load testing not required for this project (user decision, 2026-10-25).
