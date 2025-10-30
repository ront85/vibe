use eyre::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct DictationHistoryEntry {
    pub id: i64,
    pub timestamp: String, // ISO 8601
    pub transcription_text: String,
    pub destination_app: String,
    pub model_used: String,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDictationEntry {
    pub transcription_text: String,
    pub destination_app: String,
    pub model_used: String,
    pub duration_seconds: f64,
}

pub struct DictationHistory {
    conn: Connection,
}

impl DictationHistory {
    /// Create or open the dictation history database
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open dictation history database")?;

        // Create table if not exists
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dictation_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                transcription_text TEXT NOT NULL,
                destination_app TEXT NOT NULL,
                model_used TEXT NOT NULL,
                duration_seconds REAL NOT NULL
            )",
            [],
        )
        .context("Failed to create dictation_history table")?;

        // Create indexes for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON dictation_history(timestamp)",
            [],
        )
        .context("Failed to create timestamp index")?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_destination_app ON dictation_history(destination_app)",
            [],
        )
        .context("Failed to create destination_app index")?;

        Ok(Self { conn })
    }

    /// Insert a new dictation entry
    pub fn insert(&self, entry: NewDictationEntry) -> Result<i64> {
        let timestamp = chrono::Utc::now().to_rfc3339();

        self.conn
            .execute(
                "INSERT INTO dictation_history (timestamp, transcription_text, destination_app, model_used, duration_seconds)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    timestamp,
                    entry.transcription_text,
                    entry.destination_app,
                    entry.model_used,
                    entry.duration_seconds
                ],
            )
            .context("Failed to insert dictation entry")?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Get all entries ordered by timestamp (most recent first)
    pub fn get_all(&self) -> Result<Vec<DictationHistoryEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, transcription_text, destination_app, model_used, duration_seconds FROM dictation_history ORDER BY timestamp DESC")
            .context("Failed to prepare get_all query")?;

        let entries = stmt
            .query_map([], |row| {
                Ok(DictationHistoryEntry {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    transcription_text: row.get(2)?,
                    destination_app: row.get(3)?,
                    model_used: row.get(4)?,
                    duration_seconds: row.get(5)?,
                })
            })
            .context("Failed to query entries")?
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("Failed to collect entries")?;

        Ok(entries)
    }

    /// Search entries by text content, app name, or date range
    pub fn search(&self, query: &str) -> Result<Vec<DictationHistoryEntry>> {
        let search_pattern = format!("%{}%", query);

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, timestamp, transcription_text, destination_app, model_used, duration_seconds
                 FROM dictation_history
                 WHERE transcription_text LIKE ?1 OR destination_app LIKE ?1
                 ORDER BY timestamp DESC",
            )
            .context("Failed to prepare search query")?;

        let entries = stmt
            .query_map(params![search_pattern], |row| {
                Ok(DictationHistoryEntry {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    transcription_text: row.get(2)?,
                    destination_app: row.get(3)?,
                    model_used: row.get(4)?,
                    duration_seconds: row.get(5)?,
                })
            })
            .context("Failed to query search results")?
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("Failed to collect search results")?;

        Ok(entries)
    }

    /// Update an existing entry's transcription text
    pub fn update_text(&self, id: i64, new_text: &str) -> Result<()> {
        self.conn
            .execute(
                "UPDATE dictation_history SET transcription_text = ?1 WHERE id = ?2",
                params![new_text, id],
            )
            .context("Failed to update entry text")?;

        Ok(())
    }

    /// Delete an entry by ID
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM dictation_history WHERE id = ?1", params![id])
            .context("Failed to delete entry")?;

        Ok(())
    }

    /// Delete entries older than 30 days (retention policy)
    pub fn cleanup_old_entries(&self) -> Result<usize> {
        let thirty_days_ago = chrono::Utc::now() - chrono::Duration::days(30);
        let cutoff_timestamp = thirty_days_ago.to_rfc3339();

        let deleted = self
            .conn
            .execute(
                "DELETE FROM dictation_history WHERE timestamp < ?1",
                params![cutoff_timestamp],
            )
            .context("Failed to cleanup old entries")?;

        tracing::debug!("Cleaned up {} old dictation entries", deleted);
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_and_insert_entry() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        let entry = NewDictationEntry {
            transcription_text: "Hello world".to_string(),
            destination_app: "TestApp".to_string(),
            model_used: "small".to_string(),
            duration_seconds: 5.5,
        };

        let id = history.insert(entry).unwrap();
        assert!(id > 0);

        // Cleanup
        fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_retrieve_entries_ordered() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        // Insert multiple entries
        for i in 0..3 {
            let entry = NewDictationEntry {
                transcription_text: format!("Entry {}", i),
                destination_app: "TestApp".to_string(),
                model_used: "small".to_string(),
                duration_seconds: 1.0 + i as f64,
            };
            history.insert(entry).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10)); // Ensure different timestamps
        }

        let entries = history.get_all().unwrap();
        assert_eq!(entries.len(), 3);
        // Most recent first
        assert_eq!(entries[0].transcription_text, "Entry 2");
        assert_eq!(entries[1].transcription_text, "Entry 1");
        assert_eq!(entries[2].transcription_text, "Entry 0");

        // Cleanup
        fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_search_by_text() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        history
            .insert(NewDictationEntry {
                transcription_text: "Hello world".to_string(),
                destination_app: "App1".to_string(),
                model_used: "small".to_string(),
                duration_seconds: 1.0,
            })
            .unwrap();

        history
            .insert(NewDictationEntry {
                transcription_text: "Goodbye world".to_string(),
                destination_app: "App2".to_string(),
                model_used: "small".to_string(),
                duration_seconds: 2.0,
            })
            .unwrap();

        let results = history.search("Hello").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].transcription_text, "Hello world");

        // Cleanup
        fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_cleanup_old_entries() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        // Insert entry with old timestamp
        let old_timestamp = (chrono::Utc::now() - chrono::Duration::days(31)).to_rfc3339();
        history
            .conn
            .execute(
                "INSERT INTO dictation_history (timestamp, transcription_text, destination_app, model_used, duration_seconds)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![old_timestamp, "Old entry", "App", "small", 1.0],
            )
            .unwrap();

        // Insert recent entry
        history
            .insert(NewDictationEntry {
                transcription_text: "Recent entry".to_string(),
                destination_app: "App".to_string(),
                model_used: "small".to_string(),
                duration_seconds: 1.0,
            })
            .unwrap();

        let deleted = history.cleanup_old_entries().unwrap();
        assert_eq!(deleted, 1);

        let remaining = history.get_all().unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].transcription_text, "Recent entry");

        // Cleanup
        fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_handle_invalid_queries() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        // Empty search should return empty results, not error
        let results = history.search("nonexistent").unwrap();
        assert_eq!(results.len(), 0);

        // Delete non-existent entry should succeed (no-op)
        history.delete(999999).unwrap();

        // Cleanup
        fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_update_entry_text() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_dictation.db");
        let history = DictationHistory::new(&db_path).unwrap();

        let id = history
            .insert(NewDictationEntry {
                transcription_text: "Original text".to_string(),
                destination_app: "App".to_string(),
                model_used: "small".to_string(),
                duration_seconds: 1.0,
            })
            .unwrap();

        history.update_text(id, "Updated text").unwrap();

        let entries = history.get_all().unwrap();
        assert_eq!(entries[0].transcription_text, "Updated text");

        // Cleanup
        fs::remove_file(db_path).ok();
    }
}
