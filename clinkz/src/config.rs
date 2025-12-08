use std::sync::Arc;

use arc_swap::ArcSwap;
use clap::{Parser, arg, command};
use config::{Environment, File};
use serde::Deserialize;


#[derive(Debug, Default, Clone, Deserialize)]
pub struct ApiConfig {
    #[serde(default)]
    pub wechat: WechatConfig,
    #[serde(default)]
    pub phoenix: PhonexConfig,
    #[serde(default)]
    pub ocr: OcrConfig,
    #[serde(default)]
    pub ragflow: RAGFlowConfig,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct WechatConfig {
    pub app_id: Option<String>,
    pub app_secret: Option<String>,
    pub host: Option<String>,
}

impl WechatConfig {
    pub fn app_id(&self) -> String {
        if let Some(app_id) = self.app_id.clone() {
            app_id
        } else {
            String::from("wx4663ffe71f9206e7")
        }
    }

    pub fn app_secret(&self) -> String {
        if let Some(app_secret) = self.app_secret.clone() {
            app_secret
        } else {
            String::from("032f93359a7a43e2c505eff037ef8601")
        }
    }

    pub fn host(&self) -> String {
        if let Some(host) = self.host.clone() {
            host
        } else {
            String::from("https://api.weixin.qq.com")
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PhonexConfig {
    pub app_id: Option<String>,
    pub app_secret: Option<String>,
    pub host: Option<String>,
}

impl PhonexConfig {
    pub fn app_id(&self) -> String {
        if let Some(app_id) = self.app_id.clone() {
            app_id
        } else {
            String::from("34zk5FL_NhoP5_0ZJVHgKJDrfQh5a6-H0sYx")
        }
    }

    pub fn app_secret(&self) -> String {
        if let Some(app_secret) = self.app_secret.clone() {
            app_secret
        } else {
            String::from(
                "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjF6VXgxUTJieFlyQTRwN3l6SzRSM1o3RUE1TTgwdWlxSUhhYyIsInRlbmFudF9pZCI6InJIbS0zdVpCOEFxSjhqRXhqbWlzeHR5WU1raDFpanAzWS1YNSIsIm5pY2tfbmFtZSI6ImFkbWluIiwicmVhbF9uYW1lIjoiYWRtaW4iLCJlbWFpbCI6bnVsbCwibW9iaWxlIjpudWxsLCJsb2NrZWQiOmZhbHNlLCJpc19hZG1pbiI6dHJ1ZSwidXBkYXRlZF9hdCI6IjIwMjUtMTEtMDdUMDI6NTk6MTYuNzIyWiIsInNpZCI6IkgtelBkLWVLY1FNNyIsImNyZWRlbnRpYWxJZCI6IllNTWhVa0ppQ2JyNHN3MlAwd19qNTB6RkNMdElaZ2VIOUdlWiIsImlhdCI6MTc2MjQ4NDUyMiwiZXhwIjozMTU3NDUyNDAwNjQyLCJpc3MiOiJwb3J0YWwtYXBpIn0.65-ZnCHBgSG4roLffiUft_ryiV8cj7KAu_R6zmcEhq8",
            )
        }
    }

    pub fn host(&self) -> String {
        if let Some(host) = self.host.clone() {
            host
        } else {
            String::from("https://inspire.wang-sheng.com")
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct OcrConfig {
    host: Option<String>,
    app_code: Option<String>,
}
impl OcrConfig {
    pub fn host(&self) -> String {
        if let Some(host) = self.host.clone() {
            host
        } else {
            String::from("https://gjbsb.market.alicloudapi.com")
        }
    }

    pub fn app_code(&self) -> String {
        if let Some(app_code) = self.app_code.clone() {
            app_code
        } else {
            String::from("976fcff8d01d481b86e507803b80e195")
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RAGFlowConfig {
    pub host: Option<String>,
    pub api_key: Option<String>,
}

impl RAGFlowConfig {
    pub fn host(&self) -> String {
        if let Some(host) = self.host.clone() {
            host
        } else {
            String::from("http://john973ax.mycloudnas.com:19080")
        }
    }

    pub fn api_key(&self) -> String {
        if let Some(api_key) = self.api_key.clone() {
            api_key
        } else {
            String::from("ragflow-KSIhqSXQun-HTAwJBig25mIoxjNTAStN-VZLonV8HuQ")
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct MinIOConfig {
    address: Option<String>,
    bucket_name: Option<String>,
    access_key: Option<String>,
    secret_key: Option<String>,
}
impl MinIOConfig {
    pub fn address(&self) -> String {
        if let Some(address) = self.address.clone() {
            address
        } else {
            String::from("http://172.16.10.56:9000")
        }
    }

    pub fn bucket_name(&self) -> String {
        if let Some(bucket_name) = self.bucket_name.clone() {
            bucket_name
        } else {
            String::from("gas-meter-ocr")
        }
    }

    pub fn access_key(&self) -> String {
        if let Some(access_key) = self.access_key.clone() {
            access_key
        } else {
            String::from("wgyfvEcN6TlQW2aFDHHU")
        }
    }

    pub fn secret_key(&self) -> String {
        if let Some(secret_key) = self.secret_key.clone() {
            secret_key
        } else {
            String::from("rJFjPIqLICH4GZ842rsoMW3C2e7todpYZKtLNFSn")
        }
    }
}

#[derive(Parser, Debug, Clone, Deserialize)]
#[command(version, about, long_about=None)]
pub struct AppConfig {
    #[arg(short, long)]
    #[serde(default)]
    pub config: Option<String>,

    #[arg(short='H', long)]
    #[serde(default)]
    pub host: Option<String>,

    #[arg(short, long)]
    #[serde(default)]
    pub port: Option<u16>,

    #[arg(short, long)]
    #[serde(default)]
    pub log_level: Option<String>,

    #[arg(skip)]
    #[serde(default)]
    pub minio: MinIOConfig,

    #[arg(skip)]
    #[serde(default)]
    pub api: ApiConfig,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig::parse().reload()
    }

    pub fn reload(&self) -> Self {
        dotenv::dotenv().ok();

        let mut builder = config::Config::builder().add_source(
            Environment::with_prefix("INSPIRE")
                .separator("_")
                .prefix_separator("__"),
        );

        if let Some(config_file) = self.config.clone() {
            builder = builder.add_source(File::with_name(config_file.as_str()));
        }

        let mut config = builder
            .build()
            .unwrap()
            .try_deserialize::<AppConfig>()
            .unwrap();

        // Cmd -> env -> config -> default
        if let Some(host) = self.host.clone() {
            config.host = Some(host);
        }

        if let Some(port) = self.port.clone() {
            config.port = Some(port);
        }

        if let Some(log_level) = self.log_level.clone() {
            config.log_level = Some(log_level);
        }

        config
    }

    pub fn host(&self) -> String {
        if let Some(host) = self.host.clone() {
            host
        } else {
            "0.0.0.0".to_string()
        }
    }

    pub fn port(&self) -> u16 {
        if let Some(port) = self.port.clone() {
            port
        } else {
            80
        }
    }

    pub fn log_level(&self) -> String {
        if let Some(log_level) = self.log_level.clone() {
            log_level
        } else {
            "info".to_string()
        }
    }
}

pub type ConfigRef = Arc<ArcSwap<AppConfig>>;
pub struct ConfigManager {
    config: ConfigRef,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: Arc::new(ArcSwap::from_pointee(AppConfig::new())),
        }
    }

    pub fn load_config(&self) -> Arc<AppConfig> {
        self.config.load().clone()
    }

    pub fn reload_config(&mut self) {
        let config = self.load_config();
        self.config.store(Arc::new(config.reload()));
    }
}
