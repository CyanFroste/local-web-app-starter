
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

pub const DEFAULT_USER_AGENT: &str = "lucy/1.0";

#[derive(Debug, Clone, Deserialize)]
pub struct Request<T> {
    pub action: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    #[allow(unused_variables)]
    pub const FILE_FORMAT: &str = "%-Y-%-m-%-d-%-Hh%-Mm%-Ss";

    #[allow(unused_variables)]
    pub const DISPLAY_FORMAT: &str = "%d/%m/%Y %H:%M:%S";

    #[allow(unused_variables)]
    pub const DISPLAY_DATE_ONLY_FORMAT: &str = "%d/%m/%Y";

    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn format(&self, fmt: &str) -> String {
        self.0.with_timezone(&Local).format(fmt).to_string()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::str::FromStr for Timestamp {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        Ok(Self(
            DateTime::parse_from_rfc3339(value).map(|dt| dt.with_timezone(&Utc))?,
        ))
    }
}

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
