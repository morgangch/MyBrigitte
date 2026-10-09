mod common;

use axum::http::StatusCode;

#[tokio::test]
async fn test_get_user_returns_user() {
    let (status, json) = common::get_json("/users/1").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["id"], 1);
    assert_eq!(json["name"], "User");
}

#[tokio::test]
async fn test_get_user_returns_not_found() {
    let (status, json) = common::get_json("/users/101").await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["error"], "Data not found");
}

#[tokio::test]
async fn test_list_users_returns_internal_error() {
    let (status, json) = common::get_json("/users").await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(json["error"], "Internal server error");
}
