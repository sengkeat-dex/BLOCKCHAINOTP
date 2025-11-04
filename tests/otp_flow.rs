use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use blockchain_otp::{create_app_with_state, AppState};
use hyper::body::to_bytes;
use otp_core::{hash_otp, now_unix, OtpRequest};
use serde_json::json;
use tower::ServiceExt;

fn recover_otp_from_hash(hash: &str) -> Option<String> {
    for value in 0..1_000_000 {
        let candidate = format!("{value:06}");
        if hash_otp(&candidate) == hash {
            return Some(candidate);
        }
    }
    None
}

#[tokio::test]
async fn otp_request_and_verify_flow() {
    let state = AppState::new();
    let app = create_app_with_state(state.clone());

    let request_payload = json!({ "user_id": "integration-user" }).to_string();
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

    let stored = state
        .snapshot_request(&request_id)
        .expect("request should be stored");
    let otp = recover_otp_from_hash(&stored.otp_hash).expect("otp should be recoverable");

    let verify_payload = json!({ "request_id": request_id, "otp": otp }).to_string();
    let verify_response = app
        .clone()
        .oneshot(
            Request::post("/otp/verify")
                .header("content-type", "application/json")
                .body(Body::from(verify_payload))
                .unwrap(),
        )
        .await
        .expect("verify route should respond");

    assert_eq!(verify_response.status(), StatusCode::OK);
    let body = to_bytes(verify_response.into_body()).await.unwrap();
    let verified = serde_json::from_slice::<serde_json::Value>(&body).unwrap()["verified"]
        .as_bool()
        .unwrap();
    assert!(verified, "OTP should verify exactly once");
}

#[tokio::test]
async fn otp_rejects_expired_codes() {
    let state = AppState::new();
    let request_id = "expired-case".to_string();
    let otp = "123456";
    state.insert_request(OtpRequest {
        request_id: request_id.clone(),
        user_id: "expired-user".into(),
        otp_hash: hash_otp(otp),
        expires_at: now_unix().saturating_sub(5),
        used: false,
        counter: 0,
    });

    let app = create_app_with_state(state);
    let payload = json!({ "request_id": request_id, "otp": otp }).to_string();
    let response = app
        .oneshot(
            Request::post("/otp/verify")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .expect("verify route should respond");

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body()).await.unwrap();
    let verified = serde_json::from_slice::<serde_json::Value>(&body).unwrap()["verified"]
        .as_bool()
        .unwrap();
    assert!(!verified, "Expired OTP must be rejected");
}

#[tokio::test]
async fn otp_rejects_reuse_of_codes() {
    let state = AppState::new();
    let request_id = "reuse-case".to_string();
    let otp = "654321";
    state.insert_request(OtpRequest {
        request_id: request_id.clone(),
        user_id: "reuse-user".into(),
        otp_hash: hash_otp(otp),
        expires_at: now_unix() + 60,
        used: false,
        counter: 0,
    });

    let app = create_app_with_state(state.clone());
    let payload = json!({ "request_id": request_id.clone(), "otp": otp }).to_string();

    let first = app
        .clone()
        .oneshot(
            Request::post("/otp/verify")
                .header("content-type", "application/json")
                .body(Body::from(payload.clone()))
                .unwrap(),
        )
        .await
        .expect("first verification should respond");
    let first_body = to_bytes(first.into_body()).await.unwrap();
    let first_verified = serde_json::from_slice::<serde_json::Value>(&first_body).unwrap()
        ["verified"]
        .as_bool()
        .unwrap();
    assert!(first_verified, "First verification should succeed");

    let second = app
        .oneshot(
            Request::post("/otp/verify")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .expect("second verification should respond");
    let second_body = to_bytes(second.into_body()).await.unwrap();
    let second_verified = serde_json::from_slice::<serde_json::Value>(&second_body).unwrap()
        ["verified"]
        .as_bool()
        .unwrap();
    assert!(
        !second_verified,
        "Reusing the same request_id should fail after first success"
    );
}
