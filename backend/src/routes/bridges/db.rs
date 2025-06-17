use crate::State;
use axum::Json;
use axum::extract::State as StateExtractor;
use axum::response::IntoResponse;
use common::db::{
    Backup, Client, CreateUniqueIndexParams, MutateItemsParams, PRIMARY_KEY, QueryItemsParams,
};
use common::{Error, Request, Result};
use serde_json::Value as JsonValue;
use serde_json::{
    from_reader as json_from_reader, from_value as from_json, to_string as json_to_string,
    to_string_pretty as json_to_pretty_string, to_value as to_json,
};
use std::fs;
use std::path::PathBuf;

pub async fn handler(
    state: StateExtractor<State>,
    Json(mut req): Json<Request<JsonValue>>,
) -> Result<impl IntoResponse> {
    match req.action.as_str() {
        "find" => {
            let params: QueryItemsParams = from_json(req.data["params"].take())?;
            let res = state.db_client.find(params).await?;

            Ok(Json(to_json(&res)?))
        }

        "add" => {
            let params: MutateItemsParams = from_json(req.data["params"].take())?;
            let res = state.db_client.add(params).await;

            Ok(Json(to_json(&res)?))
        }

        "update" => {
            let params: MutateItemsParams = from_json(req.data["params"].take())?;
            let res = state.db_client.update(params).await;

            Ok(Json(to_json(&res)?))
        }

        "remove" => {
            let params: MutateItemsParams = from_json(req.data["params"].take())?;
            let res = state.db_client.remove(params).await;

            Ok(Json(to_json(&res)?))
        }

        "stats" => {
            let res = state.db_client.stats().await?;

            Ok(Json(to_json(&res)?))
        }

        "create-unique-indexes" => {
            let params: Vec<CreateUniqueIndexParams> = from_json(req.data["params"].take())?;
            let res = state.db_client.create_unique_indexes(params).await;

            Ok(Json(to_json(&res)?))
        }

        "drop" => {
            let name: String = from_json(req.data["name"].take())?;

            state.db_client.drop(&name).await?;
            Ok(Json(JsonValue::Null))
        }

        "backup" | "backup-meta" => {
            let (path, meta_file_path) = {
                let lock = state.config.read()?;
                let path = &lock.paths.backups;

                (path.clone(), path.join(&lock.files.meta))
            };

            match req.action.as_str() {
                "backup" => {
                    backup(&state.db_client, path, meta_file_path).await?;
                    Ok(Json(JsonValue::Null))
                }
                "backup-meta" => {
                    let res = backup_meta(meta_file_path)?;

                    Ok(Json(to_json(&res)?))
                }
                _ => unreachable!(),
            }
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}

async fn backup(db_client: &Client, path: PathBuf, meta_file_path: PathBuf) -> Result<()> {
    let skippable = |name: &str| name.starts_with("cached");

    let stats = db_client
        .stats()
        .await?
        .into_iter()
        .filter(|x| !skippable(&x.name))
        .collect();

    let meta = Backup::new(stats);

    for collection in db_client.db.list_collection_names().await? {
        if skippable(&collection) {
            continue;
        }

        let mut data = db_client.find_all(&collection).await?;

        for it in data.iter_mut() {
            // removing _id key from document
            it.as_object_mut().and_then(|map| map.remove(PRIMARY_KEY));
        }

        fs::write(
            path.join(format!("{collection}.json")),
            json_to_string(&data)?,
        )?;
    }

    fs::write(meta_file_path, json_to_pretty_string(&meta)?)?;
    Ok(())
}

fn backup_meta(meta_file_path: PathBuf) -> Result<Backup> {
    let file = fs::File::open(meta_file_path)?;
    let data: Backup = json_from_reader(file)?;

    Ok(data)
}
