use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_into_response() {
        let test_cases = vec![
            (ApiError::NotFound, StatusCode::NOT_FOUND),
            (ApiError::InternalError, StatusCode::INTERNAL_SERVER_ERROR),
        ];

        for (error, expected_status) in test_cases {
            let response = error.into_response();
            assert_eq!(response.status(), expected_status);
        }
    }
}
