use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicRecord {
    pub id: i64,
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub cover_image_path: Option<String>,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub created_at: String,
    pub tags: String,
    pub notes: Option<String>,
    #[serde(default)]
    pub is_instrumental: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMusic {
    pub title: String,
    pub prompt: String,
    pub lyrics: Option<String>,
    pub model: String,
    pub audio_path: String,
    pub cover_image_path: Option<String>,
    pub duration_ms: Option<i64>,
    pub file_size: Option<i64>,
    pub sample_rate: Option<i64>,
    pub bitrate: Option<i64>,
    pub is_instrumental: bool,
}

impl super::Database {
    pub fn insert_music(&self, music: &NewMusic) -> rusqlite::Result<MusicRecord> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO music (title, prompt, lyrics, model, audio_path, cover_image_path, duration_ms, file_size, sample_rate, bitrate, is_instrumental)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                music.title,
                music.prompt,
                music.lyrics,
                music.model,
                music.audio_path,
                music.cover_image_path,
                music.duration_ms,
                music.file_size,
                music.sample_rate,
                music.bitrate,
                music.is_instrumental,
            ],
        )?;
        let id = conn.last_insert_rowid();
        self.get_music_by_id(&conn, id)
    }

    pub fn get_music_by_id(&self, conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<MusicRecord> {
        conn.query_row(
            "SELECT id, title, prompt, lyrics, model, audio_path, cover_image_path, duration_ms, file_size, sample_rate, bitrate, created_at, tags, notes, is_instrumental
             FROM music WHERE id = ?1",
            [id],
            |row| {
                Ok(MusicRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    prompt: row.get(2)?,
                    lyrics: row.get(3)?,
                    model: row.get(4)?,
                    audio_path: row.get(5)?,
                    cover_image_path: row.get(6)?,
                    duration_ms: row.get(7)?,
                    file_size: row.get(8)?,
                    sample_rate: row.get(9)?,
                    bitrate: row.get(10)?,
                    created_at: row.get(11)?,
                    tags: row.get(12)?,
                    notes: row.get(13)?,
                    is_instrumental: row.get(14)?,
                })
            },
        )
    }

    pub fn get_all_music(&self, sort_by: Option<&str>, filter_tag: Option<&str>, filter_instrumental: Option<bool>) -> rusqlite::Result<Vec<MusicRecord>> {
        let conn = self.conn.lock().unwrap();
        let order = match sort_by {
            Some("title") => "title ASC",
            Some("duration") => "duration_ms DESC",
            _ => "created_at DESC",
        };
        let mut conditions = Vec::new();
        if filter_tag.map(|t| !t.is_empty()).unwrap_or(false) {
            conditions.push(format!("WHERE tags LIKE '%{}%'", filter_tag.unwrap()));
        }
        if let Some(is_inst) = filter_instrumental {
            let cond = if is_inst { "is_instrumental = 1" } else { "is_instrumental = 0" };
            if conditions.is_empty() {
                conditions.push(format!("WHERE {}", cond));
            } else {
                conditions.push(cond.to_string());
            }
        }
        let filter = if conditions.is_empty() {
            String::new()
        } else {
            conditions.join(" AND ")
        };
        let sql = format!("SELECT id, title, prompt, lyrics, model, audio_path, cover_image_path, duration_ms, file_size, sample_rate, bitrate, created_at, tags, notes, is_instrumental FROM music {} ORDER BY {}", if filter.is_empty() { String::new() } else { format!("WHERE {}", filter) }, order);
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(MusicRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                prompt: row.get(2)?,
                lyrics: row.get(3)?,
                model: row.get(4)?,
                audio_path: row.get(5)?,
                cover_image_path: row.get(6)?,
                duration_ms: row.get(7)?,
                file_size: row.get(8)?,
                sample_rate: row.get(9)?,
                bitrate: row.get(10)?,
                created_at: row.get(11)?,
                tags: row.get(12)?,
                notes: row.get(13)?,
                is_instrumental: row.get(14)?,
            })
        })?;
        rows.collect()
    }

    pub fn update_metadata(&self, id: i64, title: Option<&str>, tags: Option<&str>, notes: Option<&str>) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        if let Some(t) = title {
            conn.execute("UPDATE music SET title = ?1 WHERE id = ?2", rusqlite::params![t, id])?;
        }
        if let Some(t) = tags {
            conn.execute("UPDATE music SET tags = ?1 WHERE id = ?2", rusqlite::params![t, id])?;
        }
        if let Some(n) = notes {
            conn.execute("UPDATE music SET notes = ?1 WHERE id = ?2", rusqlite::params![n, id])?;
        }
        Ok(())
    }

    pub fn update_cover_path(&self, id: i64, cover_path: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE music SET cover_image_path = ?1 WHERE id = ?2", rusqlite::params![cover_path, id])?;
        Ok(())
    }

    pub fn delete_music(&self, id: i64) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM music WHERE id = ?1", [id])?;
        Ok(())
    }
}