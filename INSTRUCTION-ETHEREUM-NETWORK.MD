# INSTRUCTIONS TO RUN THE ETHEREUM NETWORK PROJECT

## Project Overview

This document provides step-by-step instructions for setting up and running the Ethereum network project. The project includes Solidity smart contracts for an OTP (One-Time Password) verification system that can be deployed to Ethereum networks.

## Prerequisites

Before running the project, ensure you have the following tools installed:

1. Node.js (version 16 or higher)
2. npm (Node Package Manager)
3. WSL (Windows Subsystem for Linux) - Ubuntu 20.04 or later
4. Git
5. Foundry toolkit (forge, cast, anvil)

## Installation Steps

### 1. Install Node.js Dependencies

```bash
cd /mnt/c/Users/USER/Documents/blockchainotp
npm install
```

### 2. Install Foundry Toolkit

If not already installed:

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
source ~/.bashrc
foundryup
```

### 3. Set Up Environment Variables

Create a `.env` file in the project root with the following content:

```env
INFURA_PROJECT_ID=your_infura_project_id
PRIVATE_KEY=your_wallet_private_key
```

## Building the Project

### 1. Compile Smart Contracts

```bash
# Using Hardhat
npx hardhat compile

# Or using Foundry
forge build
```

### 2. Run Local Ethereum Network

```bash
# Using Hardhat
npx hardhat node

# Or using Foundry's Anvil
anvil
```

## Deploying Contracts

### 1. Deploy to Local Network

```bash
# Deploy using Hardhat
npx hardhat run deploy_ethereum.js --network localhost

# Or deploy using Foundry
forge script contracts/script/DeployOtpVerifier.s.sol --rpc-url http://localhost:8545 --private-key your_private_key --broadcast
```

### 2. Deploy to Testnet (Sepolia)

```bash
# Deploy using Hardhat
npx hardhat run deploy_ethereum.js --network sepolia

# Or deploy using Foundry
forge script contracts/script/DeployOtpVerifier.s.sol --rpc-url https://sepolia.infura.io/v3/$INFURA_PROJECT_ID --private-key $PRIVATE_KEY --broadcast
```

### 3. Deploy to Mainnet

```bash
# Deploy using Hardhat
npx hardhat run deploy_ethereum.js --network mainnet

# Or deploy using Foundry
forge script contracts/script/DeployOtpVerifier.s.sol --rpc-url https://mainnet.infura.io/v3/$INFURA_PROJECT_ID --private-key $PRIVATE_KEY --broadcast
```

## Testing the Project

### 1. Run Unit Tests with Hardhat

```bash
# Run all tests
npx hardhat test

# Run specific test file
npx hardhat test contracts/test/OtpVerifier.t.sol
```

### 2. Run Tests with Foundry

```bash
# Run all tests
forge test

# Run tests with verbose output
forge test -vvv

# Run specific test function
forge test -m testSetOtp
```

### 3. Security Analysis

```bash
# Run Slither static analysis
~/.local/bin/slither .

# Run Mythril security analysis
/mnt/c/Users/USER/Documents/blockchainotp/mythril_env/bin/myth analyze contracts/OtpVerifier.sol

# Run Echidna fuzz testing
docker run -it --rm -v "$(pwd)":/code trailofbits/echidna:latest echidna /code/contracts/OtpVerifier.sol --contract OtpVerifier
```

## Interacting with Deployed Contracts

### 1. Using Hardhat Console

```bash
# Start Hardhat console
npx hardhat console --network sepolia

# Interact with contract
const OtpVerifier = await ethers.getContractFactory("OtpVerifier");
const otpVerifier = await OtpVerifier.attach("contract_address");
const issuer = await otpVerifier.issuer();
console.log("Issuer:", issuer);
```

### 2. Using Foundry Cast

```bash
# Call a view function
cast call contract_address "issuer()" --rpc-url https://sepolia.infura.io/v3/$INFURA_PROJECT_ID

# Send a transaction
cast send contract_address "setIssuer(address)" 0xNewAddress --private-key $PRIVATE_KEY --rpc-url https://sepolia.infura.io/v3/$INFURA_PROJECT_ID
```

## Development Tools

### 1. Contract Verification

```bash
# Verify contract on Etherscan
npx hardhat verify --network sepolia contract_address "issuer_address" "admin_address"
```

### 2. Gas Usage Analysis

```bash
# Analyze gas usage with Foundry
forge test --gas-report
```

### 3. Contract Size Check

```bash
# Check contract size
forge build --sizes
```

## Monitoring and Observability

### 1. Event Monitoring

```bash
# Listen to contract events
cast logs "OtpSet(bytes32,uint64)" --rpc-url https://sepolia.infura.io/v3/$INFURA_PROJECT_ID
```

### 2. Account Balance Check

```bash
# Check account balance
cast balance your_address --rpc-url https://sepolia.infura.io/v3/$INFURA_PROJECT_ID
```

## Troubleshooting

### Common Issues and Solutions

1. **Compilation Errors**:
   - Ensure Solidity version compatibility in contracts and hardhat.config.js
   - Run `npx hardhat clean` and recompile

2. **Network Connection Issues**:
   - Check your internet connection
   - Verify INFURA_PROJECT_ID in .env file
   - Ensure RPC endpoint is correct

3. **Insufficient Funds**:
   - Check account balance with `cast balance`
   - Request test ETH from faucet for testnets

4. **Gas Price Issues**:
   - Add gas price configuration to hardhat.config.js
   - Use `--gas-price` flag with cast commands

### Useful Commands

```bash
# Check Hardhat version
npx hardhat --version

# List available accounts
npx hardhat accounts

# Check network configuration
npx hardhat networks

# Get contract storage layout
forge inspect OtpVerifier storage
```

## Smart Contract Details

### OtpVerifier Contract Functions

1. `setOtp(bytes32 requestId, bytes32 otpHash, uint64 expiry)` - Sets a new OTP entry
2. `verify(bytes32 requestId, string calldata otp)` - Verifies an OTP
3. `cleanup(bytes32 requestId)` - Removes expired/used OTP entries
4. `setIssuer(address newIssuer)` - Changes the issuer address
5. `setAdmin(address newAdmin)` - Changes the admin address
6. `pause(bool _paused)` - Pauses/unpauses the contract

### Events

1. `OtpSet` - Emitted when a new OTP is set
2. `AttemptRecorded` - Emitted when a verification attempt is made
3. `OtpLocked` - Emitted when maximum attempts are exceeded
4. `OtpVerified` - Emitted when an OTP is successfully verified
5. `OtpCleared` - Emitted when an OTP entry is cleaned up
6. `IssuerChanged` - Emitted when issuer address changes
7. `AdminChanged` - Emitted when admin address changes
8. `Paused` - Emitted when contract is paused/unpaused

### Errors

1. `Unauthorized` - Caller is not authorized
2. `InvalidExpiry` - Expiry time is invalid
3. `EntryExists` - OTP entry already exists
4. `UnknownRequest` - Request ID not found
5. `EntryUsed` - OTP entry already used
6. `EntryExpired` - OTP entry has expired
7. `AttemptsExceeded` - Maximum attempts exceeded
8. `ActiveEntry` - OTP entry is still active
9. `ZeroAddress` - Address is zero
10. `ContractPaused` - Contract is paused
11. `InvalidHash` - Hash is invalid

## Next Steps

1. Explore the Hardhat documentation: https://hardhat.org/
2. Learn Foundry: https://book.getfoundry.sh/
3. Review Solidity documentation: https://docs.soliditylang.org/
4. Join Ethereum developer communities

## Additional Resources

- Ethereum Documentation: https://ethereum.org/developers/
- Solidity Documentation: https://docs.soliditylang.org/
- Hardhat Documentation: https://hardhat.org/docs/
- Foundry Documentation: https://book.getfoundry.sh/
- Etherscan: https://etherscan.io/

---
This document was created to help you set up and run the Ethereum network project. For any issues or questions, refer to the official documentation or community resources.