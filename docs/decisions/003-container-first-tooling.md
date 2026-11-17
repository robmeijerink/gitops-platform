# 003: Container-First Tooling

## Decision
Terraform, kubectl, helm, kind, kubeconform and tflint live in one pinned toolbox image (`.devcontainer/Dockerfile`). Every Task command runs inside it with `docker compose run`. The demo app uses the `rust:1-trixie` image and the shared `rust_cargo`/`rust_rustup` volumes, like rust-skeleton.

## Why
* Only Docker and Task are needed; versions are identical on every machine and in CI (`task check` is what CI runs).
* The toolbox mounts the Docker socket so kind can create its nodes on the host daemon, and uses host networking to reach the API server on localhost.

## Consequences
* Mounting the Docker socket gives the toolbox control over the host's Docker. Fine for a local dev tool; don't copy this setup into a shared CI runner you don't control.
