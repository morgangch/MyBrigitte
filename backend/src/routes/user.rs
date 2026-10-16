use axum::{Router, routing::get};

use crate::{AppState, handlers::user};

/// User routes. `DELETE /users/{id}` deactivates the account; there is no separate deactivate route.
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
