use crate::config;
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub async fn get_data_path() -> Result<String, String> {
    config::get_current_data_path()
}

#[tauri::command]
pub async fn set_data_path(new_path: String, db: State<'_, Database>) -> Result<String, String> {
    config::set_data_path(&new_path, &db)
}

#[tauri::command]
pub async fn save_api_key(key: String) -> Result<(), String> {
    config::save_api_key(&key)
}

#[tauri::command]
pub fn get_api_key_status() -> Result<bool, String> {
    Ok(config::get_api_key_configured())
}

#[tauri::command]
pub async fn test_api_connection(key: String) -> Result<String, String> {
    let client = crate::api::anthropic::AnthropicClient::new(&key)?;
    let resp = client.chat(
        "You are a connection test assistant. Respond with only the word OK if you receive this.",
        "hi"
    ).await?;
    if resp.trim().to_uppercase().contains("OK") {
        Ok("连接成功！".to_string())
    } else {
        Ok("连接正常".to_string())
    }
}
