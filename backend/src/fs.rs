use crate::error::{Error, Result};
use reqwest::Client as HttpClient;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;
use zip::ZipArchive;

pub const CONTAINER_SEP: &str = ">";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub size: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub modified_time: u64,
}

pub fn stats(path: &Path) -> Result<Stats> {
    let meta = path.metadata()?;
    let modified_time: u64 = meta
        .modified()?
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .try_into()
        .unwrap_or_default();

    Ok(Stats {
        size: meta.len(),
        is_file: meta.is_file(),
        is_dir: meta.is_dir(),
        modified_time,
    })
}

pub async fn download_file(http_client: &HttpClient, url: &str, path: &Path) -> Result<()> {
    let res = http_client.get(url).send().await?;
    let body = res.bytes().await?;
    let mut buf = BufWriter::new(File::create(path)?);

    buf.write_all(&body)?;
    buf.flush()?;

    Ok(())
}

pub fn read_archive(path: &Path, container: &str) -> Result<Vec<String>> {
    match container {
        "zip" | "cbz" => {
            let file = File::open(path)?;
            let archive = ZipArchive::new(file)?;
            let res = archive.file_names().map(|n| n.to_string()).collect();

            Ok(res)
        }

        _ => Err(Error::new("unsupported container")),
    }
}
