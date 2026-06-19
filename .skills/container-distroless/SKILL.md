---
name: container-distroless
description: Hardened multi-stage Containerfile for vaelvet-ui — rust:slim builder → distroless/static runtime serving WASM assets via static-web-server. CIS Docker L1 baseline. Trigger on 'container', 'distroless', 'Containerfile', 'compose', 'podman', 'docker'.
---

# Container Distroless — Velvet

Two-stage: `rust:1.88-slim` (build WASM via `dx`) → `gcr.io/distroless/static-debian12:nonroot` (serve via `static-web-server`).

## Containerfile Pattern

```dockerfile
# syntax=docker/dockerfile:1.7
ARG DIOXUS_CLI_VERSION=0.7.6

FROM rust:1.88-slim AS builder
# ... install dx, rustup target add wasm32-unknown-unknown
# dx build --release --platform web
# Output in /app/velvet-ui/dist/

FROM gcr.io/distroless/static-debian12:nonroot@sha256:<PINNED>
COPY --from=docker.io/joseluisq/static-web-server:2.36.0 /static-web-server /srv/static-web-server
COPY --from=builder --chown=65532:65532 /out /srv
COPY --chown=65532:65532 sws.toml /srv/sws.toml
USER 65532:65532
EXPOSE 80
ENTRYPOINT ["/srv/static-web-server"]
CMD ["--config-file=/srv/sws.toml"]
```

## SHA Pinning Rule

Always pin distroless to exact SHA:
```bash
crane digest gcr.io/distroless/static-debian12:nonroot  # get current SHA
```

## compose.yml (dev + CI)

```yaml
services:
  web:
    build: { context: ., dockerfile: Containerfile }
    ports: ["8080:80"]
    user: "65532:65532"
    read_only: true
    tmpfs: ["/tmp"]
    security_opt: ["no-new-privileges:true"]
    cap_drop: ["ALL"]
    pids_limit: 128
    mem_limit: 64m
    cpus: "0.5"
    healthcheck:
      test: ["CMD-SHELL", "wget -qO- http://localhost:80/ || exit 1"]
      interval: 30s
      timeout: 3s
      retries: 3
```

## CIS Docker L1 Compliance

| Requirement | Implementation |
|---|---|
| Non-root user | `USER 65532:65532` (distroless nonroot) |
| Read-only FS | `read_only: true` + `tmpfs: ["/tmp"]` |
| Drop all capabilities | `cap_drop: ["ALL"]` |
| No privilege escalation | `no-new-privileges:true` |
| Resource limits | `mem_limit`, `cpus`, `pids_limit` |
| Healthcheck | wget probe on `/` |
| Distroless base | No shell, no package manager |

## Build + Test

```bash
podman build -t vaelvet:test -f Containerfile .
podman run --rm -p 8080:80 vaelvet:test &
curl -f http://localhost:8080/         # serves index.html
curl -f http://localhost:8080/health   # sws health endpoint
```

## Cross-References

- `.skills/wasm-build-gate/SKILL.md` — WASM build generates the assets
- `.skills/autonomous-deploy-cicd/SKILL.md` — CI pushes the image
- `.skills/tofu-module-registry/SKILL.md` — K8s deploys the image
