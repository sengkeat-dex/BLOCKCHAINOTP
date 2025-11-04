# MetaMask Integration Guide

This guide explains how to test and use the MetaMask integration in your Blockchain OTP System.

## Prerequisites

1. MetaMask browser extension installed
2. A funded Ethereum wallet (testnet or mainnet)
3. The frontend application running

## Testing MetaMask Integration

### 1. Open the Test Page

First, open the test page to verify MetaMask is working:

1. Navigate to `http://localhost:8080/test_metamask.html`
2. Click "Check Connection" to see if MetaMask is detected
3. Click "Connect to MetaMask" to test the connection flow

### 2. Using MetaMask in the Main Application

1. Navigate to `http://localhost:8080`
2. Click the "Connect to MetaMask" button in the wallet connector section
3. MetaMask should prompt you to connect
4. Approve the connection in MetaMask
5. You should see a success message with your wallet address

## Troubleshooting

### Common Issues

1. **MetaMask Not Detected**
   - Ensure MetaMask extension is installed and enabled
   - Check that you're using a supported browser (Chrome, Firefox, Brave)
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

The integration works through the following components:

1. **metamask.js**: JavaScript bridge that communicates with MetaMask
2. **Wallet Connector Component**: Rust/Yew component that calls the JavaScript functions
3. **Event Listeners**: Listen for account and network changes

### JavaScript Bridge Functions

- `connectToMetaMask()`: Requests account access from MetaMask
- `isMetaMaskConnected()`: Checks if MetaMask is already connected
- `getCurrentAccount()`: Gets the current connected account
- `signMessage()`: Signs a message with the connected account

### Rust/WASM Integration

The Rust code uses `wasm-bindgen` to call the JavaScript functions:

```rust
#[wasm_bindgen(js_name = connectToMetaMask)]
fn connect_to_metamask() -> js_sys::Promise;
```

## Security Considerations

1. **Never store private keys** in the frontend code
2. **Always verify signatures** server-side for critical operations
3. **Use secure messaging** for communication between frontend and backend
4. **Implement proper error handling** to prevent information leakage

## Extending the Integration

You can extend the MetaMask integration by:

1. Adding support for specific Ethereum networks
2. Implementing transaction signing capabilities
3. Adding support for hardware wallets
4. Integrating with other Web3 providers

## Example Usage

Here's an example of how to use the MetaMask integration in your components:

```rust
// In your component
let on_connect_metamask = {
    Callback::from(move |_| {
        let promise = connect_to_metamask();
        wasm_bindgen_futures::spawn_local(async move {
            match wasm_bindgen_futures::JsFuture::from(promise).await {
                Ok(js_value) => {
                    // Handle successful connection
                }
                Err(e) => {
                    // Handle error
                }
            }
        });
    })
};
```

## Event Handling

The integration also listens for MetaMask events:

1. **Account Changes**: Detects when the user switches accounts
2. **Network Changes**: Detects when the user switches networks

These events are dispatched as custom browser events that can be listened to in your Rust code.