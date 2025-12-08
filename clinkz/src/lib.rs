mod error;
mod config;
pub use config::{AppConfig, ConfigManager};

mod app;
pub use app::run;
use mimalloc::MiMalloc;
use once_cell::sync::Lazy;
use tracing_subscriber::{fmt::Layer as FmtLayer, layer::SubscriberExt, reload::{self, Handle}, util::SubscriberInitExt, EnvFilter, Registry};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static TRACING: Lazy<Handle<EnvFilter, Registry>> = Lazy::new(|| {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let (layer, handle) = reload::Layer::new(filter);
    let subscriber = Registry::default().with(layer).with(FmtLayer::default());
    subscriber.init();

    handle
});

pub fn reload_log_level(level: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let new_level = level.parse::<tracing::Level>()?;
    let new_filter = EnvFilter::new(format!("{new_level}"));

    TRACING.reload(new_filter)?;

    tracing::info!("Log level reloaded to: {new_level}");
    Ok(())
}
