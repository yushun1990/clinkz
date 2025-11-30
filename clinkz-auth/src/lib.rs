mod middleware;

mod provider;
pub use provider::{AuthProvider, User};

mod oauth_server;
pub use oauth_server::OAuthServerBuilder;

mod endpoints;

mod services;

#[derive(Debug, thiserror::Error)]
#[error("Authorization error: {msg}")]
pub struct Error {
    msg: String,
}

pub type Result<T> = std::result::Result<T, Error>;
