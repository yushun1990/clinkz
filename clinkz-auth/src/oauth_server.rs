use std::time::Duration;

use axum::Router;

#[derive(Debug, Clone)]
pub struct OAuthServerConfig {
    pub issuer: Option<String>,
    pub access_token_ttl: Option<Duration>,
    pub refresh_token_ttl: Option<Duration>,
    pub enable_device_flow: Option<bool>,
    pub redis_conn_poll: Option<deadpool_redis::Pool>,
    pub pg_pool: Option<sqlx::PgPool>,
}

#[derive(Default, Debug, Clone)]
pub struct OAuthServer {
    pub issuer: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
    pub enable_device_flow: bool,
}

impl OAuthServer {
    pub fn router(&self) -> Router {
        Router::new()
    }
}


#[derive(Default, Debug, Clone)]
pub struct OAuthServerBuilder {
    oauth_server: OAuthServer
}

impl OAuthServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.oauth_server.issuer = issuer.into();
        self
    }

    pub fn access_token_ttl(mut self, access_token_ttl: impl Into<Duration>) -> Self {
        self.oauth_server.access_token_ttl = access_token_ttl.into();
        self
    }

    pub fn refresh_token_ttl(mut self, refresh_token_ttl: impl Into<Duration>) -> Self {
        self.oauth_server.refresh_token_ttl = refresh_token_ttl.into();
        self
    }

    pub fn enable_device_flow(mut self, enable_device_flow: bool) -> Self {
        self.oauth_server.enable_device_flow = enable_device_flow;
        self
    }

    pub fn build(self) -> OAuthServer {
        self.oauth_server
    }
}
