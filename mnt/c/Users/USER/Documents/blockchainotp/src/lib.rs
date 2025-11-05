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

// Add MCP modules
pub mod mcp_handler;
pub mod mcp_server;

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
