use std::{convert::Infallible, sync::Arc};

use axum::{extract::rejection::{FormRejection, JsonRejection}, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(unused)]
pub enum Error {
    #[error("SUCCESS")]
    Success,
    #[error(transparent)]
    BadRequestError(#[from] BadRequestError),
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    #[error("No authorization: {0}")]
    AuthorizationError(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String
        }
        let  (status, message, err) = match &self {
            Error::Success => {
                (StatusCode::OK, self.to_string(), None)
            },
            Error::BadRequestError(_) => {
                (StatusCode::BAD_REQUEST, self.to_string(), None)
            },
            Error::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, self.to_string(), None)
            },
            Error::AuthorizationError(_) => {
                (StatusCode::FORBIDDEN, self.to_string(), None)
            },
            Self::Other(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string(), Some(self))
            }
        };

        let mut response = (status, axum::Json(ErrorResponse{message})).into_response();
        if let Some(err) = err {
            // Insert the error into the response for logging middleware to use.
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::Success
    }
}

#[derive(Error, Debug)]
pub enum BadRequestError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    FormRejectionError(#[from] FormRejection),

    #[error(transparent)]
    JsonRejectionError(#[from] JsonRejection),
}
