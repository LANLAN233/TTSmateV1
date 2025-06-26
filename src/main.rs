/*!
 * TTSmate V1 - 智能语音合成客户端
 * 
 * 主要功能：
 * - TTS语音合成
 * - AI文案生成 (DeepSeek)
 * - 音效板
 * - 虚拟声卡集成
 * 
 * 作者：TTSmate Team
 * 版本：1.0.0
 */

use std::error::Error;
use log::{info, error};

mod config;
mod tts;
mod ai;
mod soundboard;
mod audio;
mod ui;
mod utils;
mod error;

use crate::error::AppError;
use crate::config::AppConfig;
use crate::ui::TTSmateApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志系统
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("TTSmate V1 启动中...");

    // 加载配置
    let config = match AppConfig::load_default().await {
        Ok(config) => {
            info!("配置加载成功");
            config
        }
        Err(e) => {
            error!("配置加载失败: {}", e);
            info!("使用默认配置");
            AppConfig::default()
        }
    };

    // 创建应用实例
    let app = TTSmateApp::new(config).await?;

    // 启动GUI应用
    info!("启动用户界面...");
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "TTSmate V1 - 智能语音合成客户端",
        options,
        Box::new(|_cc| Box::new(app)),
    ).map_err(|e| AppError::UI(format!("GUI启动失败: {}", e)))?;

    info!("TTSmate V1 已退出");
    Ok(())
}
