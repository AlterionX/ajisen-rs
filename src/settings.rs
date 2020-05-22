use config::{Config, ConfigError, Environment, File};
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
    s.merge(Environment::with_prefix("ajisen"))?;
    s.try_into()
}
