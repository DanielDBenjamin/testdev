#!/bin/bash
set -e

echo "Building Rust application for Linux..."
cargo build --release --target x86_64-unknown-linux-musl

echo "Building WASM components..."
cargo build --release --target wasm32-unknown-unknown

echo "Optimizing WASM with bulk memory support..."
find target/wasm32-unknown-unknown/release -name "*.wasm" -type f | while read wasm_file; do
    echo "Optimizing $wasm_file"
    wasm-opt --enable-bulk-memory -Oz "$wasm_file" -o "$wasm_file"
done

echo "Build complete!"