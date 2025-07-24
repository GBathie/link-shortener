use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::core::{
    models::{id::ShortLinkId, url::LongUrl},
    ports::service::LinkShortenerService,
};

pub fn build_app<S: LinkShortenerService>(service: S) -> Router {
    let service = Arc::new(RwLock::new(service));
    let app = Router::new()
        .route("/s/{id}", get(access_link))
        .route("/shorten", post(shorten_link))
        .with_state(service);

    app
}

#[tracing::instrument(skip(service), level = "debug")]
async fn access_link<S: LinkShortenerService + 'static>(
    Path(id): Path<String>,
    State(service): State<Arc<RwLock<S>>>,
) -> Result<Redirect, StatusCode> {
    debug!("Accessing shortlink {id}");
    let short_id = ShortLinkId::new(id);
    match service.read().await.access(short_id).await {
        Ok(Some(url)) => Ok(Redirect::permanent(url.url())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("{e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
struct ShortenLinkParams {
    target_url: String,
}

#[derive(Debug, Serialize)]
struct ShortenLinkResult {
    id: String,
}

#[tracing::instrument(skip(service), level = "debug")]
async fn shorten_link<S: LinkShortenerService + 'static>(
    State(service): State<Arc<RwLock<S>>>,
    Json(params): Json<ShortenLinkParams>,
) -> Result<Json<ShortenLinkResult>, StatusCode> {
    debug!("Shortening url {}", params.target_url);
    match service
        .write()
        .await
        .create(LongUrl::new(params.target_url))
        .await
    {
        Ok(id) => Ok(Json(ShortenLinkResult { id: id.into_id() })),
        Err(e) => {
            error!("{e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
