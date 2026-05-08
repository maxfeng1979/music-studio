mod commands;
mod config;
mod db;
mod api;
mod audio;

use commands::{music, library, image, file_reader, ai_assist, settings};
use db::Database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            music::generate_music,
            music::generate_music_streaming,
            music::preview_music,
            music::save_music_to_library,
            music::discard_preview,
            image::generate_cover_image,
            library::get_all_music,
            library::get_music,
            library::update_music_metadata,
            library::delete_music,
            file_reader::read_file_as_data_url,
            ai_assist::generate_music_ideas,
            ai_assist::generate_cover_prompt,
            settings::get_data_path,
            settings::set_data_path,
            settings::save_api_key,
            settings::get_api_key_status,
            settings::test_api_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}