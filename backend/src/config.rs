use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub microsoft: Option<MicrosoftConfig>,
}
#[derive(Clone, Debug)]
pub struct MicrosoftConfig {
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Load .env variables
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL doit être définie")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".into())
                .parse()
                .map_err(|_| "PORT doit être un nombre valide")?,
            jwt_secret: env::var("JWT_SECRET").map_err(|_| "JWT_SECRET doit être définie")?,
            microsoft: MicrosoftConfig::from_env(),
        })
    }
}

impl MicrosoftConfig {
    fn from_env() -> Option<Self> {
        Some(Self {
            client_id: non_empty_var("MICROSOFT_APP_ID")?,
            client_secret: non_empty_var("MICROSOFT_SECRET")?,
            tenant_id: non_empty_var("MICROSOFT_TENANT_ID")?,
            redirect_uri: non_empty_var("MICROSOFT_REDIRECT_URI")?,
        })
    }
}

fn non_empty_var(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}
