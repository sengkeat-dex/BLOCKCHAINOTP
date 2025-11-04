@echo off
REM Script to restart the frontend service

echo ==========================================
echo Restarting Blockchain OTP Frontend Service
echo ==========================================

echo Stopping existing frontend service...
taskkill /f /im wslrelay.exe >nul 2>&1

timeout /t 3 /nobreak >nul

echo Starting frontend service...
cd /d c:\Users\USER\Documents\blockchainotp\frontend
start cmd /k "trunk serve"

echo.
echo Frontend service restarted!
echo Access the application at http://localhost:8080
echo.