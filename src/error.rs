use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use color_eyre::eyre::Error;

pub struct ApiError(Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E: Into<Error>> From<E> for ApiError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
