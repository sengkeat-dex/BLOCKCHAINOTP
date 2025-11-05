#!/bin/bash

# Script to manually build the WASM and prepare it for Trunk

# Source cargo environment
source $HOME/.cargo/env

# Change to the frontend directory
cd /mnt/c/Users/USER/Documents/blockchainotp/frontend

echo "Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "Running wasm-bindgen..."
if command -v wasm-bindgen &> /dev/null
then
    wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/otp_frontend.wasm
else
    echo "wasm-bindgen is not installed. Skipping this step."
    echo "To install wasm-bindgen, run: cargo install wasm-bindgen-cli"
    exit 1
fi

echo "Copying files to dist directory..."
mkdir -p dist
cp pkg/* dist/
cp index.html dist/
cp styles.css dist/
cp metamask.js dist/
cp solana.js dist/

echo "Build complete! You can now serve the dist directory with any HTTP server."