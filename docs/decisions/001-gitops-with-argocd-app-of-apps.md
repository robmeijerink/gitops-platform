# 001: GitOps with Argo CD (App of Apps)

## Decision
Argo CD syncs the cluster from this repository. One root Application watches `gitops/bootstrap/`, which contains an Application for the platform services and one for the demo app. The platform Application in turn contains one Application per service (ingress-nginx, kube-prometheus-stack).

## Why
* Git is the single source of truth and audit log; a manual change is reverted by self-heal.
* Adding a service is adding one file; there is no pipeline that needs cluster credentials.
* Sync waves order the rollout: monitoring CRDs exist before the demo app's `ServiceMonitor`.

## Trade-offs
* The Applications hardcode `repoURL` and `targetRevision: main`, so a branch can't be tested end-to-end before merging. Acceptable for a reference repo; use ApplicationSets if that becomes a need.
