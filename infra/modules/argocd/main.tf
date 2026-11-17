# Terraform installs Argo CD and one root Application. From there Argo CD owns
# everything inside the cluster (see docs/decisions/002).
resource "helm_release" "argocd" {
  name             = "argocd"
  namespace        = var.namespace
  create_namespace = true
  repository       = "https://argoproj.github.io/argo-helm"
  chart            = "argo-cd"
  version          = var.chart_version
  wait             = true

  values = [
    yamlencode({
      # The root Application is rendered by the chart itself, so no CRD has to
      # exist at plan time.
      extraObjects = [
        {
          apiVersion = "argoproj.io/v1alpha1"
          kind       = "Application"
          metadata = {
            name       = "root"
            namespace  = var.namespace
            finalizers = ["resources-finalizer.argocd.argoproj.io"]
          }
          spec = {
            project = "default"
            source = {
              repoURL        = var.repo_url
              targetRevision = var.target_revision
              path           = var.bootstrap_path
            }
            destination = {
              server    = "https://kubernetes.default.svc"
              namespace = var.namespace
            }
            syncPolicy = {
              automated   = { prune = true, selfHeal = true }
              syncOptions = ["CreateNamespace=true"]
            }
          }
        }
      ]
    })
  ]
}
