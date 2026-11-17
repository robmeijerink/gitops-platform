# 004: Local kind as the Reference Environment

## Decision
The reference environment is a two-node kind cluster created by Terraform (`infra/environments/local`). The control-plane node maps host ports 8080/8444 to ingress-nginx.

## Why
* Anyone can run `task up` for free, and CI runs the same end-to-end flow on a GitHub runner.
* The demo image is built locally and loaded with `kind load` (`task app:load`), so the local overlay needs no registry.

## Consequences
* No cloud-specific parts (load balancers, managed identity, DNS) are demonstrated. A cloud environment is a new Terraform environment plus an overlay.
