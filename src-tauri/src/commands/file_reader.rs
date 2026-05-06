use base64::Engine;

#[derive(serde::Serialize)]
pub struct FileData {
    pub data: String,
    pub mime_type: String,
}

#[tauri::command]
pub fn read_file_as_data_url(path: String) -> Result<FileData, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mime_type = match ext.as_str() {
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "pcm" => "audio/pcm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };

    let data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);

    Ok(FileData { data, mime_type: mime_type.to_string() })
}
