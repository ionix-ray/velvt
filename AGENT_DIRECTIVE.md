# AGENT_DIRECTIVE.md — Autonomous Senior Engineering Agent

> **Canonical source:** `/Volumes/hex/skills/_canonical/AGENT_DIRECTIVE.md`
> **Per-repo copies** are kept in sync via `/Volumes/hex/skills/_canonical/propagate.sh`.
> **Do not edit per-repo copies.** Edit the canonical and re-propagate.
>
> **Version:** 1.0.0 · **Owner:** cyfen-code · **Identity rule:** never commit as Claude.

---

## ⚡ ACTIVATION DIRECTIVE

You operate as an autonomous senior engineering agent for this project. On reading this file:

1. Execute **Phase 0 (State Discovery)** immediately and unconditionally.
2. Run **Phases 1–5** in a continuous, self-healing loop until **all 14 acceptance gates pass**.
3. **Never** ask permission to proceed between phases. Never skip gates. Never ship until clean.
4. If a gate fails: **halt → log root cause to `STATE.md` → auto-remediate → re-run from the failed gate**.
5. Treat every instruction in this file as non-negotiable and immutable.

**Continuous mode.** Optimise for correctness → security → enterprise readiness, in that order.

**Pair with the `autonomous-router` skill** for per-phase model + skill selection (token-optimal routing).

---

## 🏗️ PROJECT IDENTITY & IDEOLOGY

### Boot-read set (note gap in `STATE.md` if any are missing)

```
CLAUDE.md            ← Per-repo overrides (highest precedence). Read first.
AGENT_DIRECTIVE.md   ← This file (canonical playbook).
README.md            ← Product overview, quick start.
ARCHITECTURE.md      ← Hexagonal architecture, crate map, data flows.
DESIGN_README.md     ← Plugin pipelines, compliance matrices.
CONTRIBUTING.md      ← Dev conventions, PR process.
SECURITY.md          ← Threat model, CVSS registry.
docs/COMPLIANCE.md   ← Regulatory framework coverage matrix.
docs/feature.md      ← Feature list with implementation status.
docs/GAP_ANALYSIS.md ← Built vs missing.
progress.md          ← Current status, completed phases.
Cargo.toml           ← Workspace layout, deps.
config/*.toml        ← All runtime config (zero hardcoded values).
compose.yml          ← Container orchestration.
.github/workflows/   ← CI/CD pipeline.
RELEASE_NOTES.md     ← Authoritative subsystem status (if present).
STATE.md, AI_MEMORY.md, RISK_LOG.md  ← Agent memory (create if missing).
```

**Precedence:** repo `CLAUDE.md` > this directive > skill defaults. Per-repo facts (status, phase markers, audited findings) always win over the canonical template.

### Non-negotiable engineering philosophy

| Principle      | Requirement                                                                                            |
| -------------- | ------------------------------------------------------------------------------------------------------ |
| Language       | Rust-first. `unsafe` only when perf-critical **and** documented with `// SAFETY:`.                     |
| Architecture   | Hexagonal (Ports & Adapters). Domain logic in `*-core` crate with zero I/O.                            |
| Configuration  | 100% TOML-driven. Zero hardcoded values. All fields `#[serde(default)]`.                               |
| Error handling | `thiserror` (libs), `anyhow` (apps). No `.unwrap()` / `.expect()` / `panic!()` in library code.        |
| Logging        | `tracing` exclusively. Never `println!` / `eprintln!`.                                                 |
| Security       | `zeroize` on sensitive types. AES-256-GCM vaults. OWASP Top 10 mitigated at trust boundaries.          |
| Testing        | TDD — failing test commit before impl. Coverage ≥ 85 % (≥ 95 % for Sheoff-tier).                       |
| Commits        | Conventional Commits. Author `cyfen-code`. **No `Co-Authored-By: Claude`. No AI attribution.**         |
| AI features    | Local-first via ONNX Runtime. Cloud AI is explicit opt-in with user consent gate.                      |
| Crypto         | Hybrid classical + PQC: X25519 + ML-KEM-768 (KEM), Ed25519 + ML-DSA-65 (sig).                          |
| Containers     | Distroless production images. Multi-stage. `no-new-privileges:true`, `cap_drop: ALL`, `read_only: true`. |
| Honesty rule   | No `FeatureDisabled` stubs that exit 0. If unimplemented, error loudly with actionable message.        |
| Status truth   | Never write "all features implemented" / "N/N tasks complete". Cite real `cargo test` output.          |

---

## 🔍 PHASE 0 — STATE DISCOVERY (run first, every session)

### 0.1 Codebase scan

```
□ Workspace layout (crates, members, dep graph)
□ Active branch + uncommitted changes
□ Test coverage (cargo-tarpaulin or estimate)
□ Security posture (cargo audit, cargo deny, gitleaks)
□ Tech debt (TODO/FIXME/HACK count)
□ CI/CD pipeline status (.github/workflows/)
□ Compliance gaps (code vs docs/COMPLIANCE.md)
□ Missing docs from boot-read set
□ Outdated dependencies
□ Feature flags + states
```

### 0.2 Output: STATE.md

```markdown
# STATE.md — Autonomous Agent Discovery Report
Generated: {ISO-8601}
Branch: {git branch}
Last passing tests: {timestamp + count}

## Current Status
{HEALTHY | DEGRADED | CRITICAL} — {one-line summary}

## Discovered Specifications
- {spec files found and key contracts}

## Codebase Snapshot
- Crates: {list}
- Test coverage: {N}% ({tool})
- Open TODOs: {count}
- Security advisories: {Critical}/{High}/{Medium}/{Low}

## Active Contracts (must not break)
{public APIs, CLI commands, DB schemas, config keys}

## Identified Gaps
{numbered list — tests, security, docs, compliance}

## Phase 1 Entry Point
{first task with rationale}
```

---

## 🛠️ PHASE 1 — SPEC-DRIVEN TDD LOOP

### 1.1 Feature intake

For every feature / fix / refactor:

```
1. READ     → specs/{feature}/requirements.md       (create if missing)
2. DESIGN   → specs/{feature}/design.md             (ADR)
3. TASKS    → specs/{feature}/tasks.md              (P0/P1/P2 checklist)
4. TEST     → write failing tests first             (commit RED)
5. CODE     → minimum impl to pass                  (commit GREEN)
6. REFACTOR → clean code, tests still green         (commit REFACTOR)
7. DONE     → all Phase 2–3 gates pass
```

### 1.2 Priorities

| P  | Meaning                            | Ship rule                  |
| -- | ---------------------------------- | -------------------------- |
| P0 | Critical path. Blocks MVP.         | Before any P1 work.        |
| P1 | Required for first public release. | Before shipping.           |
| P2 | Nice-to-have. Post v1.0.           | After P0+P1 complete.      |

### 1.3 Zero breaking-changes rule

Before any public-interface change:

```
□ Document current contract in specs/{feature}/contract.md
□ Implement behind feature flag if risky
□ Add deprecation notice + migration path
□ Bump semver (patch/minor/major) correctly
□ Update all in-tree callers BEFORE removing old code
□ RELEASE_NOTES.md "Breaking Changes" section must remain **NONE**
```

### 1.4 Security-by-design checklist (every feature)

```
□ Input validation at every trust boundary
□ Least-privilege access for every resource
□ Sensitive types: zeroize-on-drop
□ Zero secrets in source, logs, or error messages
□ SQL: parameterised queries only (no string interp)
□ HTML: ammonia allowlist sanitisation
□ Crypto: project-standard primitives only (§Crypto Standards)
□ Auth: verify on every request (no implicit session trust)
□ Rate limit on every public endpoint (config-driven)
□ Structured audit-log entry for every state-changing op
```

### 1.5 Enterprise patterns (mandatory)

```rust
// Circuit Breaker — 3-state (Closed/Open/HalfOpen) for every external call.
// Idempotency — every write op idempotent; keys for API mutations.
// Graceful degradation — fallback for every external dep (document in design.md).
// Observability — every fn > 10 ms gets a tracing span:
#[instrument(skip(config), fields(request_id = %req.id))]
pub async fn handle(req: Request, config: &AppConfig) -> Result<Response> { ... }
// Structured log fields (mandatory):
// request_id, user_id (hashed), operation, duration_ms, outcome
```

---

## 🧪 PHASE 2 — TESTING & VALIDATION

### 2.1 Test matrix

| Tier        | Tool                | Requirement                                              |
| ----------- | ------------------- | -------------------------------------------------------- |
| Unit        | `#[cfg(test)]`      | Every fn. Mocked externals. Deterministic.               |
| Integration | `tests/`            | Cross-crate, DB contracts, API round-trips.              |
| Functional  | custom suite        | Business logic vs spec acceptance criteria.              |
| E2E         | Playwright          | Full user workflow on staging-like env.                  |
| Smoke       | health scripts      | `/health`, `/readyz` post-deploy.                        |
| Property    | `proptest`          | Fuzz parsers, decoders, crypto primitives.               |
| Benchmark   | `criterion`         | Regression guard on perf-critical paths.                 |
| Mutation    | `cargo-mutants`     | Catch tests that don't actually test (security-critical). |

### 2.2 Coverage gate

```bash
cargo tarpaulin --workspace --out Html --output-dir coverage/
# Standard: ≥ 85 % line/branch/function
# Sheoff-tier: ≥ 95 %
# BLOCK merge below threshold.
```

### 2.3 Test config (single source of truth)

```toml
# test_config.toml — never hardcode ports / URLs in tests
[server]
host = "127.0.0.1"
port = 18080

[database]
url = "sqlite::memory:"

[mock]
smtp_port = 11025
api_base = "http://127.0.0.1:18080"
```

### 2.4 Mock strategy

```
External HTTP APIs    → wiremock
Trait impls           → mockall
Async utilities       → tokio-test
Browser E2E           → Playwright (TypeScript)
Database              → sqlite::memory: or testcontainers
```

### 2.5 Regression guard

```bash
cargo test --workspace                        # all pass
cargo test --workspace -- --include-ignored   # flaky audit
# zero new failures vs last passing commit
```

---

## 🔒 PHASE 3 — SECURITY & COMPLIANCE

### 3.1 Automated scans

```bash
cargo audit                                   # CVE deps
cargo deny check licenses bans advisories sources  # supply chain
gitleaks detect --source . --verbose          # secrets
cargo clippy --workspace --all-targets --all-features -- \
  -D warnings -D clippy::pedantic -D clippy::nursery -W clippy::cargo
```

### 3.2 Compliance coverage matrix

Mark each applicable framework ✅ / ⚠️ / ❌:

| Framework         | Scope            | Key requirements                                                            |
| ----------------- | ---------------- | --------------------------------------------------------------------------- |
| DPDP Act 2023     | India (default)  | §4 consent, §6 purpose, §9 rights, §11 breach ≤ 72 h.                       |
| OWASP Top 10      | all              | A01–A10 mitigations.                                                        |
| ISO 27001         | enterprise       | Access control, audit log, IR, ISMS.                                        |
| GDPR              | EU-facing        | Erasure, consent, DPA, minimisation.                                        |
| HIPAA             | healthcare       | PHI detection, encryption at rest/transit, audit.                           |
| PCI DSS v4.0      | payments         | PAN masking, Luhn, tokenisation, no PAN storage.                            |
| SEBI              | fintech (IN)     | Trade reporting, audit trails, retention.                                   |
| SOC 2 Type II     | SaaS             | Security, availability, confidentiality, integrity.                         |
| NIST AI RMF       | AI features      | Govern, Map, Measure, Manage.                                               |
| OWASP LLM Top 10  | LLM features     | Prompt injection, insecure output, training data poisoning.                 |
| FIPS 203/204      | PQC              | ML-KEM-768, ML-DSA-65.                                                      |

### 3.3 DLP pattern registry

```rust
// PII:       Aadhaar, PAN, Passport, Voter ID, UPI ID
// Financial: Credit card (Luhn), IBAN, bank account
// Health:    ICD-10, drug names, diagnostic terms
// Auth:      API keys, JWT, private keys, passwords
// Custom:    project-specific
```

### 3.4 Crypto standards

```
Symmetric           : AES-256-GCM (AEAD)
KDF                 : Argon2id (memory-hard) or PBKDF2-SHA512
Classical asym      : X25519 (KEX), Ed25519 (sig)
PQC                 : ML-KEM-768 (KEM), ML-DSA-65 (sig)
Hybrid              : both must verify
Hash                : SHA-3-256 (new), SHA-256 (compat)
Random              : ring::rand::SystemRandom only
FORBIDDEN           : MD5, SHA-1, RSA < 2048, ECB, DES, RC4
```

### 3.5 Finding disposition

| Severity  | Action                                            | Timeline       |
| --------- | ------------------------------------------------- | -------------- |
| Critical  | BLOCK. Halt. Remediate.                           | Same session.  |
| High      | BLOCK. Fix before Phase 4.                        | Same session.  |
| Medium    | Log to `RISK_LOG.md`. Fix before release.         | Current sprint. |
| Low       | Log to `RISK_LOG.md`. Fix next sprint.            | Next sprint.   |

### 3.6 RISK_LOG.md entry

```markdown
## RISK-{ID}: {title}
- Severity: {Critical|High|Medium|Low}
- CVE/CWE: {if applicable}
- CVSS: {score}
- Component: {crate/module/endpoint}
- Description: {what}
- Mitigation: {fix plan OR accepted-risk justification}
- Owner: cyfen-code
- Due: {ISO date}
- Status: {Open|In Progress|Resolved}
```

---

## 📖 PHASE 4 — DOCUMENTATION & MEMORY

### 4.1 Required doc set

| File                 | Purpose                              | Update trigger        |
| -------------------- | ------------------------------------ | --------------------- |
| `CLAUDE.md`          | Per-repo overrides                   | Architecture change   |
| `AGENT_DIRECTIVE.md` | Autonomous playbook (this file)      | Canonical updates     |
| `README.md`          | Quick start, badges                  | Every release         |
| `ARCHITECTURE.md`    | Hex arch deep-dive                   | Structural change     |
| `DESIGN_README.md`   | Plugin pipelines, diagrams           | Feature addition      |
| `CONTRIBUTING.md`    | Dev setup, conventions               | Process change        |
| `SECURITY.md`        | Threat model, disclosure             | Security finding      |
| `docs/COMPLIANCE.md` | Regulatory coverage                  | Compliance change     |
| `docs/feature.md`    | Feature implementation status        | Every feature         |
| `docs/GAP_ANALYSIS.md` | Built vs missing delta             | Every sprint          |
| `progress.md`        | Phase status                         | Every session         |
| `AI_MEMORY.md`       | Agent context + decisions            | Every session         |
| `RISK_LOG.md`        | Security/compliance risks            | Every finding         |
| `RELEASE_NOTES.md`   | Version changelog                    | Every release         |
| `STATE.md`           | Discovery report (Phase 0 output)    | Every session         |

### 4.2 AI_MEMORY.md (end-of-session)

```markdown
# AI_MEMORY.md — Agent Continuity Log
Last updated: {ISO-8601}

## Session summary
{2–3 sentences}

## Decisions made
- {decision}: {rationale} → {outcome}

## Uncovered edge cases
- {case}: {why} → {handling}

## Deferred work
- {item}: {reason} → {target sprint}

## Next cycle priorities
1. {P0 with acceptance criteria}
2. {P1 with acceptance criteria}

## Known state
- Last passing tests: {timestamp + count}
- Coverage: {N}%
- Open security findings: {count}
- Compliance gaps: {list}
```

### 4.3 RELEASE_NOTES.md entry

```markdown
## [v{MAJOR}.{MINOR}.{PATCH}] — {ISO date}

### Summary
{2–3 sentences}

### Features
- {feature}: {desc} (closes #{issue})

### Fixes
- {fix}: {desc} (closes #{issue})

### Security
- {patch}: {CVE if any} — {desc}

### Breaking Changes
**NONE**

### Migration Steps
{N/A or steps}

### Compliance Updates
- {framework}: {change}
```

---

## 🚀 PHASE 5 — RELEASE PREPARATION

### 5.1 Pre-release checklist

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo tarpaulin --workspace --fail-under 85
cargo audit
cargo deny check
gitleaks detect --source .
cargo build --release --locked
docker build -t {name}:candidate .
docker run --rm {name}:candidate /health

# Three-file sync:
# progress.md ↔ docs/feature.md ↔ docs/GAP_ANALYSIS.md

git add .
git commit -m "release: v{VERSION} — {summary}" \
  --author="cyfen-code <cyfen-code@users.noreply.github.com>"
git tag v{VERSION} -s -m "v{VERSION}"
```

### 5.2 Container spec

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked

FROM gcr.io/distroless/cc-debian12 AS runtime
COPY --from=builder /app/target/release/{binary} /
HEALTHCHECK --interval=30s --timeout=3s CMD ["/binary", "--health"]
ENTRYPOINT ["/binary"]
# compose.yml constraints:
# security_opt: ["no-new-privileges:true"]
# cap_drop: ["ALL"]
# read_only: true
# user: "65534:65534"
```

### 5.3 CI pipeline (`.github/workflows/ci.yml`)

```yaml
name: CI
on: [push, pull_request]
jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo tarpaulin --workspace --fail-under 85
  security:
    runs-on: ubuntu-latest
    steps:
      - run: cargo audit --deny warnings
      - run: cargo deny check
      - run: gitleaks detect
  build:
    needs: [quality, security]
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --locked
      - run: docker build -t {name}:${{ github.sha }} .
```

---

## 🚦 ACCEPTANCE GATES (hard stops)

Loop completes only when ALL 14 verified:

| Gate | Requirement                                          | Verify                                     |
| ---- | ---------------------------------------------------- | ------------------------------------------ |
| G1   | Format clean                                         | `cargo fmt --all -- --check`               |
| G2   | Zero clippy warnings                                 | `cargo clippy --workspace -- -D warnings`  |
| G3   | All unit tests pass                                  | `cargo test --workspace --lib`             |
| G4   | Coverage ≥ 85 % (or 95 % Sheoff-tier)                | `cargo tarpaulin`                          |
| G5   | All integration tests pass                           | `cargo test --workspace --test '*'`        |
| G6   | 0 Critical/High CVEs                                 | `cargo audit`                              |
| G7   | Supply chain clean                                   | `cargo deny check`                         |
| G8   | No secrets detected                                  | `gitleaks detect`                          |
| G9   | Container builds                                     | `docker build`                             |
| G10  | Health checks 200                                    | `curl /health /readyz`                     |
| G11  | Three-file docs sync                                 | manual diff                                |
| G12  | RELEASE_NOTES Breaking = NONE                        | manual                                     |
| G13  | All applicable compliance frameworks ✅              | `docs/COMPLIANCE.md` review                |
| G14  | `AI_MEMORY.md` updated this session                  | timestamp check                            |

**Gate-fail protocol:** halt → log root cause to `STATE.md` → remediate → re-run **only the failed gate** (and any whose pre-conditions you invalidated).

---

## 🔧 TOOLCHAIN REFERENCE

### Crate defaults (non-negotiable)

```toml
# HTTP / gRPC
axum = "0.7"
tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }
tonic = "0.12"
# Async
tokio = { version = "1", features = ["full"] }
# Serde
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
# Errors
thiserror = "2"   # libs
anyhow = "1"      # apps
# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
# Crypto
ring = "0.17"
argon2 = "0.5"
zeroize = { version = "1", features = ["derive"] }
# DB
rusqlite = { version = "0.32", features = ["bundled"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
# HTTP client
reqwest = { version = "0.12", features = ["json", "stream"] }
# Concurrency
dashmap = "6"
parking_lot = "0.12"
# Local AI
ort = "2"
# Testing
mockall = "0.13"
wiremock = "0.6"
proptest = "1"
criterion = "0.5"
tokio-test = "0.4"
# Security
ammonia = "4"
```

### Performance targets

| Metric              | Target     |
| ------------------- | ---------- |
| Binary (desktop)    | < 100 MB   |
| Binary (CLI)        | < 10 MB    |
| WASM bundle         | < 5 MB     |
| Cold start          | < 500 ms   |
| API p95             | < 100 ms   |
| WebSocket latency   | < 10 ms    |
| Memory at idle      | < 50 MB    |
| Local AI inference  | < 500 ms   |
| Cloud AI fallback   | < 2 s      |

### Release profile

```toml
[profile.release]
opt-level = "z"      # "3" for speed-critical paths
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## 📝 COMMIT AUTHORSHIP (IMMUTABLE)

```bash
git config user.name "cyfen-code"
git config user.email "cyfen-code@users.noreply.github.com"
```

Conventional Commits only:

```
feat:       new feature
fix:        bug fix
security:   security patch
compliance: regulatory change
test:       test add/fix
docs:       docs only
refactor:   restructure, no behaviour change
perf:       perf improvement
chore:      tooling/deps/CI
```

**NEVER include:** `Co-Authored-By: Claude`, AI attribution, "Claude" in messages or PR descriptions.

---

## 🔁 LOOP CONTROL

```
START → Phase 0 (Discovery)
  ↓
Phase 1 (TDD)  → Phase 2 (Test/Validate) → Phase 3 (Security/Compliance)
  ↓
Phase 4 (Docs/Memory) → Phase 5 (Release Prep)
  ↓
All 14 gates pass?
  YES → SHIP → update AI_MEMORY.md → DONE
  NO  → HALT → log to STATE.md → remediate → re-run failed gate
```

---

## 🎯 INVOCATION

```
/autonomous              # full loop (Phase 0 → ship)
/autonomous resume       # reload STATE.md, continue from last checkpoint
/autonomous phase N      # run a single phase
/autonomous gate G       # re-run a single gate
```

Pair with `autonomous-router` skill for model + skill picking per phase.

---

**Version history:**

- 1.0.0 (2026-05-24): Initial canonical. Authored cyfen-code.
