//! OTP Request Form component

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::models::OtpRequestPayload;
use crate::services::OtpService;
use crate::utils::{log, validate_user_id};

/// Properties for the OTP Request Form component
#[derive(Properties, PartialEq)]
pub struct OtpRequestFormProps {
    pub on_otp_requested: Callback<(String, u64)>, // Callback with (request_id, expires_at)
}

/// State for the OTP Request Form
#[derive(Clone)]
struct OtpRequestFormState {
    user_id: String,
    loading: bool,
    error: Option<String>,
    success: Option<String>,
}

/// Messages for the OTP Request Form component
pub enum OtpRequestFormMsg {
    UpdateUserId(String),
    RequestOtp,
    OtpRequestSuccess(String, u64),
    OtpRequestError(String),
}

/// OTP Request Form component
#[function_component(OtpRequestForm)]
pub fn otp_request_form(props: &OtpRequestFormProps) -> Html {
    let state = use_state(|| OtpRequestFormState {
        user_id: "user-123".to_string(),
        loading: false,
        error: None,
        success: None,
    });
    
    let on_user_id_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            state.set(OtpRequestFormState {
                user_id: value,
                loading: state.loading,
                error: state.error.clone(),
                success: state.success.clone(),
            });
        })
    };
    
    let on_request_otp = {
        let state = state.clone();
        let on_otp_requested = props.on_otp_requested.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let on_otp_requested = on_otp_requested.clone();
            
            // Clear previous messages
            state.set(OtpRequestFormState {
                user_id: state.user_id.clone(),
                loading: true,
                error: None,
                success: None,
            });
            
            // Validate user ID
            if !validate_user_id(&state.user_id) {
                state.set(OtpRequestFormState {
                    user_id: state.user_id.clone(),
                    loading: false,
                    error: Some("Please enter a valid user ID".to_string()),
                    success: None,
                });
                return;
            }
            
            // Make API request
            let payload = OtpRequestPayload {
                user_id: state.user_id.clone(),
            };
            
            spawn_local(async move {
                match OtpService::request_otp(payload).await {
                    Ok(response) => {
                        on_otp_requested.emit((response.request_id, response.expires_at));
                        state.set(OtpRequestFormState {
                            user_id: state.user_id.clone(),
                            loading: false,
                            error: None,
                            success: Some("OTP generated successfully! Check the backend terminal for your code.".to_string()),
                        });
                    }
                    Err(e) => {
                        log(&format!("OTP request error: {}", e));
                        state.set(OtpRequestFormState {
                            user_id: state.user_id.clone(),
                            loading: false,
                            error: Some(e),
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
        <div class="otp-request-form">
            <h2>{"Request OTP"}</h2>
            <div class="form-group">
                <label for="user-id">{"User ID:"}</label>
                <input 
                    type="text" 
                    id="user-id" 
                    value={state.user_id.clone()} 
                    oninput={on_user_id_change}
                    disabled={state.loading}
                />
            </div>
            
            { error_message }
            { success_message }
            
            <button onclick={on_request_otp} disabled={state.loading}>
                { if state.loading { "Requesting..." } else { "Request OTP" } }
            </button>
        </div>
    }
}