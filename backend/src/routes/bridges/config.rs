use crate::error::{Error, Result};
use crate::utils::{BridgeRequest, State};
use axum::Json;
use axum::extract::State as StateExtractor;
use axum::response::IntoResponse;
use serde_json::{Value as JsonValue, from_str as json_from_str};

pub async fn handler(
    state: StateExtractor<State>,
    Json(req): Json<BridgeRequest<JsonValue>>,
) -> Result<impl IntoResponse> {
    match req.action.as_str() {
        "get" => {
            let config = { state.config.read()?.meta.contents.clone() };
            let res: JsonValue = json_from_str(&config)?;

            Ok(Json(res))
        }

        _ => Err(Error::new(format!("invalid action: {}", req.action))),
    }
}
