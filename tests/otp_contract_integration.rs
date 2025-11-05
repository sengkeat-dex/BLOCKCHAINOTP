use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blockchain_otp::{create_app_with_state, AppState};
use hyper::body::to_bytes;
use otp_core::{hash_otp, now_unix};
use serde_json::json;
use std::collections::HashMap;
use tower::ServiceExt;

#[derive(Debug, Clone)]
struct Entry {
    hash: [u8; 32],
    expiry: u64,
    attempts: u8,
    used: bool,
}

#[derive(Debug, Clone)]
struct OnChainSimulator {
    entries: HashMap<[u8; 32], Entry>,
    now: u64,
}

#[derive(Debug, PartialEq)]
enum OnChainError {
    UnknownRequest,
    AlreadyUsed,
    Expired,
    AttemptsLocked,
    ActiveRequest,
}

impl OnChainSimulator {
    const MAX_ATTEMPTS: u8 = 3;

    fn new(now: u64) -> Self {
        Self {
            entries: HashMap::new(),
            now,
        }
    }

    fn set_time(&mut self, now: u64) {
        self.now = now;
    }

    fn advance_time(&mut self, delta: u64) {
        self.now = self.now.saturating_add(delta);
    }

    fn current_time(&self) -> u64 {
        self.now
    }

    fn set_otp(&mut self, request_id: [u8; 32], hash: [u8; 32], expiry: u64) -> Result<(), OnChainError> {
        if expiry <= self.now {
            return Err(OnChainError::Expired);
        }
        if self.entries.contains_key(&request_id) {
            return Err(OnChainError::ActiveRequest);
        }
        self.entries.insert(
            request_id,
            Entry {
                hash,
                expiry,
                attempts: 0,
                used: false,
            },
        );
        Ok(())
    }

    fn verify(&mut self, request_id: &[u8; 32], otp: &str) -> Result<bool, OnChainError> {
        let entry = self
            .entries
            .get_mut(request_id)
            .ok_or(OnChainError::UnknownRequest)?;
        if entry.used {
            return Err(OnChainError::AlreadyUsed);
        }
        if self.now > entry.expiry {
            return Err(OnChainError::Expired);
        }
        if entry.attempts >= Self::MAX_ATTEMPTS {
            return Err(OnChainError::AttemptsLocked);
        }

        let expected = hex_to_bytes32(&hash_otp(otp));
        if expected != entry.hash {
            entry.attempts = entry.attempts.saturating_add(1);
            return Ok(false);
        }

        entry.used = true;
        Ok(true)
    }

    fn cleanup(&mut self, request_id: &[u8; 32]) -> Result<(), OnChainError> {
        let entry = self
            .entries
            .get(request_id)
            .ok_or(OnChainError::UnknownRequest)?;
        if !entry.used && self.now <= entry.expiry && entry.attempts < Self::MAX_ATTEMPTS {
            return Err(OnChainError::ActiveRequest);
        }
        self.entries.remove(request_id);
        Ok(())
    }

    fn entry(&self, request_id: &[u8; 32]) -> Option<&Entry> {
        self.entries.get(request_id)
    }
}

fn recover_otp_from_hash(hash: &str) -> Option<String> {
    for value in 0..1_000_000 {
        let candidate = format!("{value:06}");
        if hash_otp(&candidate) == hash {
            return Some(candidate);
        }
    }
    None
}

fn hex_to_bytes32(input: &str) -> [u8; 32] {
    let trimmed = input.trim_start_matches("0x");
    let bytes = hex::decode(trimmed).expect("valid hex");
    assert_eq!(bytes.len(), 32, "expected 32-byte value");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    out
}

#[tokio::test]
async fn backend_and_contract_logic_align() {
    let state = AppState::new();
    let app = create_app_with_state(state.clone());
    let mut chain = OnChainSimulator::new(now_unix());

    let request_payload = json!({ "user_id": "contract-flow-user" }).to_string();
    let response = app
        .clone()
        .oneshot(
            Request::post("/otp/request")
                .header("content-type", "application/json")
                .body(Body::from(request_payload))
                .unwrap(),
        )
        .await
        .expect("request route should respond");

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let request_id = value["request_id"].as_str().unwrap().to_string();
    let expires_at = value["expires_at"].as_u64().unwrap();

    let stored = state
        .snapshot_request(&request_id)
        .expect("backend should retain request metadata");
    let request_id_bytes = hex_to_bytes32(&request_id);
    let otp_hash = hex_to_bytes32(&stored.otp_hash);

    chain.set_time(stored.expires_at.saturating_sub(30));
    chain.set_otp(request_id_bytes, otp_hash, stored.expires_at).unwrap();

    assert_eq!(chain.verify(&request_id_bytes, "000000"), Ok(false));
    assert_eq!(
        chain.entry(&request_id_bytes).unwrap().attempts,
        1,
        "first invalid attempt should increment counter"
    );

    assert_eq!(chain.verify(&request_id_bytes, "111111"), Ok(false));
    assert_eq!(
        chain.entry(&request_id_bytes).unwrap().attempts,
        2,
        "second invalid attempt should increment counter"
    );

    let otp = recover_otp_from_hash(&stored.otp_hash).expect("hash should map back to OTP for test");
    assert!(chain.verify(&request_id_bytes, &otp).unwrap());
    assert_eq!(
        chain.verify(&request_id_bytes, &otp),
        Err(OnChainError::AlreadyUsed)
    );

    chain.cleanup(&request_id_bytes).unwrap();
    assert!(chain.entry(&request_id_bytes).is_none());

    // Re-issue the same request id after cleanup and assert attempt limit locks verification.
    let new_expiry = expires_at + 120;
    chain
        .set_otp(request_id_bytes, otp_hash, new_expiry)
        .expect("cleanup should permit reissue");
    assert_eq!(
        chain.cleanup(&request_id_bytes),
        Err(OnChainError::ActiveRequest)
    );
    for expected_attempt in 1..=OnChainSimulator::MAX_ATTEMPTS {
        assert_eq!(chain.verify(&request_id_bytes, "999999"), Ok(false));
        assert_eq!(
            chain.entry(&request_id_bytes).unwrap().attempts,
            expected_attempt,
            "attempt counter should reflect failed verifications"
        );
    }
    assert_eq!(
        chain.verify(&request_id_bytes, "654321"),
        Err(OnChainError::AttemptsLocked)
    );
    assert!(chain.cleanup(&request_id_bytes).is_ok());

    // Expiry-driven cleanup
    let mut expiry_request = request_id_bytes;
    expiry_request[0] ^= 0x01;
    chain.set_time(now_unix());
    let expiry_time = chain.current_time() + 30;
    chain
        .set_otp(expiry_request, otp_hash, expiry_time)
        .expect("should accept new entry");
    chain.advance_time(120);
    assert_eq!(
        chain.verify(&expiry_request, "123456"),
        Err(OnChainError::Expired)
    );
    assert!(chain.cleanup(&expiry_request).is_ok());
}
