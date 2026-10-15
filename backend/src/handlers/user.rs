use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::entity::prelude::Uuid;

use crate::{
    entity::user::Model,
    error::ApiError,
    models::user::{CreateUserRequest, UpdateUserRequest, UserResponse},
    services::user::{self, CreateUser, UpdateUser},
    state::AppState,
};

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users: Vec<Model> = user::find_all(&state.db).await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user: Model = user::find_by_id(&state.db, id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
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

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
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

pub async fn deactivate_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = user::deactivate(&state.db, id).await?;
    Ok(Json(UserResponse::from(user)))
}

// delete_user still deactivate the user
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    user::deactivate(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
