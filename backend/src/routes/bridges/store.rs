use crate::error::{Error, Result};
use crate::utils::{BridgeRequest, State};
use axum::Json;
use axum::extract::State as StateExtractor;
use axum::response::IntoResponse;
use serde_json::{Value as JsonValue, from_value as from_json, to_value as to_json};

pub async fn handler(
    state: StateExtractor<State>,
    Json(mut req): Json<BridgeRequest<JsonValue>>,
) -> Result<impl IntoResponse> {
    match req.action.as_str() {
        "get" => {
            let key: String = from_json(req.data["key"].take())?;
            let map = state.store.read()?;
            let value = map.get(&key);

            Ok(Json(to_json(value)?))
        }

        "set" => {
            let key: String = from_json(req.data["key"].take())?;
            let value: JsonValue = from_json(req.data["value"].take())?;

            let mut map = state.store.write()?;
            map.insert(key, value);

            Ok(Json(JsonValue::Null))
        }

        "remove" => {
            let key: String = from_json(req.data["key"].take())?;

            let removed = {
                let mut map = state.store.write()?;
                map.remove(&key)
            };

            Ok(Json(to_json(&removed)?))
        }

        "entries" => {
            let map = &*state.store.read()?;

            Ok(Json(to_json(map)?))
        }

        "keys" => {
            let map = state.store.read()?;
            let keys: Vec<_> = map.keys().collect();

            Ok(Json(to_json(&keys)?))
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}
