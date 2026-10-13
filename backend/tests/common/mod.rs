use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{config::Config, create_app};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

fn test_config() -> Config {
    Config {
        database_url: "postgresql://test".to_string(),
        port: 0,
        jwt_secret: "secret-de-test-assez-long-pour-hs256-0123456789".to_string(),
    }
}
pub async fn get_json(uri: &str) -> (StatusCode, Value) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = create_app(test_config()).oneshot(request).await.unwrap();

    let status = response.status();
    let body = response.collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&body).unwrap();

    (status, json)
}
