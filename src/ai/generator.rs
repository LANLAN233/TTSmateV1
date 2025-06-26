/*!
 * AI文案生成器
 */

use crate::ai::content::{ContentType, GeneratedContent};
use crate::error::Result;

/// AI文案生成器
pub struct AIContentGenerator {
    api_key: String,
}

impl AIContentGenerator {
    /// 创建新的AI文案生成器
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// 生成内容
    pub async fn generate_content(
        &self,
        _prompt: &str,
        _content_type: ContentType,
    ) -> Result<GeneratedContent> {
        // TODO: 实现AI文案生成
        todo!("AI文案生成功能待实现")
    }
}
