use crate::error::{Error, Result};
use crate::fs::{download_file, read_archive, stats};
use crate::utils::{BridgeRequest, State};
use axum::Json;
use axum::extract::State as StateExtractor;
use axum::response::IntoResponse;
use serde_json::{Value as JsonValue, from_value as from_json, to_value as to_json};
use std::fs;
use std::path::PathBuf;

pub async fn handler(
    state: StateExtractor<State>,
    Json(mut req): Json<BridgeRequest<JsonValue>>,
) -> Result<impl IntoResponse> {
    match req.action.as_str() {
        "stats" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let res = stats(&path)?;

            Ok(Json(to_json(&res)?))
        }

        "read-dir" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let mut res = vec![];

            for entry in fs::read_dir(path)? {
                res.push(entry?.file_name().to_string_lossy().to_string());
            }

            Ok(Json(to_json(&res)?))
        }

        "create-dir" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let recursive: bool = from_json(req.data["recursive"].take()).unwrap_or(false);

            if recursive {
                fs::create_dir_all(path)?
            } else {
                fs::create_dir(path)?
            }

            Ok(Json(JsonValue::Null))
        }

        "rename" => {
            let src: PathBuf = from_json(req.data["src"].take())?;
            let dst: PathBuf = from_json(req.data["dst"].take())?;

            fs::rename(src, dst)?;
            Ok(Json(JsonValue::Null))
        }

        "remove" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let recursive: bool = from_json(req.data["recursive"].take()).unwrap_or(false);

            if recursive {
                fs::remove_dir_all(path)?
            } else {
                fs::remove_file(path)?
            }

            Ok(Json(JsonValue::Null))
        }

        "copy-file" => {
            let src: PathBuf = from_json(req.data["src"].take())?;
            let dst: PathBuf = from_json(req.data["dst"].take())?;

            fs::copy(src, dst)?;
            Ok(Json(JsonValue::Null))
        }

        "read-text-file" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let res = fs::read_to_string(path)?;

            Ok(Json(to_json(&res)?))
        }

        "write-text-file" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let data: String = from_json(req.data["data"].take())?;

            fs::write(path, data)?;
            Ok(Json(JsonValue::Null))
        }

        "download-file" => {
            let url: String = from_json(req.data["url"].take())?;
            let path: PathBuf = from_json(req.data["path"].take())?;

            download_file(&state.http_client, &url, &path).await?;
            Ok(Json(JsonValue::Null))
        }

        "read-archive" => {
            let path: PathBuf = from_json(req.data["path"].take())?;
            let container: String = from_json(req.data["container"].take())?;
            let res = read_archive(&path, &container)?;

            Ok(Json(to_json(&res)?))
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}
