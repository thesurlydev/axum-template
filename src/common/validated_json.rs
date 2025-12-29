//! Custom JSON extractor with automatic validation support.
//!
//! This module provides `ValidatedJson<T>`, an extractor that combines JSON
//! deserialization with automatic validation using the `validator` crate.

use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use super::dto::ApiResponse;

/// A JSON extractor that automatically validates the deserialized data.
///
/// This extractor deserializes the request body as JSON and then validates
/// it using the `validator` crate. If either deserialization or validation
/// fails, an appropriate error response is returned.
///
/// # Example
///
/// ```ignore
/// use validator::Validate;
///
/// #[derive(Deserialize, Validate)]
/// struct CreateUser {
///     #[validate(length(min = 1, max = 64))]
///     username: String,
///     #[validate(email)]
///     email: String,
/// }
///
/// async fn create_user(
///     ValidatedJson(payload): ValidatedJson<CreateUser>,
/// ) -> impl IntoResponse {
///     // payload is already validated
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

/// Error type for ValidatedJson extraction failures.
#[derive(Debug, Error)]
pub enum ValidatedJsonRejection {
    #[error("JSON parsing error: {0}")]
    JsonRejection(#[from] JsonRejection),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ValidatedJsonRejection::JsonRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            ValidatedJsonRejection::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone())
            }
        };

        let body = axum::Json(ApiResponse::<()> {
            status: status.as_u16(),
            message,
            data: None,
        });

        (status, body).into_response()
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidatedJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;

        value
            .validate()
            .map_err(|e| ValidatedJsonRejection::ValidationError(format!("Invalid input: {e}")))?;

        Ok(ValidatedJson(value))
    }
}
