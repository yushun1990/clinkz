use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub trait HttpStatus {
    fn status_code(&self) -> StatusCode;
}

impl HttpStatus for anyhow::Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Error {
    pub status_code: StatusCode,
    pub error: anyhow::Error,
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
    E: HttpStatus,
{
    fn from(error: E) -> Self {
        Self {
            // 如果实现了 HttpStatus 调用 status_code 方法， 否则直接使用500
            status_code: error.status_code(),
            error: error.into(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = json!({
            "message": self.error.to_string()
        });

        (self.status_code, Json(body)).into_response()
    }
}
