use axum::{Router, routing::get};

use crate::{config::Config, handlers::user};

pub fn user_routes() -> Router<Config> {
    Router::new()
        .route("/users", get(user::list_users))
        .route("/users/{id}", get(user::get_user))
}
