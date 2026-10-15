use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{DbErr, SqlErr};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    EmailTaken,
    Conflict,
    ForeignKey,
    Unauthorized,
    Internal,
}

impl ApiError {
    /// Logs the driver error, then returns a stable code the client can handle.
    pub fn from_db(error: DbErr) -> Self {
        eprintln!("database error: {error}");

        match error.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(constraint)) if constraint.contains("email") => {
                Self::EmailTaken
            }
            Some(SqlErr::UniqueConstraintViolation(_)) => Self::Conflict,
            Some(SqlErr::ForeignKeyConstraintViolation(_)) => Self::ForeignKey,
            _ => Self::Internal,
        }
    }
    // Returns the http code and response
    fn status_code_and_body(&self) -> (StatusCode, &'static str, &'static str) {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", "Data not found"),
            ApiError::EmailTaken => (StatusCode::CONFLICT, "email_taken", "Email already taken"),
            ApiError::Conflict => (StatusCode::CONFLICT, "conflict", "Value already taken"),
            ApiError::ForeignKey => (
                StatusCode::CONFLICT,
                "foreign_key",
                "Related data is missing",
            ),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", "Unauthorized"),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "Internal server error",
            ),
        }
    }
}
// Returns the response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = self.status_code_and_body();
        let body = Json(json!({
            "code": code,
            "message": message
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn check_api_errors_and_responses() {
        let test_cases = [
            (ApiError::NotFound, StatusCode::NOT_FOUND, "not_found"),
            (ApiError::EmailTaken, StatusCode::CONFLICT, "email_taken"),
            (ApiError::Conflict, StatusCode::CONFLICT, "conflict"),
            (ApiError::ForeignKey, StatusCode::CONFLICT, "foreign_key"),
            (
                ApiError::Unauthorized,
                StatusCode::UNAUTHORIZED,
                "unauthorized",
            ),
            (
                ApiError::Internal,
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
            ),
        ];

        for (error, expected_status, expected_code) in test_cases {
            let response = error.into_response();
            assert_eq!(response.status(), expected_status);

            let bytes = response.into_body().collect().await.unwrap().to_bytes();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["code"], expected_code);
            assert!(json["message"].is_string());
        }
    }
}
