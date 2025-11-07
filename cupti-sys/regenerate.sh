#!/usr/bin/env bash

BINDGEN_ARGS=(
    --impl-debug
    --with-derive-default
    --no-prepend-enum-name
    --allowlist-item '[Cc][Uu][Pp][Tt][Ii].*'

    # Delete all non-cupti functions
    --blocklist-function 'cu([^p]|p[^t]|pt[^i]).*'

    --blocklist-type CUcontext

    --raw-line 'use cuda_sys::cuda::*;'
)

# bindgen "${BINDGEN_ARGS[@]}"    \
#     --output src/bindings.rs    \
#     cupti.c                     \
#     --                          \
#     -I /usr/local/cuda/include

cargo xtask regenerate cupti.c      \
    --output src/bindings.rs        \
    --                              \
    -I /usr/local/cuda/include
