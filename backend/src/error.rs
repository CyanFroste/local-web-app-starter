use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use serde::Serialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl<T: std::error::Error> From<T> for Error {
    fn from(value: T) -> Self {
        Self::new(value.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

#[derive(Debug)]
struct ErrorContainer(pub Error);

impl std::error::Error for ErrorContainer {}

impl From<Error> for BoxError {
    fn from(value: Error) -> Self {
        Box::new(ErrorContainer(value))
    }
}

impl fmt::Display for ErrorContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
