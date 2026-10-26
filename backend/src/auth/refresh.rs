use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use sha2::{Digest, Sha256};

pub const REFRESH_TOKEN_DURATION_DAYS: i64 = 7;

pub fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_refresh_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_has_expected_length() {
        assert_eq!(generate_refresh_token().len(), 43);
    }

    #[test]
    fn two_tokens_are_different() {
        assert_ne!(generate_refresh_token(), generate_refresh_token());
    }

    #[test]
    fn hash_is_stable_and_differs_from_token() {
        let token = generate_refresh_token();
        assert_eq!(hash_refresh_token(&token), hash_refresh_token(&token));
        assert_ne!(hash_refresh_token(&token), token);
    }
}
