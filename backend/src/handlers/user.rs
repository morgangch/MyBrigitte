//! `/users` routes, all reserved to managers. [`ManagerUser`] is the first extractor of each
//! handler, so an anonymous or employee request is rejected before its body or query is read.

use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::entity::prelude::Uuid;
use std::collections::HashMap;

use crate::{
    auth::extractor::ManagerUser,
    error::ApiError,
    models::{
        Page,
        user::{CreateUserRequest, ListUsersQuery, UpdateUserRequest, UserResponse},
    },
    services::user::{self, CreateUser, UpdateUser, UserFilter},
    state::AppState,
};

/// `docs/API.md`: a list `limit` defaults to 50 and cannot exceed 200.
const DEFAULT_PAGE_LIMIT: u64 = 50;
const MAX_PAGE_LIMIT: u64 = 200;

/// A malformed id in the path is `404`, like an id that does not exist.
fn user_id(path: Result<Path<Uuid>, PathRejection>) -> Result<Uuid, ApiError> {
    path.map(|Path(id)| id).map_err(|_| ApiError::NotFound)
}

/// `GET /users`.
///
/// Missing `limit` becomes 50, missing `offset` becomes 0. Missing `is_active` means `true`,
/// so deactivated accounts stay out of the list unless the client asks for them.
/// `total` counts the whole filtered set, not just this page.
pub async fn list_users(
    _manager: ManagerUser,
    State(state): State<AppState>,
    query: Result<Query<ListUsersQuery>, QueryRejection>,
) -> Result<Json<Page<UserResponse>>, ApiError> {
    let Query(query) = query?;
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_LIMIT);
    let offset = query.offset.unwrap_or(0);

    if !(1..=MAX_PAGE_LIMIT).contains(&limit) {
        let mut fields = HashMap::new();
        fields.insert("limit".to_string(), "must be between 1 and 200".to_string());
        return Err(ApiError::ValidationFailed(fields));
    }

    let listed = user::find_page(
        &state.db,
        limit,
        offset,
        UserFilter {
            is_active: query.is_active.unwrap_or(true),
            team_id: query.team_id,
            role: query.role.map(Into::into),
            q: query.q,
        },
    )
    .await?;

    Ok(Json(Page {
        items: listed.items.into_iter().map(UserResponse::from).collect(),
        total: listed.total,
    }))
}

pub async fn get_user(
    _manager: ManagerUser,
    State(state): State<AppState>,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<Json<UserResponse>, ApiError> {
    let found = user::find_by_id(&state.db, user_id(path)?).await?;
    Ok(Json(UserResponse::from(found)))
}

/// `POST /users`. An unreadable body is `validation_failed` instead of Axum's plain-text 422.
pub async fn create_user(
    _manager: ManagerUser,
    State(state): State<AppState>,
    body: Result<Json<CreateUserRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let Json(body) = body?;
    let created = user::create(
        &state.db,
        CreateUser {
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            phone_number: body.phone_number,
            team_id: body.team_id,
            role: body.role.into(),
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(created))))
}

/// `PATCH /users/{id}`. Same JSON rejection handling as [`create_user`].
pub async fn update_user(
    _manager: ManagerUser,
    State(state): State<AppState>,
    path: Result<Path<Uuid>, PathRejection>,
    body: Result<Json<UpdateUserRequest>, JsonRejection>,
) -> Result<Json<UserResponse>, ApiError> {
    let id = user_id(path)?;
    let Json(body) = body?;
    let updated = user::update(
        &state.db,
        id,
        UpdateUser {
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            phone_number: body.phone_number,
            role: body.role.map(Into::into),
            team_id: body.team_id,
            is_active: body.is_active,
        },
    )
    .await?;

    Ok(Json(UserResponse::from(updated)))
}

/// `DELETE /users/{id}`. Deactivates the account and returns 204. The row is not removed.
pub async fn delete_user(
    _manager: ManagerUser,
    State(state): State<AppState>,
    path: Result<Path<Uuid>, PathRejection>,
) -> Result<StatusCode, ApiError> {
    user::deactivate(&state.db, user_id(path)?).await?;
    Ok(StatusCode::NO_CONTENT)
}
