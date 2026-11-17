variable "chart_version" {
  description = "argo-cd Helm chart version."
  type        = string
}

variable "namespace" {
  type    = string
  default = "argocd"
}

variable "repo_url" {
  description = "Git repository Argo CD syncs from."
  type        = string
}

variable "target_revision" {
  description = "Branch, tag or commit of the Git repository."
  type        = string
  default     = "main"
}

variable "bootstrap_path" {
  description = "Path in the repository holding the app-of-apps Applications."
  type        = string
  default     = "gitops/bootstrap"
}
