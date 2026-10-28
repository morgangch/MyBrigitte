use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use std::sync::LazyLock;

static DUMMY_HASH: LazyLock<String> =
    LazyLock::new(|| hash_password("timing-equalizer").expect("argon2 must hash a constant"));

pub fn verify_dummy_password(password: &str) {
    verify_password(password, &DUMMY_HASH);
}

pub fn hash_password(password: &str) -> Result<String, String> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| error.to_string())
}
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    match PasswordHash::new(stored_hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_not_the_password() {
        let hash = hash_password("secret123").unwrap();
        assert_ne!(hash, "secret123");
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn correct_password_is_accepted() {
        let hash = hash_password("secret123").unwrap();
        assert!(verify_password("secret123", &hash));
    }

    #[test]
    fn wrong_password_is_rejected() {
        let hash = hash_password("secret123").unwrap();
        assert!(!verify_password("badpassword", &hash));
    }

    #[test]
    fn same_password_gives_different_hashes() {
        let first = hash_password("secret123").unwrap();
        let second = hash_password("secret123").unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn invalid_stored_hash_is_rejected() {
        assert!(!verify_password("secret123", "not_a_valid_hash"));
    }

    #[test]
    fn dummy_hash_uses_argon2id() {
        assert!(DUMMY_HASH.starts_with("$argon2id$"));
        verify_dummy_password("anything");
    }
}
