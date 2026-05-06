use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const APP_DIR_NAME: &str = "com.minimax.music-studio";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Default)]
struct AppConfig {
    #[serde(rename = "data_path")]
    data_path: Option<String>,
    #[serde(rename = "minimax_api_key", skip_serializing_if = "Option::is_none")]
    minimax_api_key: Option<String>,
}

/// Returns the default app data directory (always in AppData)
pub fn get_default_app_data_dir() -> Result<PathBuf, String> {
    dirs::data_dir()
        .map(|p| p.join(APP_DIR_NAME))
        .ok_or_else(|| "Cannot find data directory".into())
}

/// Returns the path to config.json (always in default AppData location)
fn get_config_path() -> Result<PathBuf, String> {
    let dir = get_default_app_data_dir()?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join(CONFIG_FILE))
}

/// Reads the custom data path from config.json, returns None if not set
fn read_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(AppConfig::default());
    }
    let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

/// Writes the config.json with the given data path
fn write_config(data_path: Option<&str>) -> Result<(), String> {
    let config_path = get_config_path()?;
    let existing = read_config().ok();
    let config = AppConfig {
        data_path: data_path.map(|s| s.to_string()),
        minimax_api_key: existing.and_then(|c| c.minimax_api_key),
    };
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, content).map_err(|e| e.to_string())
}

/// Returns the effective data directory (custom if set, otherwise default)
pub fn get_effective_data_dir() -> Result<PathBuf, String> {
    let config = read_config()?;
    if let Some(ref custom_path) = config.data_path {
        let path = PathBuf::from(custom_path);
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        Ok(path)
    } else {
        let dir = get_default_app_data_dir()?;
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        Ok(dir)
    }
}

/// Returns the current data path string (for display in UI)
pub fn get_current_data_path() -> Result<String, String> {
    let dir = get_effective_data_dir()?;
    Ok(dir.to_string_lossy().to_string())
}

/// Sets a new data path and migrates all data
pub fn set_data_path(new_path: &str, _db: &crate::db::Database) -> Result<String, String> {
    let new_dir = PathBuf::from(new_path);
    let old_dir = get_effective_data_dir()?;

    // Validate new path is different
    let new_canonical = std::fs::canonicalize(&new_dir).unwrap_or_else(|_| new_dir.clone());
    let old_canonical = std::fs::canonicalize(&old_dir).unwrap_or_else(|_| old_dir.clone());
    if new_canonical == old_canonical {
        return Err("New path is the same as current path".into());
    }

    // Create target directory structure
    std::fs::create_dir_all(new_dir.join("audio")).map_err(|e| format!("Failed to create audio dir: {}", e))?;
    std::fs::create_dir_all(new_dir.join("covers")).map_err(|e| format!("Failed to create covers dir: {}", e))?;

    // Copy audio files
    let old_audio = old_dir.join("audio");
    let new_audio = new_dir.join("audio");
    if old_audio.exists() {
        copy_dir_contents(&old_audio, &new_audio)?;
    }

    // Copy cover images
    let old_covers = old_dir.join("covers");
    let new_covers = new_dir.join("covers");
    if old_covers.exists() {
        copy_dir_contents(&old_covers, &new_covers)?;
    }

    // Copy database
    let old_db = old_dir.join("music_studio.db");
    let new_db = new_dir.join("music_studio.db");
    if old_db.exists() {
        std::fs::copy(&old_db, &new_db).map_err(|e| format!("Failed to copy database: {}", e))?;
    }

    // Update paths in the NEW database (not the locked one)
    let old_dir_str = old_dir.to_string_lossy().to_string();
    let new_dir_str = new_dir.to_string_lossy().to_string();
    update_paths_in_db(&new_db, &old_dir_str, &new_dir_str)?;

    // Write new config pointer
    write_config(Some(new_path))?;

    // Old files are locked by the running process, skip cleanup.
    // They become stale after restart. User can delete manually.
    // Clean up what we can (audio/covers are not locked).
    let _ = std::fs::remove_dir_all(&old_audio);
    let _ = std::fs::remove_dir_all(&old_covers);
    let _ = std::fs::remove_file(&old_db);

    Ok(new_dir.to_string_lossy().to_string())
}

/// Opens the copied database and updates absolute file paths
fn update_paths_in_db(db_path: &PathBuf, old_base: &str, new_base: &str) -> Result<(), String> {
    let conn = rusqlite::Connection::open(db_path)
        .map_err(|e| format!("Failed to open new database: {}", e))?;
    conn.execute(
        "UPDATE music SET audio_path = REPLACE(audio_path, ?1, ?2) WHERE audio_path LIKE ?3",
        rusqlite::params![old_base, new_base, format!("{}%", old_base)],
    ).map_err(|e| format!("Failed to update audio paths: {}", e))?;
    conn.execute(
        "UPDATE music SET cover_image_path = REPLACE(cover_image_path, ?1, ?2) WHERE cover_image_path LIKE ?3",
        rusqlite::params![old_base, new_base, format!("{}%", old_base)],
    ).map_err(|e| format!("Failed to update cover paths: {}", e))?;
    Ok(())
}

fn copy_dir_contents(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    let entries = std::fs::read_dir(src).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_name = entry.file_name();
        let src_path = entry.path();
        let dst_path = dst.join(&file_name);
        if src_path.is_file() {
            std::fs::copy(&src_path, &dst_path).map_err(|e| format!("Failed to copy {:?}: {}", file_name, e))?;
        }
    }
    Ok(())
}

/// Returns the API key by priority: config.json > MINIMAX_API_KEY env var
pub fn get_api_key() -> Result<String, String> {
    if let Ok(config) = read_config() {
        if let Some(ref key) = config.minimax_api_key {
            if !key.is_empty() {
                return Ok(key.clone());
            }
        }
    }
    std::env::var("MINIMAX_API_KEY")
        .map_err(|_| "MINIMAX_API_KEY not found. Please set it in Settings.".into())
}

/// Saves the API key to config.json
pub fn save_api_key(key: &str) -> Result<(), String> {
    let config_path = get_config_path()?;
    let mut config = read_config().unwrap_or_default();
    config.minimax_api_key = Some(key.to_string());
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, content).map_err(|e| e.to_string())
}

/// Returns whether an API key is configured (either in config.json or env)
pub fn get_api_key_configured() -> bool {
    get_api_key().is_ok()
}
