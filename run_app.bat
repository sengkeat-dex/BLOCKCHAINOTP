@echo off
REM Blockchain OTP System Runner

echo ==========================================
echo Blockchain OTP System
echo ==========================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust is not installed on this system.
    echo.
    echo To run this application, you need to install Rust:
    echo 1. Visit https://www.rust-lang.org/
    echo 2. Download and install Rust for Windows
    echo 3. Restart your command prompt
    echo.
    echo After installing Rust, you can run this application with:
    echo    cargo run
    echo.
    echo The backend API will be available at http://localhost:3000
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

echo Starting Blockchain OTP Backend Service...
echo The service will be available at http://localhost:3000
echo.
echo Press Ctrl+C to stop the service
echo.

REM Run the application
cargo run

pause