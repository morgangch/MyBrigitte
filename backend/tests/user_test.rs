mod common;

use axum::http::StatusCode;
use backend::entity::user::Role;
use sea_orm::prelude::Uuid;
use serde_json::json;

use common::{call, call_without_db, insert_team, insert_user, manager_with_team, token_for};

const SOME_ID: &str = "00000000-0000-0000-0000-000000000001";

// Without a database: rejected before any SQL.

#[tokio::test]
async fn every_users_route_requires_a_token() {
    let routes = [
        ("GET", "/users".to_string(), None),
        ("POST", "/users".to_string(), Some(json!({}))),
        ("GET", format!("/users/{SOME_ID}"), None),
        ("PATCH", format!("/users/{SOME_ID}"), Some(json!({}))),
        ("DELETE", format!("/users/{SOME_ID}"), None),
    ];

    for (method, uri, body) in routes {
        let (status, json) = call_without_db(method, &uri, None, body).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{method} {uri}");
        assert_eq!(json["error"]["code"], "unauthenticated", "{method} {uri}");
    }
}

#[tokio::test]
async fn users_route_rejects_an_invalid_token() {
    let (status, json) = call_without_db("GET", "/users", Some("pas-un-jwt"), None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "unauthenticated");
}

// Roles.

#[tokio::test]
async fn employee_is_forbidden() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let team = insert_team(&db).await;
    let employee = insert_user(&db, Role::Employee, Some(team.id), "Eve", "Employee").await;

    let (status, json) = call(&db, "GET", "/users", Some(&token_for(&employee)), None).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(json["error"]["code"], "forbidden");
}

#[tokio::test]
async fn deactivated_manager_is_unauthenticated() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (manager, _, token) = manager_with_team(&db).await;
    let (status, _) = call(
        &db,
        "DELETE",
        &format!("/users/{}", manager.id),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, json) = call(&db, "GET", "/users", Some(&token), None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(json["error"]["code"], "unauthenticated");
}

#[tokio::test]
async fn token_of_unknown_user_is_unauthenticated() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let token =
        backend::auth::jwt::create_token(&Uuid::new_v4().to_string(), common::TEST_SECRET).unwrap();

    let (status, _) = call(&db, "GET", "/users", Some(&token), None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// POST /users

#[tokio::test]
async fn create_user_returns_the_user_with_its_team() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let email = format!("{}@test.local", common::unique("Ada"));

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({
            "first_name": "  Ada ",
            "last_name": "Lovelace",
            "email": email,
            "phone_number": null,
            "team_id": team.id,
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json["first_name"], "Ada");
    assert_eq!(json["email"], email);
    assert_eq!(json["phone_number"], serde_json::Value::Null);
    assert_eq!(json["role"], "employee");
    assert_eq!(json["is_active"], true);
    assert_eq!(json["team"]["id"], team.id.to_string());
    assert_eq!(json["team"]["name"], team.name);
    assert!(json.get("password_hash").is_none());
    assert!(json.get("updated_at").is_none());

    let uri = format!("/users/{}", json["id"].as_str().unwrap());
    let (status, fetched) = call(&db, "GET", &uri, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["team"]["id"], team.id.to_string());
}

#[tokio::test]
async fn create_user_keeps_an_explicit_manager_role() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({
            "first_name": "Grace",
            "last_name": "Hopper",
            "email": format!("{}@test.local", common::unique("grace")),
            "team_id": team.id,
            "role": "manager",
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(json["role"], "manager");
}

#[tokio::test]
async fn create_user_reports_each_invalid_field() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({
            "first_name": "   ",
            "last_name": "x".repeat(101),
            "email": "not-an-email",
            "phone_number": "0".repeat(21),
            "team_id": team.id,
        })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"]["code"], "validation_failed");
    let fields = &json["error"]["fields"];
    assert_eq!(fields["first_name"], "required");
    assert_eq!(fields["last_name"], "too long");
    assert_eq!(fields["email"], "invalid format");
    assert_eq!(fields["phone_number"], "too long");
}

#[tokio::test]
async fn create_user_requires_a_team() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({ "first_name": "Ada", "last_name": "Lovelace", "email": "ada@test.local" })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"]["code"], "validation_failed");
    assert!(json["error"]["fields"]["body"].is_string());
}

#[tokio::test]
async fn create_user_with_unknown_team_creates_nothing() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;
    let email = format!("{}@test.local", common::unique("orphan"));

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({
            "first_name": "Ada",
            "last_name": "Lovelace",
            "email": email,
            "team_id": Uuid::new_v4(),
        })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"]["fields"]["team_id"], "not found");

    let (_, listed) = call(&db, "GET", &format!("/users?q={email}"), Some(&token), None).await;
    assert_eq!(listed["total"], 0);
}

#[tokio::test]
async fn create_user_rejects_an_email_taken_with_another_case() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (manager, team, token) = manager_with_team(&db).await;

    let (status, json) = call(
        &db,
        "POST",
        "/users",
        Some(&token),
        Some(json!({
            "first_name": "Ada",
            "last_name": "Lovelace",
            "email": manager.email.to_uppercase(),
            "team_id": team.id,
        })),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(json["error"]["code"], "email_taken");
}

// GET /users/{id}

#[tokio::test]
async fn get_unknown_or_malformed_user_is_not_found() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;

    for uri in [
        format!("/users/{}", Uuid::new_v4()),
        "/users/abc".to_string(),
    ] {
        let (status, json) = call(&db, "GET", &uri, Some(&token), None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}");
        assert_eq!(json["error"]["code"], "not_found", "{uri}");
    }
}

// GET /users

#[tokio::test]
async fn list_users_filters_by_team_role_status_and_search() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    insert_user(&db, Role::Employee, Some(team.id), "Grace", "Hopper").await;
    let gone = insert_user(&db, Role::Employee, Some(team.id), "Alan", "Turing").await;
    call(
        &db,
        "DELETE",
        &format!("/users/{}", gone.id),
        Some(&token),
        None,
    )
    .await;
    // Another team, to check that the team filter excludes it.
    let other = insert_team(&db).await;
    insert_user(&db, Role::Employee, Some(other.id), "Ada", "Lovelace").await;

    let list = |query: String| {
        let db = db.clone();
        let token = token.clone();
        async move {
            let uri = format!("/users?team_id={}{query}", team.id);
            let (status, json) = call(&db, "GET", &uri, Some(&token), None).await;
            assert_eq!(status, StatusCode::OK, "{uri}");
            json
        }
    };

    // Active members only, sorted by last name.
    let all = list(String::new()).await;
    assert_eq!(all["total"], 3);
    let last_names: Vec<&str> = all["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["last_name"].as_str().unwrap())
        .collect();
    assert_eq!(last_names, ["Hopper", "Lovelace", "Manager"]);
    assert_eq!(all["items"][0]["team"]["id"], team.id.to_string());

    assert_eq!(list("&role=employee".into()).await["total"], 2);
    assert_eq!(list("&role=manager".into()).await["total"], 1);
    assert_eq!(
        list("&is_active=false".into()).await["items"][0]["last_name"],
        "Turing"
    );

    // Case-insensitive, on the full name or the email.
    assert_eq!(list("&q=LOVELACE".into()).await["total"], 1);
    assert_eq!(list("&q=ada%20love".into()).await["total"], 1);
    assert_eq!(list(format!("&q={}", ada.email)).await["total"], 1);
    // `%` is a literal character, not a wildcard.
    assert_eq!(list("&q=%25".into()).await["total"], 0);

    let page = list("&limit=1&offset=1".into()).await;
    assert_eq!(page["total"], 3);
    assert_eq!(page["items"][0]["last_name"], "Lovelace");
}

#[tokio::test]
async fn list_users_rejects_invalid_query() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;

    for uri in [
        "/users?limit=0",
        "/users?limit=201",
        "/users?team_id=abc",
        "/users?role=boss",
    ] {
        let (status, json) = call(&db, "GET", uri, Some(&token), None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert_eq!(json["error"]["code"], "validation_failed", "{uri}");
    }
}

// PATCH /users/{id}

#[tokio::test]
async fn update_user_changes_only_sent_fields() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    let uri = format!("/users/{}", ada.id);

    let (status, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "first_name": " Augusta ", "phone_number": "0600000000", "role": "manager" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["first_name"], "Augusta");
    assert_eq!(json["last_name"], "Lovelace");
    assert_eq!(json["phone_number"], "0600000000");
    assert_eq!(json["role"], "manager");
    assert_eq!(json["team"]["id"], team.id.to_string());

    // `null` clears the phone, a missing key leaves it.
    let (_, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "phone_number": null })),
    )
    .await;
    assert_eq!(json["phone_number"], serde_json::Value::Null);
    assert_eq!(json["first_name"], "Augusta");
}

#[tokio::test]
async fn update_user_moves_the_user_to_another_team() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let other = insert_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;

    let (status, json) = call(
        &db,
        "PATCH",
        &format!("/users/{}", ada.id),
        Some(&token),
        Some(json!({ "team_id": other.id })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["team"]["id"], other.id.to_string());
    let (_, old) = call(
        &db,
        "GET",
        &format!("/users?team_id={}", team.id),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(old["total"], 1); // only the manager is left
    let (_, new) = call(
        &db,
        "GET",
        &format!("/users?team_id={}", other.id),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(new["total"], 1);
}

#[tokio::test]
async fn update_user_with_unknown_team_changes_nothing() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    let uri = format!("/users/{}", ada.id);

    let (status, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "first_name": "Changed", "team_id": Uuid::new_v4() })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"]["fields"]["team_id"], "not found");
    let (_, fetched) = call(&db, "GET", &uri, Some(&token), None).await;
    assert_eq!(fetched["first_name"], "Ada");
    assert_eq!(fetched["team"]["id"], team.id.to_string());
}

#[tokio::test]
async fn update_user_can_reactivate_an_account() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    let uri = format!("/users/{}", ada.id);
    call(&db, "DELETE", &uri, Some(&token), None).await;

    let (status, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "is_active": true })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["is_active"], true);
}

#[tokio::test]
async fn update_user_rejects_a_taken_email_and_invalid_fields() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (manager, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    let uri = format!("/users/{}", ada.id);

    let (status, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "email": manager.email })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(json["error"]["code"], "email_taken");

    let (status, json) = call(
        &db,
        "PATCH",
        &uri,
        Some(&token),
        Some(json!({ "last_name": "" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(json["error"]["fields"]["last_name"], "required");
}

#[tokio::test]
async fn update_unknown_user_is_not_found() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;

    let (status, _) = call(
        &db,
        "PATCH",
        &format!("/users/{}", Uuid::new_v4()),
        Some(&token),
        Some(json!({ "first_name": "Ada" })),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// DELETE /users/{id}

#[tokio::test]
async fn delete_user_deactivates_and_keeps_the_row() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, team, token) = manager_with_team(&db).await;
    let ada = insert_user(&db, Role::Employee, Some(team.id), "Ada", "Lovelace").await;
    let uri = format!("/users/{}", ada.id);

    let (status, json) = call(&db, "DELETE", &uri, Some(&token), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(json, serde_json::Value::Null);

    let (status, fetched) = call(&db, "GET", &uri, Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["is_active"], false);
    assert_eq!(fetched["team"]["id"], team.id.to_string());
}

#[tokio::test]
async fn delete_unknown_user_is_not_found() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let (_, _, token) = manager_with_team(&db).await;

    let (status, _) = call(
        &db,
        "DELETE",
        &format!("/users/{}", Uuid::new_v4()),
        Some(&token),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}
