// Solana Phantom wallet integration JavaScript bridge
// This file will be included in the HTML to provide Phantom wallet functionality

// Function to connect to Phantom wallet
async function connectToPhantom() {
    // Check if Phantom is installed
    if (typeof window.solana === 'undefined' || !window.solana.isPhantom) {
        throw new Error('Phantom wallet is not installed. Please install Phantom to continue.');
    }

    try {
        // Request account access
        const response = await window.solana.connect();
        
        // Get the public key
        const publicKey = response.publicKey.toString();
        
        // Return account information
        return {
            account: publicKey,
            network: 'solana'
        };
    } catch (error) {
        // User rejected the request or other error occurred
        throw new Error('Failed to connect to Phantom: ' + error.message);
    }
}

// Function to check if Phantom is connected
async function isPhantomConnected() {
    if (typeof window.solana === 'undefined' || !window.solana.isPhantom) {
        return false;
    }
    
    try {
        return window.solana.isConnected;
    } catch (error) {
        return false;
    }
}

// Function to get current account
async function getPhantomAccount() {
    if (typeof window.solana === 'undefined' || !window.solana.isPhantom) {
        throw new Error('Phantom wallet is not installed');
    }
    
    try {
        if (window.solana.isConnected) {
            return window.solana.publicKey.toString();
        }
        return null;
    } catch (error) {
        throw new Error('Failed to get account: ' + error.message);
    }
}

// Function to sign a message with Phantom
async function signPhantomMessage(message) {
    if (typeof window.solana === 'undefined' || !window.solana.isPhantom) {
        throw new Error('Phantom wallet is not installed');
    }
    
    if (!window.solana.isConnected) {
        throw new Error('Phantom wallet is not connected');
    }
    
    try {
        const encodedMessage = new TextEncoder().encode(message);
        const signedMessage = await window.solana.signMessage(encodedMessage, 'utf8');
        
        return {
            signature: Array.from(signedMessage.signature),
            publicKey: signedMessage.publicKey.toString()
        };
    } catch (error) {
        throw new Error('Failed to sign message: ' + error.message);
    }
}

// Expose functions to global scope for WebAssembly to access
window.connectToPhantom = connectToPhantom;
window.isPhantomConnected = isPhantomConnected;
window.getPhantomAccount = getPhantomAccount;
window.signPhantomMessage = signPhantomMessage;

// Also expose as global functions for debugging
window.connectToPhantomDebug = async function() {
    try {
        console.log("Attempting to connect to Phantom...");
        const result = await connectToPhantom();
        console.log("Phantom connection result:", result);
        return result;
    } catch (error) {
        console.error("Phantom connection error:", error);
        throw error;
    }
};

// Listen for account changes
if (typeof window.solana !== 'undefined' && window.solana.isPhantom) {
    window.solana.on('accountChanged', function (publicKey) {
        // Dispatch a custom event that the Rust/WASM code can listen for
        window.dispatchEvent(new CustomEvent('phantomAccountsChanged', {
            detail: { publicKey: publicKey ? publicKey.toString() : null }
        }));
    });
}

console.log('Phantom wallet integration loaded');