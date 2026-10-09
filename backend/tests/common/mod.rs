use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::create_app;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

/// Sends a GET request to the app in memory (no network, no server started)
/// and returns the response status with its JSON body.
pub async fn get_json(uri: &str) -> (StatusCode, Value) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = create_app().oneshot(request).await.unwrap();

    let status = response.status();
    let body = response.collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&body).unwrap();

    (status, json)
}
