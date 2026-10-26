use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, header::SET_COOKIE},
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    AppState,
    auth::{
        cookie::{read_refresh_cookie, refresh_cookie},
        extractor::AuthUser,
    },
    error::ApiError,
    services::auth as auth_service,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn me(user: AuthUser) -> Json<Value> {
    Json(json!({ "user_id": user.user_id }))
}

/// `POST /auth/login`. An unreadable body becomes `validation_failed`, like the user handlers.
pub async fn login(
    State(state): State<AppState>,
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(body) = body?;
    let tokens =
        auth_service::login(&state.db, &body.email, &body.password, &state.jwt_secret).await?;
    Ok((
        [(SET_COOKIE, refresh_cookie(&tokens.refresh_token))],
        Json(json!({ "access_token": tokens.access_token })),
    ))
}
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let token = read_refresh_cookie(&headers).ok_or(ApiError::Unauthorized)?;
    let tokens = auth_service::refresh(&state.db, token, &state.jwt_secret).await?;
    Ok((
        [(SET_COOKIE, refresh_cookie(&tokens.refresh_token))],
        Json(json!({ "access_token": tokens.access_token })),
    ))
}
