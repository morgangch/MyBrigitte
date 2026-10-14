use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
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
        })
    }
}
