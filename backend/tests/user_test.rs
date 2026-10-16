mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn create_user_rejects_a_body_without_role() {
    let request = Request::builder()
        .method("POST")
        .uri("/users")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"first_name":"Ada"}"#))
        .unwrap();

    let response = backend::create_app()
        .with_state(backend::AppState::disconnect(
            common::TEST_SECRET.to_string(),
        ))
        .oneshot(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["error"]["code"], "validation_failed");
    assert!(json["error"]["fields"]["body"].is_string());
}
