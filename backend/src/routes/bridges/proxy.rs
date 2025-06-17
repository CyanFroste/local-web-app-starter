use crate::error::Result;
use crate::utils::State;
use axum::body::{Body, Bytes};
use axum::extract::{Path, State as StateExtractor};
use axum::http::{HeaderMap, Method, Uri, header};
use axum::response::IntoResponse;

pub async fn handler(
    state: StateExtractor<State>,
    method: Method,
    uri: Uri,
    Path(mut url): Path<String>,
    mut headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse> {
    if let Some(query) = uri.query() {
        url.push('?');
        url.push_str(query);
    }

    let req = state.http_client.request(method, &url);

    // ! THIS IS TRIAL AND ERROR
    // this is needed for some reason
    headers.remove(header::USER_AGENT);
    headers.remove(header::HOST);
    headers.remove(header::CONTENT_LENGTH);
    headers.remove(header::CONNECTION);
    headers.remove(header::REFERER);
    headers.remove(header::PROXY_AUTHENTICATE);
    headers.remove(header::PROXY_AUTHORIZATION);
    headers.remove(header::TE);
    headers.remove(header::TRAILER);
    headers.remove(header::TRANSFER_ENCODING);
    headers.remove(header::UPGRADE);

    let res = req.headers(headers).body(body).send().await?;

    Ok((
        res.status(),
        res.headers().clone(),
        Body::from_stream(res.bytes_stream()),
    ))
}
