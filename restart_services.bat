@echo off
REM Script to restart the Blockchain OTP services

echo ==========================================
echo Blockchain OTP Service Restart
echo ==========================================

echo Stopping existing services...
taskkill /f /im wslrelay.exe >nul 2>&1

timeout /t 3 /nobreak >nul

echo Starting backend service...
start cmd /k "cd /d c:\Users\USER\Documents\blockchainotp && cargo run"

timeout /t 5 /nobreak >nul

echo Starting frontend service...
start cmd /k "cd /d c:\Users\USER\Documents\blockchainotp\frontend && trunk serve"

echo.
echo Services restarted!
echo Backend: http://localhost:3000
echo Frontend: http://localhost:8080
echo.