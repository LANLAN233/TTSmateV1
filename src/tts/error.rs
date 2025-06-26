/*!
 * TTS错误类型定义
 */

use thiserror::Error;

/// TTS客户端错误类型
#[derive(Error, Debug)]
pub enum TTSError {
    #[error("网络连接错误: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("服务器错误: {status_code} - {message}")]
    ServerError { status_code: u16, message: String },
    
    #[error("音频格式错误: {0}")]
    AudioFormatError(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("缓存错误: {0}")]
    CacheError(String),
    
    #[error("超时错误")]
    TimeoutError,
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("编码错误: {0}")]
    EncodingError(String),
    
    #[error("Gradio API错误: {0}")]
    GradioError(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl TTSError {
    /// 创建服务器错误
    pub fn server_error(status_code: u16, message: String) -> Self {
        Self::ServerError { status_code, message }
    }
    
    /// 创建音频格式错误
    pub fn audio_format<S: Into<String>>(msg: S) -> Self {
        Self::AudioFormatError(msg.into())
    }
    
    /// 创建配置错误
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::ConfigError(msg.into())
    }
    
    /// 创建缓存错误
    pub fn cache<S: Into<String>>(msg: S) -> Self {
        Self::CacheError(msg.into())
    }
    
    /// 创建解析错误
    pub fn parse<S: Into<String>>(msg: S) -> Self {
        Self::ParseError(msg.into())
    }
    
    /// 创建编码错误
    pub fn encoding<S: Into<String>>(msg: S) -> Self {
        Self::EncodingError(msg.into())
    }
    
    /// 创建Gradio API错误
    pub fn gradio<S: Into<String>>(msg: S) -> Self {
        Self::GradioError(msg.into())
    }
    
    /// 创建未知错误
    pub fn unknown<S: Into<String>>(msg: S) -> Self {
        Self::Unknown(msg.into())
    }
    
    /// 检查是否为网络相关错误
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::NetworkError(_) | Self::TimeoutError)
    }
    
    /// 检查是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::ServerError { .. })
    }
    
    /// 检查是否可以重试
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::NetworkError(_) | Self::TimeoutError => true,
            Self::ServerError { status_code, .. } => {
                // 5xx错误可以重试，4xx错误通常不可重试
                *status_code >= 500
            }
            _ => false,
        }
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::NetworkError(_) | Self::TimeoutError => ErrorSeverity::Medium,
            Self::ServerError { status_code, .. } => {
                if *status_code >= 500 {
                    ErrorSeverity::High
                } else {
                    ErrorSeverity::Medium
                }
            }
            Self::AudioFormatError(_) | Self::ConfigError(_) => ErrorSeverity::High,
            Self::CacheError(_) => ErrorSeverity::Low,
            Self::ParseError(_) | Self::EncodingError(_) => ErrorSeverity::Medium,
            Self::GradioError(_) => ErrorSeverity::High,
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

/// TTS结果类型
pub type TTSResult<T> = std::result::Result<T, TTSError>;
