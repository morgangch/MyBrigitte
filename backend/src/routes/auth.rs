use axum::{Router, routing::get};

use crate::{AppState, handlers::auth};

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/me", get(auth::me))
}
