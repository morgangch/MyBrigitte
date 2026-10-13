pub mod auth;
pub mod user;

use axum::{Router, routing::get};

use crate::{config::Config, handlers::health::health_check};

pub fn create_app(config: Config) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .merge(user::user_routes())
        .merge(auth::auth_routes())
        .with_state(config)
}
