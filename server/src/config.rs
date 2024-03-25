use std::{error::Error, fs, path::Path};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct APIConfig {
    pub bind: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DatabaseType {
    SQLite,
    Postgres,
    Memory
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    #[serde(rename = "type")]
    pub database_type: DatabaseType,
    pub database_uri: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub api: APIConfig,
    pub database: DatabaseConfig,
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let file_content = fs::read_to_string(Path::new("./Config.toml"))?;
    Ok(toml::from_str(&file_content)?)
}