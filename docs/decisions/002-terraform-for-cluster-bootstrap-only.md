# 002: Terraform for Cluster Bootstrap Only

## Decision
Terraform creates the cluster (`modules/kind-cluster`), installs Argo CD (`modules/argocd`) and one root Application. Everything inside the cluster is owned by Argo CD.

## Why
* Two controllers managing the same resources fight each other. A hard boundary avoids drift loops.
* The cluster and Argo CD are the only things that can't be delivered by GitOps itself (chicken-and-egg), so they are the only things in Terraform.
* The root Application is passed through the chart's `extraObjects`, so no CRD has to exist at plan time (`kubernetes_manifest` would fail on a fresh cluster).

## Consequences
* Swapping kind for a cloud cluster (GKE, EKS) means a new module and environment; `gitops/` stays untouched.
* State is local and ignored. A remote backend belongs in any shared environment.
