use crate::error::{Error, Result};
use crate::fs::CONTAINER_SEP;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use bytes::Bytes;
use serde_json::Value as JsonValue;
use std::io::Read;
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::io::ReaderStream;
use zip::ZipArchive;

pub async fn handler(
    Path(path): Path<String>,
    Query(query): Query<JsonValue>,
) -> Result<impl IntoResponse> {
    match query["container"].as_str() {
        Some("zip" | "cbz") => {
            let (container_path, file_name) = path
                .split_once(CONTAINER_SEP)
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .ok_or_else(|| Error::new(format!("invalid path: {}", path)))?;

            let container = std::fs::File::open(&container_path)?;
            let (tx, rx) = mpsc::channel::<Result<Bytes>>(10);

            spawn_blocking(move || {
                if let Err(err) = (|| {
                    let mut archive = ZipArchive::new(container)?;
                    let mut file = archive.by_name(&file_name)?;
                    let mut buf = [0u8; 8192];

                    loop {
                        let read = file.read(&mut buf)?;
                        if read == 0 {
                            break;
                        }

                        let bytes = Bytes::copy_from_slice(&buf[..read]);
                        if tx.blocking_send(Ok(bytes)).is_err() {
                            break;
                        }
                    }

                    Ok(())
                })() {
                    let _ = tx.blocking_send(Err(err));
                }
            });

            let stream = ReceiverStream::new(rx);
            Ok(Body::from_stream(stream))
        }
        _ => {
            let file = tokio::fs::File::open(&path).await?;
            let stream = ReaderStream::new(file);

            Ok(Body::from_stream(stream))
        }
    }
}
