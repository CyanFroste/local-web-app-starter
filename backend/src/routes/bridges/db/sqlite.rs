use crate::db::CreateUniqueIndexParams;
use crate::db::sqlite::Client;
use crate::error::{Error, Result};
use crate::utils::{BridgeRequest, State};
use axum::Json;
use axum::extract::State as StateExtractor;
use axum::response::IntoResponse;
use serde_json::Value as JsonValue;
use serde_json::{from_value as from_json, to_value as to_json};

pub async fn handler(
    mut state: StateExtractor<State>,
    Json(mut req): Json<BridgeRequest<JsonValue>>,
) -> Result<impl IntoResponse> {
    match (&state.sqlite_client, req.action.as_str()) {
        (None, "connect") => {
            let path = { state.config.read()?.db.sqlite.path.clone() };
            let client = Client::new(&path).await?;

            state.sqlite_client.replace(client);
            Ok(Json(JsonValue::Null))
        }

        (None, _) => Err(Error::new("not connected")),

        (Some(_), "connect") => Err(Error::new("already connected")),

        (Some(client), "execute") => {
            let sql: String = from_json(req.data["sql"].take())?;
            let res = client.execute(&sql).await?;

            Ok(Json(to_json(&res)?))
        }

        (Some(client), "fetch") => {
            let sql: String = from_json(req.data["sql"].take())?;
            let res = client.fetch(&sql).await?;

            Ok(Json(to_json(&res)?))
        }

        (Some(client), "stats") => {
            let res = client.stats().await?;

            Ok(Json(to_json(&res)?))
        }

        (Some(client), "create-unique-indexes") => {
            let params: Vec<CreateUniqueIndexParams> = from_json(req.data["params"].take())?;
            let res = client.create_unique_indexes(params).await;

            Ok(Json(to_json(&res)?))
        }

        (Some(client), "drop") => {
            let name: String = from_json(req.data["name"].take())?;

            client.drop(&name).await?;
            Ok(Json(JsonValue::Null))
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}
