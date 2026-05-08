use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub mod music_crud;
pub use music_crud::*;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(&db_path)?;
        let db = Database { conn: Mutex::new(conn) };
        db.init_schema()?;
        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf> {
        let app_data = crate::config::get_effective_data_dir()
            .map_err(|e| rusqlite::Error::InvalidPath(e.into()))?;
        Ok(app_data.join("music_studio.db"))
    }

    /// Update absolute file paths in the database after data migration
    pub fn update_paths_after_migration(&self, old_base: &str, new_base: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE music SET audio_path = REPLACE(audio_path, ?1, ?2) WHERE audio_path LIKE ?3",
            rusqlite::params![old_base, new_base, format!("{}%", old_base)],
        )?;
        conn.execute(
            "UPDATE music SET cover_image_path = REPLACE(cover_image_path, ?1, ?2) WHERE cover_image_path LIKE ?3",
            rusqlite::params![old_base, new_base, format!("{}%", old_base)],
        )?;
        Ok(())
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS music (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                title           TEXT NOT NULL DEFAULT 'Untitled',
                prompt          TEXT NOT NULL,
                lyrics          TEXT,
                model           TEXT NOT NULL DEFAULT 'music-2.6',
                audio_path      TEXT NOT NULL,
                cover_image_path TEXT,
                duration_ms     INTEGER,
                file_size       INTEGER,
                sample_rate     INTEGER,
                bitrate         INTEGER,
                created_at      TEXT NOT NULL DEFAULT (datetime('now')),
                tags            TEXT DEFAULT '[]',
                notes           TEXT,
                is_instrumental INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_music_created_at ON music(created_at DESC)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_music_tags ON music(tags)",
            [],
        )?;
        // Migration: add is_instrumental column if missing
        conn.execute(
            "ALTER TABLE music ADD COLUMN is_instrumental INTEGER NOT NULL DEFAULT 0",
            [],
        ).ok();
        // Migration: add ai_description column if missing
        conn.execute(
            "ALTER TABLE music ADD COLUMN ai_description TEXT",
            [],
        ).ok();
        Ok(())
    }
}