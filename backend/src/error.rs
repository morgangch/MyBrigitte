use axum::{
    Json,
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{DbErr, SqlErr};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    EmailTaken,
    Conflict,
    ForeignKey,
    /// Token missing, invalid or expired, or its account no longer usable.
    Unauthorized,
    /// Wrong email or password on login.
    InvalidCredentials,
    /// Authenticated, but the role does not allow this route.
    Forbidden,
    Internal,
    ValidationFailed(HashMap<String, String>),
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
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message, fields) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not_found", "Data not found", None),
            ApiError::EmailTaken => (
                StatusCode::CONFLICT,
                "email_taken",
                "Email already taken",
                None,
            ),
            ApiError::Conflict => (
                StatusCode::CONFLICT,
                "conflict",
                "Value already taken",
                None,
            ),
            ApiError::ForeignKey => (
                StatusCode::CONFLICT,
                "foreign_key",
                "Related data is missing",
                None,
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthenticated",
                "Authentication required",
                None,
            ),
            ApiError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
                "Invalid email or password",
                None,
            ),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "Not allowed for this role",
                None,
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                "Internal server error",
                None,
            ),
            ApiError::ValidationFailed(fields) => (
                StatusCode::BAD_REQUEST,
                "validation_failed",
                "Invalid request",
                Some(fields),
            ),
        };

        let mut error = json!({ "code": code, "message": message });
        if let Some(fields) = fields {
            error["fields"] = json!(fields);
        }

        (status, Json(json!({ "error": error }))).into_response()
    }
}

/// Maps Axum's plain-text JSON rejection to the `{ "error": { code, message, fields } }` envelope.
///
/// Without this, a broken body never reaches the handler and the client cannot read `error.code`.
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        let mut fields = HashMap::new();
        fields.insert("body".to_string(), rejection.body_text());
        Self::ValidationFailed(fields)
    }
}

/// Same envelope for a query string that does not parse, such as `?limit=abc` or an invalid `team_id`.
impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        let mut fields = HashMap::new();
        fields.insert("query".to_string(), rejection.body_text());
        Self::ValidationFailed(fields)
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
                "unauthenticated",
            ),
            (
                ApiError::InvalidCredentials,
                StatusCode::UNAUTHORIZED,
                "invalid_credentials",
            ),
            (ApiError::Forbidden, StatusCode::FORBIDDEN, "forbidden"),
            (
                ApiError::Internal,
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
            ),
            (
                ApiError::ValidationFailed(HashMap::from([(
                    "email".to_string(),
                    "invalid email".to_string(),
                )])),
                StatusCode::BAD_REQUEST,
                "validation_failed",
            ),
        ];

        for (error, expected_status, expected_code) in test_cases {
            let response = error.into_response();
            assert_eq!(response.status(), expected_status);

            let bytes = response.into_body().collect().await.unwrap().to_bytes();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["error"]["code"], expected_code);
            assert!(json["error"]["message"].is_string());
            if expected_code == "validation_failed" {
                assert_eq!(json["error"]["fields"]["email"], "invalid email");
            }
        }
    }
}
