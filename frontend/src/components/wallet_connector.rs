//! Wallet Connector component for blockchain integration

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use gloo_utils::format::JsValueSerdeExt; // Fixed import for JsValueSerdeExt

/// Properties for the Wallet Connector component
#[derive(Properties, PartialEq)]
pub struct WalletConnectorProps {
    pub on_wallet_connected: Callback<(String, String)>, // Callback with (wallet_address, network)
}

/// State for the Wallet Connector
#[derive(Clone)]
struct WalletConnectorState {
    connecting: bool,
    error: Option<String>,
    success: Option<String>,
}

/// Messages for the Wallet Connector component
pub enum WalletConnectorMsg {
    ConnectMetaMask,
    ConnectSolana,
    WalletConnected(String, String), // (address, network)
    WalletError(String),
}

#[wasm_bindgen]
extern "C" {
    // JavaScript function to connect to MetaMask
    #[wasm_bindgen(js_name = connectToMetaMask)]
    fn connect_to_metamask() -> js_sys::Promise;
    
    // JavaScript function to check if MetaMask is installed
    #[wasm_bindgen(js_name = isMetaMaskConnected)]
    fn is_metamask_connected() -> js_sys::Promise;
    
    // JavaScript function to connect to Phantom
    #[wasm_bindgen(js_name = connectToPhantom)]
    fn connect_to_phantom() -> js_sys::Promise;
    
    // JavaScript function to check if Phantom is installed
    #[wasm_bindgen(js_name = isPhantomConnected)]
    fn is_phantom_connected() -> js_sys::Promise;
}

// Function to show an alert using web-sys
fn show_alert(message: &str) {
    web_sys::window()
        .unwrap()
        .alert_with_message(message)
        .unwrap();
}

// Function to check if a JavaScript function exists
fn check_js_function_exists(function_name: &str) -> bool {
    let window = web_sys::window().unwrap();
    match window.get(function_name) {
        Some(val) => !val.is_undefined(),
        None => false,
    }
}

// Function to check if a JavaScript object exists
fn check_js_object_exists(object_name: &str) -> bool {
    let window = web_sys::window().unwrap();
    match window.get(object_name) {
        Some(val) => !val.is_undefined(),
        None => false,
    }
}

/// Wallet Connector component
#[function_component(WalletConnector)]
pub fn wallet_connector(props: &WalletConnectorProps) -> Html {
    let state = use_state(|| WalletConnectorState {
        connecting: false,
        error: None,
        success: None,
    });
    
    let on_connect_metamask = {
        let state = state.clone();
        let on_wallet_connected = props.on_wallet_connected.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let on_wallet_connected = on_wallet_connected.clone();
            
            // Clear previous messages
            state.set(WalletConnectorState {
                connecting: true,
                error: None,
                success: None,
            });
            
            // Check if MetaMask is installed before attempting to connect
            let ethereum_exists = check_js_object_exists("ethereum");
            console::log_1(&format!("Ethereum exists: {}", ethereum_exists).into());
            
            if !ethereum_exists {
                let error_msg = "MetaMask is not installed. Please install MetaMask to continue.".to_string();
                show_alert(&error_msg);
                state.set(WalletConnectorState {
                    connecting: false,
                    error: Some(error_msg),
                    success: None,
                });
                return;
            }
            
            // Check if the connect function exists
            let connect_function_exists = check_js_function_exists("connectToMetaMask");
            console::log_1(&format!("connectToMetaMask function exists: {}", connect_function_exists).into());
            
            // Try to call the debug version first to see if there are any issues
            let debug_function_exists = check_js_function_exists("connectToMetaMaskDebug");
            if debug_function_exists {
                console::log_1(&"Using debug version of connectToMetaMask".into());
            }
            
            // Call the JavaScript function to connect to MetaMask
            console::log_1(&"Calling connect_to_metamask()".into());
            let promise = connect_to_metamask();
            
            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"Inside MetaMask promise handler".into());
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(js_value) => {
                        console::log_1(&"MetaMask connection successful".into());
                        // Convert the JavaScript object to a Rust string
                        let account_info = js_value.into_serde::<serde_json::Value>()
                            .unwrap_or_else(|_| serde_json::json!({"account": "0x0000000000000000000000000000000000000000", "network": "unknown"}));
                        
                        let account = account_info["account"].as_str().unwrap_or("0x0000000000000000000000000000000000000000").to_string();
                        let network = account_info["network"].as_str().unwrap_or("ethereum").to_string();
                        
                        console::log_1(&format!("Account: {}, Network: {}", account, network).into());
                        
                        on_wallet_connected.emit((account.clone(), network));
                        
                        state.set(WalletConnectorState {
                            connecting: false,
                            error: None,
                            success: Some(format!("Connected to MetaMask: {}...", &account[..10])),
                        });
                    }
                    Err(e) => {
                        let error_message = format!("Failed to connect to MetaMask: {:?}", e);
                        console::log_1(&e);
                        console::log_1(&"MetaMask connection failed".into());
                        
                        // Show alert to user
                        show_alert(&format!("Failed to connect to MetaMask. Please make sure MetaMask is installed and unlocked."));
                        
                        state.set(WalletConnectorState {
                            connecting: false,
                            error: Some(error_message),
                            success: None,
                        });
                    }
                }
            });
        })
    };
    
    let on_connect_solana = {
        let state = state.clone();
        let on_wallet_connected = props.on_wallet_connected.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let on_wallet_connected = on_wallet_connected.clone();
            
            // Clear previous messages
            state.set(WalletConnectorState {
                connecting: true,
                error: None,
                success: None,
            });
            
            // Check if Phantom is installed before attempting to connect
            let solana_exists = check_js_object_exists("solana");
            console::log_1(&format!("Solana exists: {}", solana_exists).into());
            
            if !solana_exists {
                let error_msg = "Phantom wallet is not installed. Please install Phantom to continue.".to_string();
                show_alert(&error_msg);
                state.set(WalletConnectorState {
                    connecting: false,
                    error: Some(error_msg),
                    success: None,
                });
                return;
            }
            
            // Check if the connect function exists
            let connect_function_exists = check_js_function_exists("connectToPhantom");
            console::log_1(&format!("connectToPhantom function exists: {}", connect_function_exists).into());
            
            // Try to call the debug version first to see if there are any issues
            let debug_function_exists = check_js_function_exists("connectToPhantomDebug");
            if debug_function_exists {
                console::log_1(&"Using debug version of connectToPhantom".into());
            }
            
            // Call the JavaScript function to connect to Phantom
            console::log_1(&"Calling connect_to_phantom()".into());
            let promise = connect_to_phantom();
            
            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"Inside Phantom promise handler".into());
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(js_value) => {
                        console::log_1(&"Phantom connection successful".into());
                        // Convert the JavaScript object to a Rust string
                        let account_info = js_value.into_serde::<serde_json::Value>()
                            .unwrap_or_else(|_| serde_json::json!({"account": "11111111111111111111111111111111", "network": "solana"}));
                        
                        let account = account_info["account"].as_str().unwrap_or("11111111111111111111111111111111").to_string();
                        let network = account_info["network"].as_str().unwrap_or("solana").to_string();
                        
                        console::log_1(&format!("Account: {}, Network: {}", account, network).into());
                        
                        on_wallet_connected.emit((account.clone(), network));
                        
                        state.set(WalletConnectorState {
                            connecting: false,
                            error: None,
                            success: Some(format!("Connected to Phantom: {}...", &account[..10])),
                        });
                    }
                    Err(e) => {
                        let error_message = format!("Failed to connect to Phantom: {:?}", e);
                        console::log_1(&e);
                        console::log_1(&"Phantom connection failed".into());
                        
                        // Show alert to user
                        show_alert(&format!("Failed to connect to Phantom. Please make sure Phantom is installed and unlocked."));
                        
                        state.set(WalletConnectorState {
                            connecting: false,
                            error: Some(error_message),
                            success: None,
                        });
                    }
                }
            });
        })
    };
    
    let error_message = if let Some(error) = &state.error {
        html! { <div class="error-message">{ error }</div> }
    } else {
        html! {}
    };
    
    let success_message = if let Some(success) = &state.success {
        html! { <div class="success-message">{ success }</div> }
    } else {
        html! {}
    };
    
    html! {
        <div class="wallet-connector">
            <h2>{"Connect Wallet"}</h2>
            <p>{"Connect your wallet for enhanced security and blockchain integration."}</p>
            
            <div class="wallet-buttons">
                <button 
                    onclick={on_connect_metamask} 
                    disabled={state.connecting}
                    class="metamask-button"
                >
                    { if state.connecting && state.success.is_none() && state.error.is_none() { 
                        "Connecting to MetaMask..." 
                    } else { 
                        "Connect to MetaMask" 
                    } }
                </button>
                
                <button 
                    onclick={on_connect_solana} 
                    disabled={state.connecting}
                    class="solana-button"
                >
                    { if state.connecting && state.success.is_none() && state.error.is_none() { 
                        "Connecting to Solana..." 
                    } else { 
                        "Connect to Solana (Phantom)" 
                    } }
                </button>
            </div>
            
            { error_message }
            { success_message }
        </div>
    }
}