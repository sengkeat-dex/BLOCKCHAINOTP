@echo off
REM Complete Blockchain OTP System Runner

echo ==========================================
echo Blockchain OTP Complete System
echo ==========================================

echo This script provides instructions for running the complete system.
echo You'll need to run the backend and frontend in separate terminals.
echo.

echo INSTRUCTIONS:
echo =============
echo 1. Backend Service:
echo    - Open a new terminal
echo    - Navigate to this directory
echo    - Run: cargo run
echo    - The backend will start on http://localhost:3000
echo.
echo 2. Frontend Application:
echo    - Open another terminal
echo    - Navigate to the frontend directory: cd frontend
echo    - Run: trunk serve
echo    - The frontend will start on http://localhost:8080
echo.
echo 3. Using the Application:
echo    - Open your browser to http://localhost:8080
echo    - Enter a user ID and click "Request OTP"
echo    - Check the backend terminal for the generated OTP
echo    - Enter the OTP in the frontend and click "Verify OTP"
echo.
echo NOTE: You must have Rust and Trunk installed to run this system.
echo If you don't have them installed, please follow these steps:
echo 1. Install Rust: https://www.rust-lang.org/
echo 2. Install Trunk: cargo install trunk
echo 3. Add WebAssembly target: rustup target add wasm32-unknown-unknown
echo.
pause