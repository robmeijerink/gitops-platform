# Toolbox image: every infra tool this repo needs, pinned. Nothing is installed on the host.
FROM debian:trixie-slim

ARG TERRAFORM_VERSION=1.9.8
ARG KUBECTL_VERSION=v1.31.2
ARG HELM_VERSION=v3.16.2
ARG KIND_VERSION=v0.24.0
ARG KUBECONFORM_VERSION=v0.6.7
ARG TFLINT_VERSION=v0.53.0
ARG DOCKER_CLI_VERSION=27.3.1

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl unzip git jq \
    && rm -rf /var/lib/apt/lists/*

RUN set -eux; \
    arch="$(dpkg --print-architecture)"; \
    case "$arch" in amd64) docker_arch=x86_64 ;; arm64) docker_arch=aarch64 ;; esac; \
    curl -fsSL "https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION#v}/terraform_${TERRAFORM_VERSION#v}_linux_${arch}.zip" -o /tmp/tf.zip \
    && unzip -q /tmp/tf.zip -d /usr/local/bin terraform && rm /tmp/tf.zip; \
    curl -fsSL "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/${arch}/kubectl" -o /usr/local/bin/kubectl; \
    curl -fsSL "https://get.helm.sh/helm-${HELM_VERSION}-linux-${arch}.tar.gz" | tar -xz --strip-components=1 -C /usr/local/bin "linux-${arch}/helm"; \
    curl -fsSL "https://kind.sigs.k8s.io/dl/${KIND_VERSION}/kind-linux-${arch}" -o /usr/local/bin/kind; \
    curl -fsSL "https://github.com/yannh/kubeconform/releases/download/${KUBECONFORM_VERSION}/kubeconform-linux-${arch}.tar.gz" | tar -xz -C /usr/local/bin kubeconform; \
    curl -fsSL "https://github.com/terraform-linters/tflint/releases/download/${TFLINT_VERSION}/tflint_linux_${arch}.zip" -o /tmp/tflint.zip \
    && unzip -q /tmp/tflint.zip -d /usr/local/bin tflint && rm /tmp/tflint.zip; \
    curl -fsSL "https://download.docker.com/linux/static/stable/${docker_arch}/docker-${DOCKER_CLI_VERSION}.tgz" | tar -xz --strip-components=1 -C /usr/local/bin docker/docker; \
    chmod +x /usr/local/bin/*

WORKDIR /workspace
