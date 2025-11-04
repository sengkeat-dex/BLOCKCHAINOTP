// MetaMask integration JavaScript bridge
// This file will be included in the HTML to provide MetaMask functionality

// Function to connect to MetaMask
async function connectToMetaMask() {
    // Check if MetaMask is installed
    if (typeof window.ethereum === 'undefined') {
        throw new Error('MetaMask is not installed. Please install MetaMask to continue.');
    }

    try {
        // Request account access
        const accounts = await window.ethereum.request({ 
            method: 'eth_requestAccounts' 
        });
        
        // Get the first account
        const account = accounts[0];
        
        // Get the current network
        const network = await window.ethereum.request({ 
            method: 'net_version' 
        });
        
        // Return account and network information
        return {
            account: account,
            network: network
        };
    } catch (error) {
        // User rejected the request or other error occurred
        throw new Error('Failed to connect to MetaMask: ' + error.message);
    }
}

// Function to check if MetaMask is connected
async function isMetaMaskConnected() {
    if (typeof window.ethereum === 'undefined') {
        return false;
    }
    
    try {
        const accounts = await window.ethereum.request({ 
            method: 'eth_accounts' 
        });
        
        return accounts.length > 0;
    } catch (error) {
        return false;
    }
}

// Function to get current account
async function getCurrentAccount() {
    if (typeof window.ethereum === 'undefined') {
        throw new Error('MetaMask is not installed');
    }
    
    try {
        const accounts = await window.ethereum.request({ 
            method: 'eth_accounts' 
        });
        
        return accounts.length > 0 ? accounts[0] : null;
    } catch (error) {
        throw new Error('Failed to get account: ' + error.message);
    }
}

// Function to sign a message with MetaMask
async function signMessage(message, account) {
    if (typeof window.ethereum === 'undefined') {
        throw new Error('MetaMask is not installed');
    }
    
    try {
        const signature = await window.ethereum.request({
            method: 'personal_sign',
            params: [message, account]
        });
        
        return signature;
    } catch (error) {
        throw new Error('Failed to sign message: ' + error.message);
    }
}

// Expose functions to global scope for WebAssembly to access
window.connectToMetaMask = connectToMetaMask;
window.isMetaMaskConnected = isMetaMaskConnected;
window.getCurrentAccount = getCurrentAccount;
window.signMessage = signMessage;

// Listen for account changes
if (typeof window.ethereum !== 'undefined') {
    window.ethereum.on('accountsChanged', function (accounts) {
        // Dispatch a custom event that the Rust/WASM code can listen for
        window.dispatchEvent(new CustomEvent('metamaskAccountsChanged', {
            detail: { accounts: accounts }
        }));
    });

    // Listen for chain changes
    window.ethereum.on('chainChanged', function (chainId) {
        // Dispatch a custom event that the Rust/WASM code can listen for
        window.dispatchEvent(new CustomEvent('metamaskChainChanged', {
            detail: { chainId: chainId }
        }));
    });
}

console.log('MetaMask integration loaded');