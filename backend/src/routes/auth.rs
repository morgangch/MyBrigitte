use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::{auth, microsoft},
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(auth::me))
        .route("/auth/login", post(auth::login))
        .route("/auth/refresh", post(auth::refresh))
        .route("/auth/microsoft/login", get(microsoft::login))
}
