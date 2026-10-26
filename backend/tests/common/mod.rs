#![allow(dead_code)]

use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
};
use backend::{
    AppState,
    auth::jwt::create_token,
    create_app,
    entity::{team, team_member, user},
};
use http_body_util::BodyExt;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set, prelude::Uuid};
use serde_json::Value;
use tokio::sync::OnceCell;
use tower::ServiceExt;

pub const TEST_SECRET: &str = "secret-de-test-assez-long-pour-hs256-0123456789";

/// Migrations run once per test binary, even when tests start in parallel.
static MIGRATED: OnceCell<()> = OnceCell::const_new();

async fn send(state: AppState, request: Request<Body>) -> (StatusCode, Value) {
    let response = create_app()
        .with_state(state)
        .oneshot(request)
        .await
        .unwrap();
    let status = response.status();
    let body = response.collect().await.unwrap().to_bytes();
    // A 204 has no body.
    let json = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap()
    };
    (status, json)
}

fn disconnected() -> AppState {
    AppState::disconnect(TEST_SECRET.to_string())
}

pub async fn get_json(uri: &str) -> (StatusCode, Value) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    send(disconnected(), request).await
}

pub async fn get_json_with_token(uri: &str, token: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .uri(uri)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    send(disconnected(), request).await
}

/// Sends a request to an app without a database. Enough for anything rejected before SQL.
pub async fn call_without_db(
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    send(disconnected(), build(method, uri, token, body)).await
}

/// Sends a request to an app backed by the test database.
pub async fn call(
    db: &DatabaseConnection,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let state = AppState::new(db.clone(), TEST_SECRET.to_string());
    send(state, build(method, uri, token, body)).await
}

fn build(method: &str, uri: &str, token: Option<&str>, body: Option<Value>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
    }
    match body {
        Some(body) => builder
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap(),
        None => builder.body(Body::empty()).unwrap(),
    }
}

/// The migrated test database, or `None` when `TEST_DATABASE_URL` is unset.
///
/// Database tests start with `let Some(db) = common::test_db().await else { return };`
/// so `cargo test` still passes on a machine without Postgres. Tests never clean up:
/// each one creates its own teams and users with unique names, and filters on them.
pub async fn test_db() -> Option<DatabaseConnection> {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("database test skipped: TEST_DATABASE_URL is not set");
        return None;
    };
    let db = Database::connect(&url)
        .await
        .expect("cannot connect to TEST_DATABASE_URL");
    MIGRATED
        .get_or_init(|| async {
            Migrator::up(&db, None).await.expect("migrations failed");
        })
        .await;
    Some(db)
}

pub fn unique(prefix: &str) -> String {
    format!("{prefix}-{}", Uuid::new_v4().simple())
}

pub fn token_for(user: &user::Model) -> String {
    create_token(&user.id.to_string(), TEST_SECRET).unwrap()
}

pub async fn insert_team(db: &DatabaseConnection) -> team::Model {
    team::ActiveModel {
        name: Set(unique("team")),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
}

/// Inserts an active user with a unique email, in `team_id` when given.
pub async fn insert_user(
    db: &DatabaseConnection,
    role: user::Role,
    team_id: Option<Uuid>,
    first_name: &str,
    last_name: &str,
) -> user::Model {
    let created = user::ActiveModel {
        first_name: Set(first_name.to_string()),
        last_name: Set(last_name.to_string()),
        email: Set(format!("{}@test.local", unique("user"))),
        role: Set(role),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    if let Some(team_id) = team_id {
        team_member::ActiveModel {
            user_id: Set(created.id),
            team_id: Set(team_id),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();
    }

    created
}

/// A manager in a fresh team, with a token: the usual caller of `/users`.
pub async fn manager_with_team(db: &DatabaseConnection) -> (user::Model, team::Model, String) {
    let team = insert_team(db).await;
    let manager = insert_user(db, user::Role::Manager, Some(team.id), "Mia", "Manager").await;
    let token = token_for(&manager);
    (manager, team, token)
}
