# Blockchain OTP System - Project Summary

This document provides a comprehensive overview of the blockchain-based OTP system project structure and components.

## Project Overview

The Blockchain OTP System is a hybrid solution that combines:
- **Rust backend** for high-performance OTP generation and management
- **Yew WebAssembly frontend** for a responsive user interface
- **Solidity smart contracts** for secure on-chain OTP verification
- **Axum web framework** for REST API endpoints

## Directory Structure

```
blockchain-otp/
├── Cargo.toml              # Workspace configuration
├── src/main.rs             # Main backend service
├── crates/
│   ├── otp-core/           # Core OTP logic (generation, hashing)
│   └── otp-contract/       # Blockchain contract interactions
├── frontend/               # Yew WebAssembly frontend
│   ├── src/lib.rs          # Frontend logic
│   ├── index.html          # HTML entry point
│   ├── styles.css          # Styling
│   └── Trunk.toml          # Frontend build configuration
├── contracts/              # Solidity smart contracts
│   └── OtpVerifier.sol     # Main OTP verification contract
├── scripts/                # Deployment scripts
│   └── deploy.js           # Hardhat deployment script
├── build.sh                # Unix build script
├── build.bat               # Windows build script
├── hardhat.config.js       # Hardhat configuration
├── package.json            # Node.js dependencies (for contracts)
├── .gitignore              # Git ignore rules
└── README.md               # Project documentation
```

## Component Details

### 1. Rust Backend (src/ and crates/)

**Main Service ([src/main.rs](file:///c:/Users/USER/Documents/blockchainotp/src/main.rs))**
- Axum-based REST API with endpoints for OTP request and verification
- In-memory storage for OTP requests (replaceable with Redis/DB)
- Integration with core OTP logic

**Core Logic ([crates/otp-core/](file:///c:/Users/USER/Documents/blockchainotp/crates/otp-core))**
- Cryptographically secure OTP generation (6-digit codes)
- Keccak256 hashing for OTP values
- Utility functions for request ID generation and timestamp management
- Comprehensive unit tests

**Contract Interaction ([crates/otp-contract/](file:///c:/Users/USER/Documents/blockchainotp/crates/otp-contract))**
- Rust bindings for the OtpVerifier smart contract
- Functions for setting and verifying OTPs on-chain
- Pause and issuer management functionality

### 2. Yew Frontend ([frontend/](file:///c:/Users/USER/Documents/blockchainotp/frontend))

**Main Component ([frontend/src/lib.rs](file:///c:/Users/USER/Documents/blockchainotp/frontend/src/lib.rs))**
- User interface for requesting and verifying OTPs
- WebAssembly-based for high performance
- Integration with backend REST API

**Styling ([frontend/styles.css](file:///c:/Users/USER/Documents/blockchainotp/frontend/styles.css))**
- Responsive design for various screen sizes
- Clean, user-friendly interface

### 3. Smart Contracts ([contracts/](file:///c:/Users/USER/Documents/blockchainotp/contracts))

**OtpVerifier.sol**
- On-chain storage of OTP hashes and metadata
- Verification logic with security checks
- Emergency pause functionality
- Issuer rotation for key management

### 4. Deployment ([scripts/](file:///c:/Users/USER/Documents/blockchainotp/scripts))

**deploy.js**
- Hardhat deployment script for the OtpVerifier contract
- Configurable for different networks

## Build and Deployment

### Prerequisites
- Rust and Cargo
- Trunk (for frontend)
- Node.js and npm (for smart contracts)

### Building
1. **Backend**: `cargo build`
2. **Frontend**: `cd frontend && trunk build`
3. **Contracts**: `npx hardhat compile`

### Running
1. **Backend**: `cargo run`
2. **Frontend**: `cd frontend && trunk serve`
3. **Local Blockchain**: `npx hardhat node`
4. **Deploy Contracts**: `npx hardhat run scripts/deploy.js --network localhost`

## API Endpoints

- `GET /health` - Health check
- `POST /otp/request` - Request new OTP
- `POST /otp/verify` - Verify OTP

## Security Features

- Cryptographically secure random number generation
- Keccak256 hashing for OTP values
- 60-second expiration for OTPs
- One-time use enforcement
- Rate limiting (to be implemented with Redis)
- Emergency pause mechanism
- Issuer-based access control

## Future Enhancements

- Redis integration for distributed storage and rate limiting
- Email/SMS delivery mechanisms for OTP distribution
- Wallet signature verification for user registration
- Multi-network blockchain support
- Advanced monitoring and alerting
- Comprehensive integration testing