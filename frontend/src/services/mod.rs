//! Services for interacting with the backend API

use gloo_net::http::Request;

use crate::models::{OtpRequestPayload, OtpResponse, OtpVerifyPayload, VerifyResponse};

/// API service for OTP operations
pub struct OtpService;

impl OtpService {
    /// Request a new OTP for a user
    pub async fn request_otp(payload: OtpRequestPayload) -> Result<OtpResponse, String> {
        let url = "http://localhost:3001/otp/request";

        let resp = Request::post(url)
            .json(&payload)
            .map_err(|e| format!("Failed to serialize request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {:?}. Please check your connection and try again.", e))?;

        if resp.ok() {
            let otp_response: OtpResponse = resp
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {:?}", e))?;
            Ok(otp_response)
        } else {
            let status = resp.status();
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Server error {}: {}. Please try again.", status, error_text))
        }
    }

    /// Verify an OTP
    pub async fn verify_otp(payload: OtpVerifyPayload) -> Result<VerifyResponse, String> {
        let url = "http://localhost:3001/otp/verify";

        let resp = Request::post(url)
            .json(&payload)
            .map_err(|e| format!("Failed to serialize request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {:?}. Please check your connection and try again.", e))?;

        if resp.ok() {
            let verify_response: VerifyResponse = resp
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {:?}", e))?;
            Ok(verify_response)
        } else {
            let status = resp.status();
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Server error {}: {}. Please try again.", status, error_text))
        }
    }
}