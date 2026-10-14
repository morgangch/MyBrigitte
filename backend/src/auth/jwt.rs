use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const ACCESS_TOKEN_DURATION_SECONDS: usize = 15 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
pub fn create_token(user_id: &str, secret: &str) -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + ACCESS_TOKEN_DURATION_SECONDS,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| error.to_string())
}
pub fn verify_token(token: &str, secret: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims)
}
#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "secret-de-test-assez-long-pour-hs256-0123456789";

    #[test]
    fn valid_token_is_accepted() {
        let token = create_token("42", SECRET).unwrap();
        let claims = verify_token(&token, SECRET).unwrap();
        assert_eq!(claims.sub, "42");
    }

    #[test]
    fn token_with_wrong_secret_is_rejected() {
        let token = create_token("42", SECRET).unwrap();
        assert!(verify_token(&token, "un-autre-secret-tout-aussi-long-0123456789").is_none());
    }

    #[test]
    fn forged_token_is_rejected() {
        let token_42 = create_token("42", SECRET).unwrap();
        let token_1 = create_token("1", SECRET).unwrap();
        let parts_42: Vec<&str> = token_42.split('.').collect();
        let parts_1: Vec<&str> = token_1.split('.').collect();
        let forged = format!("{}.{}.{}", parts_42[0], parts_1[1], parts_42[2]);
        assert!(verify_token(&forged, SECRET).is_none());
    }

    #[test]
    fn expired_token_is_rejected() {
        let claims = Claims {
            sub: "42".to_string(),
            iat: 0,
            exp: 1,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_bytes()),
        )
        .unwrap();
        assert!(verify_token(&token, SECRET).is_none());
    }

    #[test]
    fn garbage_token_is_rejected() {
        assert!(verify_token("pas-un-jwt", SECRET).is_none());
    }
}
