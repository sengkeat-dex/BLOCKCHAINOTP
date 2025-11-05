//! Utility functions for the frontend application

use wasm_bindgen::JsValue;
use web_sys::console;

/// Log a message to the console
pub fn log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// Log an error to the console
pub fn error(message: &str) {
    console::error_1(&JsValue::from_str(message));
}

/// Format a timestamp as a human-readable string
pub fn format_timestamp(timestamp: u64) -> String {
    // For now, just return the timestamp as a string
    // In a real application, you might want to format this as a proper date/time
    timestamp.to_string()
}

/// Validate an OTP input (should be exactly 6 digits)
pub fn validate_otp(otp: &str) -> bool {
    otp.len() == 6 && otp.chars().all(|c| c.is_digit(10))
}

/// Validate a user ID (basic validation)
pub fn validate_user_id(user_id: &str) -> bool {
    !user_id.is_empty() && user_id.len() <= 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_timestamp_returns_string() {
        let result = format_timestamp(1_640_995_200);
        assert_eq!(result, "1640995200");
    }

    #[test]
    fn validate_otp_accepts_six_digits() {
        assert!(validate_otp("123456"));
    }

    #[test]
    fn validate_otp_rejects_non_digits() {
        assert!(!validate_otp("12a456"));
        assert!(!validate_otp("12345"));
        assert!(!validate_otp("1234567"));
    }

    #[test]
    fn validate_user_id_rules() {
        assert!(validate_user_id("user-123"));
        assert!(!validate_user_id(""));
        let long_id = "a".repeat(101);
        assert!(!validate_user_id(&long_id));
    }
}
