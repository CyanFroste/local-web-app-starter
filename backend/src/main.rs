pub mod config;
pub mod db;
pub mod error;
pub mod fs;
pub mod routes;
pub mod utils;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::routes::bridges;
use crate::utils::State;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use reqwest::Client as HttpClient;
use std::collections::BTreeMap;
use std::env;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::find("config.json", 5)?;
    let port = config.port;
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let http_client = HttpClient::builder().user_agent(user_agent).build()?;

    let config = Arc::new(RwLock::new(config));
    let store = Arc::new(RwLock::new(BTreeMap::new()));

    let state = State {
        http_client,
        config,
        store,
        mongo_client: None,
        sqlite_client: None,
    };

    let exe_path = env::current_exe()?
        .parent()
        .ok_or_else(|| Error::new("failed to get exe path"))?
        .to_path_buf();

    let static_service = ServeDir::new(exe_path.join("client"))
        .fallback(ServeFile::new(exe_path.join("client").join("index.html")));

    let url = format!("http://localhost:{port}");
    println!("[backend] running on {url}");

    axum::serve(
        listener,
        Router::new()
            .nest("/bridges", bridges::router())
            .fallback_service(static_service)
            .layer(CorsLayer::permissive())
            .layer(DefaultBodyLimit::disable())
            .with_state(state),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
}
