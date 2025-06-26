/*!
 * TTSmate集成测试
 */

use ttsmate::config::AppConfig;
use ttsmate::tts::TTSClient;
use ttsmate::ai::AIContentGenerator;
use ttsmate::soundboard::SoundBoard;
use ttsmate::audio::AudioRouter;

#[tokio::test]
async fn test_config_loading() {
    // 测试配置加载
    let config = AppConfig::default();
    assert_eq!(config.tts.server_url, "http://192.168.11.153:8080");
    assert_eq!(config.audio.sample_rate, 44100);
    assert_eq!(config.ui.font_size, 14.0);
}

#[tokio::test]
async fn test_tts_client_creation() {
    // 测试TTS客户端创建
    let config = AppConfig::default();
    let result = TTSClient::new(config.tts);
    assert!(result.is_ok());
    
    let client = result.unwrap();
    let voices = client.get_voices().await;
    assert!(voices.is_ok());
    assert!(!voices.unwrap().is_empty());
}

#[tokio::test]
async fn test_ai_generator_creation() {
    // 测试AI生成器创建
    let config = AppConfig::default();
    
    // 使用空API密钥测试
    let mut ai_config = config.ai;
    ai_config.api_key = "test_key".to_string();
    
    let result = AIContentGenerator::new(ai_config);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_soundboard_creation() {
    // 测试音效板创建
    let result = SoundBoard::new();
    assert!(result.is_ok());
    
    let soundboard = result.unwrap();
    let stats = soundboard.get_stats();
    assert_eq!(stats.total_sounds, 0);
    assert!(stats.total_categories > 0); // 应该有默认分类
}

#[tokio::test]
async fn test_audio_router_creation() {
    // 测试音频路由器创建
    let config = AppConfig::default();
    let result = AudioRouter::new(config.audio);
    assert!(result.is_ok());
    
    let router = result.unwrap();
    let stats = router.get_audio_stats();
    assert!(stats.input_devices > 0);
    assert!(stats.output_devices > 0);
}

#[test]
fn test_error_types() {
    // 测试错误类型
    use ttsmate::error::AppError;
    
    let tts_error = AppError::ai("测试AI错误");
    assert!(matches!(tts_error, AppError::AI(_)));
    
    let audio_error = AppError::audio("测试音频错误");
    assert!(matches!(audio_error, AppError::Audio(_)));
    
    let soundboard_error = AppError::soundboard("测试音效板错误");
    assert!(matches!(soundboard_error, AppError::SoundBoard(_)));
}

#[test]
fn test_version_info() {
    // 测试版本信息
    use ttsmate::{VERSION, APP_NAME, APP_DESCRIPTION};
    
    assert_eq!(VERSION, "1.0.0");
    assert_eq!(APP_NAME, "TTSmate V1");
    assert_eq!(APP_DESCRIPTION, "智能语音合成客户端");
}
