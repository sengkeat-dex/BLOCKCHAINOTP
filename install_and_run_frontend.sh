#!/bin/bash

# Script to install and run the frontend when tools are available

echo "=========================================="
echo "Blockchain OTP Frontend Installer & Runner"
echo "=========================================="

# Check if we're in WSL
if grep -q microsoft /proc/version; then
    echo "Running in WSL"
    
    # Source cargo environment
    source $HOME/.cargo/env
    
    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        
        # Verify installation
        if ! command -v rustc &> /dev/null; then
            echo "ERROR: Failed to install Rust"
            exit 1
        fi
    fi
    
    # Check if Trunk is installed
    if ! command -v trunk &> /dev/null; then
        echo "Installing Trunk..."
        cargo install trunk
        
        if ! command -v trunk &> /dev/null; then
            echo "ERROR: Failed to install Trunk"
            exit 1
        fi
    fi
    
    # Add WebAssembly target if not present
    if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
        echo "Adding WebAssembly target..."
        rustup target add wasm32-unknown-unknown
        
        if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
            echo "ERROR: Failed to add WebAssembly target"
            exit 1
        fi
    fi
    
    echo "All tools installed. Starting frontend..."
    cd /mnt/c/Users/USER/Documents/blockchainotp/frontend
    
    # Kill any existing process on port 8080
    lsof -i :8080 | grep LISTEN | awk '{print $2}' | xargs kill -9 2>/dev/null || true
    
    # Run trunk serve on a different port to avoid conflicts
    trunk serve --port 8081
else
    echo "This script is intended to run in WSL"
    exit 1
fi