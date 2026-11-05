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
#[tokio::test]
async fn login_creates_a_session_storing_only_the_token_hash() {
    use backend::{auth::refresh::hash_refresh_token, entity::auth_session, services::auth};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let Some(db) = common::test_db().await else {
        return;
    };
    let email = user_with_password(&db, "correct horse battery").await;

    let tokens = auth::login(&db, &email, "correct horse battery", common::TEST_SECRET)
        .await
        .unwrap();

    let session = auth_session::Entity::find()
        .filter(
            auth_session::Column::RefreshTokenHash.eq(hash_refresh_token(&tokens.refresh_token)),
        )
        .one(&db)
        .await
        .unwrap()
        .expect("login must create a session");

    assert_ne!(session.refresh_token_hash, tokens.refresh_token);
    assert!(session.revoked_at.is_none());
    let hours = (session.expires_at - session.created_at).num_hours();
    assert!((167..=168).contains(&hours));
}

#[tokio::test]
async fn failed_login_creates_no_session() {
    use backend::{
        entity::{auth_session, user},
        services::auth,
    };
    use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

    let Some(db) = common::test_db().await else {
        return;
    };
    let email = user_with_password(&db, "correct horse battery").await;

    let result = auth::login(&db, &email, "wrong", common::TEST_SECRET).await;
    assert!(result.is_err());

    let created = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    let sessions = auth_session::Entity::find()
        .filter(auth_session::Column::UserId.eq(created.id))
        .count(&db)
        .await
        .unwrap();
    assert_eq!(sessions, 0);
}
