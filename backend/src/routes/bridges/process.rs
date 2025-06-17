use crate::error::{Error, Result};
use crate::utils::BridgeRequest;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::{Value as JsonValue, from_value as from_json, to_value as to_json};
use tokio::process::Command;

pub async fn handler(Json(mut req): Json<BridgeRequest<JsonValue>>) -> Result<impl IntoResponse> {
    match req.action.as_str() {
        "open" => {
            let path: String = from_json(req.data["path"].take())?;
            let using: Option<String> = from_json(req.data["using"].take())?;

            if using.is_some() {
                // ! this inherits the parent handles and doesn't free the port
                // https://stackoverflow.com/questions/75767291
                // open::with_detached(path, using)?;
                unimplemented!();
            } else {
                open::that(path)?;
            }

            Ok(Json(JsonValue::Null))
        }

        "output" => {
            let cmd: String = from_json(req.data["cmd"].take())?;
            let args: Vec<String> = from_json(req.data["args"].take())?;
            let output = Command::new(cmd).args(args).output().await?;

            let res = Output {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status.code(),
            };

            Ok(Json(to_json(&res)?))
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
}
