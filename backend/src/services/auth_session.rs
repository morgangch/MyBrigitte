use chrono::{Duration, Utc};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    prelude::Uuid,
};

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
    .map_err(ApiError::from_db)?;

    Ok(token)
}

pub async fn find_session(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<auth_session::Model>, ApiError> {
    auth_session::Entity::find()
        .filter(auth_session::Column::RefreshTokenHash.eq(hash_refresh_token(token)))
        .one(db)
        .await
        .map_err(|_| ApiError::InternalError)
}

pub async fn revoke_session(db: &DatabaseConnection, session_id: Uuid) -> Result<bool, ApiError> {
    let result = auth_session::Entity::update_many()
        .col_expr(
            auth_session::Column::RevokedAt,
            Expr::value(Utc::now().fixed_offset()),
        )
        .filter(auth_session::Column::Id.eq(session_id))
        .filter(auth_session::Column::RevokedAt.is_null())
        .exec(db)
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(result.rows_affected == 1)
}

pub async fn revoke_all_sessions(db: &DatabaseConnection, user_id: Uuid) -> Result<(), ApiError> {
    auth_session::Entity::update_many()
        .col_expr(
            auth_session::Column::RevokedAt,
            Expr::value(Utc::now().fixed_offset()),
        )
        .filter(auth_session::Column::UserId.eq(user_id))
        .filter(auth_session::Column::RevokedAt.is_null())
        .exec(db)
        .await
        .map_err(|_| ApiError::InternalError)?;

    Ok(())
}
