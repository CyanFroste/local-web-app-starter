use crate::error::{Error, Result};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

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
