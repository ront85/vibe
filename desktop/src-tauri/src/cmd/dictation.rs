/// Dictation feature Tauri commands
///
/// Provides commands for:
/// - Starting/stopping dictation recording
/// - Managing dictation history
/// - Configuring dictation settings

use crate::text_input::{PasteResult, TextInput};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::command;
use vibe_core::dictation_history::{DictationHistory, DictationHistoryEntry, NewDictationEntry};

/// Result of a text paste operation (for frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasteResponse {
    pub success: bool,
    pub destination_app: Option<String>,
    pub error: Option<String>,
}

impl From<PasteResult> for PasteResponse {
    fn from(result: PasteResult) -> Self {
        Self {
            success: result.success,
            destination_app: result.destination_app,
            error: result.error,
        }
    }
}

/// Paste transcribed text at cursor location
///
/// # Arguments
/// * `text` - The text to paste
///
/// # Returns
/// * PasteResponse with success status and destination app
#[command]
pub async fn paste_text(text: String) -> Result<PasteResponse, String> {
    tracing::debug!("paste_text command called with {} chars", text.len());

    let text_input = TextInput::new().map_err(|e| format!("Failed to create text input: {}", e))?;

    let result = text_input
        .paste(&text)
        .map_err(|e| format!("Failed to paste text: {}", e))?;

    Ok(result.into())
}

/// Get the name of the currently focused application
///
/// # Returns
/// * Application name or None
#[command]
pub async fn get_active_app() -> Result<Option<String>, String> {
    tracing::debug!("get_active_app command called");

    let text_input = TextInput::new().map_err(|e| format!("Failed to create text input: {}", e))?;

    text_input
        .get_active_app()
        .map_err(|e| format!("Failed to get active app: {}", e))
}

/// Check if text can be pasted (e.g., there's an active window)
///
/// # Returns
/// * true if text can be pasted
#[command]
pub async fn can_paste() -> Result<bool, String> {
    tracing::debug!("can_paste command called");

    let text_input = TextInput::new().map_err(|e| format!("Failed to create text input: {}", e))?;

    text_input
        .can_paste()
        .map_err(|e| format!("Failed to check if can paste: {}", e))
}

/// Get all dictation history entries
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
///
/// # Returns
/// * Vector of dictation history entries
#[command]
pub async fn get_dictation_history(db_path: String) -> Result<Vec<DictationHistoryEntry>, String> {
    tracing::debug!("get_dictation_history command called");

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    history
        .get_all()
        .map_err(|e| format!("Failed to get history entries: {}", e))
}

/// Search dictation history by text content
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
/// * `query` - Search query string
///
/// # Returns
/// * Vector of matching dictation history entries
#[command]
pub async fn search_dictation_history(
    db_path: String,
    query: String,
) -> Result<Vec<DictationHistoryEntry>, String> {
    tracing::debug!("search_dictation_history command called with query: {}", query);

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    history
        .search(&query)
        .map_err(|e| format!("Failed to search history: {}", e))
}

/// Add a new entry to dictation history
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
/// * `transcription_text` - The transcribed text
/// * `destination_app` - Name of the application where text was pasted
/// * `model_used` - Name of the Whisper model used
/// * `duration_seconds` - Duration of the audio recording
///
/// # Returns
/// * Unit result
#[command]
pub async fn add_dictation_history(
    db_path: String,
    transcription_text: String,
    destination_app: Option<String>,
    model_used: String,
    duration_seconds: f64,
) -> Result<(), String> {
    tracing::debug!("add_dictation_history command called");

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    let entry = NewDictationEntry {
        transcription_text,
        destination_app: destination_app.unwrap_or_else(|| "Unknown".to_string()),
        model_used,
        duration_seconds,
    };

    history
        .insert(entry)
        .map_err(|e| format!("Failed to add history entry: {}", e))?;

    Ok(())
}

/// Update the text of a history entry
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
/// * `id` - ID of the entry to update
/// * `new_text` - New text content
///
/// # Returns
/// * Unit result
#[command]
pub async fn update_dictation_entry(
    db_path: String,
    id: i64,
    new_text: String,
) -> Result<(), String> {
    tracing::debug!("update_dictation_entry command called for id: {}", id);

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    history
        .update_text(id, &new_text)
        .map_err(|e| format!("Failed to update history entry: {}", e))
}

/// Delete a history entry
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
/// * `id` - ID of the entry to delete
///
/// # Returns
/// * Unit result
#[command]
pub async fn delete_dictation_entry(db_path: String, id: i64) -> Result<(), String> {
    tracing::debug!("delete_dictation_entry command called for id: {}", id);

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    history
        .delete(id)
        .map_err(|e| format!("Failed to delete history entry: {}", e))
}

/// Clean up old history entries (older than 30 days)
///
/// # Arguments
/// * `db_path` - Path to the dictation history database
///
/// # Returns
/// * Number of entries deleted
#[command]
pub async fn cleanup_old_history(db_path: String) -> Result<usize, String> {
    tracing::debug!("cleanup_old_history command called");

    let path = PathBuf::from(db_path);
    let history = DictationHistory::new(&path)
        .map_err(|e| format!("Failed to open history database: {}", e))?;

    history
        .cleanup_old_entries()
        .map_err(|e| format!("Failed to cleanup old entries: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_paste_empty_text() {
        let result = paste_text(String::new()).await;
        // Should succeed but with success=false
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.success);
    }

    #[tokio::test]
    async fn test_can_paste_command() {
        let result = can_paste().await;
        // Should return Ok with boolean value
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_dictation_history_commands() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_history.db");
        let db_path_str = db_path.to_string_lossy().to_string();

        // Add entry
        let result = add_dictation_history(
            db_path_str.clone(),
            "Test transcription".to_string(),
            Some("TextEdit".to_string()),
            "small".to_string(),
            5.5,
        )
        .await;
        assert!(result.is_ok());

        // Get all entries
        let entries = get_dictation_history(db_path_str.clone()).await;
        assert!(entries.is_ok());
        let entries = entries.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].transcription_text, "Test transcription");

        // Search entries
        let search_results = search_dictation_history(db_path_str.clone(), "Test".to_string()).await;
        assert!(search_results.is_ok());
        let search_results = search_results.unwrap();
        assert_eq!(search_results.len(), 1);

        // Update entry
        let entry_id = entries[0].id;
        let update_result =
            update_dictation_entry(db_path_str.clone(), entry_id, "Updated text".to_string()).await;
        assert!(update_result.is_ok());

        // Verify update
        let entries = get_dictation_history(db_path_str.clone()).await.unwrap();
        assert_eq!(entries[0].transcription_text, "Updated text");

        // Delete entry
        let delete_result = delete_dictation_entry(db_path_str.clone(), entry_id).await;
        assert!(delete_result.is_ok());

        // Verify deletion
        let entries = get_dictation_history(db_path_str.clone()).await.unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[tokio::test]
    async fn test_cleanup_old_history() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_cleanup.db");
        let db_path_str = db_path.to_string_lossy().to_string();

        // Add an entry
        add_dictation_history(
            db_path_str.clone(),
            "Test".to_string(),
            None,
            "small".to_string(),
            1.0,
        )
        .await
        .unwrap();

        // Cleanup (should not delete recent entries)
        let deleted = cleanup_old_history(db_path_str.clone()).await.unwrap();
        assert_eq!(deleted, 0);

        // Verify entry still exists
        let entries = get_dictation_history(db_path_str).await.unwrap();
        assert_eq!(entries.len(), 1);
    }
}
