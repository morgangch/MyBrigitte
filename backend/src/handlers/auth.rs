use axum::Json;
use serde_json::{Value, json};

use crate::auth::extractor::AuthUser;

pub async fn me(user: AuthUser) -> Json<Value> {
    Json(json!({ "user_id": user.user_id }))
}
