pub mod user;

use axum::{Router, routing::get};

use crate::{AppState, handlers::health::health_check};

pub fn create_app() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .merge(user::user_routes())
}
