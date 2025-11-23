# list all available subcommands
_default:
  @just --list

# build for wasm32-unknown-unknown
build:
  cargo build \
  --profile wasm-release \
  --target wasm32-unknown-unknown

# run a given example
run NAME:
  cargo run --example {{NAME}}
