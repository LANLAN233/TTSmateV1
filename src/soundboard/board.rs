/*!
 * 音效板实现
 */

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error, debug};
use uuid::Uuid;

use crate::soundboard::sound::SoundEffect;
use crate::soundboard::keybinding::KeyCode;
use crate::audio::AudioPlayer;
use crate::error::{AppError, Result};

/// 音效板
pub struct SoundBoard {
    sounds: HashMap<String, SoundEffect>,
    categories: HashMap<String, SoundCategory>,
    keybindings: HashMap<KeyCode, String>,
    config: SoundBoardConfig,
    player: Arc<Mutex<AudioPlayer>>,
}

/// 音效分类
#[derive(Debug, Clone)]
pub struct SoundCategory {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
}

/// 音效板配置
#[derive(Debug, Clone)]
pub struct SoundBoardConfig {
    pub master_volume: f32,
    pub fade_duration: std::time::Duration,
    pub max_concurrent_sounds: usize,
    pub default_category: String,
}

impl Default for SoundBoardConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            fade_duration: std::time::Duration::from_millis(100),
            max_concurrent_sounds: 5,
            default_category: "默认".to_string(),
        }
    }
}

impl SoundBoard {
    /// 创建新的音效板
    pub fn new() -> Result<Self> {
        info!("初始化音效板");

        let mut categories = HashMap::new();

        // 添加默认分类
        let default_category = SoundCategory {
            id: Uuid::new_v4().to_string(),
            name: "默认".to_string(),
            color: "#4CAF50".to_string(),
            icon: None,
        };
        categories.insert(default_category.id.clone(), default_category);

        // 添加常用分类
        let reaction_category = SoundCategory {
            id: Uuid::new_v4().to_string(),
            name: "反应".to_string(),
            color: "#2196F3".to_string(),
            icon: Some("😄".to_string()),
        };
        categories.insert(reaction_category.id.clone(), reaction_category);

        let music_category = SoundCategory {
            id: Uuid::new_v4().to_string(),
            name: "音乐".to_string(),
            color: "#FF9800".to_string(),
            icon: Some("🎵".to_string()),
        };
        categories.insert(music_category.id.clone(), music_category);

        // 创建音频播放器
        let player = AudioPlayer::new()
            .map_err(|e| AppError::soundboard(format!("无法创建音频播放器: {}", e)))?;

        Ok(Self {
            sounds: HashMap::new(),
            categories,
            keybindings: HashMap::new(),
            config: SoundBoardConfig::default(),
            player: Arc::new(Mutex::new(player)),
        })
    }

    /// 添加音效文件
    pub fn add_sound(
        &mut self,
        file_path: &Path,
        name: &str,
        category: &str,
    ) -> Result<String> {
        info!("添加音效: {} -> {}", name, file_path.display());

        // 验证文件存在
        if !file_path.exists() {
            return Err(AppError::soundboard(format!("音效文件不存在: {}", file_path.display())));
        }

        // 验证文件格式
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !matches!(extension.as_str(), "wav" | "mp3" | "ogg" | "flac") {
            return Err(AppError::soundboard(format!("不支持的音频格式: {}", extension)));
        }

        // 获取或创建分类
        let category_id = self.get_or_create_category(category);

        // 创建音效
        let sound_id = Uuid::new_v4().to_string();
        let sound_effect = SoundEffect {
            id: sound_id.clone(),
            name: name.to_string(),
            file_path: file_path.to_path_buf(),
            category: category_id,
            volume: 1.0,
            duration: std::time::Duration::from_secs(0), // TODO: 从文件获取实际时长
            created_at: chrono::Utc::now(),
        };

        self.sounds.insert(sound_id.clone(), sound_effect);
        info!("音效添加成功: {}", sound_id);

        Ok(sound_id)
    }

    /// 播放音效
    pub async fn play_sound(&self, sound_id: &str) -> Result<()> {
        debug!("播放音效: {}", sound_id);

        let sound = self.sounds.get(sound_id)
            .ok_or_else(|| AppError::soundboard("音效不存在"))?;

        info!("播放音效: {} ({})", sound.name, sound.file_path.display());

        // 使用音频播放器播放文件
        let player = self.player.lock().await;
        player.play_file(&sound.file_path).await
            .map_err(|e| AppError::soundboard(format!("播放音效失败: {}", e)))?;

        Ok(())
    }

    /// 停止音效
    pub async fn stop_sound(&self, sound_id: &str) -> Result<()> {
        debug!("停止音效: {}", sound_id);

        let _sound = self.sounds.get(sound_id)
            .ok_or_else(|| AppError::soundboard("音效不存在"))?;

        info!("停止音效: {}", sound_id);

        // 停止当前播放
        let player = self.player.lock().await;
        player.stop().await;

        Ok(())
    }

    /// 停止所有音效
    pub async fn stop_all_sounds(&self) {
        info!("停止所有音效");

        let player = self.player.lock().await;
        player.stop().await;
    }

    /// 获取音效列表
    pub fn get_sounds(&self, category: Option<&str>) -> Vec<&SoundEffect> {
        if let Some(cat) = category {
            self.sounds.values()
                .filter(|sound| {
                    self.categories.get(&sound.category)
                        .map(|c| c.name == cat)
                        .unwrap_or(false)
                })
                .collect()
        } else {
            self.sounds.values().collect()
        }
    }

    /// 删除音效
    pub fn remove_sound(&mut self, sound_id: &str) -> Result<()> {
        info!("删除音效: {}", sound_id);

        self.sounds.remove(sound_id)
            .ok_or_else(|| AppError::soundboard("音效不存在"))?;

        // 移除相关的快捷键绑定
        self.keybindings.retain(|_, id| id != sound_id);

        Ok(())
    }

    /// 绑定快捷键
    pub fn bind_key(&mut self, key: KeyCode, sound_id: &str) -> Result<()> {
        debug!("绑定快捷键: {:?} -> {}", key, sound_id);

        // 验证音效存在
        if !self.sounds.contains_key(sound_id) {
            return Err(AppError::soundboard("音效不存在"));
        }

        // 检查快捷键冲突
        if let Some(existing_sound) = self.keybindings.get(&key) {
            warn!("快捷键 {:?} 已绑定到音效 {}", key, existing_sound);
        }

        self.keybindings.insert(key, sound_id.to_string());
        info!("快捷键绑定成功: {:?} -> {}", key, sound_id);

        Ok(())
    }

    /// 解除快捷键绑定
    pub fn unbind_key(&mut self, key: KeyCode) -> Result<()> {
        debug!("解除快捷键绑定: {:?}", key);

        self.keybindings.remove(&key)
            .ok_or_else(|| AppError::soundboard("快捷键未绑定"))?;

        Ok(())
    }

    /// 获取快捷键绑定
    pub fn get_keybindings(&self) -> &HashMap<KeyCode, String> {
        &self.keybindings
    }

    /// 处理按键事件
    pub async fn handle_key_event(&self, key: KeyCode) -> Result<()> {
        if let Some(sound_id) = self.keybindings.get(&key) {
            self.play_sound(sound_id).await?;
        }
        Ok(())
    }

    /// 获取分类列表
    pub fn get_categories(&self) -> Vec<&SoundCategory> {
        self.categories.values().collect()
    }

    /// 获取或创建分类
    fn get_or_create_category(&mut self, category_name: &str) -> String {
        // 查找现有分类
        for category in self.categories.values() {
            if category.name == category_name {
                return category.id.clone();
            }
        }

        // 创建新分类
        let category_id = Uuid::new_v4().to_string();
        let category = SoundCategory {
            id: category_id.clone(),
            name: category_name.to_string(),
            color: "#9E9E9E".to_string(), // 默认灰色
            icon: None,
        };

        self.categories.insert(category_id.clone(), category);
        info!("创建新分类: {}", category_name);

        category_id
    }

    /// 设置主音量
    pub fn set_master_volume(&mut self, volume: f32) {
        self.config.master_volume = volume.clamp(0.0, 2.0);
        info!("设置主音量: {}", self.config.master_volume);
    }

    /// 获取主音量
    pub fn get_master_volume(&self) -> f32 {
        self.config.master_volume
    }

    /// 获取音效统计信息
    pub fn get_stats(&self) -> SoundBoardStats {
        let total_sounds = self.sounds.len();
        let total_categories = self.categories.len();
        let total_keybindings = self.keybindings.len();

        let total_duration: std::time::Duration = self.sounds.values()
            .map(|sound| sound.duration)
            .sum();

        SoundBoardStats {
            total_sounds,
            total_categories,
            total_keybindings,
            total_duration,
        }
    }
}

/// 音效板统计信息
#[derive(Debug, Clone)]
pub struct SoundBoardStats {
    pub total_sounds: usize,
    pub total_categories: usize,
    pub total_keybindings: usize,
    pub total_duration: std::time::Duration,
}
