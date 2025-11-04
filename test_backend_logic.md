# Backend Logic Testing Guide

This document explains how to test the backend logic of the Blockchain OTP System.

## Overview

The backend consists of several key components:
1. OTP generation using TOTP algorithm
2. Rate limiting using token bucket algorithm
3. OTP verification with security checks
4. State management for OTP requests

## Testing the Components

### 1. OTP Generation

The system generates TOTP codes using:
- Time step of 60 seconds
- 6-digit codes
- HMAC-SHA256 algorithm
- Secure user binding for request IDs

To test this logic:
1. The [crates/otp-core](file:///c:/Users/USER/Documents/blockchainotp/crates/otp-core) crate contains the TOTP implementation
2. Tests are included in [crates/otp-core/tests/totp_test.rs](file:///c:/Users/USER/Documents/blockchainotp/crates/otp-core/tests/totp_test.rs)
3. Run with: `cargo test -p otp-core`

### 2. Rate Limiting

The system implements token bucket rate limiting:
- 3 tokens capacity
- Refill rate of 1 token per 100 seconds
- Per-user limiting

To test this logic:
1. The rate limiting function is in [src/lib.rs](file:///c:/Users/USER/Documents/blockchainotp/src/lib.rs)
2. Tests can be added to verify the token bucket behavior

### 3. OTP Verification

The verification process includes:
- Checking OTP hasn't expired
- Checking OTP hasn't been used
- Comparing hashed OTP values
- Marking OTP as used after successful verification

To test this logic:
1. The verification function is in [src/lib.rs](file:///c:/Users/USER/Documents/blockchainotp/src/lib.rs)
2. Integration tests can verify the complete flow

## Running Tests

When Rust is installed, you can run tests with:

```bash
# Run all tests
cargo test

# Run tests for specific components
cargo test -p otp-core
cargo test -p blockchain-otp

# Run tests with output
cargo test -- --nocapture
```

## Test Scenarios

### Successful OTP Flow
1. Request OTP for a user
2. Verify the OTP within 60 seconds
3. Confirm OTP is marked as used
4. Confirm subsequent verifications fail

### Expired OTP
1. Request OTP for a user
2. Wait 61 seconds
3. Attempt verification
4. Confirm verification fails

### Used OTP
1. Request OTP for a user
2. Verify the OTP
3. Attempt to verify the same OTP again
4. Confirm second verification fails

### Rate Limiting
1. Request 3 OTPs rapidly for the same user
2. Confirm all 3 succeed
3. Request a 4th OTP immediately
4. Confirm the 4th request is rejected
5. Wait appropriate time and confirm requests succeed again

### Invalid OTP
1. Request OTP for a user
2. Attempt verification with wrong OTP
3. Confirm verification fails

## Integration Testing

The system can be tested end-to-end by:

1. Starting the backend service: `cargo run`
2. Making HTTP requests to the endpoints:
   - `POST /otp/request` with JSON `{"user_id": "test-user"}`
   - `POST /otp/verify` with JSON `{"request_id": "...", "otp": "123456"}`
3. Verifying responses match expectations

## Example Test Requests

### Request OTP
```bash
curl -X POST http://localhost:3000/otp/request \
  -H "Content-Type: application/json" \
  -d '{"user_id": "test-user"}'
```

Expected response:
```json
{
  "request_id": "0x...",
  "expires_at": 1234567890
}
```

### Verify OTP
```bash
curl -X POST http://localhost:3000/otp/verify \
  -H "Content-Type: application/json" \
  -d '{"request_id": "0x...", "otp": "123456"}'
```

Expected response:
```json
{
  "verified": true
}
```

## Health Check
```bash
curl http://localhost:3000/health
```

Expected response:
```
OK
```

## Troubleshooting

### Common Issues

1. **Port already in use**: Change the port in [src/main.rs](file:///c:/Users/USER/Documents/blockchainotp/src/main.rs)
2. **CORS errors**: Configure CORS middleware in the Axum router
3. **Rate limiting**: Wait for token bucket to refill
4. **Expired OTPs**: Request a new OTP

### Debugging

The backend includes logging to help with debugging:
- OTP generation is logged to console
- Errors are returned as HTTP status codes
- State can be inspected through the AppState methods

## Performance Testing

For performance testing, you can:

1. Use tools like `ab` (Apache Bench) or `wrk`
2. Test concurrent requests
3. Measure response times
4. Verify rate limiting works under load

Example with Apache Bench:
```bash
ab -n 100 -c 10 -p request.json -T "application/json" http://localhost:3000/otp/request
```

Where `request.json` contains:
```json
{"user_id": "test-user"}
```

## Security Testing

Security aspects to verify:

1. **OTP secrecy**: Only hashes are stored, never plaintext OTPs
2. **Rate limiting**: Prevents brute force attacks
3. **Replay protection**: OTPs can only be used once
4. **Expiration**: OTPs expire after 60 seconds
5. **User binding**: Request IDs are bound to specific users