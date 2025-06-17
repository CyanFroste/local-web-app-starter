pub mod mongo;
pub mod sqlite;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUniqueIndexParams {
    pub collection: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionStats {
    pub name: String,
    pub count: u64,
}
