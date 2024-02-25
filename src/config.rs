use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

impl Config {
    pub fn new(config_type: String) -> anyhow::Result<Config> {
        dotenv::from_filename(format!(".env.{}", config_type))?;
        envy::from_env().map_err(anyhow::Error::new)
    }
}
