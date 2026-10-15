use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, prelude::Uuid};

use crate::{
    auth::refresh::{REFRESH_TOKEN_DURATION_DAYS, generate_refresh_token, hash_refresh_token},
    entity::auth_session,
    error::ApiError,
};

pub async fn create_session(db: &DatabaseConnection, user_id: Uuid) -> Result<String, ApiError> {
    let token = generate_refresh_token();
    let expires_at = (Utc::now() + Duration::days(REFRESH_TOKEN_DURATION_DAYS)).fixed_offset();

    auth_session::ActiveModel {
        user_id: Set(user_id),
        refresh_token_hash: Set(hash_refresh_token(&token)),
        expires_at: Set(expires_at),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|_| ApiError::InternalError)?;

    Ok(token)
}
