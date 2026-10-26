use chrono::Utc;

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, sea_query::Expr};

use crate::{
    auth::{jwt::create_token, password::verify_password},
    entity::user,
    error::ApiError,
    services::auth_session,
};

pub struct LoginTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn login(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
    jwt_secret: &str,
) -> Result<LoginTokens, ApiError> {
    let user = user::Entity::find()
        .filter(Expr::cust_with_values("lower(email) = lower($1)", [email]))
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or(ApiError::Unauthorized)?;

    let hash = user
        .password_hash
        .as_deref()
        .ok_or(ApiError::Unauthorized)?;

    if !user.is_active || !verify_password(password, hash) {
        return Err(ApiError::Unauthorized);
    }

    let access_token =
        create_token(&user.id.to_string(), jwt_secret).map_err(|_| ApiError::Internal)?;
    let refresh_token = auth_session::create_session(db, user.id).await?;

    Ok(LoginTokens {
        access_token,
        refresh_token,
    })
}
pub async fn refresh(
    db: &DatabaseConnection,
    token: &str,
    jwt_secret: &str,
) -> Result<LoginTokens, ApiError> {
    let session = auth_session::find_session(db, token)
        .await?
        .ok_or(ApiError::Unauthorized)?;

    if session.revoked_at.is_some() {
        auth_session::revoke_all_sessions(db, session.user_id).await?;
        return Err(ApiError::Unauthorized);
    }

    if session.expires_at < Utc::now() {
        return Err(ApiError::Unauthorized);
    }

    if !auth_session::revoke_session(db, session.id).await? {
        return Err(ApiError::Unauthorized);
    }

    let user = user::Entity::find_by_id(session.user_id)
        .one(db)
        .await
        .map_err(ApiError::from_db)?
        .ok_or(ApiError::Unauthorized)?;

    if !user.is_active {
        return Err(ApiError::Unauthorized);
    }

    let access_token =
        create_token(&user.id.to_string(), jwt_secret).map_err(|_| ApiError::Internal)?;
    let refresh_token = auth_session::create_session(db, user.id).await?;

    Ok(LoginTokens {
        access_token,
        refresh_token,
    })
}
