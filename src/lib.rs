use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use otp_core::{bind_request_to_user, generate_totp, hash_otp, now_unix, OtpRequest};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Add the MCP server modules
pub mod mcp_server;
pub mod mcp_handler;

/// Shared application state backing the Axum router.
#[derive(Clone, Default)]
pub struct AppState {
    otp_requests: Arc<Mutex<HashMap<String, OtpRequest>>>,
    rate_limits: Arc<Mutex<HashMap<String, (f64, u64)>>>,
}

impl AppState {
    /// Creates an empty state holder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a cloned OTP request for inspection (primarily for tests).
    pub fn snapshot_request(&self, request_id: &str) -> Option<OtpRequest> {
        self.otp_requests
            .lock()
            .ok()
            .and_then(|map| map.get(request_id).cloned())
    }

    /// Inserts or replaces an OTP request (used by tests and fixtures).
    pub fn insert_request(&self, request: OtpRequest) {
        if let Ok(mut map) = self.otp_requests.lock() {
            map.insert(request.request_id.clone(), request);
        }
    }
}

/// Payload accepted by the OTP generation endpoint.
#[derive(Deserialize)]
pub struct OtpRequestPayload {
    pub user_id: String,
}

/// Payload accepted by the OTP verification endpoint.
#[derive(Deserialize)]
pub struct OtpVerifyPayload {
    pub request_id: String,
    pub otp: String,
}

/// Response sent back after a successful OTP request.
#[derive(Serialize)]
pub struct OtpResponse {
    pub request_id: String,
    pub expires_at: u64,
}

/// Response returned by the verification endpoint.
#[derive(Serialize)]
pub struct VerifyResponse {
    pub verified: bool,
}

/// Simple health endpoint response.
pub async fn health_check() -> &'static str {
    "OK"
}

/// CORS test endpoint
pub async fn cors_test() -> &'static str {
    "CORS test successful"
}

/// Token bucket rate limiting implementation.
fn check_rate_limit(
    rate_limits: &mut HashMap<String, (f64, u64)>,
    user_id: &str,
    capacity: f64,
    refill_rate: f64,
) -> bool {
    let now = now_unix();
    let (tokens, last_refill) = rate_limits
        .entry(user_id.to_string())
        .or_insert((capacity, now));

    let delta = (now - *last_refill) as f64;
    *tokens = (*tokens + delta * refill_rate).min(capacity);
    *last_refill = now;

    if *tokens >= 1.0 {
        *tokens -= 1.0;
        true
    } else {
        false
    }
}

/// Generate a new OTP for a user.
pub async fn request_otp(
    State(state): State<AppState>,
    Json(payload): Json<OtpRequestPayload>,
) -> Result<Json<OtpResponse>, StatusCode> {
    {
        let mut rate_limits = state.rate_limits.lock().unwrap();
        if !check_rate_limit(&mut rate_limits, &payload.user_id, 3.0, 1.0 / 100.0) {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    let secret = b"blockchain-otp-secret-key-for-user";
    let otp = generate_totp(secret, 60, 6, 0);
    let otp_hash = hash_otp(&otp);

    let request_id = bind_request_to_user(&payload.user_id);
    let expires_at = now_unix() + 60;

    let otp_request = OtpRequest {
        request_id: request_id.clone(),
        user_id: payload.user_id.clone(),
        otp_hash,
        expires_at,
        used: false,
        counter: 0,
    };

    {
        let mut requests = state.otp_requests.lock().unwrap();
        requests.insert(request_id.clone(), otp_request);
    }

    println!("Generated TOTP for user {}: {}", payload.user_id, otp);

    Ok(Json(OtpResponse {
        request_id,
        expires_at,
    }))
}

/// Verify an OTP submission and enforce expiry and single-use semantics.
pub async fn verify_otp(
    State(state): State<AppState>,
    Json(payload): Json<OtpVerifyPayload>,
) -> Result<Json<VerifyResponse>, StatusCode> {
    let now = now_unix();

    let mut requests = state.otp_requests.lock().unwrap();
    let otp_request = match requests.get_mut(&payload.request_id) {
        Some(req) => req,
        None => return Ok(Json(VerifyResponse { verified: false })),
    };

    if otp_request.used || now > otp_request.expires_at {
        return Ok(Json(VerifyResponse { verified: false }));
    }

    let provided_hash = hash_otp(&payload.otp);
    if provided_hash != otp_request.otp_hash {
        return Ok(Json(VerifyResponse { verified: false }));
    }

    otp_request.used = true;
    Ok(Json(VerifyResponse { verified: true }))
}

/// Build an Axum router bound to the supplied state.
pub fn create_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/cors-test", get(cors_test))
        .route("/otp/request", post(request_otp))
        .route("/otp/verify", post(verify_otp))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}

/// Build a router with a fresh, empty state.
pub fn create_app() -> Router {
    create_app_with_state(AppState::new())
}
