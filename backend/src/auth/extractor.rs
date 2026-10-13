use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{auth::jwt::verify_token, config::Config, error::ApiError};

pub struct AuthUser {
    pub user_id: String,
}

impl FromRequestParts<Config> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        config: &Config,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(ApiError::Unauthorized)?;

        let claims = verify_token(token, &config.jwt_secret).ok_or(ApiError::Unauthorized)?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::create_token;
    use axum::http::Request;

    const SECRET: &str = "secret-de-test-assez-long-pour-hs256-0123456789";

    fn test_config() -> Config {
        Config {
            database_url: "postgresql://test".to_string(),
            port: 0,
            jwt_secret: SECRET.to_string(),
        }
    }

    async fn extract(header: Option<String>) -> Result<AuthUser, ApiError> {
        let mut builder = Request::builder();
        if let Some(value) = header {
            builder = builder.header(AUTHORIZATION, value);
        }
        let (mut parts, _) = builder.body(()).unwrap().into_parts();
        AuthUser::from_request_parts(&mut parts, &test_config()).await
    }

    #[tokio::test]
    async fn valid_token_is_accepted() {
        let token = create_token("42", SECRET).unwrap();
        let user = extract(Some(format!("Bearer {token}"))).await.unwrap();
        assert_eq!(user.user_id, "42");
    }

    #[tokio::test]
    async fn missing_header_is_rejected() {
        assert!(matches!(extract(None).await, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn missing_bearer_prefix_is_rejected() {
        let token = create_token("42", SECRET).unwrap();
        assert!(matches!(
            extract(Some(token)).await,
            Err(ApiError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn invalid_token_is_rejected() {
        let header = Some("Bearer pas-un-jwt".to_string());
        assert!(matches!(extract(header).await, Err(ApiError::Unauthorized)));
    }
}
