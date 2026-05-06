use serde::{Deserialize, Deserializer, Serialize};

/// Deserialize a value that may be a string or a number into i32
fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Int(i32),
        Str(String),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::Int(i) => Ok(i),
        StringOrInt::Str(s) => s.parse().map_err(serde::de::Error::custom),
    }
}

const BASE_URL: &str = "https://api.minimaxi.com";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioSettings {
    pub sample_rate: u32,
    pub bitrate: u32,
    pub format: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateMusicRequest {
    pub model: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub is_instrumental: bool,
    pub lyrics_optimizer: bool,
    pub output_format: String,
    pub stream: bool,
    #[serde(rename = "audio_setting")]
    pub audio_setting: AudioSettings,
    pub aigc_watermark: bool,
}

#[derive(Debug, Deserialize)]
pub struct MusicGenerationResponse {
    pub data: MusicData,
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    #[serde(rename = "extra_info")]
    pub extra_info: Option<ExtraInfo>,
    #[serde(rename = "base_resp")]
    pub base_resp: BaseResp,
}

#[derive(Debug, Deserialize)]
pub struct MusicData {
    pub status: i32,
    pub audio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExtraInfo {
    #[serde(rename = "music_duration")]
    pub music_duration: Option<i64>,
    #[serde(rename = "music_sample_rate")]
    pub music_sample_rate: Option<i64>,
    #[serde(rename = "music_channel")]
    pub music_channel: Option<i64>,
    pub bitrate: Option<i64>,
    #[serde(rename = "music_size")]
    pub music_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BaseResp {
    #[serde(rename = "status_code")]
    pub status_code: i32,
    #[serde(rename = "status_msg")]
    pub status_msg: String,
}

#[derive(Debug, Serialize)]
pub struct ImageGenerationRequest {
    pub model: String,
    pub prompt: String,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: String,
    #[serde(rename = "response_format")]
    pub response_format: String,
    pub n: u8,
}

#[derive(Debug, Deserialize)]
pub struct ImageGenerationResponse {
    pub data: ImageData,
    pub metadata: Option<ImageMetadata>,
    pub id: Option<String>,
    #[serde(rename = "base_resp")]
    pub base_resp: Option<BaseResp>,
}

#[derive(Debug, Deserialize)]
pub struct ImageData {
    #[serde(rename = "image_urls")]
    pub image_urls: Option<Vec<String>>,
    #[serde(rename = "image_base64")]
    pub image_base64: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ImageMetadata {
    #[serde(rename = "success_count", deserialize_with = "deserialize_string_or_int")]
    pub success_count: i32,
    #[serde(rename = "failed_count", deserialize_with = "deserialize_string_or_int")]
    pub failed_count: i32,
}

pub struct MinimaxClient {
    api_key: String,
    client: reqwest::Client,
}

impl MinimaxClient {
    pub fn new(api_key: &str) -> Result<Self, String> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        })
    }

    pub async fn generate_music(&self, req: GenerateMusicRequest) -> Result<MusicGenerationResponse, String> {
        let url = format!("{}/v1/music_generation", BASE_URL);
        let body = serde_json::to_string(&req).map_err(|e| e.to_string())?;

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let resp: MusicGenerationResponse = response
            .json()
            .await
            .map_err(|e| e.to_string())?;

        if resp.base_resp.status_code != 0 {
            return Err(format!("API error {}: {}", resp.base_resp.status_code, resp.base_resp.status_msg));
        }

        Ok(resp)
    }

    pub async fn generate_image(&self, req: ImageGenerationRequest) -> Result<ImageGenerationResponse, String> {
        let url = format!("{}/v1/image_generation", BASE_URL);
        let body = serde_json::to_string(&req).map_err(|e| e.to_string())?;

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let raw = response.text().await.map_err(|e| e.to_string())?;
        eprintln!("[minimax] image API raw response: {}", &raw[..raw.len().min(2000)]);

        let resp: ImageGenerationResponse = serde_json::from_str(&raw)
            .map_err(|e| format!("Failed to parse image response: {}. Raw: {}", e, &raw[..raw.len().min(500)]))?;

        if let Some(ref base) = resp.base_resp {
            if base.status_code != 0 {
                return Err(format!("API error {}: {}", base.status_code, base.status_msg));
            }
        }

        Ok(resp)
    }
}