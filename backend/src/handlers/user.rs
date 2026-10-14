use axum::{Json, extract::Path};

use crate::{error::ApiError, models::user::User, services::user as user_service};

pub async fn list_users() -> Result<Json<Vec<User>>, ApiError> {
    user_service::list_users().map(Json)
}

pub async fn get_user(Path(id): Path<u32>) -> Result<Json<User>, ApiError> {
    user_service::get_user(id).map(Json)
}
