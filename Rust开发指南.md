TTSmate V1 Rust项目开发指南

1 项目结构设计

1.1 推荐的项目目录结构
```
TTSmate/
├── Cargo.toml                 # 项目配置文件
├── Cargo.lock                 # 依赖锁定文件
├── README.md                  # 项目说明文档
├── LICENSE                    # 许可证文件
├── .gitignore                 # Git忽略文件
├── build.rs                   # 构建脚本
├── src/                       # 源代码目录
│   ├── main.rs               # 程序入口点
│   ├── lib.rs                # 库入口点
│   ├── config/               # 配置管理模块
│   │   ├── mod.rs
│   │   ├── app_config.rs
│   │   └── config_manager.rs
│   ├── tts/                  # TTS客户端模块
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── cache.rs
│   │   └── error.rs
│   ├── ai/                   # AI文案生成模块
│   │   ├── mod.rs
│   │   ├── generator.rs
│   │   ├── template.rs
│   │   └── content.rs
│   ├── soundboard/           # 音效板模块
│   │   ├── mod.rs
│   │   ├── board.rs
│   │   ├── sound.rs
│   │   └── keybinding.rs
│   ├── audio/                # 音频处理模块
│   │   ├── mod.rs
│   │   ├── router.rs
│   │   ├── device.rs
│   │   └── stream.rs
│   ├── ui/                   # 用户界面模块
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── panels/
│   │   │   ├── mod.rs
│   │   │   ├── tts_panel.rs
│   │   │   ├── ai_panel.rs
│   │   │   └── sound_panel.rs
│   │   └── components/
│   │       ├── mod.rs
│   │       ├── button.rs
│   │       └── input.rs
│   ├── utils/                # 工具函数模块
│   │   ├── mod.rs
│   │   ├── logger.rs
│   │   └── helpers.rs
│   └── error.rs              # 全局错误定义
├── tests/                    # 集成测试
│   ├── integration_tests.rs
│   └── common/
│       └── mod.rs
├── benches/                  # 性能测试
│   └── benchmark.rs
├── examples/                 # 示例代码
│   ├── basic_usage.rs
│   └── advanced_features.rs
├── assets/                   # 资源文件
│   ├── icons/
│   ├── sounds/
│   └── config/
├── docs/                     # 文档目录
│   ├── api.md
│   └── user_guide.md
└── scripts/                  # 构建和部署脚本
    ├── build.sh
    └── package.sh
```

1.2 Cargo.toml配置示例
```toml
[package]
name = "ttsmate"
version = "1.0.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "智能语音合成客户端"
license = "MIT"
repository = "https://github.com/yourusername/ttsmate"
keywords = ["tts", "ai", "audio", "voice", "synthesis"]
categories = ["multimedia::audio", "gui"]

[dependencies]
# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# HTTP客户端
reqwest = { version = "0.11", features = ["json", "stream"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 音频处理
rodio = "0.17"
cpal = "0.15"

# UI框架 (选择其一)
egui = "0.24"
eframe = { version = "0.24", features = ["default_fonts"] }
# 或者使用 tauri
# tauri = { version = "1.0", features = ["api-all"] }

# 配置管理
config = "0.13"
toml = "0.8"

# 日志
log = "0.4"
env_logger = "0.10"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 数据库 (SQLite)
rusqlite = { version = "0.29", features = ["bundled"] }

# 系统集成
winapi = { version = "0.3", features = ["winuser", "processthreadsapi"] }
windows = { version = "0.48", features = ["Win32_Media_Audio"] }

# 加密
aes-gcm = "0.10"
rand = "0.8"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 文件系统
walkdir = "2.3"

[dev-dependencies]
# 测试框架
tokio-test = "0.4"
mockall = "0.11"
criterion = "0.5"

[build-dependencies]
# Windows资源编译
winres = "0.1"

[[bin]]
name = "ttsmate"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

2 核心模块实现指南

2.1 TTS客户端模块实现
```rust
// src/tts/mod.rs
pub mod client;
pub mod cache;
pub mod error;

pub use client::TTSClient;
pub use error::TTSError;

// src/tts/client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct TTSClient {
    base_url: String,
    client: Client,
    config: TTSConfig,
}

#[derive(Debug, Clone)]
pub struct TTSConfig {
    pub timeout: Duration,
    pub retry_count: u32,
    pub cache_enabled: bool,
    pub default_voice: String,
}

impl TTSClient {
    pub fn new(base_url: &str) -> Result<Self, TTSError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            base_url: base_url.to_string(),
            client,
            config: TTSConfig::default(),
        })
    }

    pub async fn synthesize(
        &self,
        text: &str,
        options: Option<SynthesizeOptions>,
    ) -> Result<AudioData, TTSError> {
        // 基于Gradio API的TTS实现
        // 注意：当前API文档中缺少主要的文本转语音生成端点

        // 第一步：设置语音类型
        let voice = options.as_ref()
            .and_then(|o| o.voice.clone())
            .unwrap_or_else(|| self.config.default_voice.clone());

        let audio_seed = self.setup_voice_and_seeds(&voice).await?;

        // TODO: 需要找到实际的文本转语音生成API端点
        // 当前API文档显示的主要是配置和预处理端点
        // 缺少关键的文本输入和音频生成端点

        // 临时实现：返回空音频数据，等待完整API文档
        log::warn!("TTS生成API端点缺失，返回空音频数据");

        Ok(AudioData {
            data: vec![],
            format: AudioFormat::Wav,
            duration: Duration::from_secs(0),
            sample_rate: 44100,
        })
    }

    // 设置语音和种子的辅助方法
    async fn setup_voice_and_seeds(&self, voice: &str) -> Result<f64, TTSError> {
        // 1. 设置语音类型
        let voice_event_id = self.call_gradio_api("/on_voice_change",
            vec![serde_json::Value::String(voice.to_string())]).await?;
        let _audio_seed = self.get_gradio_result::<f64>("/on_voice_change", &voice_event_id).await?;

        // 2. 生成音频种子
        let seed_event_id = self.call_gradio_api("/generate_seed", vec![]).await?;
        let audio_seed = self.get_gradio_result::<f64>("/generate_seed", &seed_event_id).await?;

        // 3. 生成文本种子
        let text_seed_event_id = self.call_gradio_api("/generate_seed_1", vec![]).await?;
        let _text_seed = self.get_gradio_result::<f64>("/generate_seed_1", &text_seed_event_id).await?;

        // 4. 设置音频种子
        let embedding_event_id = self.call_gradio_api("/on_audio_seed_change",
            vec![serde_json::Value::Number(serde_json::Number::from_f64(audio_seed).unwrap())]).await?;
        let _speaker_embedding = self.get_gradio_result::<String>("/on_audio_seed_change", &embedding_event_id).await?;

        Ok(audio_seed)
    }

    // Gradio API通用调用方法
    async fn call_gradio_api(&self, endpoint: &str, data: Vec<serde_json::Value>) -> Result<String, TTSError> {
        let request = GradioRequest { data };
        let url = format!("{}/gradio_api/call{}", self.base_url, endpoint);

        let response = timeout(
            self.config.timeout,
            self.client.post(&url).json(&request).send()
        ).await??;

        if !response.status().is_success() {
            return Err(TTSError::ServerError {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        let event_response: GradioEventResponse = response.json().await?;
        Ok(event_response.event_id)
    }

    // 获取Gradio API结果
    async fn get_gradio_result<T>(&self, endpoint: &str, event_id: &str) -> Result<T, TTSError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/gradio_api/call{}/{}", self.base_url, endpoint, event_id);

        let response = timeout(
            self.config.timeout,
            self.client.get(&url).send()
        ).await??;

        if !response.status().is_success() {
            return Err(TTSError::ServerError {
                status_code: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        let result: GradioResult<T> = response.json().await?;
        Ok(result.data)
    }
}

// Gradio API相关数据结构
#[derive(Debug, Serialize)]
struct GradioRequest {
    data: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GradioEventResponse {
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct GradioResult<T> {
    data: T,
}

// 保留原有结构以备将来使用
#[derive(Debug, Serialize)]
struct SynthesizeRequest {
    text: String,
    voice: String,
    speed: f32,
    pitch: f32,
    volume: f32,
}

#[derive(Debug, Deserialize)]
struct SynthesizeResponse {
    success: bool,
    audio_data: String,
    duration: f32,
    format: String,
}
```

2.2 AI文案生成模块实现
```rust
// src/ai/generator.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AIContentGenerator {
    api_key: String,
    client: Client,
    config: AIConfig,
}

impl AIContentGenerator {
    pub fn new(api_key: &str) -> Result<Self, crate::error::AppError> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: Client::new(),
            config: AIConfig::default(),
        })
    }

    pub async fn generate_content(
        &self,
        prompt: &str,
        content_type: ContentType,
        options: Option<GenerationOptions>,
    ) -> Result<GeneratedContent, crate::error::AppError> {
        let system_prompt = self.build_system_prompt(&content_type);
        let user_prompt = self.build_user_prompt(prompt, &content_type);

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            max_tokens: options.as_ref()
                .and_then(|o| o.max_length)
                .unwrap_or(1000),
            temperature: self.config.temperature,
        };

        let response = self.client
            .post(&self.config.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        let content = chat_response.choices
            .first()
            .ok_or_else(|| crate::error::AppError::AI("No response generated".into()))?
            .message
            .content
            .clone();

        Ok(GeneratedContent {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            content_type,
            created_at: chrono::Utc::now(),
            metadata: ContentMetadata {
                word_count: content.chars().count() as u32,
                estimated_duration: Duration::from_secs(content.len() as u64 / 10),
                quality_score: 0.8, // 可以后续实现质量评估算法
                tags: vec![],
            },
        })
    }

    fn build_system_prompt(&self, content_type: &ContentType) -> String {
        match content_type {
            ContentType::Chat => "你是一个友好的聊天助手，生成自然流畅的对话内容。",
            ContentType::Meeting => "你是一个专业的会议助手，生成正式的会议发言内容。",
            ContentType::GameNarration => "你是一个游戏旁白员，生成生动有趣的游戏解说内容。",
            ContentType::Announcement => "你是一个公告助手，生成清晰明确的通知内容。",
            ContentType::Custom(desc) => desc,
        }.to_string()
    }
}
```

2.3 音效板模块实现
```rust
// src/soundboard/board.rs
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct SoundBoard {
    sounds: HashMap<String, SoundEffect>,
    keybindings: HashMap<KeyCode, String>,
    _stream: OutputStream,
    sinks: HashMap<String, Sink>,
}

impl SoundBoard {
    pub fn new() -> Result<Self, crate::error::AppError> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        
        Ok(Self {
            sounds: HashMap::new(),
            keybindings: HashMap::new(),
            _stream,
            sinks: HashMap::new(),
        })
    }

    pub fn add_sound(
        &mut self,
        file_path: &std::path::Path,
        name: &str,
        category: &str,
    ) -> Result<String, crate::error::AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        
        // 验证音频文件
        let file = File::open(file_path)?;
        let _decoder = Decoder::new(BufReader::new(file))?;
        
        let sound_effect = SoundEffect {
            id: id.clone(),
            name: name.to_string(),
            file_path: file_path.to_path_buf(),
            category: category.to_string(),
            volume: 1.0,
            duration: Duration::from_secs(0), // 可以通过解析音频文件获取
            format: AudioFormat::from_path(file_path),
            created_at: chrono::Utc::now(),
        };

        self.sounds.insert(id.clone(), sound_effect);
        Ok(id)
    }

    pub async fn play_sound(&mut self, sound_id: &str) -> Result<(), crate::error::AppError> {
        let sound = self.sounds.get(sound_id)
            .ok_or_else(|| crate::error::AppError::SoundBoard("Sound not found".into()))?;

        let file = File::open(&sound.file_path)?;
        let decoder = Decoder::new(BufReader::new(file))?;
        
        let sink = Sink::try_new(&self.stream_handle)?;
        sink.set_volume(sound.volume);
        sink.append(decoder);
        
        self.sinks.insert(sound_id.to_string(), sink);
        
        Ok(())
    }
}
```

3 开发最佳实践

3.1 错误处理策略
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("TTS错误: {0}")]
    TTS(#[from] crate::tts::TTSError),
    
    #[error("AI服务错误: {0}")]
    AI(String),
    
    #[error("音频错误: {0}")]
    Audio(#[from] rodio::PlayError),
    
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("网络错误: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("IO错误: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

3.2 日志配置
```rust
// src/utils/logger.rs
use log::{info, warn, error};
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("Logger initialized");
    Ok(())
}

pub fn log_to_file(message: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("ttsmate.log")?;
    
    writeln!(file, "{}: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), message)?;
    Ok(())
}
```

3.3 配置管理
```rust
// src/config/app_config.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub tts: TTSConfig,
    pub ai: AIConfig,
    pub audio: AudioConfig,
    pub ui: UIConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tts: TTSConfig::default(),
            ai: AIConfig::default(),
            audio: AudioConfig::default(),
            ui: UIConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, crate::error::AppError> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), crate::error::AppError> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
```

4 测试策略

4.1 单元测试示例
```rust
// src/tts/client.rs 中的测试
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use tokio_test;

    #[tokio::test]
    async fn test_synthesize_success() {
        let client = TTSClient::new("http://localhost:8080").unwrap();
        
        // 使用mock服务器进行测试
        let result = client.synthesize("测试文本", None).await;
        
        assert!(result.is_ok());
        let audio_data = result.unwrap();
        assert!(!audio_data.data.is_empty());
    }

    #[tokio::test]
    async fn test_synthesize_empty_text() {
        let client = TTSClient::new("http://localhost:8080").unwrap();
        let result = client.synthesize("", None).await;
        
        assert!(result.is_err());
    }
}
```

4.2 集成测试示例
```rust
// tests/integration_tests.rs
use ttsmate::{TTSClient, AIContentGenerator, SoundBoard};

#[tokio::test]
async fn test_full_workflow() {
    // 测试完整的工作流程
    let tts_client = TTSClient::new("http://localhost:8080").unwrap();
    let ai_generator = AIContentGenerator::new("test_key").unwrap();
    let mut soundboard = SoundBoard::new().unwrap();

    // 生成AI文案
    let content = ai_generator.generate_content(
        "生成一段问候语",
        ContentType::Chat,
        None
    ).await.unwrap();

    // 转换为语音
    let audio = tts_client.synthesize(&content.content, None).await.unwrap();
    
    // 验证音频数据
    assert!(!audio.data.is_empty());
}
```

5 性能优化建议

5.1 异步编程最佳实践
- 使用tokio作为异步运行时
- 合理使用async/await语法
- 避免阻塞操作在异步上下文中执行
- 使用tokio::spawn进行并发任务处理

5.2 内存管理优化
- 使用Arc和Mutex进行线程安全的共享状态
- 及时释放不需要的资源
- 使用对象池减少内存分配
- 监控内存使用情况

5.3 网络通信优化
- 使用连接池复用HTTP连接
- 实现请求重试和超时机制
- 使用流式处理大文件传输
- 实现请求缓存机制

6 部署和打包

6.1 Windows平台打包
```toml
# Cargo.toml 中添加
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

[target.'cfg(windows)'.build-dependencies]
winres = "0.1"
```

6.2 构建脚本示例
```rust
// build.rs
#[cfg(windows)]
extern crate winres;

fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set_language(0x0804); // 中文简体
        res.compile().unwrap();
    }
}
```

6.3 发布配置
```bash
# 构建发布版本
cargo build --release

# 生成安装包
cargo install cargo-wix
cargo wix --nocapture
```

7 代码质量保证

7.1 代码格式化和检查
```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy -- -D warnings

# 运行测试
cargo test

# 性能测试
cargo bench
```

7.2 持续集成配置
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: windows-latest
    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Run tests
      run: cargo test --verbose
    - name: Run clippy
      run: cargo clippy -- -D warnings
```

这个开发指南提供了TTSmate V1项目的完整Rust开发框架，包括项目结构、核心模块实现、最佳实践和质量保证措施。开发团队可以基于这个指南进行具体的代码实现。
