use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_cors_origin")]
    pub cors_origin: String,
}

fn default_host() -> String {
    "0.0.0.0".into()
}

fn default_port() -> u16 {
    8090
}

fn default_cors_origin() -> String {
    "http://localhost:3000".into()
}

impl ServerConfig {
    pub fn load() -> Result<Self> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::with_prefix("PODMESH").separator("__"))
            .set_default("host", default_host())?
            .set_default("port", default_port() as i64)?
            .set_default("cors_origin", default_cors_origin())?
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}
