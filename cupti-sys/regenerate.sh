#!/usr/bin/env bash

cargo xtask regenerate cupti.cpp    \
    --output src/bindings.rs        \
    --                              \
    -I /usr/local/cuda/include

# Format the generated bindings with rustfmt
rustfmt src/bindings.rs
