#!/usr/bin/env bash

cargo xtask regenerate cupti.c      \
    --output src/bindings.rs        \
    --                              \
    -I /usr/local/cuda/include
