# Test the OTP API endpoints

Write-Host "Testing Blockchain OTP Backend API" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Test health endpoint
Write-Host "`n1. Testing health endpoint..." -ForegroundColor Yellow
try {
    $healthResponse = Invoke-WebRequest -Uri "http://localhost:3000/health" -UseBasicParsing
    Write-Host "   Status: $($healthResponse.StatusCode)" -ForegroundColor Green
    Write-Host "   Response: $($healthResponse.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
}

# Test OTP request endpoint
Write-Host "`n2. Testing OTP request endpoint..." -ForegroundColor Yellow
try {
    $body = @{
        user_id = "user-123"
    } | ConvertTo-Json
    
    $otpResponse = Invoke-WebRequest -Uri "http://localhost:3000/otp/request" -Method POST -Headers @{ "Content-Type" = "application/json" } -Body $body -UseBasicParsing
    Write-Host "   Status: $($otpResponse.StatusCode)" -ForegroundColor Green
    Write-Host "   Response: $($otpResponse.Content)" -ForegroundColor Cyan
} catch {
    Write-Host "   Error: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        Write-Host "   Response Status: $($_.Exception.Response.StatusCode.value__)" -ForegroundColor Red
    }
}

Write-Host "`nTest completed." -ForegroundColor Green