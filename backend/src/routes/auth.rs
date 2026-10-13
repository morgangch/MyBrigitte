use axum::{Router, routing::get};

use crate::{config::Config, handlers::auth};

pub fn auth_routes() -> Router<Config> {
    Router::new().route("/me", get(auth::me))
}
