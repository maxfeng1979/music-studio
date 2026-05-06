use std::path::PathBuf;

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    crate::config::get_effective_data_dir()
}

pub fn get_audio_dir() -> Result<PathBuf, String> {
    let dir = get_app_data_dir()?.join("audio");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn get_cover_dir() -> Result<PathBuf, String> {
    let dir = get_app_data_dir()?.join("covers");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn save_audio(hex_data: &str, format: &str) -> Result<PathBuf, String> {
    let bytes = hex::decode(hex_data).map_err(|e| e.to_string())?;
    let filename = format!("{}.{}", uuid::Uuid::new_v4(), format);
    let path = get_audio_dir()?.join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn save_cover_image(base64_data: &str, music_id: i64) -> Result<PathBuf, String> {
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, base64_data)
        .map_err(|e| e.to_string())?;
    let filename = format!("cover_{}.png", music_id);
    let path = get_cover_dir()?.join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn delete_audio(path: &str) -> Result<(), String> {
    std::fs::remove_file(path).map_err(|e| e.to_string())
}