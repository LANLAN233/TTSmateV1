/*!
 * 配置管理器
 */

use crate::config::AppConfig;
use crate::error::Result;

/// 配置管理器
pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, config: AppConfig) {
        self.config = config;
    }

    /// 保存配置
    pub async fn save(&self) -> Result<()> {
        let config_path = AppConfig::get_config_path()?;
        self.config.save_to_file(&config_path).await
    }

    /// 重新加载配置
    pub async fn reload(&mut self) -> Result<()> {
        self.config = AppConfig::load_default().await?;
        Ok(())
    }
}
