#!/bin/bash

# Script to manually build the WASM and prepare it for Trunk

# Source cargo environment
source $HOME/.cargo/env

# Change to the frontend directory
cd /mnt/c/Users/USER/Documents/blockchainotp/frontend

echo "Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "Running wasm-bindgen..."
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/otp_frontend.wasm

echo "Copying files to dist directory..."
mkdir -p dist
cp pkg/* dist/
cp index.html dist/
cp styles.css dist/
cp metamask.js dist/

echo "Build complete! You can now serve the dist directory with any HTTP server."