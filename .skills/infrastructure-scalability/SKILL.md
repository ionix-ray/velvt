---
name: infrastructure-scalability
description: K8s deployment patterns for vaelvet-ui on GKE — via OpenTofu modules, resource limits, health probes, observability. Trigger on 'k8s', 'kubernetes', 'GKE', 'deploy', 'tofu apply', 'cluster'.
---

# Infrastructure & Scalability — Velvet

GKE cluster + Kubernetes deployment for vaelvet-ui via OpenTofu modules.

## Architecture

```
ops/tofu/
├── modules/networking/   # VPC + subnet
├── modules/cluster/      # GKE Autopilot-ready node pool
└── modules/app/          # K8s Namespace + Deployment + Service (LoadBalancer)
```

## Resource Sizing (vaelvet-ui is a static site)

```yaml
resources:
  requests: { cpu: "50m",  memory: "32Mi" }
  limits:   { cpu: "200m", memory: "128Mi" }
```

WASM static assets — zero compute after cold start. Scale to 1 replica in dev.

## Health Probes

```yaml
readinessProbe:
  httpGet: { path: /, port: 8080 }
  initialDelaySeconds: 2
  periodSeconds: 5
livenessProbe:
  httpGet: { path: /, port: 8080 }
  initialDelaySeconds: 10
  periodSeconds: 15
```

## Security Context

```yaml
securityContext:
  runAsNonRoot: true
  runAsUser: 65532
  readOnlyRootFilesystem: true
  allowPrivilegeEscalation: false
  capabilities: { drop: ["ALL"] }
```

## Performance Targets (static site)

| Metric | Target | Notes |
|---|---|---|
| WASM bundle | < 5 MB | `opt-level = "z"` + `wasm-opt` |
| CSS | < 15 KB | Compacted from 49 KB |
| Cold start (container) | < 500 ms | static-web-server is fast |
| p95 latency | < 50 ms | Static assets — CDN ideal |

## Cross-References

- `.skills/tofu-module-registry/SKILL.md` — module structure + import
- `.skills/container-distroless/SKILL.md` — image spec
- `.skills/autonomous-deploy-cicd/SKILL.md` — CD pipeline
