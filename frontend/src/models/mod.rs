//! Data models for the blockchain OTP system

use serde::{Deserialize, Serialize};

/// Request payload for OTP generation
#[derive(Serialize)]
pub struct OtpRequestPayload {
    pub user_id: String,
}

/// Request payload for OTP verification
#[derive(Serialize)]
pub struct OtpVerifyPayload {
    pub request_id: String,
    pub otp: String,
}

/// Response for OTP generation
#[derive(Deserialize)]
pub struct OtpResponse {
    pub request_id: String,
    pub expires_at: u64,
}

/// Response for OTP verification
#[derive(Deserialize)]
pub struct VerifyResponse {
    pub verified: bool,
}
