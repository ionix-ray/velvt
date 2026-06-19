---
name: defensive-security
description: Security controls for Velvet — no secrets in source, cargo audit in CI, CIS-hardened container, CSP headers via sws.toml. Trigger on 'security', 'audit', 'CVE', 'secret', 'CSP', 'harden'.
---

# Defensive Security — Velvet

A static WASM site has a small attack surface. Controls focused accordingly.

## Iron Laws

1. No secrets in source — `.gitignore` gates `*.pem`, `*.key`, service-account JSON
2. `cargo audit` in every CI run — blocks on any CRITICAL or HIGH CVE
3. Container: distroless, non-root, read-only FS, cap_drop ALL
4. CSP headers in `sws.toml` — no inline scripts, WASM `wasm-unsafe-eval` only
5. All Terraform secrets in `terraform.tfvars` (gitignored) — never in `.tf` files

## Supply Chain

```bash
cargo audit                                     # CVE check
cargo deny check licenses bans advisories       # supply chain
# deny.toml: deny unlicensed, deny unmaintained crates
```

## Content Security Policy (sws.toml)

```toml
[security]
[[security.headers]]
origin = "."
headers = [
  ["Content-Security-Policy", "default-src 'self'; script-src 'wasm-unsafe-eval' 'self'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; img-src 'self' data:"],
  ["X-Content-Type-Options", "nosniff"],
  ["X-Frame-Options", "DENY"],
  ["Referrer-Policy", "strict-origin-when-cross-origin"],
  ["Permissions-Policy", "geolocation=(), camera=(), microphone=()"],
]
```

## Secrets Gate

```gitignore
# ops/tofu/
terraform.tfvars
*.tfstate
*.tfstate.*
.terraform/
.terraform.lock.hcl  # only if not pinned — keep if checked in
```

## Cross-References

- `.skills/container-distroless/SKILL.md` — container hardening
- `.skills/autonomous-deploy-cicd/SKILL.md` — cargo audit in CI
- `.skills/tofu-module-registry/SKILL.md` — infra secrets handling
