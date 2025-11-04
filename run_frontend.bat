@echo off
REM Blockchain OTP Frontend Runner

echo ==========================================
echo Blockchain OTP Frontend
echo ==========================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust is not installed on this system.
    echo.
    echo To run this frontend application, you need to install Rust:
    echo 1. Visit https://www.rust-lang.org/
    echo 2. Download and install Rust for Windows
    echo 3. Restart your command prompt
    echo.
    echo After installing Rust, you also need to:
    echo 1. Install Trunk: cargo install trunk
    echo 2. Add WebAssembly target: rustup target add wasm32-unknown-unknown
    echo.
    echo Then you can run the frontend with:
    echo    cd frontend
    echo    trunk serve
    echo.
    echo The frontend will be available at http://localhost:8080
    echo.
    pause
    exit /b 1
)

REM Check if Trunk is installed
trunk --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Trunk is not installed.
    echo.
    echo Please install Trunk with:
    echo    cargo install trunk
    echo.
    pause
    exit /b 1
)

REM Check if WebAssembly target is installed
rustup target list --installed | findstr wasm32-unknown-unknown >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: WebAssembly target is not installed.
    echo.
    echo Please add the WebAssembly target with:
    echo    rustup target add wasm32-unknown-unknown
    echo.
    pause
    exit /b 1
)

echo Starting Blockchain OTP Frontend...
echo.
echo Navigate to http://localhost:8080 in your browser
echo.
echo Press Ctrl+C to stop the frontend server
echo.

REM Change to frontend directory and run trunk serve
cd frontend
trunk serve

pause