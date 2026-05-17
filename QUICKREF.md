# Velvet PR Agency — Quick Reference Card

## 🚀 Daily Workflow
```bash
# Start dev server (hot reload, auto-open Safari + Chrome)
just dev
# or
./scripts/bootstrap.sh dev

# Full release pipeline (format → clippy → test → audit → build → serve → open)
just release:full
# or
./scripts/bootstrap.sh release:full

# Docker container mode (build → docker → port forward → open browsers)
just container
# or
./scripts/bootstrap.sh container
```

## 📦 Build Commands
| Command | Description |
|---------|-------------|
| `just dev` | Dev server with hot reload |
| `just build` | Production WASM build (format + clippy + test + build) |
| `just serve` | Build + serve + open in Safari + Chrome |
| `just release:full` | Full release pipeline with quality gates |
| `just container` | Docker container with port forward + open browsers |
| `just static` | Generate static output for CDN deploy |

## 🧪 Test & Quality
| Command | Description |
|---------|-------------|
| `just test` | Run all Rust tests (104 tests) |
| `just lint` | Format + clippy |
| `just audit` | Security audit |
| `just e2e` | Run Playwright E2E tests |
| `just e2e-ui` | Run Playwright E2E with UI |

## 🐳 Docker
| Command | Description |
|---------|-------------|
| `just container` | Build + run container + open browsers |
| `just docker-build` | Build Docker image only |
| `just docker-stop` | Stop and remove container |
| `just docker-logs` | Show container logs |

## 🔧 Utilities
| Command | Description |
|---------|-------------|
| `just clean` | Clean build artifacts |
| `just health` | Quick health check |
| `just port` | Show current port status |
| `just help` | Show all commands |

## 📁 Key Files
| File | Purpose |
|------|---------|
| `scripts/bootstrap.sh` | Main orchestration script |
| `justfile` | Command aliases |
| `Containerfile` | Docker multi-stage build |
| `scripts/Caddyfile` | Production HTTP server config |
| `velvet-ui/src/` | Application source |
| `velvet-ui/assets/theme.css` | Design system CSS |
| `test-suite/playwright/` | E2E tests |

## 🔧 Troubleshooting
| Issue | Solution |
|-------|----------|
| CSS not loading (404) | Run `just build` — post_build.sh fixes hashed CSS references |
| Routes return 404 | Use `just serve` or `just release:full` — SPA server handles routing |
| Old styles showing | Run `just clean` then `just build` — clears cached hashed assets |
| Port already in use | Script auto-finds free port 8080-8100 |

## 🎯 Quality Gates (All Must Pass)
- ✅ `cargo fmt --all -- --check`
- ✅ `cargo clippy --workspace -- -D warnings`
- ✅ `cargo test --package velvet-ui` (104 tests)
- ✅ `cargo audit` (0 vulnerabilities)
- ✅ WASM bundle < 1.5MB (current: 425KB)
