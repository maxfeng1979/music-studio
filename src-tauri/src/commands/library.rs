use tauri::State;
use crate::db::{Database, MusicRecord};

#[tauri::command]
pub fn get_all_music(
    db: State<Database>,
    sort_by: Option<String>,
    filter_tag: Option<String>,
    filter_instrumental: Option<bool>,
) -> Result<Vec<MusicRecord>, String> {
    db.get_all_music(sort_by.as_deref(), filter_tag.as_deref(), filter_instrumental)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_music(id: i64, db: State<Database>) -> Result<Option<MusicRecord>, String> {
    let conn = db.conn.lock().unwrap();
    db.get_music_by_id(&conn, id).map(|r| Some(r)).or(Ok(None))
}

#[tauri::command]
pub fn update_music_metadata(
    id: i64,
    title: Option<String>,
    tags: Option<String>,
    notes: Option<String>,
    db: State<Database>,
) -> Result<(), String> {
    db.update_metadata(id, title.as_deref(), tags.as_deref(), notes.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_music(id: i64, db: State<Database>) -> Result<(), String> {
    db.delete_music(id).map_err(|e| e.to_string())
}