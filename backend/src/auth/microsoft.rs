use reqwest::Url;

use crate::config::MicrosoftConfig;

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
