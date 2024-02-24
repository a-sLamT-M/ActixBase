use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

impl Config {
    pub fn new(config_type: String) -> envy::Result<Config> {
        dotenv::from_filename(format!(".env.{}", config_type))?;
        envy::from_env::<Config>()
    }
}
