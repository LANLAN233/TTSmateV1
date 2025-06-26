/*!
 * 音频播放器
 */

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use log::{info, warn, error, debug};

use crate::error::{AppError, Result};

/// 音频播放器
pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    current_sink: Arc<Mutex<Option<Sink>>>,
    volume: f32,
}

/// 播放状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

impl AudioPlayer {
    /// 创建新的音频播放器
    pub fn new() -> Result<Self> {
        info!("初始化音频播放器");

        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AppError::audio(format!("无法创建音频输出流: {}", e)))?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            current_sink: Arc::new(Mutex::new(None)),
            volume: 1.0,
        })
    }

    /// 播放音频数据
    pub async fn play_audio(&self, audio_data: &[u8]) -> Result<()> {
        info!("开始播放音频，数据大小: {} 字节", audio_data.len());

        // 停止当前播放
        self.stop().await;

        // 创建新的Sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AppError::audio(format!("无法创建音频Sink: {}", e)))?;

        // 设置音量
        sink.set_volume(self.volume);

        // 解码音频数据
        let cursor = Cursor::new(audio_data.to_vec());
        let source = Decoder::new(cursor)
            .map_err(|e| AppError::audio(format!("无法解码音频数据: {}", e)))?;

        // 播放音频
        sink.append(source);

        // 保存当前Sink
        {
            let mut current_sink = self.current_sink.lock().await;
            *current_sink = Some(sink);
        }

        info!("音频播放开始");
        Ok(())
    }

    /// 播放文件
    pub async fn play_file(&self, file_path: &std::path::Path) -> Result<()> {
        info!("播放音频文件: {}", file_path.display());

        if !file_path.exists() {
            return Err(AppError::audio("音频文件不存在"));
        }

        // 读取文件数据
        let audio_data = tokio::fs::read(file_path).await
            .map_err(|e| AppError::audio(format!("无法读取音频文件: {}", e)))?;

        self.play_audio(&audio_data).await
    }

    /// 停止播放
    pub async fn stop(&self) {
        debug!("停止音频播放");

        let mut current_sink = self.current_sink.lock().await;
        if let Some(sink) = current_sink.take() {
            sink.stop();
            info!("音频播放已停止");
        }
    }

    /// 暂停播放
    pub async fn pause(&self) {
        debug!("暂停音频播放");

        let current_sink = self.current_sink.lock().await;
        if let Some(ref sink) = *current_sink {
            sink.pause();
            info!("音频播放已暂停");
        }
    }

    /// 恢复播放
    pub async fn resume(&self) {
        debug!("恢复音频播放");

        let current_sink = self.current_sink.lock().await;
        if let Some(ref sink) = *current_sink {
            sink.play();
            info!("音频播放已恢复");
        }
    }

    /// 设置音量
    pub async fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 2.0);
        debug!("设置音量: {}", self.volume);

        let current_sink = self.current_sink.lock().await;
        if let Some(ref sink) = *current_sink {
            sink.set_volume(self.volume);
        }
    }

    /// 获取音量
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    /// 获取播放状态
    pub async fn get_state(&self) -> PlaybackState {
        let current_sink = self.current_sink.lock().await;
        match current_sink.as_ref() {
            Some(sink) => {
                if sink.is_paused() {
                    PlaybackState::Paused
                } else if sink.empty() {
                    PlaybackState::Stopped
                } else {
                    PlaybackState::Playing
                }
            }
            None => PlaybackState::Stopped,
        }
    }

    /// 检查是否正在播放
    pub async fn is_playing(&self) -> bool {
        matches!(self.get_state().await, PlaybackState::Playing)
    }

    /// 等待播放完成
    pub async fn wait_for_completion(&self) {
        let current_sink = self.current_sink.lock().await;
        if let Some(ref sink) = *current_sink {
            sink.sleep_until_end();
        }
    }

    /// 获取播放进度（如果支持）
    pub async fn get_position(&self) -> Option<Duration> {
        // rodio目前不支持获取播放位置
        // 这里返回None，未来可以通过其他方式实现
        None
    }

    /// 设置播放速度（如果支持）
    pub async fn set_speed(&self, speed: f32) -> Result<()> {
        let current_sink = self.current_sink.lock().await;
        if let Some(ref sink) = *current_sink {
            sink.set_speed(speed.clamp(0.1, 3.0));
            debug!("设置播放速度: {}", speed);
            Ok(())
        } else {
            Err(AppError::audio("没有正在播放的音频"))
        }
    }

    /// 播放多个音频（队列播放）
    pub async fn play_queue(&self, audio_files: Vec<&std::path::Path>) -> Result<()> {
        info!("开始队列播放，文件数量: {}", audio_files.len());

        // 停止当前播放
        self.stop().await;

        // 创建新的Sink
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AppError::audio(format!("无法创建音频Sink: {}", e)))?;

        sink.set_volume(self.volume);

        // 添加所有音频文件到队列
        for file_path in audio_files {
            if file_path.exists() {
                match std::fs::File::open(file_path) {
                    Ok(file) => {
                        match Decoder::new(std::io::BufReader::new(file)) {
                            Ok(source) => {
                                sink.append(source);
                                debug!("添加到播放队列: {}", file_path.display());
                            }
                            Err(e) => {
                                warn!("无法解码音频文件 {}: {}", file_path.display(), e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("无法打开音频文件 {}: {}", file_path.display(), e);
                    }
                }
            } else {
                warn!("音频文件不存在: {}", file_path.display());
            }
        }

        // 保存当前Sink
        {
            let mut current_sink = self.current_sink.lock().await;
            *current_sink = Some(sink);
        }

        info!("队列播放开始");
        Ok(())
    }

    /// 获取支持的音频格式
    pub fn get_supported_formats() -> Vec<&'static str> {
        vec![
            "wav", "mp3", "ogg", "flac", "m4a", "aac"
        ]
    }

    /// 检查文件格式是否支持
    pub fn is_format_supported(file_path: &std::path::Path) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                let ext_lower = ext_str.to_lowercase();
                return Self::get_supported_formats().contains(&ext_lower.as_str());
            }
        }
        false
    }
}
