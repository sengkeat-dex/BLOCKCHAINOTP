# Blockchain OTP System

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Solidity](https://img.shields.io/badge/Solidity-0.8.20-blue.svg)](https://soliditylang.org/)

A blockchain-based one-time password (OTP) system that provides enhanced security through smart contract verification on both Ethereum and Solana networks.

## 🌟 Features

- **Time-based One-Time Passwords (TOTP)**: Secure 6-digit codes with 60-second expiration
- **Multi-chain Support**: Deploy and verify OTPs on both Ethereum and Solana blockchains
- **Enhanced Security**: On-chain verification with rate limiting and attempt tracking
- **Web3 Wallet Integration**: Connect with MetaMask (Ethereum) and Phantom (Solana) wallets
- **Rust Backend**: High-performance backend service built with Axum web framework
- **WebAssembly Frontend**: Responsive UI built with Yew framework
- **MCP Server**: Model Context Protocol server for AI assistant integration (port 3002)

## 🏗️ Architecture

The system implements a hybrid approach combining off-chain OTP generation with on-chain verification:

```
[Client] ↔ [Backend API] ↔ [Smart Contracts]
              ↓
         [WebAssembly UI]
```

### Components

1. **Smart Contracts**:
   - Stores OTP hashes with expiration timestamps
   - Enforces one-time use and rate limiting (max 3 attempts)
   - Supports emergency pause functionality
   - Implements issuer-based access control

2. **Rust Backend** (`src/` and `crates/`):
   - OTP generation using cryptographically secure methods
   - REST API endpoints for requesting and verifying OTPs
   - Integration with both Ethereum and Solana networks
   - Redis-based rate limiting (configurable)

3. **Yew Frontend** (`frontend/`):
   - WebAssembly-based user interface
   - Wallet connection for both Ethereum and Solana
   - OTP request and verification forms
   - Real-time status updates

4. **MCP Server**:
   - Provides context to AI assistants about the system
   - Running on port 3002
   - Compatible with Claude Desktop and other MCP-compatible assistants

## 🚀 Getting Started

### Prerequisites

- Rust and Cargo (1.70 or later)
- Node.js and npm (for smart contract development)
- Trunk (for frontend development)
- WebAssembly target for Rust: `rustup target add wasm32-unknown-unknown`
- Docker (for security testing tools like Echidna)

### Installation

1. **Clone the repository**:
   ```bash
   git clone git@github.com:sengkeat-dex/BLOCKCHAINOTP.git
   cd BLOCKCHAINOTP
   ```

2. **Install dependencies**:
   ```bash
   # Install Rust dependencies
   cargo build
   
   # Install Node.js dependencies for smart contracts
   npm install
   ```

3. **Set up environment variables**:
   ```bash
   cp .env.example .env
   # Edit .env with your Infura project ID and private key
   ```

### Running the Application

1. **Start the backend service**:
   ```bash
   cargo run
   ```

2. **Start the frontend**:
   ```bash
   cd frontend
   trunk serve
   ```

3. **Deploy smart contracts** (optional):
   ```bash
   # For Ethereum (Hardhat local network)
   npx hardhat node
   npx hardhat run deploy_ethereum.js --network localhost
   
   # For Solana (requires Solana CLI)
   solana config set --url devnet
   # Then run the Solana deployment script
   ```

## 📡 API Endpoints

- `GET /health` - Health check endpoint
- `POST /otp/request` - Request a new OTP
- `POST /otp/verify` - Verify an OTP

## 🔐 Security Features

- **Cryptographically Secure**: Uses Keccak256 hashing for OTP values
- **One-Time Use**: Each OTP can only be verified once
- **Rate Limiting**: Maximum 3 attempts per OTP request
- **Time-Bounded**: OTPs expire 60 seconds after generation
- **Emergency Pause**: Contract can be paused in case of emergency
- **Issuer Control**: Only authorized issuers can set OTPs
- **Zero Plaintext Storage**: OTPs are never stored in plaintext

## 🧪 Testing

The project includes comprehensive tests for all components:

- **Unit Tests**: Core OTP logic and contract interactions
- **Integration Tests**: End-to-end OTP flow testing
- **Smart Contract Tests**: Solidity contract verification
- **Security Testing**: Integration with tools like Echidna and Mythril

Run tests with:
```bash
# Rust tests
cargo test

# Smart contract tests
npx hardhat test
```

## 🌍 Multi-Network Support

### Ethereum
- Deploy using Hardhat to any Ethereum network (mainnet, testnets)
- Supports Infura for easy network access
- Verified on Etherscan for transparency

### Solana
- Deploy to Solana devnet, testnet, or mainnet
- Uses Solana Web3.js for interactions
- Compatible with Phantom wallet

## 🤖 AI Assistant Integration

The project includes an MCP (Model Context Protocol) server that allows AI assistants like Claude Desktop to understand and interact with the system:

- Running on port 3002
- Provides system architecture information
- Offers tools for querying system status
- See [MCP_SERVER.md](MCP_SERVER.md) for detailed documentation

## 📚 Documentation

- [Project Summary](PROJECT_SUMMARY.md) - Detailed overview of components
- [Deployment Guide](DEPLOYMENT-GUIDE.md) - Step-by-step deployment instructions
- [Security Enhancements](SECURITY_ENHANCEMENTS.md) - Security features and considerations
- [Testing Guide](TESTING_GUIDE.md) - Comprehensive testing documentation
- [Algorithm Details](algorithm-otp.md) - Technical details of OTP generation
- [MCP Server Documentation](MCP_SERVER.md) - AI assistant integration details

## 🛠️ Development

### Project Structure

```
blockchain-otp/
├── Cargo.toml              # Workspace configuration
├── src/
│   ├── main.rs             # Main backend service
│   └── lib.rs              # Shared backend logic
├── crates/
│   ├── otp-core/           # Core OTP logic (generation, hashing)
│   └── otp-contract/       # Blockchain contract interactions
├── frontend/               # Yew WebAssembly frontend
│   ├── src/lib.rs          # Frontend logic
│   ├── index.html          # HTML entry point
│   └── styles.css          # Styling
├── contracts/              # Solidity smart contracts
│   └── OtpVerifier.sol     # Main OTP verification contract
├── scripts/                # Deployment scripts
│   ├── deploy_ethereum.js  # Ethereum deployment
│   └── deploy_solana.js    # Solana deployment
└── tests/                  # Integration tests
```

### Building

1. **Backend**: `cargo build`
2. **Frontend**: `cd frontend && trunk build`
3. **Contracts**: `npx hardhat compile`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

sengkeat-dex - [@sengkeat-dex](https://github.com/sengkeat-dex) - sengkeat-dex@outlook.com

Project Link: [https://github.com/sengkeat-dex/BLOCKCHAINOTP](https://github.com/sengkeat-dex/BLOCKCHAINOTP)