/*!
 * 音频流处理
 */

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// 音频流
#[derive(Debug, Clone)]
pub struct AudioStream {
    pub id: String,
    pub input_device: String,
    pub output_device: String,
    pub config: StreamConfig,
    pub status: StreamStatus,
}

/// 音频流配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: u32,
    pub volume: f32,
    pub latency: Duration,
    pub format: AudioFormat,
}

/// 音频流状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamStatus {
    Created,    // 已创建
    Starting,   // 启动中
    Running,    // 运行中
    Paused,     // 暂停
    Stopping,   // 停止中
    Stopped,    // 已停止
    Error,      // 错误状态
}

/// 音频格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    I16,    // 16位整数
    I24,    // 24位整数
    I32,    // 32位整数
    F32,    // 32位浮点
}

/// 音频流统计信息
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub created_at: Instant,
    pub total_frames: u64,
    pub dropped_frames: u64,
    pub underruns: u32,
    pub overruns: u32,
    pub current_latency: Duration,
    pub average_cpu_usage: f32,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 1024,
            volume: 1.0,
            latency: Duration::from_millis(10),
            format: AudioFormat::F32,
        }
    }
}

impl StreamStatus {
    /// 获取状态的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            StreamStatus::Created => "已创建",
            StreamStatus::Starting => "启动中",
            StreamStatus::Running => "运行中",
            StreamStatus::Paused => "暂停",
            StreamStatus::Stopping => "停止中",
            StreamStatus::Stopped => "已停止",
            StreamStatus::Error => "错误",
        }
    }

    /// 检查状态是否为活跃状态
    pub fn is_active(&self) -> bool {
        matches!(self, StreamStatus::Starting | StreamStatus::Running)
    }

    /// 检查状态是否为错误状态
    pub fn is_error(&self) -> bool {
        matches!(self, StreamStatus::Error)
    }
}

impl AudioFormat {
    /// 获取格式的字节大小
    pub fn byte_size(&self) -> usize {
        match self {
            AudioFormat::I16 => 2,
            AudioFormat::I24 => 3,
            AudioFormat::I32 => 4,
            AudioFormat::F32 => 4,
        }
    }

    /// 获取格式的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            AudioFormat::I16 => "16位整数",
            AudioFormat::I24 => "24位整数",
            AudioFormat::I32 => "32位整数",
            AudioFormat::F32 => "32位浮点",
        }
    }

    /// 检查格式是否为浮点格式
    pub fn is_float(&self) -> bool {
        matches!(self, AudioFormat::F32)
    }
}

impl AudioStream {
    /// 创建新的音频流
    pub fn new(
        id: String,
        input_device: String,
        output_device: String,
        config: StreamConfig,
    ) -> Self {
        Self {
            id,
            input_device,
            output_device,
            config,
            status: StreamStatus::Created,
        }
    }

    /// 计算每秒字节数
    pub fn bytes_per_second(&self) -> u32 {
        self.config.sample_rate
            * self.config.channels as u32
            * self.config.format.byte_size() as u32
    }

    /// 计算缓冲区持续时间
    pub fn buffer_duration(&self) -> Duration {
        let frames_per_second = self.config.sample_rate;
        let buffer_frames = self.config.buffer_size;
        let duration_ms = (buffer_frames * 1000) / frames_per_second;
        Duration::from_millis(duration_ms as u64)
    }

    /// 获取流的描述信息
    pub fn description(&self) -> String {
        format!(
            "音频流 {} -> {} ({} Hz, {} 通道, {})",
            self.input_device,
            self.output_device,
            self.config.sample_rate,
            self.config.channels,
            self.config.format.display_name()
        )
    }
}

impl Default for StreamStats {
    fn default() -> Self {
        Self {
            created_at: Instant::now(),
            total_frames: 0,
            dropped_frames: 0,
            underruns: 0,
            overruns: 0,
            current_latency: Duration::from_millis(0),
            average_cpu_usage: 0.0,
        }
    }
}

impl StreamStats {
    /// 计算丢帧率
    pub fn drop_rate(&self) -> f32 {
        if self.total_frames == 0 {
            0.0
        } else {
            self.dropped_frames as f32 / self.total_frames as f32
        }
    }

    /// 获取运行时间
    pub fn uptime(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// 格式化统计信息
    pub fn format(&self) -> String {
        format!(
            "运行时间: {:.1}s, 总帧数: {}, 丢帧率: {:.2}%, 延迟: {:.1}ms, CPU: {:.1}%",
            self.uptime().as_secs_f32(),
            self.total_frames,
            self.drop_rate() * 100.0,
            self.current_latency.as_secs_f32() * 1000.0,
            self.average_cpu_usage * 100.0
        )
    }
}
