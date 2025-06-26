/*!
 * TTSmate主应用程序
 */

use eframe::egui;
use log::{info, error};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::AppConfig;
use crate::tts::TTSClient;
use crate::error::{AppError, Result};

/// TTSmate主应用程序
pub struct TTSmateApp {
    config: AppConfig,
    tts_client: Option<TTSClient>,
    
    // UI状态
    current_tab: Tab,
    text_input: String,
    status_message: String,
    is_processing: bool,
    
    // TTS状态
    available_voices: Vec<String>,
    selected_voice: String,
    
    // 错误状态
    last_error: Option<String>,
}

/// 应用程序标签页
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    TTS,
    AI,
    SoundBoard,
    Settings,
}

impl TTSmateApp {
    /// 创建新的应用程序实例
    pub async fn new(config: AppConfig) -> Result<Self> {
        info!("初始化TTSmate应用程序");

        // 创建TTS客户端
        let tts_client = match TTSClient::new(config.tts.clone()) {
            Ok(client) => {
                info!("TTS客户端创建成功");
                Some(client)
            }
            Err(e) => {
                error!("TTS客户端创建失败: {}", e);
                None
            }
        };

        // 获取可用语音列表
        let available_voices = if let Some(ref client) = tts_client {
            match client.get_voices().await {
                Ok(voices) => voices,
                Err(e) => {
                    error!("获取语音列表失败: {}", e);
                    vec!["Default".to_string()]
                }
            }
        } else {
            vec!["Default".to_string()]
        };

        let selected_voice = config.tts.default_voice.clone();

        Ok(Self {
            config,
            tts_client,
            current_tab: Tab::TTS,
            text_input: String::new(),
            status_message: "就绪".to_string(),
            is_processing: false,
            available_voices,
            selected_voice,
            last_error: None,
        })
    }

    /// 渲染TTS标签页
    fn render_tts_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("文本转语音");

        ui.separator();

        // 语音选择
        ui.horizontal(|ui| {
            ui.label("语音类型:");
            egui::ComboBox::from_label("")
                .selected_text(&self.selected_voice)
                .show_ui(ui, |ui| {
                    for voice in &self.available_voices {
                        ui.selectable_value(&mut self.selected_voice, voice.clone(), voice);
                    }
                });
        });

        ui.add_space(10.0);

        // 文本输入
        ui.label("输入文本:");
        ui.add(
            egui::TextEdit::multiline(&mut self.text_input)
                .desired_rows(5)
                .hint_text("请输入要转换为语音的文本...")
        );

        ui.add_space(10.0);

        // 控制按钮
        ui.horizontal(|ui| {
            let generate_button = ui.add_enabled(
                !self.is_processing && !self.text_input.trim().is_empty() && self.tts_client.is_some(),
                egui::Button::new("生成语音")
            );

            if generate_button.clicked() {
                self.generate_speech();
            }

            if ui.button("清除文本").clicked() {
                self.text_input.clear();
            }

            if ui.button("测试连接").clicked() {
                self.test_tts_connection();
            }
        });

        ui.add_space(10.0);

        // 状态显示
        if self.is_processing {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("正在处理...");
            });
        } else {
            ui.label(format!("状态: {}", self.status_message));
        }

        // 错误显示
        if let Some(ref error) = self.last_error {
            ui.add_space(5.0);
            ui.colored_label(egui::Color32::RED, format!("错误: {}", error));
        }
    }

    /// 渲染AI标签页
    fn render_ai_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("AI文案生成");
        ui.separator();
        
        ui.label("AI文案生成功能正在开发中...");
        
        // TODO: 实现AI文案生成界面
        ui.add_space(20.0);
        ui.label("功能包括:");
        ui.label("• 智能文案生成");
        ui.label("• 多种内容类型");
        ui.label("• 模板管理");
        ui.label("• 历史记录");
    }

    /// 渲染音效板标签页
    fn render_soundboard_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("音效板");
        ui.separator();
        
        ui.label("音效板功能正在开发中...");
        
        // TODO: 实现音效板界面
        ui.add_space(20.0);
        ui.label("功能包括:");
        ui.label("• 音效文件管理");
        ui.label("• 快捷键绑定");
        ui.label("• 音效分类");
        ui.label("• 实时播放");
    }

    /// 渲染设置标签页
    fn render_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("设置");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("TTS设置", |ui| {
                ui.horizontal(|ui| {
                    ui.label("服务器地址:");
                    ui.text_edit_singleline(&mut self.config.tts.server_url);
                });

                ui.horizontal(|ui| {
                    ui.label("超时时间(秒):");
                    ui.add(egui::DragValue::new(&mut self.config.tts.timeout_seconds).range(1..=300));
                });

                ui.checkbox(&mut self.config.tts.cache_enabled, "启用缓存");
            });

            ui.collapsing("AI设置", |ui| {
                ui.horizontal(|ui| {
                    ui.label("API密钥:");
                    ui.text_edit_singleline(&mut self.config.ai.api_key);
                });

                ui.horizontal(|ui| {
                    ui.label("模型:");
                    ui.text_edit_singleline(&mut self.config.ai.model);
                });

                ui.horizontal(|ui| {
                    ui.label("温度:");
                    ui.add(egui::DragValue::new(&mut self.config.ai.temperature).range(0.0..=2.0).speed(0.01));
                });
            });

            ui.collapsing("音频设置", |ui| {
                ui.horizontal(|ui| {
                    ui.label("采样率:");
                    ui.add(egui::DragValue::new(&mut self.config.audio.sample_rate).range(8000..=96000));
                });

                ui.horizontal(|ui| {
                    ui.label("主音量:");
                    ui.add(egui::Slider::new(&mut self.config.audio.master_volume, 0.0..=2.0));
                });

                ui.checkbox(&mut self.config.audio.enable_virtual_cable, "启用虚拟声卡");
            });

            ui.add_space(20.0);
            if ui.button("保存设置").clicked() {
                self.save_config();
            }
        });
    }

    /// 生成语音
    fn generate_speech(&mut self) {
        if let Some(ref _client) = self.tts_client {
            self.is_processing = true;
            self.status_message = "正在生成语音...".to_string();
            self.last_error = None;

            // TODO: 实现异步语音生成
            // 这里需要使用异步任务来调用TTS API
            info!("开始生成语音: {}", self.text_input);
            
            // 临时模拟处理
            self.is_processing = false;
            self.status_message = "语音生成完成（模拟）".to_string();
        }
    }

    /// 测试TTS连接
    fn test_tts_connection(&mut self) {
        if let Some(ref _client) = self.tts_client {
            self.status_message = "正在测试连接...".to_string();
            
            // TODO: 实现异步连接测试
            info!("测试TTS服务器连接");
            
            // 临时模拟测试
            self.status_message = "连接测试完成（模拟）".to_string();
        } else {
            self.last_error = Some("TTS客户端未初始化".to_string());
        }
    }

    /// 保存配置
    fn save_config(&mut self) {
        // TODO: 实现异步配置保存
        info!("保存配置");
        self.status_message = "配置已保存".to_string();
    }
}

impl eframe::App for TTSmateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("退出").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("帮助", |ui| {
                    if ui.button("关于").clicked() {
                        // TODO: 显示关于对话框
                    }
                });
            });
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("TTSmate V1.0.0");
                ui.separator();
                
                if let Some(ref client) = self.tts_client {
                    ui.label("🟢 TTS已连接");
                } else {
                    ui.colored_label(egui::Color32::RED, "🔴 TTS未连接");
                }
            });
        });

        // 左侧标签页选择
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("功能");
            ui.separator();

            ui.selectable_value(&mut self.current_tab, Tab::TTS, "文本转语音");
            ui.selectable_value(&mut self.current_tab, Tab::AI, "AI文案生成");
            ui.selectable_value(&mut self.current_tab, Tab::SoundBoard, "音效板");
            ui.selectable_value(&mut self.current_tab, Tab::Settings, "设置");
        });

        // 主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::TTS => self.render_tts_tab(ui),
                Tab::AI => self.render_ai_tab(ui),
                Tab::SoundBoard => self.render_soundboard_tab(ui),
                Tab::Settings => self.render_settings_tab(ui),
            }
        });
    }
}
