mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
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

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
