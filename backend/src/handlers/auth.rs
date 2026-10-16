use axum::{Json, extract::State, extract::rejection::JsonRejection};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{AppState, auth::extractor::AuthUser, error::ApiError, services::auth as auth_service};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn me(user: AuthUser) -> Json<Value> {
    Json(json!({ "user_id": user.user_id }))
}

pub async fn login(
    State(state): State<AppState>,
    body: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Json<Value>, ApiError> {
    let Json(body) = body?;
    let token =
        auth_service::login(&state.db, &body.email, &body.password, &state.jwt_secret).await?;
    Ok(Json(json!({ "access_token": token })))
}
