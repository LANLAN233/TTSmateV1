mod config;
mod api_client;
mod error;

use std::sync::{mpsc, Arc};
use eframe::egui;
use tokio::runtime::Runtime;
use rodio::{OutputStream, OutputStreamHandle, Decoder, Sink};
use rodio::cpal::traits::{HostTrait, DeviceTrait};

use crate::api_client::ApiClient;
use crate::config::{Config, load_config, VOICES, SoundboardItem};

// --- App State & Messages ---

enum AppState {
    Idle,
    GeneratingText,
    SynthesizingAudio,
}

impl AppState {
    fn to_string(&self) -> String {
        match self {
            AppState::Idle => "就绪".to_string(),
            AppState::GeneratingText => "正在生成文本...".to_string(),
            AppState::SynthesizingAudio => "正在合成语音...".to_string(),
        }
    }
}

enum UIMessage {
    UpdateState(AppState),
    SetResponseText(String),
    PlayTts(Vec<u8>),
    PlaySound(Vec<u8>),
    Error(String),
}

// --- Main App Struct ---

struct TTSApp {
    rt: Runtime,
    prompt_text: String,
    response_text: String,
    status_text: String,
    config: Arc<Config>,
    api_client: Arc<ApiClient>,
    ui_sender: mpsc::Sender<UIMessage>,
    ui_receiver: mpsc::Receiver<UIMessage>,
    
    // --- Audio State ---
    audio_devices: Vec<rodio::cpal::Device>,
    audio_device_names: Vec<String>,
    selected_device_index: usize,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    tts_sink: Sink,
    sound_sinks: Vec<Sink>,
    last_tts_audio: Option<Arc<Vec<u8>>>,

    // --- Audio Controls ---
    master_volume: f32,
    tts_volume: f32,
    sound_volume: f32,
    is_tts_paused: bool,
    repeat_tts: bool,

    // --- TTS parameters ---
    speed: i32,
    pitch: i32,
    volume: i32,
    person: i32,
    // --- AI control ---
    use_deepseek: bool,
    selected_prompt_index: usize,
    custom_prompt: String,
    // --- Soundboard ---
    soundboard_items: Vec<SoundboardItem>,
}

impl TTSApp {
    fn new(config: Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (ui_sender, ui_receiver) = mpsc::channel();
        let speed = config.app_settings.speed;
        let pitch = config.app_settings.pitch;
        let volume = config.app_settings.volume;
        let person = config.app_settings.person;
        let soundboard_items = config.soundboard.clone();

        // --- Audio Device Initialization ---
        let host = rodio::cpal::default_host();
        let devices = host.output_devices()?.collect::<Vec<_>>();
        let device_names = devices.iter().map(|d| d.name().unwrap_or_else(|_| "未知设备".to_string())).collect();
        let default_device = host.default_output_device().ok_or("未找到默认音频输出设备")?;
        
        let selected_device_index = devices.iter().position(|d| d.name().ok() == default_device.name().ok()).unwrap_or(0);

        let (_stream, stream_handle) = OutputStream::try_from_device(&devices[selected_device_index])?;
        let tts_sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            rt: tokio::runtime::Builder::new_multi_thread().enable_all().build()?,
            prompt_text: "你好".to_string(),
            response_text: "".to_string(),
            status_text: AppState::Idle.to_string(),
            custom_prompt: config.ai_settings.default_prompt.clone(),
            config: Arc::new(config),
            api_client: Arc::new(ApiClient::new()),
            ui_sender,
            ui_receiver,
            audio_devices: devices,
            audio_device_names: device_names,
            selected_device_index,
            _stream,
            stream_handle,
            tts_sink,
            sound_sinks: Vec::new(),
            last_tts_audio: None,
            master_volume: 1.0,
            tts_volume: 1.0,
            sound_volume: 0.5,
            is_tts_paused: false,
            repeat_tts: false,
            speed,
            pitch,
            volume,
            person,
            use_deepseek: true,
            selected_prompt_index: 0,
            soundboard_items,
        })
    }

    fn play_tts_data(&self, data: Arc<Vec<u8>>) {
        if let Ok(source) = Decoder::new(std::io::Cursor::new(data)) {
            self.tts_sink.clear();
            self.tts_sink.append(source);
            self.tts_sink.play();
        } else {
            log::error!("解码TTS音频失败");
        }
    }

    fn play_sound_data(&mut self, data: Vec<u8>) {
        if let Ok(source) = Decoder::new(std::io::Cursor::new(data)) {
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.append(source);
                self.sound_sinks.push(sink);
            }
        } else {
            log::error!("解码音效失败");
        }
    }

    fn change_output_device(&mut self, device_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if device_index == self.selected_device_index || device_index >= self.audio_devices.len() {
            return Ok(());
        }
        
        self.tts_sink.stop();
        self.sound_sinks.clear();

        let device = &self.audio_devices[device_index];
        let (_stream, stream_handle) = OutputStream::try_from_device(device)?;
        self.tts_sink = Sink::try_new(&stream_handle)?;
        
        self._stream = _stream;
        self.stream_handle = stream_handle;
        self.selected_device_index = device_index;

        Ok(())
    }

    fn handle_ui_messages(&mut self) {
        while let Ok(msg) = self.ui_receiver.try_recv() {
            match msg {
                UIMessage::UpdateState(state) => self.status_text = state.to_string(),
                UIMessage::SetResponseText(text) => self.response_text = text,
                UIMessage::Error(e) => self.status_text = format!("错误: {}", e),
                UIMessage::PlayTts(audio_data) => {
                    self.status_text = AppState::Idle.to_string();
                    self.is_tts_paused = false;
                    let audio_arc = Arc::new(audio_data);
                    self.last_tts_audio = Some(audio_arc.clone());
                    self.play_tts_data(audio_arc);
                }
                UIMessage::PlaySound(audio_data) => {
                    self.play_sound_data(audio_data);
                }
            }
        }
    }

    fn start_generation_task(&mut self) {
        let sender = self.ui_sender.clone();
        let api_client = self.api_client.clone();
        let config = self.config.clone();
        let prompt_text = self.prompt_text.clone();
        let speed = self.speed;
        let pitch = self.pitch;
        let volume = self.volume;
        let person = self.person;
        let use_deepseek = self.use_deepseek;

        let system_prompt = if self.selected_prompt_index == self.config.ai_settings.prompts.len() {
            self.custom_prompt.clone()
        } else {
            self.config.ai_settings.prompts[self.selected_prompt_index].template.clone()
        };

        self.rt.spawn(async move {
            let text_to_speak = if use_deepseek {
                sender.send(UIMessage::UpdateState(AppState::GeneratingText)).unwrap();
                match api_client.call_deepseek_api(&config.api_keys.deepseek_api_key, &system_prompt, &prompt_text).await {
                    Ok(text) => {
                        sender.send(UIMessage::SetResponseText(text.clone())).unwrap();
                        text
                    }
                    Err(e) => {
                        sender.send(UIMessage::Error(format!("DeepSeek: {}", e))).unwrap();
                        return;
                    }
                }
            } else {
                sender.send(UIMessage::SetResponseText(prompt_text.clone())).unwrap();
                prompt_text
            };

            if text_to_speak.trim().is_empty() {
                sender.send(UIMessage::Error("无有效文本".to_string())).unwrap();
                return;
            }

            sender.send(UIMessage::UpdateState(AppState::SynthesizingAudio)).unwrap();
            match api_client.call_baidu_tts_api(&config.api_keys, &text_to_speak, speed, pitch, volume, person).await {
                Ok(audio_data) => sender.send(UIMessage::PlayTts(audio_data)).unwrap(),
                Err(e) => sender.send(UIMessage::Error(format!("BaiduTTS: {}", e))).unwrap(),
            }
        });
    }
}

// --- Eframe App Implementation ---

impl eframe::App for TTSApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Process background messages & state updates ---
        self.handle_ui_messages();
        self.sound_sinks.retain(|s| !s.empty());

        if self.repeat_tts && self.tts_sink.empty() {
            if let Some(audio) = self.last_tts_audio.clone() {
                self.play_tts_data(audio);
            }
        }
        
        self.tts_sink.set_volume(self.master_volume * self.tts_volume);
        for sink in &self.sound_sinks {
            sink.set_volume(self.master_volume * self.sound_volume);
        }

        let mut new_device_index_to_set = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TTSmate");
            ui.separator();

            // --- AI Controls ---
            ui.collapsing("AI 设置", |ui| {
                ui.checkbox(&mut self.use_deepseek, "使用 DeepSeek 生成文案");
                let prompts = &self.config.ai_settings.prompts;
                let mut prompt_names: Vec<&str> = prompts.iter().map(|p| p.name.as_str()).collect();
                prompt_names.push("自定义模板");
                ui.horizontal(|ui| {
                    ui.label("提示词模板:");
                    egui::ComboBox::from_id_source("prompt_template_combobox")
                        .selected_text(prompt_names[self.selected_prompt_index])
                        .show_ui(ui, |ui| {
                            for (i, name) in prompt_names.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_prompt_index, i, *name);
                            }
                        });
                });
                if self.selected_prompt_index == prompts.len() {
                    ui.label("自定义提示词:");
                    ui.text_edit_multiline(&mut self.custom_prompt);
                }
            });
            ui.separator();

            // --- Main Input ---
            ui.horizontal(|ui| {
                ui.label("输入话题/文本:");
                ui.text_edit_singleline(&mut self.prompt_text);
            });

            let is_running_task = self.status_text != AppState::Idle.to_string() && !self.status_text.starts_with("错误:");
            if ui.add_enabled(!is_running_task, egui::Button::new("生成并播放")).clicked() {
                self.start_generation_task();
            }

            // --- Audio Playback Controls ---
            ui.collapsing("音频设置", |ui| {
                // Device Selection
                let selected_name = self.audio_device_names[self.selected_device_index].clone();
                egui::ComboBox::from_label("输出设备")
                    .selected_text(selected_name)
                    .show_ui(ui, |ui| {
                        for (i, device_name) in self.audio_device_names.iter().enumerate() {
                            if ui.selectable_label(self.selected_device_index == i, device_name).clicked() {
                                new_device_index_to_set = Some(i);
                            }
                        }
                    });
                
                ui.separator();
                
                // Volume Controls
                ui.add(egui::Slider::new(&mut self.master_volume, 0.0..=1.5).text("主音量"));
                ui.add(egui::Slider::new(&mut self.tts_volume, 0.0..=1.5).text("语音音量"));
                ui.add(egui::Slider::new(&mut self.sound_volume, 0.0..=1.5).text("音效音量"));
                
                ui.separator();

                // Play/Pause/Repeat Controls
                ui.horizontal(|ui| {
                    let tts_button_text = if self.is_tts_paused { "▶ 播放" } else { "⏸ 暂停" };
                    if ui.add_enabled(!self.tts_sink.empty(), egui::Button::new(tts_button_text)).clicked() {
                        if self.tts_sink.is_paused() {
                            self.tts_sink.play();
                            self.is_tts_paused = false;
                        } else {
                            self.tts_sink.pause();
                            self.is_tts_paused = true;
                        }
                    }
                    ui.checkbox(&mut self.repeat_tts, "循环播放");
                });
            });
            ui.separator();

            // --- TTS Parameter Controls ---
            ui.collapsing("语音参数", |ui| {
                ui.add(egui::Slider::new(&mut self.speed, 0..=15).text("语速"));
                ui.add(egui::Slider::new(&mut self.pitch, 0..=15).text("音调"));
                ui.add(egui::Slider::new(&mut self.volume, 0..=15).text("音量"));
                egui::ComboBox::from_label("发音人")
                    .selected_text(VOICES.iter().find(|&&(_, p)| p == self.person).unwrap_or(&("未知",-1)).0)
                    .show_ui(ui, |ui| {
                        for (name, person_code) in VOICES.iter() {
                            ui.selectable_value(&mut self.person, *person_code, *name);
                        }
                    });
            });
            ui.separator();

            // --- Soundboard ---
            ui.collapsing("音效板", |ui| {
                if ui.button("➕ 添加音效").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("音频文件", &["mp3", "wav", "ogg", "flac"])
                        .pick_file()
                    {
                        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("未知音效").to_string();
                        self.soundboard_items.push(SoundboardItem {
                            name,
                            path: path.to_string_lossy().to_string(),
                        });
                    }
                }
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    for sound_item in self.soundboard_items.iter() {
                        if ui.button(&sound_item.name).clicked() {
                            let path = sound_item.path.clone();
                            let sender = self.ui_sender.clone();
                            self.rt.spawn(async move {
                                match tokio::fs::read(&path).await {
                                    Ok(data) => {
                                        let _ = sender.send(UIMessage::PlaySound(data));
                                    }
                                    Err(e) => {
                                        log::error!("读取音效文件 '{}' 失败: {}", path, e);
                                    }
                                }
                            });
                        }
                    }
                });
            });
            ui.separator();

            // --- AI Response Display ---
            ui.horizontal(|ui| {
                ui.label("AI 生成文本:");
                let save_button_enabled = self.last_tts_audio.is_some();
                if ui.add_enabled(save_button_enabled, egui::Button::new("💾 保存音频")).clicked() {
                    if let Some(audio_data) = self.last_tts_audio.clone() {
                        let rt = self.rt.clone();
                        let sender = self.ui_sender.clone();
                        self.status_text = "准备保存...".to_string();
                        std::thread::spawn(move || {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("MPEG Audio", &["mp3"])
                                .set_file_name("tts_audio.mp3")
                                .save_file()
                            {
                                rt.spawn(async move {
                                    match tokio::fs::write(&path, &*audio_data).await {
                                        Ok(_) => { let _ = sender.send(UIMessage::UpdateState(AppState::Idle)); },
                                        Err(e) => { let _ = sender.send(UIMessage::Error(format!("保存失败: {}", e))); }
                                    }
                                });
                            } else {
                                // User cancelled dialog
                                let _ = sender.send(UIMessage::UpdateState(AppState::Idle));
                            }
                        });
                    }
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.response_text);
            });
            ui.separator();

            // --- Footer / Status ---
            ui.label(&self.status_text);
        });

        if let Some(index) = new_device_index_to_set {
            if let Err(e) = self.change_output_device(index) {
                log::error!("切换音频设备失败: {}", e);
                self.status_text = "切换音频设备失败".to_string();
            }
        }

        ctx.request_repaint();
    }
}

// --- Main Function ---

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 700.0]),
        ..Default::default()
    };
    let config = load_config().expect("加载 config.toml 失败");
    
    eframe::run_native(
        "TTSmate v1.2.1",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            let font_paths = ["C:/Windows/Fonts/msyh.ttf", "C:/Windows/Fonts/deng.ttf", "C:/Windows/Fonts/simhei.ttf", "C:/Windows/Fonts/simsun.ttc"];
            for path in font_paths {
                if let Ok(font_data) = std::fs::read(path) {
                    let font_name = path.split('/').last().unwrap_or("unknown_font").to_string();
                    fonts.font_data.insert(font_name.clone(), egui::FontData::from_owned(font_data));
                    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, font_name.clone());
                    fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, font_name);
                    break;
                }
            }
            cc.egui_ctx.set_fonts(fonts);

            match TTSApp::new(config) {
                Ok(app) => Ok(Box::new(app)),
                Err(e) => {
                    log::error!("应用初始化失败: {}", e);
                    Err(e)
                }
            }
        }),
    )
    .unwrap();
} 