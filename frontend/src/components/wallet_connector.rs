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
            
            // Call the JavaScript function to connect to MetaMask
            let promise = connect_to_metamask();
            
            wasm_bindgen_futures::spawn_local(async move {
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(js_value) => {
                        // Convert the JavaScript object to a Rust string
                        let account_info = js_value.into_serde::<serde_json::Value>()
                            .unwrap_or_else(|_| serde_json::json!({"account": "0x0000000000000000000000000000000000000000", "network": "unknown"}));
                        
                        let account = account_info["account"].as_str().unwrap_or("0x0000000000000000000000000000000000000000").to_string();
                        let network = account_info["network"].as_str().unwrap_or("ethereum").to_string();
                        
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
            
            // In a real implementation, this would connect to Solana wallet
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate async operation
                gloo_timers::future::TimeoutFuture::new(1000).await;
                
                // Simulate successful connection with a fake wallet address
                let wallet_address = "742d35Cc6634C0532925a3b8D4C0532925a3b8D4".to_string();
                on_wallet_connected.emit((wallet_address.clone(), "solana".to_string()));
                
                state.set(WalletConnectorState {
                    connecting: false,
                    error: None,
                    success: Some(format!("Connected to Solana wallet: {}...", &wallet_address[..10])),
                });
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
                        "Connect to Solana" 
                    } }
                </button>
            </div>
            
            { error_message }
            { success_message }
        </div>
    }
}