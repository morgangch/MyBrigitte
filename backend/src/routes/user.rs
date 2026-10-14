use axum::{Router, routing::get};

use crate::{AppState, handlers::user};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::list_users))
        .route("/users/{id}", get(user::get_user))
}
