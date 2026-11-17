locals {
  kubeconfig_path = abspath("${path.root}/../../../.kube/config")
}

module "cluster" {
  source = "../../modules/kind-cluster"

  name            = var.cluster_name
  node_image      = var.node_image
  kubeconfig_path = local.kubeconfig_path
}

provider "helm" {
  kubernetes {
    host                   = module.cluster.endpoint
    client_certificate     = module.cluster.client_certificate
    client_key             = module.cluster.client_key
    cluster_ca_certificate = module.cluster.cluster_ca_certificate
  }
}

module "argocd" {
  source = "../../modules/argocd"

  chart_version   = var.argocd_chart_version
  repo_url        = var.repo_url
  target_revision = var.target_revision
}
