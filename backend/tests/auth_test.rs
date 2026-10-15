mod common;

use axum::http::StatusCode;
use backend::auth::jwt::create_token;

#[tokio::test]
async fn me_without_token_is_unauthorized() {
    let (status, json) = common::get_json("/me").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "unauthorized");
}

#[tokio::test]
async fn me_with_invalid_token_is_unauthorized() {
    let (status, _) = common::get_json_with_token("/me", "pas-un-jwt").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_with_valid_token_returns_user_id() {
    let token = create_token("42", common::TEST_SECRET).unwrap();
    let (status, json) = common::get_json_with_token("/me", &token).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["user_id"], "42");
}
