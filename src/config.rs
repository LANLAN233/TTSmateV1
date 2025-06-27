use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct ApiKeys {
    pub deepseek_api_key: String,
    pub baidu_api_key: String,
    pub baidu_secret_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
}

#[derive(Deserialize, Debug)]
pub struct AiSettings {
    pub default_prompt: String,
    pub prompts: Vec<PromptTemplate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SoundboardItem {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub speed: i32,
    pub pitch: i32,
    pub volume: i32,
    pub person: i32,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_keys: ApiKeys,
    pub app_settings: AppSettings,
    pub ai_settings: AiSettings,
    #[serde(default)]
    pub soundboard: Vec<SoundboardItem>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

pub const VOICES: [(&str, i32); 11] = [
    ("度小美 (女声)", 0),
    ("度小宇 (男声)", 1),
    ("度逍遥 (基础)", 3),
    ("度丫丫 (女声)", 4),
    ("度小娇 (女声)", 5),
    ("度博文 (男声)", 106),
    ("度小童 (女声)", 110),
    ("度小萌 (女声)", 111),
    ("度米朵 (女声)", 103),
    ("度逍遥 (精品)", 5003),
    ("度小鹿 (精品)", 5118),
]; 