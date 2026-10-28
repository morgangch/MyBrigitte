mod common;

use axum::http::StatusCode;
use backend::auth::jwt::create_token;

#[tokio::test]
async fn me_without_token_is_unauthorized() {
    let (status, json) = common::get_json("/me").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "unauthenticated");
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

// POST /auth/login, against the test database.

async fn user_with_password(db: &sea_orm::DatabaseConnection, password: &str) -> String {
    use backend::{auth::password::hash_password, entity::user};
    use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};

    let created = common::insert_user(db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let email = created.email.clone();
    let mut active_model = created.into_active_model();
    active_model.password_hash = Set(Some(hash_password(password).unwrap()));
    active_model.update(db).await.unwrap();
    email
}

#[tokio::test]
async fn login_with_wrong_password_is_invalid_credentials() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let email = user_with_password(&db, "correct horse battery").await;

    let body = serde_json::json!({ "email": email, "password": "wrong" });
    let (status, json) = common::call(&db, "POST", "/auth/login", None, Some(body)).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "invalid_credentials");
}

#[tokio::test]
async fn login_with_unknown_email_is_invalid_credentials() {
    let Some(db) = common::test_db().await else {
        return;
    };

    let body = serde_json::json!({ "email": "nobody@test.local", "password": "whatever" });
    let (status, json) = common::call(&db, "POST", "/auth/login", None, Some(body)).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "invalid_credentials");
}

#[tokio::test]
async fn login_ignores_email_case() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let email = user_with_password(&db, "correct horse battery").await;

    let body =
        serde_json::json!({ "email": email.to_uppercase(), "password": "correct horse battery" });
    let (status, json) = common::call(&db, "POST", "/auth/login", None, Some(body)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(json["access_token"].is_string());
}
