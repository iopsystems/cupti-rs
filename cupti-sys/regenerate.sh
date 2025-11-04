#!/usr/bin/env bash

BINDGEN_ARGS=(
    --impl-debug
    --with-derive-default
    --no-prepend-enum-name
    --allowlist-item '[Cc][Uu][Pp][Tt][Ii].*'
)

bindgen "${BINDGEN_ARGS[@]}"    \
    --output src/bindings.rs    \
    cupti.c                     \
    --                          \
    -I /usr/local/cuda/include
