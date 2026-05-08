use serde::Deserialize;
use tauri::{State, Emitter};
use crate::db::{Database, NewMusic, MusicRecord as DbMusicRecord};
use crate::api::minimax::{MinimaxClient, GenerateMusicRequest, AudioSettings};
use crate::audio::storage;

#[derive(Debug, Deserialize, Clone)]
pub struct GenerateMusicParams {
    #[serde(default = "default_title")]
    pub title: String,
    pub model: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    #[serde(default)]
    pub is_instrumental: bool,
    #[serde(default)]
    pub lyrics_optimizer: bool,
    #[serde(default = "default_output_format")]
    pub output_format: String,
    #[serde(default)]
    pub stream: bool,
    pub audio_setting: AudioSettings,
    #[serde(default)]
    pub aigc_watermark: bool,
    pub ai_description: Option<String>,
}

fn default_output_format() -> String {
    "hex".to_string()
}

fn default_title() -> String {
    "Untitled".to_string()
}

#[tauri::command]
pub async fn generate_music(params: GenerateMusicParams, db: State<'_, Database>) -> Result<DbMusicRecord, String> {
    let api_key = crate::config::get_api_key()?;
    let client = MinimaxClient::new(&api_key)?;

    let req = GenerateMusicRequest {
        model: params.model.clone(),
        prompt: params.prompt.clone(),
        lyrics: params.lyrics.clone(),
        is_instrumental: params.is_instrumental,
        lyrics_optimizer: params.lyrics_optimizer,
        output_format: params.output_format.clone(),
        stream: params.stream,
        audio_setting: params.audio_setting.clone(),
        aigc_watermark: params.aigc_watermark,
    };

    let resp = client.generate_music(req).await?;

    let audio_data = resp.data.audio.unwrap_or_default();
    let audio_path = storage::save_audio(&audio_data, &params.audio_setting.format)?;

    let file_size = std::fs::metadata(&audio_path)
        .map(|m| m.len() as i64)
        .ok();

    let extra = resp.extra_info;
    let duration_ms = extra.as_ref().and_then(|e| e.music_duration);
    let sample_rate = extra.as_ref().and_then(|e| e.music_sample_rate);
    let bitrate = extra.as_ref().and_then(|e| e.bitrate);

    let new_music = NewMusic {
        title: params.title.clone(),
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: audio_path.to_string_lossy().to_string(),
        cover_image_path: None,
        duration_ms,
        file_size,
        sample_rate: sample_rate.map(|s| s as i64),
        bitrate: bitrate.map(|b| b as i64),
        is_instrumental: params.is_instrumental,
        ai_description: params.ai_description,
    };

    db.insert_music(&new_music).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_music_streaming(
    params: GenerateMusicParams,
    db: State<'_, Database>,
    window: tauri::Window,
) -> Result<(), String> {
    let api_key = crate::config::get_api_key()?;
    let client = MinimaxClient::new(&api_key)?;

    let req = GenerateMusicRequest {
        model: params.model.clone(),
        prompt: params.prompt.clone(),
        lyrics: params.lyrics.clone(),
        is_instrumental: params.is_instrumental,
        lyrics_optimizer: params.lyrics_optimizer,
        output_format: params.output_format.clone(),
        stream: true,
        audio_setting: params.audio_setting.clone(),
        aigc_watermark: params.aigc_watermark,
    };

    window.emit("music-generation-started", ()).map_err(|e| e.to_string())?;

    let resp = client.generate_music(req).await?;

    window.emit("music-generation-progress", 50).map_err(|e| e.to_string())?;

    let audio_data = resp.data.audio.unwrap_or_default();
    let audio_path = storage::save_audio(&audio_data, &params.audio_setting.format)?;

    window.emit("music-generation-progress", 80).map_err(|e| e.to_string())?;

    let new_music = NewMusic {
        title: params.title.clone(),
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: audio_path.to_string_lossy().to_string(),
        cover_image_path: None,
        duration_ms: resp.extra_info.as_ref().and_then(|e| e.music_duration),
        file_size: std::fs::metadata(&audio_path).map(|m| m.len() as i64).ok(),
        sample_rate: Some(params.audio_setting.sample_rate as i64),
        bitrate: Some(params.audio_setting.bitrate as i64),
        is_instrumental: params.is_instrumental,
        ai_description: params.ai_description,
    };

    let record = db.insert_music(&new_music).map_err(|e| e.to_string())?;

    window.emit("music-generation-complete", record.id).map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PreviewMusicResult {
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub is_instrumental: bool,
    pub ai_description: Option<String>,
}

#[tauri::command]
pub async fn preview_music(params: GenerateMusicParams) -> Result<PreviewMusicResult, String> {
    let api_key = crate::config::get_api_key()?;
    let client = MinimaxClient::new(&api_key)?;

    let req = GenerateMusicRequest {
        model: params.model.clone(),
        prompt: params.prompt.clone(),
        lyrics: params.lyrics.clone(),
        is_instrumental: params.is_instrumental,
        lyrics_optimizer: params.lyrics_optimizer,
        output_format: params.output_format.clone(),
        stream: params.stream,
        audio_setting: params.audio_setting.clone(),
        aigc_watermark: params.aigc_watermark,
    };

    let resp = client.generate_music(req).await?;

    let audio_data = resp.data.audio.unwrap_or_default();
    let audio_path = storage::save_audio(&audio_data, &params.audio_setting.format)?;

    let file_size = std::fs::metadata(&audio_path)
        .map(|m| m.len() as i64)
        .ok();

    let extra = resp.extra_info;
    let duration_ms = extra.as_ref().and_then(|e| e.music_duration);
    let sample_rate = extra.as_ref().and_then(|e| e.music_sample_rate);
    let bitrate = extra.as_ref().and_then(|e| e.bitrate);

    Ok(PreviewMusicResult {
        title: params.title,
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: audio_path.to_string_lossy().to_string(),
        duration_ms,
        file_size,
        sample_rate: sample_rate.map(|s| s as i64),
        bitrate: bitrate.map(|b| b as i64),
        is_instrumental: params.is_instrumental,
        ai_description: params.ai_description,
    })
}

#[derive(Debug, serde::Deserialize)]
pub struct SaveMusicParams {
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub is_instrumental: bool,
    pub ai_description: Option<String>,
}

#[tauri::command]
pub async fn save_music_to_library(params: SaveMusicParams, db: State<'_, Database>) -> Result<DbMusicRecord, String> {
    let new_music = NewMusic {
        title: params.title,
        prompt: params.prompt,
        lyrics: params.lyrics,
        model: params.model,
        audio_path: params.audio_path,
        cover_image_path: None,
        duration_ms: params.duration_ms,
        file_size: params.file_size,
        sample_rate: params.sample_rate,
        bitrate: params.bitrate,
        is_instrumental: params.is_instrumental,
        ai_description: params.ai_description,
    };
    db.insert_music(&new_music).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn discard_preview(audio_path: String) -> Result<(), String> {
    storage::delete_audio(&audio_path)
}
