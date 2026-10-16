use axum::{Router, routing::get};

use crate::{AppState, handlers::user};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::list_users).post(user::create_user))
        .route(
            "/users/{id}",
            get(user::get_user)
                .patch(user::update_user)
                .delete(user::delete_user),
        )
}
