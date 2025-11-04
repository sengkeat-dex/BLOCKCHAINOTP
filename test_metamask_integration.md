# Testing MetaMask Integration

Since we're currently unable to run the full frontend application due to missing Rust/Cargo/Trunk tools, we can test the MetaMask integration directly using the standalone HTML test file.

## Prerequisites

1. MetaMask browser extension installed and set up
2. A funded Ethereum wallet (testnet or mainnet)
3. A web browser (Chrome, Firefox, Brave, etc.)

## Testing Steps

### 1. Open the Test File

1. Navigate to the project directory:
   ```
   c:\Users\USER\Documents\blockchainotp\frontend\
   ```

2. Open the file `metamask_test.html` directly in your browser:
   - Right-click on the file
   - Select "Open with" and choose your browser
   - Or drag and drop the file into an open browser window

### 2. Test Connection Status

When the page loads, it will automatically check if:
- MetaMask is installed
- You're already connected to MetaMask

You'll see one of the following messages:
- Success message with your account and network information
- Info message indicating MetaMask is installed but not connected
- Error message indicating MetaMask is not installed

### 3. Connect to MetaMask

1. Click the "Connect to MetaMask" button
2. MetaMask should prompt you to connect
3. Select the account you want to connect with
4. Click "Next" and then "Connect" in MetaMask

After successful connection, you should see:
- Your Ethereum account address
- The network you're connected to
- A success message

### 4. Test Account and Network Changes

1. Try switching accounts in MetaMask:
   - Open MetaMask
   - Click on your account name
   - Select a different account
   - The page should automatically update with the new account information

2. Try switching networks in MetaMask:
   - Open MetaMask
   - Click on the network name at the top
   - Select a different network
   - The page should automatically update with the new network information

## Troubleshooting

### Common Issues

1. **MetaMask Not Detected**
   - Ensure MetaMask extension is installed and enabled
   - Check that you're using a supported browser
   - Refresh the page after installing MetaMask

2. **Connection Rejected**
   - Make sure you're approving the connection in MetaMask
   - Check that your wallet is unlocked
   - Try disconnecting and reconnecting

3. **Network Issues**
   - Ensure you're on a supported network (Mainnet, Goerli, Sepolia)
   - Check that your wallet has sufficient funds for transactions

### Debugging Steps

1. Open browser developer tools (F12)
2. Check the Console tab for error messages
3. Check the Network tab for failed requests
4. Verify MetaMask is injecting the ethereum object

## How It Works

The test page works by using the Ethereum JavaScript API that MetaMask injects into the browser:

1. `window.ethereum` - The main Ethereum provider object
2. `eth_requestAccounts` - Method to request account access
3. `eth_accounts` - Method to get currently connected accounts
4. `net_version` - Method to get the current network ID

The page also listens for events:
- `accountsChanged` - Triggered when the user switches accounts
- `chainChanged` - Triggered when the user switches networks

## Next Steps

Once you've verified that MetaMask integration is working with this test file, you can:

1. Install Rust and Trunk to run the full application
2. Implement the actual OTP functionality with blockchain verification
3. Deploy the smart contracts to Ethereum and Solana networks
4. Connect the frontend to the backend services

## Security Considerations

1. This test page only reads account information and does not initiate transactions
2. All interactions are handled by MetaMask, which provides security prompts
3. No private keys or sensitive information are stored in the test page
4. Always verify you're on the correct network before interacting with contracts