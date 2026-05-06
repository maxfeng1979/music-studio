use tauri::{State, Emitter};
use crate::db::Database;
use crate::api::minimax::{MinimaxClient, ImageGenerationRequest};
use crate::audio::storage;

#[derive(Debug, serde::Deserialize)]
pub struct GenerateCoverParams {
    pub music_id: i64,
    pub prompt: String,
    #[serde(default = "default_aspect_ratio")]
    pub aspect_ratio: String,
    #[serde(default = "default_response_format")]
    pub response_format: String,
    #[serde(default)]
    pub n: u8,
}

fn default_aspect_ratio() -> String {
    "1:1".to_string()
}

fn default_response_format() -> String {
    "base64".to_string()
}

#[tauri::command]
pub async fn generate_cover_image(params: GenerateCoverParams, db: State<'_, Database>) -> Result<String, String> {
    let api_key = crate::config::get_api_key()?;
    let client = MinimaxClient::new(&api_key)?;

    let req = ImageGenerationRequest {
        model: "image-01".to_string(),
        prompt: params.prompt,
        aspect_ratio: params.aspect_ratio,
        response_format: params.response_format.clone(),
        n: if params.n > 0 { params.n } else { 1 },
    };

    let resp = client.generate_image(req).await?;

    let image_data = if params.response_format == "base64" {
        resp.data.image_base64.and_then(|v| v.into_iter().next())
    } else {
        let url = resp.data.image_urls.and_then(|v| v.into_iter().next()).ok_or("No URL returned")?;
        let bytes = reqwest::get(&url).await.map_err(|e| e.to_string())?
            .bytes().await.map_err(|e| e.to_string())?;
        Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes))
    }.ok_or("No image data returned")?;

    let cover_path = storage::save_cover_image(&image_data, params.music_id)?;
    db.update_cover_path(params.music_id, &cover_path.to_string_lossy()).map_err(|e| e.to_string())?;

    Ok(cover_path.to_string_lossy().to_string())
}