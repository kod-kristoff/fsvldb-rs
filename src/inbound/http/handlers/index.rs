use std::collections::HashMap;

use axum::body::Body;
use axum::extract::Query;
use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum_template::RenderHtml;

use crate::inbound::http::error::ApiError;
use crate::inbound::http::templates::AppEngine;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct IndexParams {
    word: Option<String>,
    limit: Option<u16>,
}

pub async fn index_html(
    engine: AppEngine,
    Query(IndexParams { word, limit }): Query<IndexParams>,
) -> Result<Response<Body>, ApiError> {
    let index_ctx = IndexContext { word, limit };
    Ok(RenderHtml("index.html", engine, index_ctx).into_response())
}

#[derive(Debug, serde::Serialize)]
struct IndexContext {
    word: Option<String>,
    limit: Option<u16>,
}
