# build for wasm32-unknown-unknown
build:
    cargo build \
    --profile wasm-release \
    --target wasm32-unknown-unknown

# run a given example
rune NAME:
    cargo run --example {{NAME}}
