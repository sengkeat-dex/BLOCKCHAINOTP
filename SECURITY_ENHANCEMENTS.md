# Security Enhancements Based on OTP Algorithm Document

This document explains how our blockchain OTP implementation incorporates the security recommendations from the [algorithm-otp.md](file:///c:/Users/USER/Documents/blockchainotp/algorithm-otp.md) document.

## 1. Algorithm Selection

Our implementation uses a **hybrid TOTP approach** as recommended in the algorithm document:

- **Off-chain TOTP generation**: We generate time-based OTPs using HMAC-SHA256 according to RFC 6238
- **On-chain verification**: We store only the Keccak256 hash of the OTP plus expiration time
- **One-time use**: Verified OTPs are marked as used to prevent replay attacks

## 2. Cryptographic Primitives

We've implemented the recommended production-grade cryptographic primitives:

### Hash Functions
- **Keccak-256**: Used for hashing OTPs (native on EVM)
- **Domain Separation**: Added "OTP:v1:" prefix to prevent cross-application hash reuse

### HMAC
- **HMAC-SHA256**: Used for TOTP generation according to RFC 6238

### CSPRNG
- **OS RNG**: Used for generating secure random values for request IDs and salts

## 3. Recommended Parameters

Our implementation follows the recommended parameters:

| Setting | Our Implementation | Recommended Value |
|---------|-------------------|-------------------|
| OTP digits | 6 | 6 (or 8 for higher security) |
| TOTP step | 60 seconds | 60s |
| Expiry | 60 seconds | ≤ 60s |
| Max attempts | 3 per requestId | 3 per requestId |
| Hash domain tag | "OTP:v1:" | "OTP:v1:<chainId>:<app>" |

## 4. Security Features Implemented

### Freshness
- OTPs expire after 60 seconds
- Time-based generation ensures each OTP is fresh

### Uniqueness
- Each OTP is tied to a unique request ID
- One-time use enforcement with `used` flag

### Unforgeability
- Cryptographically secure random number generation
- HMAC-SHA256 for TOTP generation
- Only hash stored on-chain, not the OTP itself

### Bind to User/Action
- Request IDs are bound to users using `keccak256(userId || random32)`
- Off-chain storage maintains user-to-request mapping

### Auditability
- Events emitted for all important actions:
  - `OtpSet` when OTP is stored
  - `OtpVerified` when OTP is successfully verified
  - `AttemptFailed` when verification fails
- On-chain storage allows for transparent verification

### No Secret Leakage
- Only OTP hashes stored on-chain
- Secrets used for TOTP generation kept off-chain

## 5. Rate Limiting Implementation

We've implemented the recommended token bucket algorithm:

- **Token Bucket**: Per-user rate limiting with capacity of 3 tokens
- **Refill Rate**: 1 token per 100 seconds (3 tokens per 5 minutes)
- **Consumption**: 1 token per OTP request

## 6. Binding & Anti-Replay

### User Binding
- Request IDs derived from `keccak256(userId || random32)`
- Off-chain storage maintains `(requestId → userId)` mapping for audit

### One-time Use
- `used=true` flag set on successful verification
- Rejected replays with "used" error

### Time Guard
- `block.timestamp ≤ expiry` check during verification
- Automatic expiration after 60 seconds

## 7. Attempt Limiting

We've implemented attempt counting per `requestId`:

- Maximum 3 attempts per OTP
- Counter reset on successful verification
- Automatic blocking after 3 failed attempts

## 8. Gas Optimization

Our on-chain data layout follows the gas-aware recommendations:

- `expiry` packed into `uint64`
- `used` as a single `bool`
- Both stored in the same slot as `bytes32 hash` (2 storage slots total)
- Events for audit without additional storage costs

## 9. Key Derivation

While not fully implemented yet, our architecture supports the recommended key derivation pattern:

```
master_key  --HKDF(info="otp/seed/<app>")--> issuer_seed
issuer_seed --HKDF(info="otp/user/<userId>")--> user_seed
user_seed   --HKDF(info="otp/action/<scope>")--> action_seed
```

This would allow isolation across users and actions even if one seed leaks.

## 10. Attack Mitigations

Our implementation addresses the following attacks:

| Attack | Mitigation Implemented |
|--------|------------------------|
| Brute force | Token bucket + attempt counter; 6 digits; 60s TTL |
| Replay | `used=true` + expiry check |
| Mix-up (wrong user) | `requestId` derived from `userId` |
| Commit rainbow | Domain separation in hashing |
| Front-running reveal | User-bound requestId |
| Time drift (TOTP) | Small window (60s) |

## 11. Future Enhancements

Based on the "Pick Two" Playbook, we could implement:

1. **Privacy-sensitive upgrade**: Replace plaintext OTP reveal with ZK-OTP
2. **Trust-minimizing upgrade**: Add oracle-signed or threshold-signed attestations

## 12. Compliance with RFC Standards

Our TOTP implementation is compliant with:
- **RFC 4226** (HOTP): HMAC-based One-Time Password Algorithm
- **RFC 6238** (TOTP): Time-based One-Time Password Algorithm

This ensures compatibility with standard OTP tools and libraries.