use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, handlers::auth};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(auth::me))
        .route("/auth/login", post(auth::login))
}
