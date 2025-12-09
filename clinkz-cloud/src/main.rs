use axum::{http::StatusCode, routing::get, Json, Router};
use clinkz::ConfigManager;
use clinkz_macro::http_status;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let cm = ConfigManager::new();
    let config = cm.load_config();

    clinkz::reload_log_level(config.log_level().as_str()).unwrap();

    let router = Router::new()
        .route("/badrequest", get(bad_request))
        .route("/othererror", get(other_error))
        .with_state(config.clone())
        .layer(TraceLayer::new_for_http());

    clinkz::run(cm, router).await;
}

#[derive(Debug, thiserror::Error)]
#[error("Bad request!")]
#[http_status(StatusCode::BAD_REQUEST)]
struct BadRequest;

#[derive(Debug, thiserror::Error)]
#[error("Other error")]
struct OtherError;

pub async fn bad_request() -> clinkz_core::Result<Json<String>> {
    Err(BadRequest)?
}

pub async fn other_error() -> clinkz_core::Result<Json<String>> {
    Err(anyhow::Error::from(OtherError))?
}
