use sea_orm::DatabaseConnection;

use crate::config::MicrosoftConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
    pub microsoft: Option<MicrosoftConfig>,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        jwt_secret: String,
        microsoft: Option<MicrosoftConfig>,
    ) -> Self {
        Self {
            db,
            jwt_secret,
            microsoft,
        }
    }

    pub fn disconnect(jwt_secret: String) -> Self {
        Self {
            db: DatabaseConnection::default(),
            jwt_secret,
            microsoft: None,
        }
    }
}
