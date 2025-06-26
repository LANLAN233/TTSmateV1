/*!
 * TTSmate错误类型定义
 */

use thiserror::Error;

/// TTSmate应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("TTS错误: {0}")]
    TTS(#[from] crate::tts::TTSError),
    
    #[error("AI服务错误: {0}")]
    AI(String),
    
    #[error("音频错误: {0}")]
    Audio(String),
    
    #[error("音效板错误: {0}")]
    SoundBoard(String),
    
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("IO错误: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("加密错误: {0}")]
    Encryption(String),
    
    #[error("UI错误: {0}")]
    UI(String),
    
    #[error("系统错误: {0}")]
    System(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// TTSmate结果类型
pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    /// 创建AI错误
    pub fn ai<S: Into<String>>(msg: S) -> Self {
        Self::AI(msg.into())
    }
    
    /// 创建音频错误
    pub fn audio<S: Into<String>>(msg: S) -> Self {
        Self::Audio(msg.into())
    }
    
    /// 创建音效板错误
    pub fn soundboard<S: Into<String>>(msg: S) -> Self {
        Self::SoundBoard(msg.into())
    }
    
    /// 创建加密错误
    pub fn encryption<S: Into<String>>(msg: S) -> Self {
        Self::Encryption(msg.into())
    }
    
    /// 创建UI错误
    pub fn ui<S: Into<String>>(msg: S) -> Self {
        Self::UI(msg.into())
    }
    
    /// 创建系统错误
    pub fn system<S: Into<String>>(msg: S) -> Self {
        Self::System(msg.into())
    }
    
    /// 创建未知错误
    pub fn unknown<S: Into<String>>(msg: S) -> Self {
        Self::Unknown(msg.into())
    }
    
    /// 检查是否为网络相关错误
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::Network(_) | Self::TTS(crate::tts::TTSError::NetworkError(_)))
    }
    
    /// 检查是否为配置相关错误
    pub fn is_config_error(&self) -> bool {
        matches!(self, Self::Config(_))
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::TTS(_) | Self::AI(_) => ErrorSeverity::High,
            Self::Audio(_) | Self::SoundBoard(_) => ErrorSeverity::Medium,
            Self::Config(_) | Self::Database(_) => ErrorSeverity::High,
            Self::Network(_) => ErrorSeverity::Medium,
            Self::IO(_) | Self::Serialization(_) => ErrorSeverity::Low,
            Self::Encryption(_) | Self::System(_) => ErrorSeverity::High,
            Self::UI(_) => ErrorSeverity::Low,
            Self::Unknown(_) => ErrorSeverity::Medium,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl ErrorSeverity {
    /// 获取严重程度的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "低",
            Self::Medium => "中",
            Self::High => "高",
            Self::Critical => "严重",
        }
    }
}
