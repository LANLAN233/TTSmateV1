use crate::config::{ApiKeys, AppSettings};
use crate::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// --- DeepSeek Structures ---
#[derive(Serialize)]
struct DeepSeekRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

// --- Baidu TTS Structures ---
#[derive(Deserialize, Debug)]
struct BaiduTokenResponse {
    access_token: String,
}

// --- API Client ---
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn call_deepseek_api(
        &self,
        api_key: &str,
        user_prompt: &str,
    ) -> Result<String, AppError> {
        let request_payload = DeepSeekRequest {
            model: "deepseek-chat",
            messages: vec![
                Message {
                    role: "system",
                    content: "你是一个为TTS语音合成生成文本的助手，请将回答限制在100个汉字以内。",
                },
                Message {
                    role: "user",
                    content: user_prompt,
                },
            ],
        };

        let response: DeepSeekResponse = self
            .client
            .post("https://api.deepseek.com/chat/completions")
            .bearer_auth(api_key)
            .json(&request_payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }

    async fn get_baidu_access_token(
        &self,
        api_key: &str,
        secret_key: &str,
    ) -> Result<String, AppError> {
        let url = "https://aip.baidubce.com/oauth/2.0/token";
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", api_key),
            ("client_secret", secret_key),
        ];

        let response: BaiduTokenResponse = self
            .client
            .post(url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.access_token)
    }

    pub async fn call_baidu_tts_api(
        &self,
        api_keys: &ApiKeys,
        settings: &AppSettings,
        text: &str,
    ) -> Result<Vec<u8>, AppError> {
        let access_token = self
            .get_baidu_access_token(&api_keys.baidu_api_key, &api_keys.baidu_secret_key)
            .await?;

        let url = "https://tsn.baidu.com/text2audio";
        
        let spd = &settings.speed.to_string();
        let pit = &settings.pitch.to_string();
        let vol = &settings.volume.to_string();
        let per = &settings.person.to_string();

        let params = [
            ("tex", text),
            ("tok", &access_token),
            ("cuid", "ttsmate_rust_client"),
            ("ctp", "1"),
            ("lan", "zh"),
            ("spd", spd),
            ("pit", pit),
            ("vol", vol),
            ("per", per),
            ("aue", "3"), // aue=3 for mp3 format
        ];

        let response = self.client.post(url).form(&params).send().await?;
        
        // Check if the response is an error JSON or audio data
        let content_type = response.headers().get("Content-Type").cloned();
        let audio_data = response.bytes().await?;

        if let Some(ct) = content_type {
            if ct.to_str().unwrap_or("").contains("application/json") {
                let error_text = String::from_utf8_lossy(&audio_data).to_string();
                log::error!("Baidu TTS Error: {}", error_text);
                return Err(AppError::BaiduApi(error_text));
            }
        }

        Ok(audio_data.to_vec())
    }
} 