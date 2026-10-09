mod common;

use axum::http::StatusCode;

#[tokio::test]
async fn test_health_check() {
    let (status, json) = common::get_json("/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
    assert_eq!(json["message"], "Server is running");
}
