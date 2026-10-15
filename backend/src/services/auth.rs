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
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::Unauthorized)?;

    let hash = user
        .password_hash
        .as_deref()
        .ok_or(ApiError::Unauthorized)?;

    if !user.is_active || !verify_password(password, hash) {
        return Err(ApiError::Unauthorized);
    }

    let access_token =
        create_token(&user.id.to_string(), jwt_secret).map_err(|_| ApiError::InternalError)?;
    let refresh_token = auth_session::create_session(db, user.id).await?;

    Ok(LoginTokens {
        access_token,
        refresh_token,
    })
}
