#!/usr/bin/env bash

set -euo pipefail
cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null

cargo xtask regenerate cupti.cpp    \
    --output src/bindings.rs        \
    --                              \
    -I /usr/local/cuda/include
