/*!
 * AI内容类型定义
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Chat,           // 聊天对话
    Meeting,        // 会议发言
    GameNarration,  // 游戏旁白
    Announcement,   // 公告通知
    Custom(String), // 自定义类型
}

/// 生成的内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedContent {
    pub id: String,
    pub content: String,
    pub content_type: ContentType,
    pub created_at: DateTime<Utc>,
    pub metadata: ContentMetadata,
}

/// 内容元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub word_count: u32,
    pub estimated_duration: Duration,
    pub quality_score: f32,
    pub tags: Vec<String>,
}
