@echo off
REM Blockchain OTP System Build Script for Windows

echo Building Blockchain OTP System...

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed. Please install Rust from https://www.rust-lang.org/
    exit /b 1
)

REM Check if Trunk is installed
trunk --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Trunk is not installed. Installing Trunk...
    cargo install trunk
)

REM Add WebAssembly target
echo Adding WebAssembly target...
rustup target add wasm32-unknown-unknown

REM Build backend
echo Building backend...
cargo build

REM Build frontend
echo Building frontend...
cd frontend
trunk build

echo Build complete!
echo To run the backend: cargo run
echo To run the frontend: cd frontend && trunk serve