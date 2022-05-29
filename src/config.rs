use std::fs::read_to_string;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_path: String,
    pub log_level: String,
    pub sync: Sync,
}

#[derive(Deserialize)]
pub struct Sync {
    pub enabled: bool,
    pub url: String,
    pub access_token: String,
}

pub fn read_config(path: &str) -> Config {
    let text = read_to_string(path).expect("Config file not found");
    toml::from_str(&text).expect("Failed to read config")
}