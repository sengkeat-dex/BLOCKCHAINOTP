use crate::admin::{ActionOutcome, AdminController, AdminError};
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

pub mod admin;
// Add the anomaly detection and alerting modules
pub mod anomaly_detection;
use anomaly_detection::{AnomalyDetector, VerificationAttempt};

pub mod alerts;
use alerts::AlertManager;

/// Shared application state backing the Axum router.
#[derive(Clone)]
pub struct AppState {
    otp_requests: Arc<Mutex<HashMap<String, OtpRequest>>>,
    rate_limits: Arc<Mutex<HashMap<String, (f64, u64)>>>,
    // Add anomaly detector and alert manager to the state
    anomaly_detector: Arc<AnomalyDetector>,
    alert_manager: Arc<AlertManager>,
    admin: Option<Arc<AdminController>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            otp_requests: Arc::new(Mutex::new(HashMap::new())),
            rate_limits: Arc::new(Mutex::new(HashMap::new())),
            anomaly_detector: Arc::new(AnomalyDetector::new(3600, 10)), // 1 hour, 10 attempts
            alert_manager: Arc::new(AlertManager::new()),
            admin: None,
        }
    }
}

impl AppState {
    /// Creates an empty state holder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to load admin automation from environment configuration.
    pub fn bootstrap() -> Self {
        let mut state = Self::default();
        match AdminController::maybe_from_env() {
            Ok(Some(controller)) => {
                println!(
                    "Admin automation enabled for {} networks",
                    controller.network_count()
                );
                state.admin = Some(Arc::new(controller));
            }
            Ok(None) => {
                println!("Admin automation not configured; admin endpoints disabled");
            }
            Err(err) => {
                eprintln!("Failed to configure admin automation: {}", err);
            }
        }
        state
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

    /// Get reference to the anomaly detector
    pub fn anomaly_detector(&self) -> &Arc<AnomalyDetector> {
        &self.anomaly_detector
    }

    /// Get reference to the alert manager
    pub fn alert_manager(&self) -> &Arc<AlertManager> {
        &self.alert_manager
    }

    /// Returns the admin automation controller if configured.
    pub fn admin_controller(&self) -> Option<Arc<AdminController>> {
        self.admin.as_ref().map(Arc::clone)
    }
}

/// Payload accepted by the OTP generation endpoint.
#[derive(Deserialize)]
pub struct OtpRequestPayload {
    pub user_id: String,
}

// Add validation for user_id
impl OtpRequestPayload {
    fn validate(&self) -> Result<(), &'static str> {
        if self.user_id.is_empty() {
            return Err("User ID cannot be empty");
        }
        if self.user_id.len() > 100 {
            return Err("User ID too long");
        }
        // Add more validation as needed
        Ok(())
    }
}

/// Payload accepted by the OTP verification endpoint.
#[derive(Deserialize)]
pub struct OtpVerifyPayload {
    pub request_id: String,
    pub otp: String,
}

// Add validation for OTP verification payload
impl OtpVerifyPayload {
    fn validate(&self) -> Result<(), &'static str> {
        if self.request_id.is_empty() {
            return Err("Request ID cannot be empty");
        }
        if self.request_id.len() > 100 {
            return Err("Request ID too long");
        }
        if self.otp.is_empty() {
            return Err("OTP cannot be empty");
        }
        if self.otp.len() != 6 {
            return Err("OTP must be 6 digits");
        }
        // Check that OTP contains only digits
        if !self.otp.chars().all(|c| c.is_ascii_digit()) {
            return Err("OTP must contain only digits");
        }
        Ok(())
    }
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

/// Payload accepted by the admin pause endpoint.
#[derive(Deserialize)]
pub struct PauseContractPayload {
    pub network: String,
    pub paused: bool,
}

/// Payload accepted by the issuer rotation endpoint.
#[derive(Deserialize)]
pub struct RotateIssuerPayload {
    pub network: String,
    pub new_issuer: String,
}

/// Payload accepted by the admin rotation endpoint.
#[derive(Deserialize)]
pub struct RotateAdminPayload {
    pub network: String,
    pub new_admin: String,
}

/// Standard JSON error payload returned by admin automation endpoints.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
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
) -> Result<Json<OtpResponse>, (StatusCode, String)> {
    // Validate input
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    {
        let mut rate_limits = state.rate_limits.lock().unwrap();
        if !check_rate_limit(&mut rate_limits, &payload.user_id, 3.0, 1.0 / 100.0) {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
            ));
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
) -> Result<Json<VerifyResponse>, (StatusCode, String)> {
    // Validate input
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let now = now_unix();

    let mut requests = state.otp_requests.lock().unwrap();
    let otp_request = match requests.get_mut(&payload.request_id) {
        Some(req) => req,
        None => {
            // Record failed attempt for anomaly detection
            let attempt = VerificationAttempt {
                user_id: "unknown".to_string(),
                ip_address: "unknown".to_string(), // In a real implementation, we'd get the actual IP
                timestamp: now,
                success: false,
            };
            state.anomaly_detector.record_attempt(attempt);

            return Ok(Json(VerifyResponse { verified: false }));
        }
    };

    let verification_result = if otp_request.used || now > otp_request.expires_at {
        false
    } else {
        let provided_hash = hash_otp(&payload.otp);
        provided_hash == otp_request.otp_hash
    };

    if verification_result {
        otp_request.used = true;
    }

    // Record the verification attempt for anomaly detection
    let attempt = VerificationAttempt {
        user_id: otp_request.user_id.clone(),
        ip_address: "unknown".to_string(), // In a real implementation, we'd get the actual IP
        timestamp: now,
        success: verification_result,
    };
    state.anomaly_detector.record_attempt(attempt);

    // Check for anomalies and generate alerts
    if state
        .anomaly_detector
        .is_user_anomalous(&otp_request.user_id)
    {
        if let Some(stats) = state.anomaly_detector.get_user_stats(&otp_request.user_id) {
            state
                .alert_manager
                .alert_user_anomaly(&otp_request.user_id, stats.attempt_count);
        }
    }

    if state.anomaly_detector.is_ip_anomalous("unknown") {
        if let Some(stats) = state.anomaly_detector.get_ip_stats("unknown") {
            state.alert_manager.alert_ip_anomaly(
                "unknown",
                stats.attempt_count,
                stats.failed_attempts,
            );
        }
    }

    Ok(Json(VerifyResponse {
        verified: verification_result,
    }))
}

/// Pause or unpause the deployed OtpVerifier contract via automation.
pub async fn pause_contract(
    State(state): State<AppState>,
    Json(payload): Json<PauseContractPayload>,
) -> Result<Json<ActionOutcome>, (StatusCode, Json<ErrorResponse>)> {
    let controller = state.admin_controller().ok_or_else(|| {
        into_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "admin automation not configured",
        )
    })?;

    controller
        .pause(&payload.network, payload.paused)
        .await
        .map(Json)
        .map_err(|err| admin_error_response(&err))
}

/// Rotate the issuer key using the configured admin automation controller.
pub async fn rotate_issuer(
    State(state): State<AppState>,
    Json(payload): Json<RotateIssuerPayload>,
) -> Result<Json<ActionOutcome>, (StatusCode, Json<ErrorResponse>)> {
    let controller = state.admin_controller().ok_or_else(|| {
        into_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "admin automation not configured",
        )
    })?;

    controller
        .rotate_issuer(&payload.network, &payload.new_issuer)
        .await
        .map(Json)
        .map_err(|err| admin_error_response(&err))
}

/// Rotate the admin key using the configured automation controller.
pub async fn rotate_admin(
    State(state): State<AppState>,
    Json(payload): Json<RotateAdminPayload>,
) -> Result<Json<ActionOutcome>, (StatusCode, Json<ErrorResponse>)> {
    let controller = state.admin_controller().ok_or_else(|| {
        into_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "admin automation not configured",
        )
    })?;

    controller
        .rotate_admin(&payload.network, &payload.new_admin)
        .await
        .map(Json)
        .map_err(|err| admin_error_response(&err))
}

fn admin_error_response(err: &AdminError) -> (StatusCode, Json<ErrorResponse>) {
    let status = status_from_admin_error(err);
    eprintln!("Admin automation error ({status}): {err}");
    (
        status,
        Json(ErrorResponse {
            error: err.to_string(),
        }),
    )
}

fn status_from_admin_error(err: &AdminError) -> StatusCode {
    match err {
        AdminError::UnknownNetwork(_) => StatusCode::NOT_FOUND,
        AdminError::InvalidAddress(_) => StatusCode::BAD_REQUEST,
        AdminError::Rpc(_) | AdminError::Contract(_) | AdminError::MissingReceipt => {
            StatusCode::BAD_GATEWAY
        }
        AdminError::MissingAdminKey
        | AdminError::InvalidAdminKey(_)
        | AdminError::MissingConfig(_)
        | AdminError::Io(_)
        | AdminError::Toml(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn into_error(status: StatusCode, message: impl Into<String>) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            error: message.into(),
        }),
    )
}

// Add an endpoint to retrieve recent alerts
pub async fn get_alerts(
    State(state): State<AppState>,
) -> Result<Json<Vec<alerts::SecurityAlert>>, StatusCode> {
    let recent_alerts = state.alert_manager.get_recent_alerts(50);
    Ok(Json(recent_alerts))
}

/// Build an Axum router bound to the supplied state.
pub fn create_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/cors-test", get(cors_test))
        .route("/otp/request", post(request_otp))
        .route("/otp/verify", post(verify_otp))
        .route("/admin/pause", post(pause_contract))
        .route("/admin/rotate-issuer", post(rotate_issuer))
        .route("/admin/rotate-admin", post(rotate_admin))
        // Add endpoint for retrieving alerts
        .route("/alerts", get(get_alerts))
        // Add a simple authentication endpoint for testing
        .route("/auth/test", get(test_auth))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}

/// Build a router with a fresh, empty state.
pub fn create_app() -> Router {
    create_app_with_state(AppState::bootstrap())
}

// Add a simple test authentication endpoint
pub async fn test_auth() -> Result<&'static str, StatusCode> {
    // In a real implementation, this would check authentication tokens
    // For now, we just return success to show the route exists
    Ok("Authentication endpoint ready")
}
