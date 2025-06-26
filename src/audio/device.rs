/*!
 * 音频设备定义
 */

use serde::{Deserialize, Serialize};

/// 音频设备
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub channels: u16,
    pub sample_rate: u32,
    pub is_default: bool,
    pub is_available: bool,
}

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Input,
    Output,
    Duplex,
}

impl AudioDevice {
    /// 创建新的音频设备
    pub fn new(
        id: String,
        name: String,
        device_type: DeviceType,
        channels: u16,
        sample_rate: u32,
    ) -> Self {
        Self {
            id,
            name,
            device_type,
            channels,
            sample_rate,
            is_default: false,
            is_available: true,
        }
    }

    /// 检查设备是否为输入设备
    pub fn is_input(&self) -> bool {
        matches!(self.device_type, DeviceType::Input | DeviceType::Duplex)
    }

    /// 检查设备是否为输出设备
    pub fn is_output(&self) -> bool {
        matches!(self.device_type, DeviceType::Output | DeviceType::Duplex)
    }

    /// 获取设备类型的显示名称
    pub fn type_name(&self) -> &'static str {
        match self.device_type {
            DeviceType::Input => "输入",
            DeviceType::Output => "输出",
            DeviceType::Duplex => "双工",
        }
    }

    /// 获取设备的完整描述
    pub fn description(&self) -> String {
        format!(
            "{} ({}, {} 通道, {} Hz)",
            self.name,
            self.type_name(),
            self.channels,
            self.sample_rate
        )
    }

    /// 检查设备是否支持指定的采样率
    pub fn supports_sample_rate(&self, sample_rate: u32) -> bool {
        // 简化实现，实际应该查询设备支持的采样率范围
        matches!(sample_rate, 8000 | 16000 | 22050 | 44100 | 48000 | 96000)
    }

    /// 检查设备是否支持指定的通道数
    pub fn supports_channels(&self, channels: u16) -> bool {
        channels <= self.channels
    }
}
