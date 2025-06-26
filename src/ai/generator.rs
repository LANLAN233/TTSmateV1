/*!
 * AI文案生成器
 */

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use log::{info, warn, error, debug};
use chrono::Utc;
use uuid::Uuid;

use crate::ai::content::{ContentType, GeneratedContent, ContentMetadata};
use crate::config::AIConfig;
use crate::error::{AppError, Result};

/// AI文案生成器
pub struct AIContentGenerator {
    api_key: String,
    client: Client,
    config: AIConfig,
}

/// 生成选项
#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub max_length: Option<u32>,
    pub style: Option<String>,
    pub tone: Option<Tone>,
    pub language: Option<String>,
}

/// 语调类型
#[derive(Debug, Clone)]
pub enum Tone {
    Formal,     // 正式
    Casual,     // 随意
    Humorous,   // 幽默
    Serious,    // 严肃
    Friendly,   // 友好
}

/// DeepSeek API请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
}

/// 消息
#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// DeepSeek API响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

/// 选择
#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
    finish_reason: Option<String>,
}

/// 响应消息
#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

/// 使用统计
#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl AIContentGenerator {
    /// 创建新的AI文案生成器
    pub fn new(config: AIConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(AppError::Network)?;

        Ok(Self {
            api_key: config.api_key.clone(),
            client,
            config,
        })
    }

    /// 生成内容
    pub async fn generate_content(
        &self,
        prompt: &str,
        content_type: ContentType,
        options: Option<GenerationOptions>,
    ) -> Result<GeneratedContent> {
        info!("开始生成AI文案: {}", prompt);

        if self.api_key.is_empty() {
            return Err(AppError::ai("AI API密钥未配置"));
        }

        let system_prompt = self.build_system_prompt(&content_type, &options);
        let user_prompt = self.build_user_prompt(prompt, &content_type, &options);

        let content = self.call_deepseek_api(&system_prompt, &user_prompt, &options).await?;

        let generated_content = GeneratedContent {
            id: Uuid::new_v4().to_string(),
            content: content.clone(),
            content_type,
            created_at: Utc::now(),
            metadata: self.create_metadata(&content),
        };

        info!("AI文案生成完成，长度: {} 字符", content.len());
        Ok(generated_content)
    }

    /// 构建系统提示词
    fn build_system_prompt(&self, content_type: &ContentType, options: &Option<GenerationOptions>) -> String {
        let base_prompt = match content_type {
            ContentType::Chat => "你是一个友好的聊天助手，生成自然流畅的对话内容。",
            ContentType::Meeting => "你是一个专业的会议助手，生成正式的会议发言内容。",
            ContentType::GameNarration => "你是一个游戏旁白员，生成生动有趣的游戏解说内容。",
            ContentType::Announcement => "你是一个公告助手，生成清晰明确的通知内容。",
            ContentType::Custom(desc) => desc,
        };

        let mut prompt = base_prompt.to_string();

        if let Some(opts) = options {
            if let Some(ref tone) = opts.tone {
                let tone_desc = match tone {
                    Tone::Formal => "请使用正式的语调",
                    Tone::Casual => "请使用轻松随意的语调",
                    Tone::Humorous => "请使用幽默风趣的语调",
                    Tone::Serious => "请使用严肃认真的语调",
                    Tone::Friendly => "请使用友好亲切的语调",
                };
                prompt.push_str(&format!("。{}", tone_desc));
            }

            if let Some(ref style) = opts.style {
                prompt.push_str(&format!("。风格要求：{}", style));
            }

            if let Some(ref language) = opts.language {
                prompt.push_str(&format!("。请使用{}回复", language));
            } else {
                prompt.push_str("。请使用中文回复");
            }
        } else {
            prompt.push_str("。请使用中文回复");
        }

        prompt
    }

    /// 构建用户提示词
    fn build_user_prompt(&self, prompt: &str, content_type: &ContentType, options: &Option<GenerationOptions>) -> String {
        let mut user_prompt = String::new();

        // 添加内容类型特定的前缀
        match content_type {
            ContentType::Chat => user_prompt.push_str("请生成一段聊天内容："),
            ContentType::Meeting => user_prompt.push_str("请生成一段会议发言："),
            ContentType::GameNarration => user_prompt.push_str("请生成一段游戏旁白："),
            ContentType::Announcement => user_prompt.push_str("请生成一段公告内容："),
            ContentType::Custom(_) => user_prompt.push_str("请生成内容："),
        }

        user_prompt.push_str(prompt);

        // 添加长度限制
        if let Some(opts) = options {
            if let Some(max_length) = opts.max_length {
                user_prompt.push_str(&format!("。请控制在{}字以内", max_length));
            }
        }

        user_prompt
    }

    /// 调用DeepSeek API
    async fn call_deepseek_api(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        options: &Option<GenerationOptions>,
    ) -> Result<String> {
        let max_tokens = options.as_ref()
            .and_then(|o| o.max_length)
            .unwrap_or(self.config.max_tokens);

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            max_tokens,
            temperature: self.config.temperature,
            stream: false,
        };

        debug!("调用DeepSeek API: {}", self.config.base_url);

        let url = format!("{}/v1/chat/completions", self.config.base_url);

        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
        ).await
        .map_err(|_| AppError::ai("API调用超时"))?
        .map_err(AppError::Network)?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
            error!("DeepSeek API错误 {}: {}", status_code, error_text);
            return Err(AppError::ai(format!("API错误 {}: {}", status_code, error_text)));
        }

        let chat_response: ChatResponse = response.json().await
            .map_err(|e| AppError::ai(format!("解析响应失败: {}", e)))?;

        if let Some(choice) = chat_response.choices.first() {
            let content = choice.message.content.trim().to_string();

            if let Some(usage) = chat_response.usage {
                debug!("API使用统计: {} tokens", usage.total_tokens);
            }

            Ok(content)
        } else {
            Err(AppError::ai("API响应中没有生成内容"))
        }
    }

    /// 创建内容元数据
    fn create_metadata(&self, content: &str) -> ContentMetadata {
        let word_count = content.chars().count() as u32;
        let estimated_duration = Duration::from_secs((word_count as f32 * 0.5) as u64); // 假设每秒2个字

        ContentMetadata {
            word_count,
            estimated_duration,
            quality_score: 0.8, // 默认质量分数，可以后续实现质量评估算法
            tags: vec![], // 可以后续实现标签提取
        }
    }

    /// 测试API连接
    pub async fn test_connection(&self) -> Result<bool> {
        if self.api_key.is_empty() {
            return Ok(false);
        }

        debug!("测试DeepSeek API连接");

        match self.generate_content(
            "测试连接",
            ContentType::Chat,
            Some(GenerationOptions {
                max_length: Some(10),
                style: None,
                tone: None,
                language: Some("中文".to_string()),
            })
        ).await {
            Ok(_) => {
                info!("DeepSeek API连接成功");
                Ok(true)
            }
            Err(e) => {
                warn!("DeepSeek API连接失败: {}", e);
                Ok(false)
            }
        }
    }

    /// 获取可用模型列表
    pub fn get_available_models(&self) -> Vec<String> {
        vec![
            "deepseek-chat".to_string(),
            "deepseek-coder".to_string(),
        ]
    }

    /// 批量生成内容
    pub async fn generate_batch(
        &self,
        prompts: Vec<(String, ContentType)>,
        options: Option<GenerationOptions>,
    ) -> Result<Vec<GeneratedContent>> {
        let mut results = Vec::new();

        for (prompt, content_type) in prompts {
            match self.generate_content(&prompt, content_type, options.clone()).await {
                Ok(content) => results.push(content),
                Err(e) => {
                    error!("批量生成失败: {}", e);
                    // 继续处理其他项目，不中断整个批量操作
                }
            }

            // 添加延迟避免API限流
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        Ok(results)
    }
}
