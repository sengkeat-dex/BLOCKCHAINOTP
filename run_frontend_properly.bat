@echo off
REM Script to run the frontend properly when tools are available

echo ==========================================
echo Blockchain OTP Frontend Runner
echo ==========================================

echo Checking for required tools...

REM Check if we're in WSL
wsl -e bash -c "grep -q microsoft /proc/version" >nul 2>&1
if %errorlevel% equ 0 (
    echo Running in WSL environment
    echo Starting installation and run script...
    wsl -e /mnt/c/Users/USER/Documents/blockchainotp/install_and_run_frontend.sh
    exit /b 0
)

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust is not installed on this system.
    echo.
    echo To run this application properly, you need to install Rust:
    echo 1. Visit https://www.rust-lang.org/
    echo 2. Download and install Rust for Windows
    echo 3. Restart your command prompt
    echo.
    pause
    exit /b 1
)

REM Check if Cargo is available
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Cargo is not available.
    echo.
    echo Please ensure you have a complete Rust installation.
    echo.
    pause
    exit /b 1
)

REM Check if Trunk is installed
trunk --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing Trunk...
    cargo install trunk
    if %errorlevel% neq 0 (
        echo ERROR: Failed to install Trunk.
        pause
        exit /b 1
    )
)

REM Check if WebAssembly target is installed
rustup target list --installed | findstr wasm32-unknown-unknown >nul 2>&1
if %errorlevel% neq 0 (
    echo Adding WebAssembly target...
    rustup target add wasm32-unknown-unknown
    if %errorlevel% neq 0 (
        echo ERROR: Failed to add WebAssembly target.
        pause
        exit /b 1
    )
)

echo All tools are installed. Starting the frontend...

REM Change to frontend directory and run trunk serve
cd /d c:\Users\USER\Documents\blockchainotp\frontend
trunk serve

pause