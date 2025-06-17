pub mod config;
pub mod db;
pub mod error;
pub mod fs;
pub mod utils;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Request<T> {
    pub action: String,
    pub data: T,
}
