/*!
 * 日志工具
 */

use log::info;

/// 初始化日志系统
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("日志系统初始化完成");
    Ok(())
}
