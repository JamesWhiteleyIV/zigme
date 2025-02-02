use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Custom error for any errors in endpoints
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = format!("Something went wrong: {}", self.0);
        tracing::error!(message);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>`
// to turn them into `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
