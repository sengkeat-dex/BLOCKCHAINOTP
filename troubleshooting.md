# Blockchain OTP System Troubleshooting Guide

## Common Issues and Solutions

### 1. "Failed to request OTP. Please try again" Error

#### Symptoms
- Frontend shows "Failed to request OTP. Please try again"
- Backend API works when tested directly
- No obvious error messages in the console

#### Possible Causes and Solutions

1. **CORS (Cross-Origin Resource Sharing) Issue**
   - **Cause**: Frontend (port 8080) and backend (port 3000) are on different ports
   - **Solution**: CORS middleware has been added to the backend. Restart services to apply changes.

2. **Network Connectivity Issues**
   - **Cause**: Frontend cannot reach backend due to network configuration
   - **Solution**: 
     - Verify both services are running: `netstat -ano | findstr ":3000"`
     - Test direct API access: Use the test script provided
     - Check firewall settings

3. **Browser Security Restrictions**
   - **Cause**: Browser blocking requests due to security policies
   - **Solution**:
     - Ensure you're accessing the frontend via `http://localhost:8080`
     - Check browser console for specific error messages
     - Try a different browser

4. **Backend Service Issues**
   - **Cause**: Backend service is running but experiencing internal errors
   - **Solution**:
     - Check backend terminal for error messages
     - Restart backend service
     - Verify all dependencies are correctly installed

### 2. How to Verify Services Are Running

#### Check Backend
```bash
# Test health endpoint
curl http://localhost:3000/health

# Should return: OK
```

#### Check Frontend
```bash
# Open in browser
http://localhost:8080
```

#### Check Ports
```bash
# Windows command to check listening ports
netstat -ano | findstr ":3000"
netstat -ano | findstr ":8080"
```

### 3. Testing the API Directly

You can test the backend API using PowerShell:

```powershell
# Test health endpoint
Invoke-WebRequest -Uri "http://localhost:3000/health" -UseBasicParsing

# Test OTP request
$body = @{user_id="user-123"} | ConvertTo-Json
Invoke-WebRequest -Uri "http://localhost:3000/otp/request" -Method POST -Headers @{"Content-Type"="application/json"} -Body $body -UseBasicParsing
```

### 4. Browser Console Debugging

To debug frontend issues:

1. Open your browser's developer tools (F12)
2. Go to the Console tab
3. Look for error messages when clicking "Request OTP"
4. Common errors:
   - CORS errors (blocked by CORS policy)
   - Network errors (failed to fetch)
   - JSON parsing errors

### 5. Restarting Services

If changes have been made to the code:

1. Stop the current services:
   ```bash
   # In the terminal running the backend
   Ctrl+C
   
   # In the terminal running the frontend
   Ctrl+C
   ```

2. Restart the backend:
   ```bash
   cargo run
   ```

3. Restart the frontend:
   ```bash
   cd frontend
   trunk serve
   ```

### 6. Verifying CORS Configuration

The backend now includes CORS middleware to allow requests from the frontend:

```rust
Router::new()
    .route("/health", get(health_check))
    .route("/otp/request", post(request_otp))
    .route("/otp/verify", post(verify_otp))
    .layer(tower_http::cors::CorsLayer::permissive())
    .with_state(state)
```

This allows all origins, methods, and headers. In production, you should restrict this to only the necessary origins.

### 7. Common Error Messages

| Error Message | Likely Cause | Solution |
|---------------|--------------|----------|
| "Failed to fetch" | Network/CORS issue | Check CORS configuration, verify services are running |
| "blocked by CORS policy" | CORS misconfiguration | Verify CORS middleware is properly configured |
| "429 Too Many Requests" | Rate limiting | Wait for token bucket to refill (1 request per 100 seconds after 3 requests) |
| "500 Internal Server Error" | Backend error | Check backend logs for specific error |

### 8. Additional Debugging Steps

1. **Check the backend logs**: Look for any error messages when the frontend makes requests
2. **Verify the request format**: Ensure the frontend is sending the correct JSON format
3. **Test with a simple HTML page**: Use the provided cors_test.html to verify basic connectivity
4. **Check browser extensions**: Some browser extensions can interfere with requests

### 9. If Problems Persist

1. Restart both backend and frontend services
2. Clear browser cache and cookies
3. Try accessing from an incognito/private browsing window
4. Check Windows Firewall settings
5. Verify that no other services are using ports 3000 or 8080

### 10. Verifying the Complete Flow

Once the services are running correctly:

1. Open your browser to http://localhost:8080
2. Enter a user ID (e.g., "user-123")
3. Click "Request OTP"
4. Check the backend terminal for the generated OTP
5. Enter the OTP in the frontend
6. Click "Verify OTP"
7. You should see a success message

If you continue to experience issues, please provide:
1. The exact error message
2. Browser console output
3. Backend terminal output
4. Results of the direct API tests