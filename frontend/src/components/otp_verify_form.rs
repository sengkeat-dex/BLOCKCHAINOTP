//! OTP Verification Form component

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::models::OtpVerifyPayload;
use crate::services::OtpService;
use crate::utils::{log, validate_otp};

/// Properties for the OTP Verification Form component
#[derive(Properties, PartialEq)]
pub struct OtpVerifyFormProps {
    pub request_id: String,
    pub on_verification_complete: Callback<bool>, // Callback with verification result
}

/// State for the OTP Verification Form
#[derive(Clone)]
struct OtpVerifyFormState {
    otp: String,
    loading: bool,
    error: Option<String>,
}

/// Messages for the OTP Verification Form component
pub enum OtpVerifyFormMsg {
    UpdateOtp(String),
    VerifyOtp,
    VerificationSuccess(bool),
    VerificationError(String),
}

/// OTP Verification Form component
#[function_component(OtpVerifyForm)]
pub fn otp_verify_form(props: &OtpVerifyFormProps) -> Html {
    let state = use_state(|| OtpVerifyFormState {
        otp: String::new(),
        loading: false,
        error: None,
    });

    let on_otp_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            // Only allow numeric input and limit to 6 characters
            if value.len() <= 6 && value.chars().all(|c| c.is_digit(10)) {
                state.set(OtpVerifyFormState {
                    otp: value,
                    loading: state.loading,
                    error: state.error.clone(),
                });
            }
        })
    };

    let on_verify_otp = {
        let state = state.clone();
        let request_id = props.request_id.clone();
        let on_verification_complete = props.on_verification_complete.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let request_id = request_id.clone();
            let on_verification_complete = on_verification_complete.clone();

            // Validate OTP
            if !validate_otp(&state.otp) {
                state.set(OtpVerifyFormState {
                    otp: state.otp.clone(),
                    loading: false,
                    error: Some("Please enter a valid 6-digit OTP".to_string()),
                });
                return;
            }

            // Set loading state
            state.set(OtpVerifyFormState {
                otp: state.otp.clone(),
                loading: true,
                error: None,
            });

            // Make API request
            let payload = OtpVerifyPayload {
                request_id: request_id.clone(),
                otp: state.otp.clone(),
            };

            spawn_local(async move {
                match OtpService::verify_otp(payload).await {
                    Ok(response) => {
                        on_verification_complete.emit(response.verified);
                        state.set(OtpVerifyFormState {
                            otp: if response.verified {
                                state.otp.clone()
                            } else {
                                String::new()
                            },
                            loading: false,
                            error: if response.verified {
                                None
                            } else {
                                Some("Invalid OTP. Please try again.".to_string())
                            },
                        });
                    }
                    Err(e) => {
                        log(&format!("OTP verification error: {}", e));
                        state.set(OtpVerifyFormState {
                            otp: state.otp.clone(),
                            loading: false,
                            error: Some("Failed to verify OTP. Please try again.".to_string()),
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

    html! {
        <div class="otp-verify-form">
            <h2>{"Verify OTP"}</h2>
            <div class="form-group">
                <label for="otp">{"OTP:"}</label>
                <input
                    type="text"
                    id="otp"
                    value={state.otp.clone()}
                    oninput={on_otp_change}
                    disabled={state.loading}
                    maxlength="6"
                    placeholder="Enter 6-digit code"
                />
            </div>

            { error_message }

            <button onclick={on_verify_otp} disabled={state.loading || props.request_id.is_empty()}>
                { if state.loading { "Verifying..." } else { "Verify OTP" } }
            </button>
        </div>
    }
}
