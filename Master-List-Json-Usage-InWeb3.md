# Master List: JSON Usage in Web3 Blockchain OTP System

This document provides a comprehensive overview of how JSON is used throughout the Blockchain OTP system, covering all components from frontend to smart contracts.

## 1. API Communication: JSON for API Requests and Responses

### Backend API Endpoints
The Rust-based backend uses JSON for all API communication between the frontend, backend, and indirectly with smart contracts:

**OTP Request Endpoint (`POST /otp/request`):**
- **Request Payload:**
  ```json
  {
    "user_id": "string"
  }
  ```
- **Response:**
  ```json
  {
    "request_id": "string",
    "expires_at": "unix_timestamp"
  }
  ```

**OTP Verification Endpoint (`POST /otp/verify`):**
- **Request Payload:**
  ```json
  {
    "request_id": "string",
    "otp": "string"
  }
  ```
- **Response:**
  ```json
  {
    "verified": "boolean"
  }
  ```

### Implementation Details
1. **Frontend Services**: The Yew frontend uses `gloo-net` for HTTP requests, serializing request payloads to JSON and deserializing responses.

2. **Backend Processing**: The Axum backend uses Serde for JSON serialization/deserialization.

3. **Data Models**: Strongly-typed structures ensure proper JSON format.

## 2. Smart Contract Data: JSON-RPC Calls and Blockchain Interactions

### Ethereum Smart Contract Integration
The system uses JSON extensively for blockchain interactions:

1. **JSON-RPC Communication**: The backend communicates with Ethereum nodes using JSON-RPC for method calls.
2. **Contract ABI**: The smart contract ABI is defined as a JSON string in the Rust code.
3. **Contract Interaction**: The `ethers-rs` library handles JSON serialization/deserialization for contract calls.

### Solana Integration
For Solana, JSON is used for:
1. **RPC Endpoint Configuration**: Defined in TOML but converted to JSON for RPC calls
2. **Account Data**: Public keys and transaction data are handled as JSON-compatible structures
3. **API Responses**: Wallet connection responses use JSON format.

## 3. Configuration Files: JSON in Blockchain Tooling

### Node.js Package Management
The project uses `package.json` for Node.js dependencies:
```json
{
  "name": "blockchain-otp-contracts",
  "scripts": {
    "test": "hardhat test",
    "deploy": "hardhat run deploy_ethereum.js"
  },
  "dependencies": {
    "@openzeppelin/contracts": "^5.4.0",
    "ethers": "^5.0.0"
  }
}
```

### Hardhat Configuration
Hardhat uses a JavaScript configuration file that works with JSON-like structures.

### Foundry Configuration
Foundry uses TOML configuration files, but they're conceptually similar to JSON.

### Deployment Configuration
Network endpoints and deployed contract addresses are stored in TOML format.

### Frontend Configuration
The frontend uses `Trunk.toml` for build configuration.

## 4. Web3 Integration: Wallet Integrations Using JSON

### MetaMask Integration
MetaMask interactions use JSON extensively:

1. **Connection Responses**: JavaScript functions return JSON-like objects.
2. **Event Data**: Account and chain change events use JSON.
3. **Rust/WASM Bridge**: The Yew frontend converts JavaScript objects to Rust using Serde.

### Phantom Wallet Integration
Similar patterns exist for Solana's Phantom wallet:

1. **Connection Responses**
2. **Signed Message Responses**

### Wallet Connector Component
The Rust/WASM frontend component handles JSON data from both wallets.