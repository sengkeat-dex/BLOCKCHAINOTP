use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blockchain_otp::{create_app_with_state, AppState};
use hyper::body::to_bytes;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn admin_endpoints_require_configuration() {
    let app = create_app_with_state(AppState::new());
    let payload = serde_json::json!({ "network": "ethereum_sepolia", "paused": true });

    let response = app
        .clone()
        .oneshot(
            Request::post("/admin/pause")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .expect("pause endpoint should respond");

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body()).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        json["error"].as_str(),
        Some("admin automation not configured")
    );
}
