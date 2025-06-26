/*!
 * 应用配置定义
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use crate::error::{AppError, Result};

/// 应用程序主配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub tts: TTSConfig,
    pub ai: AIConfig,
    pub audio: AudioConfig,
    pub ui: UIConfig,
    pub keybindings: KeyBindingConfig,
}

/// TTS配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TTSConfig {
    pub server_url: String,
    pub timeout_seconds: u64,
    pub retry_count: u32,
    pub cache_enabled: bool,
    pub default_voice: String,
    pub audio_format: String,
}

/// AI配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
}

/// 音频配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub channels: u16,
    pub bit_depth: u16,
    pub master_volume: f32,
    pub enable_virtual_cable: bool,
    pub enable_voicemeeter: bool,
}

/// UI配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UIConfig {
    pub theme: String,
    pub font_size: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub auto_save: bool,
    pub show_tooltips: bool,
}

/// 快捷键配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyBindingConfig {
    pub tts_generate: String,
    pub ai_generate: String,
    pub stop_all: String,
    pub volume_up: String,
    pub volume_down: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tts: TTSConfig::default(),
            ai: AIConfig::default(),
            audio: AudioConfig::default(),
            ui: UIConfig::default(),
            keybindings: KeyBindingConfig::default(),
        }
    }
}

impl Default for TTSConfig {
    fn default() -> Self {
        Self {
            server_url: "http://192.168.11.153:8080".to_string(),
            timeout_seconds: 30,
            retry_count: 3,
            cache_enabled: true,
            default_voice: "Default".to_string(),
            audio_format: "wav".to_string(),
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.deepseek.com".to_string(),
            model: "deepseek-chat".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
            timeout_seconds: 30,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            buffer_size: 1024,
            channels: 2,
            bit_depth: 16,
            master_volume: 1.0,
            enable_virtual_cable: true,
            enable_voicemeeter: false,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14.0,
            window_width: 1200.0,
            window_height: 800.0,
            auto_save: true,
            show_tooltips: true,
        }
    }
}

impl Default for KeyBindingConfig {
    fn default() -> Self {
        Self {
            tts_generate: "F1".to_string(),
            ai_generate: "F2".to_string(),
            stop_all: "Escape".to_string(),
            volume_up: "Ctrl+Up".to_string(),
            volume_down: "Ctrl+Down".to_string(),
        }
    }
}

impl AppConfig {
    /// 加载默认配置
    pub async fn load_default() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            Self::load_from_file(&config_path).await
        } else {
            let default_config = Self::default();
            default_config.save_to_file(&config_path).await?;
            Ok(default_config)
        }
    }
    
    /// 从文件加载配置
    pub async fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: AppConfig = toml::from_str(&content)
            .map_err(|e| AppError::Config(config::ConfigError::Message(e.to_string())))?;
        Ok(config)
    }
    
    /// 保存配置到文件
    pub async fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        // 确保配置目录存在
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::Config(config::ConfigError::Message(e.to_string())))?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
    
    /// 获取配置文件路径
    pub fn get_config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| AppError::system("无法获取配置目录"))?;
        path.push("TTSmate");
        path.push("config.toml");
        Ok(path)
    }
    
    /// 获取数据目录路径
    pub fn get_data_dir() -> Result<PathBuf> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| AppError::system("无法获取数据目录"))?;
        path.push("TTSmate");
        Ok(path)
    }
    
    /// 获取缓存目录路径
    pub fn get_cache_dir() -> Result<PathBuf> {
        let mut path = dirs::cache_dir()
            .ok_or_else(|| AppError::system("无法获取缓存目录"))?;
        path.push("TTSmate");
        Ok(path)
    }
    
    /// 验证配置有效性
    pub fn validate(&self) -> Result<()> {
        // 验证TTS配置
        if self.tts.server_url.is_empty() {
            return Err(AppError::Config(config::ConfigError::Message(
                "TTS服务器URL不能为空".to_string()
            )));
        }
        
        // 验证AI配置
        if self.ai.api_key.is_empty() {
            log::warn!("AI API密钥未设置，AI功能将不可用");
        }
        
        // 验证音频配置
        if self.audio.sample_rate == 0 {
            return Err(AppError::Config(config::ConfigError::Message(
                "音频采样率不能为0".to_string()
            )));
        }
        
        Ok(())
    }
    
    /// 获取超时时间
    pub fn get_tts_timeout(&self) -> Duration {
        Duration::from_secs(self.tts.timeout_seconds)
    }
    
    /// 获取AI超时时间
    pub fn get_ai_timeout(&self) -> Duration {
        Duration::from_secs(self.ai.timeout_seconds)
    }
}
