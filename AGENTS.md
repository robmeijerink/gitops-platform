# AGENTS.md

GitOps reference platform: Terraform creates a kind cluster and installs Argo CD; Argo CD syncs everything else from `gitops/`. All tooling runs in Docker via Task; Terraform, kubectl, helm and Rust on the host are not used.

## Commands

- `task check`: Terraform fmt/validate/tflint, kubeconform on the manifests, then fmt, Clippy (warnings are errors) and tests for the demo app. Run it after every change; work is done only when it passes.
- `task up` / `task down`: create / destroy the whole platform (needs Docker; takes minutes). Don't run them unless asked.
- `task tf -- <args>`, `task kubectl -- <args>`: any other Terraform or kubectl command.
- Don't run `task argocd:ui` or `task grafana:ui` (they never exit) or a bare `task` (interactive menu).
- Never run `terraform`, `kubectl`, `helm` or `cargo` directly: the host versions may differ from the pinned ones.

## Layout

- `infra/`: Terraform only. `modules/` are reusable, `environments/local` wires them together.
- `gitops/`: everything Argo CD syncs. `bootstrap/` is the app-of-apps entry point, `platform/` holds ingress and monitoring, `apps/` holds workloads (Kustomize base + overlays).
- `apps/demo-app/`: Rust service, same conventions as rust-skeleton (strict lints, `lib.rs` holds the logic).
- Architecture decisions live in `docs/decisions/`; read the relevant one before changing that area, and add one for each new decision.

## Rules

- Terraform owns the cluster, Argo CD and the root Application, nothing else. Never add `helm_release` or `kubernetes_*` resources for platform or app workloads: add an Argo CD Application under `gitops/` instead.
- Change the cluster through Git, not `kubectl apply`. Argo CD self-heals manual edits away.
- Pin every version: provider constraints, Helm chart versions, tool versions in `.devcontainer/Dockerfile`, image tags (no `latest` in overlays). Commit `.terraform.lock.hcl`.
- No secrets in Git. Terraform state is local and ignored.
- Workloads run non-root, with a read-only root filesystem, dropped capabilities, resource requests and probes.
- Demo app: for new behavior, write a failing test first. Ask before adding a dependency. Servers listen on `0.0.0.0` and shut down gracefully on SIGTERM. No `unwrap`/`panic` outside tests.
- Keep changes focused; don't refactor unrelated code.
