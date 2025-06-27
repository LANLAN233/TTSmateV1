mod config;
mod api_client;
mod error;

use std::io::Cursor;
use std::sync::{mpsc, Arc};
use eframe::egui;
use rodio::{Decoder, OutputStream, Sink};
use tokio::runtime::Runtime;

use crate::api_client::ApiClient;
use crate::config::{Config, load_config};

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    let config = load_config().expect("Failed to load config.toml. Make sure it's present and correctly formatted.");
    
    eframe::run_native(
        "TTSmate v1.0",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            // 依次尝试加载多种常见的中文字体
            let font_paths = [
                "C:/Windows/Fonts/msyh.ttf",    // 微软雅黑 Microsoft YaHei
                "C:/Windows/Fonts/deng.ttf",    // 等线 Dengxian
                "C:/Windows/Fonts/simhei.ttf",  // 黑体 SimHei
                "C:/Windows/Fonts/simsun.ttc",  // 宋体 SimSun (TTC)
            ];

            let mut font_loaded = false;
            for path in font_paths {
                if let Ok(font_data) = std::fs::read(path) {
                    let font_name = path.split('/').last().unwrap_or("unknown_font").to_string();
                    log::info!("成功加载系统字体 '{}' 以支持中文显示。", font_name);

                    fonts.font_data.insert(
                        font_name.clone(),
                        egui::FontData::from_owned(font_data),
                    );

                    // 将找到的字体设置为首选
                    fonts
                        .families
                        .entry(egui::FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_name.clone());

                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .insert(0, font_name);
                    
                    font_loaded = true;
                    break; // 找到第一个可用字体后即停止
                }
            }

            if !font_loaded {
                log::warn!("未能找到任何可用的中文字体。界面中文可能无法正确显示。");
            }

            cc.egui_ctx.set_fonts(fonts);

            Box::new(TTSApp::new(config))
        }),
    ).unwrap();
}

enum AppState {
    Idle,
    GeneratingText,
    SynthesizingAudio,
    PlayingAudio,
    Error(String),
}

impl AppState {
    fn to_string(&self) -> String {
        match self {
            AppState::Idle => "就绪".to_string(),
            AppState::GeneratingText => "正在生成文本...".to_string(),
            AppState::SynthesizingAudio => "正在合成语音...".to_string(),
            AppState::PlayingAudio => "正在播放...".to_string(),
            AppState::Error(e) => format!("错误: {}", e),
        }
    }
}

// Message from background task to UI thread
enum UIMessage {
    UpdateState(AppState),
    SetResponseText(String),
    PlayAudio(Vec<u8>),
}

struct TTSApp {
    rt: Runtime,
    prompt_text: String,
    response_text: String,
    status_text: String,
    config: Arc<Config>,
    api_client: Arc<ApiClient>,
    ui_sender: mpsc::Sender<UIMessage>,
    ui_receiver: mpsc::Receiver<UIMessage>,
}

impl TTSApp {
    fn new(config: Config) -> Self {
        let (ui_sender, ui_receiver) = mpsc::channel();
        Self {
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            prompt_text: "你好".to_string(),
            response_text: "".to_string(),
            status_text: AppState::Idle.to_string(),
            config: Arc::new(config),
            api_client: Arc::new(ApiClient::new()),
            ui_sender,
            ui_receiver,
        }
    }

    fn handle_ui_messages(&mut self) {
        while let Ok(msg) = self.ui_receiver.try_recv() {
            match msg {
                UIMessage::UpdateState(state) => {
                    self.status_text = state.to_string();
                }
                UIMessage::SetResponseText(text) => {
                    self.response_text = text;
                }
                UIMessage::PlayAudio(audio_data) => {
                    let sender = self.ui_sender.clone();
                    self.rt.spawn(async move {
                         // This runs on a tokio thread, so it won't block UI.
                        match OutputStream::try_default() {
                            Ok((_stream, stream_handle)) => {
                                let sink = Sink::try_new(&stream_handle).unwrap();
                                let source = Decoder::new(Cursor::new(audio_data)).unwrap();
                                sink.append(source);
                                sink.sleep_until_end(); // This blocks the current worker thread, but not the UI
                                sender.send(UIMessage::UpdateState(AppState::Idle)).unwrap();
                            }
                            Err(e) => {
                                let err_msg = format!("找不到音频输出设备: {}", e);
                                log::error!("{}", err_msg);
                                sender.send(UIMessage::UpdateState(AppState::Error(err_msg))).unwrap();
                            }
                        }
                    });
                }
            }
        }
    }

    fn start_generation_task(&mut self) {
        let sender = self.ui_sender.clone();
        let api_client = self.api_client.clone();
        let config = self.config.clone();
        let prompt = self.prompt_text.clone();

        self.rt.spawn(async move {
            sender.send(UIMessage::UpdateState(AppState::GeneratingText)).unwrap();
            
            let generated_text = match api_client.call_deepseek_api(&config.api_keys.deepseek_api_key, &prompt).await {
                Ok(text) => {
                    sender.send(UIMessage::SetResponseText(text.clone())).unwrap();
                    text
                }
                Err(e) => {
                    let err_msg = format!("DeepSeek API 调用失败: {}", e);
                    log::error!("{}", &err_msg);
                    sender.send(UIMessage::UpdateState(AppState::Error(err_msg))).unwrap();
                    return;
                }
            };

            sender.send(UIMessage::UpdateState(AppState::SynthesizingAudio)).unwrap();

            if generated_text.trim().is_empty() {
                let err_msg = "AI未生成有效文本，无法合成语音。".to_string();
                log::warn!("{}", &err_msg);
                sender.send(UIMessage::UpdateState(AppState::Error(err_msg))).unwrap();
                return;
            }

            match api_client.call_baidu_tts_api(&config.api_keys, &config.app_settings, &generated_text).await {
                Ok(audio_data) => {
                    sender.send(UIMessage::UpdateState(AppState::PlayingAudio)).unwrap();
                    sender.send(UIMessage::PlayAudio(audio_data)).unwrap();
                }
                Err(e) => {
                    let err_msg = format!("Baidu TTS API 调用失败: {}", e);
                    log::error!("{}", &err_msg);
                    sender.send(UIMessage::UpdateState(AppState::Error(err_msg))).unwrap();
                }
            }
        });
    }
}

impl eframe::App for TTSApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle any messages from background tasks
        self.handle_ui_messages();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TTSmate 讯飞语者");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("输入话题:");
                ui.text_edit_singleline(&mut self.prompt_text);
            });
            
            let is_running_task = self.status_text != AppState::Idle.to_string() 
                && !self.status_text.starts_with("错误:");

            if ui.add_enabled(!is_running_task, egui::Button::new("生成并播放")).clicked() {
                self.start_generation_task();
            }

            ui.separator();
            ui.label("AI 生成内容:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.response_text);
            });

            ui.separator();
            // Footer
            ui.horizontal(|ui| {
                ui.label("状态:");
                ui.label(&self.status_text);
            });
        });
        
        // Request a repaint to check for new messages
        ctx.request_repaint();
    }
}
