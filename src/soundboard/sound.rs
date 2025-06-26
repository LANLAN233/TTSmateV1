/*!
 * 音效定义
 */

use std::path::PathBuf;
use std::time::Duration;
use chrono::{DateTime, Utc};

/// 音效
#[derive(Debug, Clone)]
pub struct SoundEffect {
    pub id: String,
    pub name: String,
    pub file_path: PathBuf,
    pub category: String,
    pub volume: f32,
    pub duration: Duration,
    pub created_at: DateTime<Utc>,
}
