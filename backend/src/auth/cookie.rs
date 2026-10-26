use axum::http::{HeaderMap, header::COOKIE};

use crate::auth::refresh::REFRESH_TOKEN_DURATION_DAYS;

pub const REFRESH_COOKIE_NAME: &str = "refresh_token";
const REFRESH_COOKIE_PATH: &str = "/api/auth";

pub fn refresh_cookie(token: &str) -> String {
    let max_age = REFRESH_TOKEN_DURATION_DAYS * 24 * 60 * 60;
    format!(
        "{REFRESH_COOKIE_NAME}={token}; HttpOnly; Secure; SameSite=Strict; Path={REFRESH_COOKIE_PATH}; Max-Age={max_age}"
    )
}
pub fn read_refresh_cookie(headers: &HeaderMap) -> Option<&str> {
    headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .find_map(|pair| {
            pair.trim()
                .strip_prefix(REFRESH_COOKIE_NAME)?
                .strip_prefix('=')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_starts_with_name_and_token() {
        assert!(refresh_cookie("abc").starts_with("refresh_token=abc;"));
    }

    #[test]
    fn cookie_has_security_attributes() {
        let cookie = refresh_cookie("abc");
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("SameSite=Strict"));
        assert!(cookie.contains("Path=/api/auth"));
    }

    #[test]
    fn cookie_lasts_seven_days() {
        assert!(refresh_cookie("abc").contains("Max-Age=604800"));
    }

    #[test]
    fn reads_refresh_token_among_other_cookies() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            "theme=dark; refresh_token=abc123; lang=fr".parse().unwrap(),
        );
        assert_eq!(read_refresh_cookie(&headers), Some("abc123"));
    }

    #[test]
    fn missing_refresh_cookie_returns_none() {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, "theme=dark".parse().unwrap());
        assert_eq!(read_refresh_cookie(&headers), None);
    }
}
