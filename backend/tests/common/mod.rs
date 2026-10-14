#![allow(dead_code)]

use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use backend::{AppState, create_app};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

pub const TEST_SECRET: &str = "secret-de-test-assez-long-pour-hs256-0123456789";

async fn send(request: Request<Body>) -> (StatusCode, Value) {
    let response = create_app()
        .with_state(AppState::disconnect(TEST_SECRET.to_string()))
        .oneshot(request)
        .await
        .unwrap();
    let status = response.status();
    let body = response.collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&body).unwrap();
    (status, json)
}

pub async fn get_json(uri: &str) -> (StatusCode, Value) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    send(request).await
}

pub async fn get_json_with_token(uri: &str, token: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .uri(uri)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    send(request).await
}