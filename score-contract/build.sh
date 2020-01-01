#!/bin/bash
set -e

cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/score_contract.wasm ./res/
#wasm-opt -Oz --output ./res/cross_contract_high_level.wasm ./res/cross_contract_high_level.wasm
rm -rf target
