use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    Config(String),
    Audio(String),
    BaiduApi(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Reqwest(e) => write!(f, "网络请求错误: {}", e),
            AppError::Io(e) => write!(f, "IO错误: {}", e),
            AppError::Config(s) => write!(f, "配置错误: {}", s),
            AppError::Audio(s) => write!(f, "音频错误: {}", s),
            AppError::BaiduApi(s) => write!(f, "百度API错误: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
} 