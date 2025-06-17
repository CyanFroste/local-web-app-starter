use crate::db::{CollectionStats, CreateUniqueIndexParams};
use crate::error::{Error, Result};
use serde::Serialize;
use serde_json::from_str as json_from_str;
use serde_json::{Map as JsonMap, Number as JsonNumber, Value as JsonValue};
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{Column, Pool, Row, Sqlite, SqlitePool};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Client {
    pub pool: Pool<Sqlite>,
}

impl Client {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;

        Ok(Self { pool })
    }

    pub async fn execute(&self, sql: &str) -> Result<ExecutionResult> {
        let res = sqlx::query(sql).execute(&self.pool).await?;

        Ok(ExecutionResult {
            rows_affected: res.rows_affected(),
            last_insert_row: res.last_insert_rowid(),
        })
    }

    pub async fn fetch(&self, sql: &str) -> Result<Vec<JsonValue>> {
        sqlx::query(sql)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect()
    }

    pub async fn drop(&self, name: &str) -> Result<()> {
        self.execute(&format!("DROP TABLE IF EXISTS {name}"))
            .await?;

        Ok(())
    }

    pub async fn stats(&self) -> Result<Vec<CollectionStats>> {
        let mut res = vec![];

        for it in self
            .fetch(
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
            )
            .await?
        {
            if let Some(name) = it["name"].as_str().map(|it| it.to_string()) {
                let count = self
                    .fetch(&format!("SELECT COUNT(*) FROM {name}"))
                    .await?
                    .first()
                    .and_then(|it| it.as_u64())
                    .unwrap_or_default();

                res.push(CollectionStats { name, count });
            }
        }

        Ok(res)
    }

    pub async fn create_unique_indexes(&self, params: Vec<CreateUniqueIndexParams>) -> Vec<String> {
        let mut res = Vec::with_capacity(params.len());

        for CreateUniqueIndexParams { collection, fields } in params {
            let index_name = fields.join("_") + "_unique_index";
            let sql = format!(
                "CREATE UNIQUE INDEX IF NOT EXISTS {index_name} ON {collection} ({})",
                fields.join(", ")
            );

            if self.execute(&sql).await.is_ok() {
                res.push(format!("{collection}: {index_name}"));
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Text(String),
    Integer(i64),
    Real(f64),
    Boolean(u8),
}

impl fmt::Display for SqlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Text(v) => write!(f, "'{v}'"),
            Self::Integer(v) => write!(f, "{v}"),
            Self::Real(v) => write!(f, "{v}"),
            Self::Boolean(v) => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResult {
    pub rows_affected: u64,
    pub last_insert_row: i64,
}

trait SqliteRowExt {
    fn to_json(&self) -> Result<JsonValue>;
}

impl SqliteRowExt for SqliteRow {
    fn to_json(&self) -> Result<JsonValue> {
        let mut map = JsonMap::new();

        for col in self.columns() {
            let key = col.name().to_string();

            let value = match col.type_info().to_string().as_str() {
                "NULL" => JsonValue::Null,
                "TEXT" => {
                    let s: String = self.get(key.as_str());
                    if s.starts_with("```json") && s.ends_with("```") {
                        json_from_str(&s[7..s.len() - 3]).unwrap_or_else(|_| JsonValue::String(s))
                    } else {
                        JsonValue::String(s)
                    }
                }
                "INTEGER" => {
                    let n: i64 = self.get(key.as_str());
                    JsonValue::Number(JsonNumber::from(n))
                }
                "REAL" => {
                    let n = JsonNumber::from_f64(self.get(key.as_str()))
                        .ok_or_else(|| Error::new("invalid number"))?;
                    JsonValue::Number(n)
                }
                "BOOLEAN" => JsonValue::Bool(self.get(key.as_str())),
                _ => return Err(Error::new("invalid type")),
            };

            map.insert(key, value);
        }

        Ok(JsonValue::Object(map))
    }
}
