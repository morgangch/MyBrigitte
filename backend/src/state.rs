use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub fn disconnect(jwt_secret: String) -> Self {
        Self {
            db: DatabaseConnection::default(),
            jwt_secret,
        }
    }
}
