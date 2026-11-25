use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, header::SET_COOKIE},
    response::IntoResponse,
};
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde_json::json;

use crate::{
    AppState,
    auth::{
        cookie::{read_refresh_cookie, refresh_cookie},
        extractor::AuthUser,
        jwt::ACCESS_TOKEN_DURATION_SECONDS,
    },
    error::ApiError,
    models::user::{MeResponse, UserResponse},
    services::{
        auth::{self as auth_service, LoginTokens},
        user as user_service,
    },
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// `GET /me`. A deleted or deactivated account is treated as signed out.
pub async fn me(
    user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<MeResponse>, ApiError> {
    let id = Uuid::parse_str(&user.user_id).map_err(|_| ApiError::Unauthorized)?;
    let found = user_service::find_by_id(&state.db, id)
        .await
        .map_err(|error| match error {
            ApiError::NotFound => ApiError::Unauthorized,
            other => other,
        })?;
    if !found.user.is_active {
        return Err(ApiError::Unauthorized);
    }
    let has_password = found.user.password_hash.is_some();
    Ok(Json(MeResponse {
        user: UserResponse::from(found),
        has_password,
    }))
}

/// `POST /auth/login`. An unreadable body becomes `validation_failed`, like the user handlers.
pub async fn login(
    State(state): State<AppState>,
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(body) = body?;
    let tokens =
        auth_service::login(&state.db, &body.email, &body.password, &state.jwt_secret).await?;
    Ok(token_response(tokens))
}

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let token = read_refresh_cookie(&headers).ok_or(ApiError::Unauthorized)?;
    let tokens = auth_service::refresh(&state.db, token, &state.jwt_secret).await?;
    Ok(token_response(tokens))
}

fn token_response(tokens: LoginTokens) -> impl IntoResponse {
    (
        [(SET_COOKIE, refresh_cookie(&tokens.refresh_token))],
        Json(json!({
            "access_token": tokens.access_token,
            "token_type": "Bearer",
            "expires_in": ACCESS_TOKEN_DURATION_SECONDS,
        })),
    )
}
