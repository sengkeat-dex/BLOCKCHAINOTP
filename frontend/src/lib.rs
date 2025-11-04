//! Main entry point for the Blockchain OTP frontend application

use yew::prelude::*;
use wasm_bindgen::prelude::*;

// Import components
mod components;
use components::header::Header;
use components::otp_request_form::OtpRequestForm;
use components::otp_verify_form::OtpVerifyForm;
use components::status_display::StatusDisplay;
use components::wallet_connector::WalletConnector;

// Import models
mod models;

// Import services
mod services;

// Import utils
mod utils;

/// Main application state
#[derive(Clone)]
struct AppState {
    request_id: String,
    expires_at: Option<u64>,
    message: String,
    verified: bool,
    wallet_connected: bool,
    wallet_address: Option<String>,
    network: Option<String>, // "ethereum" or "solana"
}

/// Messages for the main application
pub enum AppMsg {
    OtpRequested(String, u64),
    VerificationComplete(bool),
    WalletConnected(String, String), // (address, network)
    Reset,
}

/// Main application component
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| AppState {
        request_id: String::new(),
        expires_at: None,
        message: "Welcome to the Blockchain OTP System".to_string(),
        verified: false,
        wallet_connected: false,
        wallet_address: None,
        network: None,
    });
    
    let on_otp_requested = {
        let state = state.clone();
        Callback::from(move |(request_id, expires_at): (String, u64)| {
            state.set(AppState {
                request_id,
                expires_at: Some(expires_at),
                message: "OTP generated successfully! Check the backend terminal for your code.".to_string(),
                verified: false,
                wallet_connected: state.wallet_connected,
                wallet_address: state.wallet_address.clone(),
                network: state.network.clone(),
            });
        })
    };
    
    let on_verification_complete = {
        let state = state.clone();
        Callback::from(move |verified: bool| {
            state.set(AppState {
                request_id: state.request_id.clone(),
                expires_at: state.expires_at,
                message: if verified {
                    format!("OTP verified successfully on {} network!", state.network.as_ref().unwrap_or(&"unknown".to_string()))
                } else {
                    "Invalid OTP or expired. Please try again.".to_string()
                },
                verified,
                wallet_connected: state.wallet_connected,
                wallet_address: state.wallet_address.clone(),
                network: state.network.clone(),
            });
        })
    };
    
    let on_wallet_connected = {
        let state = state.clone();
        Callback::from(move |(wallet_address, network): (String, String)| {
            state.set(AppState {
                request_id: state.request_id.clone(),
                expires_at: state.expires_at,
                message: state.message.clone(),
                verified: state.verified,
                wallet_connected: true,
                wallet_address: Some(wallet_address),
                network: Some(network),
            });
        })
    };
    
    let on_reset = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(AppState {
                request_id: String::new(),
                expires_at: None,
                message: "Welcome to the Blockchain OTP System".to_string(),
                verified: false,
                wallet_connected: state.wallet_connected,
                wallet_address: state.wallet_address.clone(),
                network: state.network.clone(),
            });
        })
    };
    
    let reset_button = if !state.request_id.is_empty() && !state.verified {
        html! {
            <button onclick={on_reset} class="reset-button">
                {"Start Over"}
            </button>
        }
    } else {
        html! {}
    };
    
    let network_info = if let Some(network) = &state.network {
        html! {
            <div class="network-info">
                <p>{ format!("Connected to: {} network", network.to_uppercase()) }</p>
            </div>
        }
    } else {
        html! {}
    };
    
    html! {
        <div class="app-container">
            <Header title="Blockchain OTP System" />
            
            <main class="app-main">
                <WalletConnector on_wallet_connected={on_wallet_connected} />
                
                if state.wallet_connected {
                    { network_info }
                    
                    <OtpRequestForm on_otp_requested={on_otp_requested} />
                    
                    if !state.request_id.is_empty() {
                        <OtpVerifyForm 
                            request_id={state.request_id.clone()} 
                            on_verification_complete={on_verification_complete} 
                        />
                    }
                    
                    <StatusDisplay 
                        request_id={state.request_id.clone()}
                        expires_at={state.expires_at}
                        message={state.message.clone()}
                        verified={state.verified}
                    />
                    
                    { reset_button }
                }
            </main>
        </div>
    }
}

// Use the correct Yew entry point
#[wasm_bindgen(start)]
pub fn run() {
    yew::Renderer::<App>::new().render();
}