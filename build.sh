#!/bin/bash

# Blockchain OTP System Build Script

echo "Building Blockchain OTP System..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Please install Rust from https://www.rust-lang.org/"
    exit 1
fi

# Check if Trunk is installed
if ! command -v trunk &> /dev/null
then
    echo "Trunk is not installed. Installing Trunk..."
    cargo install trunk
fi

# Add WebAssembly target
echo "Adding WebAssembly target..."
rustup target add wasm32-unknown-unknown

# Build backend
echo "Building backend..."
cargo build

# Build frontend
echo "Building frontend..."
cd frontend
trunk build

echo "Build complete!"
echo "To run the backend: cargo run"
echo "To run the frontend: cd frontend && trunk serve"