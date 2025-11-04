# Blockchain OTP System Development Rules

## Overview
This document defines the development rules and standards for implementing the blockchain-based OTP system described in [layout.md](layout.md) and [security-protection-testing-layer.md](security-protection-testing-layer.md).

## Core Principles

### Security First
- Never store OTP plaintext values after generation
- Always hash OTP values before storing or transmitting
- Implement rate limiting (max 3 attempts per request)
- Use short expiration times (60 seconds maximum)
- Bind OTP requests to specific users
- Mark OTP entries as used after successful verification

### Transparency and Auditability
- Store only hashes and metadata on-chain
- Log all OTP generation and verification events
- Maintain immutable records of transactions
- Enable traceability from request to verification

### Minimal Viable Implementation
- Focus on essential features only
- Avoid unnecessary complexity
- Prioritize correctness over performance initially
- Keep smart contracts small and focused

## Technical Requirements

### Smart Contract Rules
1. Use Solidity 0.8.20 or higher
2. Implement proper access controls (onlyIssuer modifier)
3. Validate all inputs with require statements
4. Emit events for all state-changing operations
5. Ensure proper handling of edge cases (expired, used, invalid OTPs)
6. Implement pause functionality for emergency situations
7. Support issuer rotation for key management

### Backend Service Rules
1. Use Rust with Axum framework
2. Generate cryptographically secure 6-digit OTP codes
3. Hash OTPs using Keccak256 before blockchain storage
4. Implement rate limiting per user/IP
5. Securely deliver OTPs to users via approved channels
6. Store only necessary metadata off-chain
7. Implement proper error handling and logging

### Cryptographic Standards
1. Use Keccak256 for hashing OTP values
2. Generate random request IDs using secure random functions
3. Implement proper nonce handling for wallet signatures
4. Use TLS 1.2+ for all network communications
5. Encrypt sensitive data at rest using AES-GCM

## Implementation Constraints

### OTP Generation
- OTPs must be exactly 6 digits (0-9)
- Must use cryptographically secure random number generation
- No predictable patterns in generated codes
- Reject codes with leading zeros (optional)

### Storage Requirements
- Never store plaintext OTP values
- Store only hashes, expiration times, and usage status on-chain
- Off-chain storage for user mapping and metadata only
- Implement proper encryption for any sensitive off-chain data

### Verification Process
- Check OTP validity against stored hash
- Verify expiration time before acceptance
- Reject already-used OTP entries
- Mark entries as used upon successful verification
- Implement attempt counting and locking

### Rate Limiting
- Maximum 3 OTP requests per 5 minutes per user
- Maximum 20 OTP requests per hour per IP address
- Exponential backoff for repeated violations
- Proper error responses for rate-limited requests

## Testing Requirements

### Unit Tests
- 100% branch coverage for smart contract functions
- Test all success and failure paths
- Verify cryptographic implementations match expectations
- Test edge cases (boundary times, invalid inputs)

### Integration Tests
- End-to-end flow from OTP generation to verification
- Rate limiting enforcement
- Attempt counting and locking mechanisms
- Emergency pause and resume functionality

### Security Tests
- Attempt to bypass verification mechanisms
- Test replay attack prevention
- Validate rate limiting effectiveness
- Verify proper access controls

## Deployment Guidelines

### Smart Contract Deployment
- Deploy using a secure deployment script
- Verify contract on block explorer
- Set initial issuer to secure relayer address
- Test all functions before production use

### Backend Deployment
- Use secure environment variables for secrets
- Implement proper monitoring and alerting
- Configure rate limiting at infrastructure level
- Ensure secure communication between components

### Operational Procedures
- Rotate issuer keys regularly
- Monitor for unusual activity patterns
- Maintain audit logs for compliance
- Implement incident response procedures

## Code Quality Standards

### Smart Contracts
- Follow Solidity style guide
- Use descriptive variable and function names
- Add comprehensive NatSpec documentation
- Minimize gas usage where possible
- Avoid complex logic in contract functions

### Backend Services
- Follow Rust best practices
- Use async/await appropriately
- Handle errors gracefully
- Implement proper logging
- Write modular, testable code

### Documentation
- Document all public interfaces
- Provide examples for common operations
- Keep documentation synchronized with code
- Explain security considerations for each component