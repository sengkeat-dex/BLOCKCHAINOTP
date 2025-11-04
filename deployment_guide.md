# Blockchain OTP System Deployment Guide

This guide explains how to deploy the OtpVerifier smart contract to both Ethereum and Solana networks.

## Prerequisites

### Ethereum Deployment
1. Node.js and npm installed
2. Hardhat installed globally: `npm install -g hardhat`
3. Ethereum wallet with testnet or mainnet funds
4. Infura or Alchemy API key for Ethereum network access

### Solana Deployment
1. Solana CLI tools installed
2. Solana wallet with testnet or mainnet funds
3. Rust and Cargo installed
4. Solana Program Library (SPL) tools

## Ethereum Deployment

### 1. Install Dependencies
```bash
npm install
```

### 2. Configure Network
Edit `hardhat.config.js` to add your network configuration:

```javascript
require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-ethers");

module.exports = {
  solidity: "0.8.20",
  networks: {
    hardhat: {
      chainId: 1337
    },
    // Add your network configuration here
    goerli: {
      url: "https://goerli.infura.io/v3/YOUR_INFURA_KEY",
      accounts: ["YOUR_PRIVATE_KEY"]
    },
    mainnet: {
      url: "https://mainnet.infura.io/v3/YOUR_INFURA_KEY",
      accounts: ["YOUR_PRIVATE_KEY"]
    }
  }
};
```

### 3. Compile Contracts
```bash
npx hardhat compile
```

### 4. Deploy to Network
```bash
# Deploy to Goerli testnet
npx hardhat run deploy_ethereum.js --network goerli

# Deploy to mainnet
npx hardhat run deploy_ethereum.js --network mainnet
```

### 5. Verify Deployment
After deployment, check the generated `ethereum_deployment.json` file for:
- Contract address
- Issuer address
- Deployment timestamp

## Solana Deployment

### 1. Install Solana CLI
Follow the official Solana installation guide: https://docs.solana.com/cli/install

### 2. Create and Fund Wallet
```bash
# Generate a new keypair
solana-keygen new

# Get your public key
solana-keygen pubkey

# Airdrop SOL on devnet (for testing)
solana airdrop 1
```

### 3. Compile the Program
```bash
# If you have a Rust-based Solana program
cargo build-bpf
```

### 4. Deploy the Program
```bash
# Deploy to devnet
solana program deploy target/deploy/your_program.so --keypair ~/.config/solana/id.json --url devnet

# Deploy to mainnet
solana program deploy target/deploy/your_program.so --keypair ~/.config/solana/id.json --url mainnet-beta
```

### 5. Verify Deployment
After deployment, check the generated `solana_deployment.json` file for:
- Program ID
- Deployer address
- Deployment timestamp

## Configuration Files

### Hardhat Configuration
Create or update `hardhat.config.js`:

```javascript
require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-ethers");

module.exports = {
  solidity: "0.8.20",
  networks: {
    hardhat: {
      chainId: 1337
    },
    goerli: {
      url: "https://goerli.infura.io/v3/YOUR_INFURA_KEY",
      accounts: ["YOUR_PRIVATE_KEY"]
    }
  }
};
```

### Solana Configuration
Set your Solana configuration:

```bash
# Set cluster
solana config set --url devnet

# View configuration
solana config get
```

## Testing Deployments

### Ethereum
```bash
# Run tests
npx hardhat test

# Interact with deployed contract
npx hardhat console --network goerli
```

### Solana
```bash
# Test locally
solana program test

# Interact with deployed program
solana program show <PROGRAM_ID>
```

## Integration with Frontend

After deployment, update your frontend configuration to use the deployed contract addresses:

1. Update the contract address in your frontend code
2. Ensure the ABI matches the deployed contract
3. Test the integration with the deployed contracts

## Troubleshooting

### Common Ethereum Issues
1. **Insufficient funds**: Ensure your wallet has enough ETH for gas fees
2. **Network errors**: Check your RPC endpoint and API key
3. **Compilation errors**: Ensure Solidity version matches requirements

### Common Solana Issues
1. **Insufficient SOL**: Ensure your wallet has enough SOL for deployment
2. **Program errors**: Check program compilation and dependencies
3. **Network connectivity**: Verify Solana CLI configuration

## Security Considerations

1. **Private Keys**: Never commit private keys to version control
2. **Environment Variables**: Use .env files for sensitive information
3. **Contract Ownership**: Consider using multi-signature wallets for contract ownership
4. **Upgradeability**: Implement upgrade patterns if needed
5. **Audits**: Consider having your contracts audited before mainnet deployment

## Next Steps

1. Test the deployed contracts thoroughly
2. Integrate with your frontend application
3. Monitor contract activity
4. Set up alerts for important events
5. Plan for contract upgrades if needed

This deployment guide provides a foundation for deploying your blockchain OTP system to both Ethereum and Solana networks. Remember to test thoroughly on testnets before deploying to mainnet.