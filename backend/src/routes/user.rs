use axum::{Router, routing::get};

use crate::handlers::user;

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(user::list_users))
        .route("/users/{id}", get(user::get_user))
}
