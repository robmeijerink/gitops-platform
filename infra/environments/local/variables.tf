variable "cluster_name" {
  type    = string
  default = "gitops-platform"
}

variable "node_image" {
  description = "kindest/node image; pins the Kubernetes version."
  type        = string
  default     = "kindest/node:v1.31.2"
}

variable "argocd_chart_version" {
  type    = string
  default = "7.7.5"
}

variable "repo_url" {
  type    = string
  default = "https://github.com/robmeijerink/gitops-platform.git"
}

variable "target_revision" {
  description = "Git revision Argo CD tracks. Use a branch name to test changes before merging."
  type        = string
  default     = "main"
}
