/*!
 * TTS客户端实现
 */

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use log::{info, warn, error, debug};

use crate::config::TTSConfig;
use crate::tts::error::{TTSError, TTSResult};
use crate::tts::cache::TTSCache;

/// TTS客户端
#[derive(Debug, Clone)]
pub struct TTSClient {
    base_url: String,
    client: Client,
    config: TTSConfig,
    cache: Arc<Mutex<TTSCache>>,
}

/// 语音合成选项
#[derive(Debug, Clone)]
pub struct SynthesizeOptions {
    pub voice: Option<String>,
    pub speed: Option<f32>,
    pub pitch: Option<f32>,
    pub volume: Option<f32>,
    pub format: Option<AudioFormat>,
}

/// 音频数据
#[derive(Debug, Clone)]
pub struct AudioData {
    pub data: Vec<u8>,
    pub format: AudioFormat,
    pub duration: Duration,
    pub sample_rate: u32,
}

/// 音频格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Ogg,
}

/// Gradio API请求
#[derive(Debug, Serialize)]
struct GradioRequest {
    data: Vec<serde_json::Value>,
}

/// Gradio API事件响应
#[derive(Debug, Deserialize)]
struct GradioEventResponse {
    event_id: String,
}

/// Gradio API结果
#[derive(Debug, Deserialize)]
struct GradioResult<T> {
    data: T,
}

impl TTSClient {
    /// 创建新的TTS客户端
    pub fn new(config: TTSConfig) -> TTSResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(TTSError::NetworkError)?;

        let cache = Arc::new(Mutex::new(TTSCache::new(100))); // 默认缓存100个条目

        Ok(Self {
            base_url: config.server_url.clone(),
            client,
            config,
            cache,
        })
    }

    /// 语音合成
    pub async fn synthesize(
        &self,
        text: &str,
        options: Option<SynthesizeOptions>,
    ) -> TTSResult<AudioData> {
        info!("开始语音合成: {}", text);

        // 检查缓存
        let cache_key = self.generate_cache_key(text, &options);
        if self.config.cache_enabled {
            let cache = self.cache.lock().await;
            if let Some(cached_data) = cache.get(&cache_key) {
                info!("使用缓存的音频数据");
                return Ok(cached_data.clone());
            }
        }

        // 执行语音合成
        let audio_data = self.perform_synthesis(text, options).await?;

        // 存储到缓存
        if self.config.cache_enabled {
            let mut cache = self.cache.lock().await;
            cache.insert(cache_key, audio_data.clone());
        }

        info!("语音合成完成");
        Ok(audio_data)
    }

    /// 执行实际的语音合成
    async fn perform_synthesis(
        &self,
        text: &str,
        options: Option<SynthesizeOptions>,
    ) -> TTSResult<AudioData> {
        // 基于Gradio API的TTS实现
        debug!("设置语音合成参数");

        // 第一步：设置语音类型
        let voice = options.as_ref()
            .and_then(|o| o.voice.clone())
            .unwrap_or_else(|| self.config.default_voice.clone());

        let audio_seed = self.setup_voice_and_seeds(&voice).await?;
        debug!("语音设置完成，音频种子: {}", audio_seed);

        // TODO: 实现实际的文本转语音生成
        // 当前API文档中缺少主要的文本转语音生成端点
        // 这里返回空音频数据作为占位符
        warn!("TTS生成API端点缺失，返回空音频数据");

        Ok(AudioData {
            data: vec![],
            format: AudioFormat::Wav,
            duration: Duration::from_secs(0),
            sample_rate: 44100,
        })
    }

    /// 设置语音和种子
    async fn setup_voice_and_seeds(&self, voice: &str) -> TTSResult<f64> {
        debug!("设置语音类型: {}", voice);

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

    /// 调用Gradio API
    async fn call_gradio_api(&self, endpoint: &str, data: Vec<serde_json::Value>) -> TTSResult<String> {
        let request = GradioRequest { data };
        let url = format!("{}/gradio_api/call{}", self.base_url, endpoint);

        debug!("调用Gradio API: {}", url);

        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.client.post(&url).json(&request).send()
        ).await
        .map_err(|_| TTSError::TimeoutError)?
        .map_err(TTSError::NetworkError)?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
            return Err(TTSError::server_error(status_code, message));
        }

        let event_response: GradioEventResponse = response.json().await
            .map_err(|e| TTSError::parse(format!("解析事件响应失败: {}", e)))?;

        Ok(event_response.event_id)
    }

    /// 获取Gradio API结果
    async fn get_gradio_result<T>(&self, endpoint: &str, event_id: &str) -> TTSResult<T> 
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/gradio_api/call{}/{}", self.base_url, endpoint, event_id);

        debug!("获取Gradio API结果: {}", url);

        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.client.get(&url).send()
        ).await
        .map_err(|_| TTSError::TimeoutError)?
        .map_err(TTSError::NetworkError)?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let message = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
            return Err(TTSError::server_error(status_code, message));
        }

        let result: GradioResult<T> = response.json().await
            .map_err(|e| TTSError::parse(format!("解析结果失败: {}", e)))?;

        Ok(result.data)
    }

    /// 生成缓存键
    fn generate_cache_key(&self, text: &str, options: &Option<SynthesizeOptions>) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        
        if let Some(opts) = options {
            opts.voice.hash(&mut hasher);
            if let Some(speed) = opts.speed {
                speed.to_bits().hash(&mut hasher);
            }
            if let Some(pitch) = opts.pitch {
                pitch.to_bits().hash(&mut hasher);
            }
            if let Some(volume) = opts.volume {
                volume.to_bits().hash(&mut hasher);
            }
        }

        format!("tts_{:x}", hasher.finish())
    }

    /// 获取可用语音列表
    pub async fn get_voices(&self) -> TTSResult<Vec<String>> {
        // TODO: 实现获取语音列表的API调用
        // 当前返回默认语音列表
        Ok(vec![
            "Default".to_string(),
            "Timbre1".to_string(),
            "Timbre2".to_string(),
            "Timbre3".to_string(),
            "Timbre4".to_string(),
            "Timbre5".to_string(),
            "Timbre6".to_string(),
            "Timbre7".to_string(),
            "Timbre8".to_string(),
            "Timbre9".to_string(),
        ])
    }

    /// 测试连接
    pub async fn test_connection(&self) -> TTSResult<bool> {
        debug!("测试TTS服务器连接");

        let url = format!("{}/", self.base_url);
        
        match timeout(
            Duration::from_secs(5),
            self.client.get(&url).send()
        ).await {
            Ok(Ok(response)) => {
                let is_ok = response.status().is_success();
                if is_ok {
                    info!("TTS服务器连接成功");
                } else {
                    warn!("TTS服务器响应错误: {}", response.status());
                }
                Ok(is_ok)
            }
            Ok(Err(e)) => {
                error!("TTS服务器连接失败: {}", e);
                Err(TTSError::NetworkError(e))
            }
            Err(_) => {
                error!("TTS服务器连接超时");
                Err(TTSError::TimeoutError)
            }
        }
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
        info!("TTS缓存已清除");
    }

    /// 获取缓存统计信息
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().await;
        (cache.len(), cache.capacity())
    }
}
