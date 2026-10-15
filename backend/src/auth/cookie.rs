use crate::auth::refresh::REFRESH_TOKEN_DURATION_DAYS;

pub const REFRESH_COOKIE_NAME: &str = "refresh_token";
const REFRESH_COOKIE_PATH: &str = "/api/auth";

pub fn refresh_cookie(token: &str) -> String {
    let max_age = REFRESH_TOKEN_DURATION_DAYS * 24 * 60 * 60;
    format!(
        "{REFRESH_COOKIE_NAME}={token}; HttpOnly; Secure; SameSite=Strict; Path={REFRESH_COOKIE_PATH}; Max-Age={max_age}"
    )
}
