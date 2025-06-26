/*!
 * TTSmate V1 库模块
 * 
 * 提供TTSmate的核心功能模块
 */

pub mod config;
pub mod tts;
pub mod ai;
pub mod soundboard;
pub mod audio;
pub mod ui;
pub mod utils;
pub mod error;

// 重新导出主要类型
pub use config::{AppConfig, TTSConfig, AIConfig, AudioConfig, UIConfig};
pub use tts::{TTSClient, TTSError};
pub use ai::{AIContentGenerator, ContentType, GeneratedContent};
pub use soundboard::{SoundBoard, SoundEffect};
pub use audio::{AudioRouter, AudioDevice};
pub use error::AppError;

/// TTSmate版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// TTSmate应用名称
pub const APP_NAME: &str = "TTSmate V1";

/// TTSmate描述
pub const APP_DESCRIPTION: &str = "智能语音合成客户端";
