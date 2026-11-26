use reqwest::Url;

use crate::config::MicrosoftConfig;

pub const LOGIN_COOKIE_NAME: &str = "microsoft_login";
const LOGIN_COOKIE_PATH: &str = "/auth/microsoft";
const LOGIN_COOKIE_MAX_AGE_SECONDS: u32 = 600;

pub fn login_cookie(state: &str, nonce: &str) -> String {
    format!(
        "{LOGIN_COOKIE_NAME}={state}.{nonce}; HttpOnly; Secure; SameSite=Lax; Path={LOGIN_COOKIE_PATH}; Max-Age={LOGIN_COOKIE_MAX_AGE_SECONDS}"
    )
}

pub fn authorize_url(config: &MicrosoftConfig, state: &str, nonce: &str) -> Url {
    let endpoint = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
        config.tenant_id
    );
    Url::parse_with_params(
        &endpoint,
        &[
            ("client_id", config.client_id.as_str()),
            ("response_type", "code"),
            ("redirect_uri", config.redirect_uri.as_str()),
            ("response_mode", "query"),
            ("scope", "openid profile email"),
            ("state", state),
            ("nonce", nonce),
        ],
    )
    .expect("the Microsoft authorize endpoint is a valid URL")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn test_config() -> MicrosoftConfig {
        MicrosoftConfig {
            client_id: "client-123".to_string(),
            client_secret: "super-secret-value".to_string(),
            tenant_id: "tenant-456".to_string(),
            redirect_uri: "http://localhost:3001/auth/microsoft/callback".to_string(),
        }
    }

    #[test]
    fn authorize_url_targets_the_tenant_endpoint() {
        let url = authorize_url(&test_config(), "state-abc", "nonce-xyz");
        assert_eq!(url.host_str(), Some("login.microsoftonline.com"));
        assert_eq!(url.path(), "/tenant-456/oauth2/v2.0/authorize");
    }

    #[test]
    fn authorize_url_carries_the_expected_parameters() {
        let url = authorize_url(&test_config(), "state-abc", "nonce-xyz");
        let params: HashMap<String, String> = url.query_pairs().into_owned().collect();
        assert_eq!(params["client_id"], "client-123");
        assert_eq!(params["response_type"], "code");
        assert_eq!(
            params["redirect_uri"],
            "http://localhost:3001/auth/microsoft/callback"
        );
        assert_eq!(params["scope"], "openid profile email");
        assert_eq!(params["state"], "state-abc");
        assert_eq!(params["nonce"], "nonce-xyz");
    }

    #[test]
    fn authorize_url_never_contains_the_client_secret() {
        let url = authorize_url(&test_config(), "state-abc", "nonce-xyz");
        assert!(!url.as_str().contains("super-secret-value"));
    }
}
