variable "name" {
  description = "Name of the kind cluster."
  type        = string
}

variable "node_image" {
  description = "kindest/node image, which pins the Kubernetes version."
  type        = string
}

variable "kubeconfig_path" {
  description = "Absolute path the kubeconfig is written to."
  type        = string
}

variable "http_port" {
  description = "Host port mapped to the ingress controller (HTTP)."
  type        = number
  default     = 8080
}

variable "https_port" {
  description = "Host port mapped to the ingress controller (HTTPS)."
  type        = number
  default     = 8444
}
