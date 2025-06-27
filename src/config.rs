use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_keys: ApiKeys,
    pub app_settings: AppSettings,
}

#[derive(Deserialize, Debug)]
pub struct ApiKeys {
    pub deepseek_api_key: String,
    pub baidu_api_key: String,
    pub baidu_secret_key: String,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub speed: i32,
    pub pitch: i32,
    pub volume: i32,
    pub person: i32,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
} 