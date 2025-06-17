use crate::config::Config;
use crate::db::mongo::Client as MongoClient;
use crate::db::sqlite::Client as SqliteClient;
use crate::error::{Error, Result};
use chrono::{DateTime, Local, Utc};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct State {
    pub http_client: HttpClient,
    pub config: Arc<RwLock<Config>>,
    pub store: Arc<RwLock<BTreeMap<String, JsonValue>>>,
    pub mongo_client: Option<MongoClient>,
    pub sqlite_client: Option<SqliteClient>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BridgeRequest<T> {
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
