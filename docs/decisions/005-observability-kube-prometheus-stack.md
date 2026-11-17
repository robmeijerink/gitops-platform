# 005: Observability with kube-prometheus-stack

## Decision
Prometheus, Alertmanager and Grafana come from the `kube-prometheus-stack` chart, deployed by Argo CD. Apps ship their own monitoring as manifests next to the workload: a `ServiceMonitor`, a `PrometheusRule` and a Grafana dashboard `ConfigMap` (loaded by the Grafana sidecar).

## Why
* Monitoring is part of the app's definition of done and is versioned and reviewed with it.
* The selector `*NilUsesHelmValues` options are set to `false` so Prometheus discovers monitors and rules in any namespace without special labels.

## Consequences
* The stack is sized for local use (2 days retention, no persistence). Persistence and Alertmanager receivers are needed before production use.
