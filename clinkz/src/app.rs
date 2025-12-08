use std::net::SocketAddr;

use axum::Router;
use tracing::{error, info};

use crate::{ConfigManager, reload_log_level};

pub async fn run(cm: ConfigManager, router: Router) {
    let config = cm.load_config();
    let addr: SocketAddr = format!("{}:{}", config.host(), config.port())
        .parse()
        .expect("Invalid socket binding!");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(cm))
        .await
        .unwrap();

    info!("Service stopped!")
}

async fn shutdown_signal(mut cm: ConfigManager) {
    use tokio::signal::unix::{SignalKind, signal};

    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Received `Ctrl+C`, shutting down...");
                break;
            }
            _ = sigterm.recv() => {
                info!("Received `SIGTERM`, shutting down...");
                break;
            }
            _ = sighup.recv() => {
                info!("Received `SIGHUP`, reloading configuration...");

                cm.reload_config();
                let config = cm.load_config();
                if let Err(e) = reload_log_level(config.log_level().as_str()) {
                    error!("Failed reload log level: {e}");
                }

                info!("Config reload successfully!");
            }
        }
    }
}
