use axum::response::Html;

pub async fn health_check() -> Html<&'static str> {
    Html("OK")
}
