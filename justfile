VERSION := "1.22.0"

default:
  @just --list

# Download protos schemas from upstream
protos-dl:
    #!/usr/bin/env bash
    set -exuo pipefail
    rm -rf protos && mkdir protos && cd protos
    for x in api apimachinery apiextensions-apiserver kube-aggregator metrics; do
        mkdir ./$x -p
        curl -sSL https://github.com/kubernetes/$x/archive/refs/tags/kubernetes-{{VERSION}}.tar.gz | tar xzf - -C ./$x/ --strip-components=1
        fd -e proto -x sh -c "mkdir -p k8s.io/'{//}'; mv '{}' k8s.io/'{}'" ';' . ./$x
        rm -rf ./$x
    done

# Patch protos schemas to fix import paths
protos-patch:
    #!/usr/bin/env bash
    set -exuo pipefail
    fd -e proto -x sd 'k8s\.io\.(.+);' '$1;' {}
    fd -e proto -x sd 'import "k8s\.io/(.+)";' 'import "$1";' {}
    mv protos/k8s.io/* protos/
    rmdir protos/k8s.io/

# Generate protos path list for prost
protos-list:
    fd -e proto | sort > protos.list

# Download and generate all protos dependent files
protos: protos-dl protos-patch protos-list

# Download swagger
swagger-dl:
    #!/usr/bin/env bash
    set -exuo pipefail
    curl -sSL -o openapi/swagger.json \
        https://raw.githubusercontent.com/kubernetes/kubernetes/v{{VERSION}}/api/openapi-spec/swagger.json

# Patch swagger schema for upstream bugs
swagger-patch:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd openapi
    jq -f patches/patch-nonexistent-gvk.jq < swagger.json > swagger-patched.json
    mv swagger-patched.json swagger.json

# Transform swagger schema into api-resources.json
swagger-transform:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd openapi
    jq -f list-resources.jq < swagger.json > api-resources.json

# Download and generate all swagger dependent files
swagger: swagger-dl swagger-patch swagger-transform

# Build a FileDescriptorSet for custom code generation
build-fds:
    #!/usr/bin/env bash
    set -exuo pipefail
    shopt -s globstar
    protoc \
        --include_imports \
        --include_source_info \
        --descriptor_set_out=protos.fds \
        --proto_path=./protos \
        ./protos/**/*.proto

# Generate the library code from completed swagger and protos
build: build-fds
    #!/usr/bin/env bash
    set -exuo pipefail
    rm -rf out/ && mkdir out
    touch justfile
    cargo build
