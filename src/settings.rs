use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Discord {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub level: String,
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub discord: Discord,
    pub logging: Logging,
}

pub fn read() -> Result<Settings, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name("config/default").required(false))?;
    s.merge(File::with_name("production").required(false))?;
    s.try_into()
}
