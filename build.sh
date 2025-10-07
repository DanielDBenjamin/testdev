#!/bin/bash
set -e

echo "Building backend..."
cargo build --release --bin clock-it --no-default-features --features ssr

echo "Building frontend WASM..."
cargo build --lib --target wasm32-unknown-unknown --no-default-features --features hydrate --profile wasm-release --target-dir ./target/front

echo "Installing wasm-bindgen-cli..."
cargo install wasm-bindgen-cli --version 0.2.103 || true

echo "Running wasm-bindgen..."
wasm-bindgen --target web \
    --no-typescript \
    --out-dir target/site/pkg \
    --out-name clock-it \
    target/front/wasm32-unknown-unknown/wasm-release/clock_it.wasm

echo "Optimizing WASM with all required features..."
wasm-opt --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext --enable-mutable-globals -Oz \
    target/site/pkg/clock-it_bg.wasm \
    -o target/site/pkg/clock-it_bg.wasm

echo "Compiling SCSS to CSS..."
mkdir -p target/site/pkg
sass style/main.scss target/site/pkg/clock-it.css

echo "Copying backend binary..."
cp target/release/clock-it ./clock-it

echo "Copying public assets..."
if [ -d "public" ]; then
    echo "Copying public directory..."
    cp -r public/* target/site/ 2>/dev/null || true
fi

echo "Final pkg directory contents:"
ls -la target/site/pkg/

echo "✅ Build complete!"