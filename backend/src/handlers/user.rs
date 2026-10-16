use axum::{
    Json,
    extract::rejection::JsonRejection,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::entity::prelude::Uuid;
use std::collections::HashMap;

use crate::{
    entity::user::Model,
    error::ApiError,
    models::{
        Page,
        user::{CreateUserRequest, ListUsersQuery, UpdateUserRequest, UserResponse},
    },
    services::user::{self, CreateUser, UpdateUser},
    state::AppState,
};

/// `docs/API.md`: a list `limit` defaults to 50 and cannot exceed 200.
const DEFAULT_PAGE_LIMIT: u64 = 50;
const MAX_PAGE_LIMIT: u64 = 200;

/// `GET /users`.
///
/// Missing `limit` becomes 50, missing `offset` becomes 0. Missing `is_active` means `true`,
/// so deactivated accounts stay out of the list unless the client asks for them.
/// `total` counts the whole filtered set, not just this page.
pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Page<UserResponse>>, ApiError> {
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_LIMIT);
    let offset = query.offset.unwrap_or(0);

    if !(1..=MAX_PAGE_LIMIT).contains(&limit) {
        let mut fields = HashMap::new();
        fields.insert("limit".to_string(), "must be between 1 and 200".to_string());
        return Err(ApiError::ValidationFailed(fields));
    }

    let listed = user::find_page(&state.db, limit, offset, query.is_active.unwrap_or(true)).await?;

    Ok(Json(Page {
        items: listed.items.into_iter().map(UserResponse::from).collect(),
        total: listed.total,
    }))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user: Model = user::find_by_id(&state.db, id).await?;
    Ok(Json(UserResponse::from(user)))
}

/// `POST /users`. An unreadable body is `validation_failed` instead of Axum's plain-text 422.
pub async fn create_user(
    State(state): State<AppState>,
    body: Result<Json<CreateUserRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let Json(body) = body?;
    let created: Model = user::create(
        &state.db,
        CreateUser {
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            phone_number: body.phone_number,
            password: body.password,
            role: body.role.into(),
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(created))))
}

/// `PATCH /users/{id}`. Same JSON rejection handling as [`create_user`].
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    body: Result<Json<UpdateUserRequest>, JsonRejection>,
) -> Result<Json<UserResponse>, ApiError> {
    let Json(body) = body?;
    let updated: Model = user::update(
        &state.db,
        id,
        UpdateUser {
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            phone_number: body.phone_number,
            password: body.password,
            role: body.role.map(Into::into),
        },
    )
    .await?;

    Ok(Json(UserResponse::from(updated)))
}

/// `DELETE /users/{id}`. Deactivates the account and returns 204. The row is not removed.
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    user::deactivate(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
