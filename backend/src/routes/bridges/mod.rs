pub mod asset;
pub mod config;
pub mod db;
pub mod fs;
pub mod process;
pub mod proxy;
pub mod store;

use crate::utils::State;
use axum::Router;
use axum::routing::{any, post};

pub fn router() -> Router<State> {
    Router::new()
        .route("/config", post(config::handler))
        .route("/fs", post(fs::handler))
        .route("/db/mongo", post(db::mongo::handler))
        .route("/db/sqlite", post(db::sqlite::handler))
        .route("/store", post(store::handler))
        .route("/process", post(process::handler))
        .route("/proxy/{*url}", any(proxy::handler))
        .route("/asset/{*path}", any(asset::handler))
}
