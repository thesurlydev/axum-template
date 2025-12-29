use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError,
};

use sqlx::Error as SqlxError;
use thiserror::Error;
use tracing::error;

use crate::common::dto::RestApiResponse;

use super::dto::ApiResponse;

/// AppError is an enum that represents various types of errors that can occur in the application.
/// It implements the `std::error::Error` trait and the `axum::response::IntoResponse` trait.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError), // Used for database-related errors

    #[error("Not found: {0}")]
    NotFound(String), // Used for not found errors

    #[error("Internal server error")]
    InternalError,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Forbidden Request")]
    Forbidden,

    /// Used for authentication-related errors
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token creation error")]
    TokenCreation,
}

/// Converts the AppError enum into an HTTP response.
/// It maps the error to an appropriate HTTP status code and constructs a JSON response body.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, format!("Validation error: {msg}")),
            AppError::DatabaseError(ref db_err) => {
                // Log the full database error for debugging, but return a generic message to clients
                error!(error = %db_err, "Database error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, format!("Not found: {msg}")),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden request".to_string()),
            AppError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),
            AppError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_string()),
        };
        let body = axum::Json(ApiResponse::<()> {
            status: status.as_u16(),
            message,
            data: None,
        });

        (status, body).into_response()
    }
}

/// handle_error is a function that middlewares the error handling in the application.
/// It takes a BoxError as input and returns an HTTP response.
/// It maps the error to an appropriate HTTP status code and constructs a JSON response body.
/// The function is used to handle errors that occur during the request processing.
/// It is designed to be used with the axum framework.
pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    let status = if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    let message = error.to_string();
    error!(?status, %message, "Request failed");

    let body = RestApiResponse::<()>::failure(status.as_u16(), message);

    (status, body)
}
