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
        .route("/demo", get(demo))
        .with_state(config.clone())
        .layer(TraceLayer::new_for_http());

    clinkz::run(cm, router).await;
}

#[derive(Debug, thiserror::Error)]
#[error("Bad request!")]
#[http_status(StatusCode::BAD_REQUEST)]
struct BadRequest;


pub async fn demo() -> clinkz_core::Result<Json<String>> {
    Err(BadRequest.into())
}
