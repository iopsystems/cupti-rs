#!/usr/bin/env bash

set -euo pipefail
cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null

cargo xtask regenerate cupti.cpp    \
    --output src/bindings.rs        \
    --                              \
    -I /usr/local/cuda/include

grep -EhR '#define (CUpti_[a-zA-Z0-9_]+_STRUCT_SIZE)\s+CUPTI_[A-Za-z0-9_]*STRUCT_SIZE\([A-Za-z_0-9]+,\s+[a-zA-Z0-9]+\)' /usr/local/cuda/include \
    | sed -E 's/#define (CUpti_[a-zA-Z0-9_]+_STRUCT_SIZE)\s+CUPTI_[A-Z]+_STRUCT_SIZE\(([A-Za-z0-9_]+), ([A-Za-z0-9_]+)\)/pub const \1: usize = cupti_struct_size!(\2, \3);/g' \
    > src/sizes.rs