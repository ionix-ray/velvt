---
name: tofu-module-registry
description: OpenTofu module conventions for ops/tofu/ — module structure, tofu import (no reprovision), remote GCS state backend, environment tfvars, audit checklist. Trigger on 'tofu', 'terraform', 'OpenTofu', 'ops/', 'infra', 'GKE', 'tofu import', 'tofu plan'.
---

# OpenTofu Module Registry — Velvet

Module conventions for `ops/tofu/`. State is imported from live GCP resources — never re-provisioned.

## Module Structure

```
ops/tofu/
├── main.tf                # Root: calls 3 modules
├── variables.tf           # All input vars
├── outputs.tf             # Forwarded from modules
├── versions.tf            # tofu >= 1.8, google ~> 6.0, kubernetes ~> 2.30
├── backend.tf             # GCS remote state
├── terraform.tfvars       # GITIGNORED — live values
├── .gitignore             # *.tfstate, *.tfstate.*, .terraform/, terraform.tfvars
├── modules/
│   ├── networking/        # VPC + subnet + APIs + Artifact Registry
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   ├── cluster/           # GKE cluster + node pool + kubeconfig
│   │   ├── main.tf
│   │   ├── variables.tf
│   │   └── outputs.tf
│   └── app/               # K8s namespace + deployment + service
│       ├── main.tf
│       ├── variables.tf
│       └── outputs.tf
└── scripts/
    └── tofu-import.sh     # Import existing live resources into state
```

## Iron Laws

1. **`tofu import` before any `tofu apply`** on existing resources
2. **`tofu plan` must show 0 changes** after successful import
3. **`terraform.tfvars` NEVER committed** — add to `.gitignore`
4. **`*.tfstate` NEVER committed** — remote GCS backend required
5. **Modules called with versioned inputs** — no floating references
6. **`tofu validate` passes in CI** before plan/apply
7. **`null_resource` with `local-exec`** is the ONLY allowed provisioner

## Import Workflow (one-time setup)

```bash
cd ops/tofu
tofu init                      # initialise providers
bash scripts/tofu-import.sh   # import all live resources
tofu plan                      # verify: must show 0 changes
# If plan shows changes: reconcile .tf files to match live state
# then re-run tofu plan until 0 changes
```

## State Backend (GCS)

```hcl
# backend.tf
terraform {
  backend "gcs" {
    bucket = "velvet-tfstate-${var.project_id}"
    prefix = "velvet/state"
  }
}
```

## Module Call Pattern

```hcl
# main.tf
module "networking" {
  source       = "./modules/networking"
  project_id   = var.project_id
  region       = var.region
  cluster_name = var.cluster_name
  ar_repo      = var.ar_repo
}

module "cluster" {
  source       = "./modules/cluster"
  project_id   = var.project_id
  zone         = var.zone
  cluster_name = var.cluster_name
  network_id   = module.networking.network_id
  subnet_id    = module.networking.subnet_id
  depends_on   = [module.networking]
}

module "app" {
  source      = "./modules/app"
  namespace   = var.namespace
  image_uri   = local.image_uri
  depends_on  = [module.cluster]
}
```

## PR Review Checklist for Tofu Changes

- [ ] `tofu validate` passes
- [ ] `tofu plan` output shows only intended changes
- [ ] No `destroy` actions unless explicitly requested
- [ ] `terraform.tfvars` not in diff
- [ ] `*.tfstate` not in diff
- [ ] New variables have `description` and `type`
- [ ] New outputs have `description`

## Cross-References

- `.skills/infrastructure-scalability/SKILL.md` — K8s patterns
- `.skills/container-distroless/SKILL.md` — image the cluster runs
- `.skills/defensive-security/SKILL.md` — secrets in tfvars
- `.skills/code-review-gate/SKILL.md` — tofu review items
