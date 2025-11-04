use hmac::{Hmac, Mac};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::{Digest, Keccak256};
use std::time::{SystemTime, UNIX_EPOCH};

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// Represents an OTP request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtpRequest {
    pub request_id: String,
    pub user_id: String,
    pub otp_hash: String,
    pub expires_at: u64,
    pub used: bool,
    pub counter: u64, // For HOTP support
}

/// Generates a secure 6-digit OTP using CSPRNG
pub fn generate_otp_6() -> String {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new_inclusive(0, 9);
    (0..6)
        .map(|_| char::from(b'0' + dist.sample(&mut rng) as u8))
        .collect()
}

/// Generates a random 32-byte request ID
pub fn generate_request_id() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    format!("0x{}", hex::encode(bytes))
}

/// Hashes an OTP using Keccak256 with domain separation
pub fn hash_otp(otp: &str) -> String {
    let mut hasher = Keccak256::new();
    // Add domain separation as recommended in the algorithm document
    hasher.update(b"OTP:v1");
    hasher.update(otp.as_bytes());
    format!("0x{}", hex::encode(hasher.finalize()))
}

/// Gets current Unix timestamp
pub fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

/// Converts a 64-bit integer to big-endian bytes
fn to_be64(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

/// Truncates HMAC output to generate OTP digits (RFC 4226 compliant)
fn truncate(hmac_result: &[u8], digits: usize) -> String {
    let offset = (hmac_result[19] & 0x0F) as usize;
    let binary = ((hmac_result[offset] as u32 & 0x7F) << 24)
        | ((hmac_result[offset + 1] as u32 & 0xFF) << 16)
        | ((hmac_result[offset + 2] as u32 & 0xFF) << 8)
        | (hmac_result[offset + 3] as u32 & 0xFF);

    let otp = binary % 10_u32.pow(digits as u32);
    format!("{:0digits$}", otp, digits = digits)
}

/// Generates HOTP (HMAC-based One-Time Password) according to RFC 4226
pub fn generate_hotp(secret: &[u8], counter: u64, digits: usize) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(&to_be64(counter));
    let result = mac.finalize().into_bytes();
    truncate(&result, digits)
}

/// Generates TOTP (Time-based One-Time Password) according to RFC 6238
pub fn generate_totp(secret: &[u8], time_step: u64, digits: usize, t0: u64) -> String {
    let timestamp = now_unix();
    let counter = (timestamp - t0) / time_step;
    generate_hotp(secret, counter, digits)
}

/// Binds a request to a user with secure derivation
pub fn bind_request_to_user(user_id: &str) -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

    let mut hasher = Keccak256::new();
    hasher.update(user_id.as_bytes());
    hasher.update(&random_bytes);
    format!("0x{}", hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_otp_6() {
        let otp = generate_otp_6();
        assert_eq!(otp.len(), 6);
        assert!(otp.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn test_hash_otp() {
        let otp = "123456";
        let hash = hash_otp(otp);
        assert!(!hash.is_empty());
        assert!(hash.starts_with("0x"));
    }

    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
        assert_ne!(id1, id2); // Very high probability of being different
    }

    #[test]
    fn test_hotp_rfc_reference() {
        // RFC 4226 test vector
        let secret = b"12345678901234567890";
        let expected_otps = vec!["755224", "287082", "359152", "969429", "338314"];

        for (i, expected) in expected_otps.iter().enumerate() {
            let otp = generate_hotp(secret, i as u64, 6);
            assert_eq!(&otp, expected);
        }
    }

    #[test]
    fn test_bind_request_to_user() {
        let user_id = "user123";
        let binding1 = bind_request_to_user(user_id);
        let binding2 = bind_request_to_user(user_id);

        assert!(!binding1.is_empty());
        assert!(!binding2.is_empty());
        assert_ne!(binding1, binding2); // Should be different due to random component
    }
}
