use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictationSettings {
    pub enabled: bool,
    pub keyboard_shortcut: String,
    pub microphone_device_id: Option<String>,
    pub model_name: String,
    pub show_floating_widget: bool,
    pub audio_feedback_enabled: bool,
}

impl Default for DictationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            // Platform-specific keyboard shortcuts
            #[cfg(target_os = "macos")]
            keyboard_shortcut: "Cmd+Shift+Space".to_string(),
            #[cfg(not(target_os = "macos"))]
            keyboard_shortcut: "Ctrl+Alt+S".to_string(),
            microphone_device_id: None, // Use system default
            model_name: "small".to_string(),
            show_floating_widget: true,
            audio_feedback_enabled: true,
        }
    }
}

impl DictationSettings {
    /// Load settings from file, or return defaults if file doesn't exist
    #[allow(dead_code)]
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let contents = fs::read_to_string(path).context("Failed to read dictation settings file")?;
            let settings: Self = serde_json::from_str(&contents).context("Failed to parse dictation settings")?;
            Ok(settings)
        } else {
            tracing::debug!("Dictation settings file not found, using defaults");
            Ok(Self::default())
        }
    }

    /// Save settings to file
    #[allow(dead_code)]
    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create settings directory")?;
        }

        let contents = serde_json::to_string_pretty(self).context("Failed to serialize dictation settings")?;
        fs::write(path, contents).context("Failed to write dictation settings file")?;

        tracing::debug!("Dictation settings saved to {:?}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_settings() {
        let settings = DictationSettings::default();

        assert!(settings.enabled);
        assert!(settings.show_floating_widget);
        assert!(settings.audio_feedback_enabled);
        assert_eq!(settings.model_name, "small");

        #[cfg(target_os = "macos")]
        assert_eq!(settings.keyboard_shortcut, "Cmd+Shift+Space");

        #[cfg(not(target_os = "macos"))]
        assert_eq!(settings.keyboard_shortcut, "Ctrl+Alt+S");
    }

    #[test]
    fn test_save_and_load_settings() {
        let dir = tempdir().unwrap();
        let settings_path = dir.path().join("dictation_settings.json");

        let mut settings = DictationSettings::default();
        settings.model_name = "large".to_string();
        settings.audio_feedback_enabled = false;

        // Save
        settings.save(&settings_path).unwrap();
        assert!(settings_path.exists());

        // Load
        let loaded = DictationSettings::load(&settings_path).unwrap();
        assert_eq!(loaded.model_name, "large");
        assert!(!loaded.audio_feedback_enabled);
    }

    #[test]
    fn test_load_nonexistent_returns_defaults() {
        let dir = tempdir().unwrap();
        let settings_path = dir.path().join("nonexistent.json");

        let loaded = DictationSettings::load(&settings_path).unwrap();
        assert_eq!(loaded.model_name, "small");
        assert!(loaded.enabled);
    }

    #[test]
    fn test_settings_persistence() {
        let dir = tempdir().unwrap();
        let settings_path = dir.path().join("settings.json");

        let settings = DictationSettings {
            enabled: false,
            keyboard_shortcut: "Ctrl+Alt+D".to_string(),
            microphone_device_id: Some("device_123".to_string()),
            model_name: "medium".to_string(),
            show_floating_widget: false,
            audio_feedback_enabled: false,
        };

        settings.save(&settings_path).unwrap();
        let loaded = DictationSettings::load(&settings_path).unwrap();

        assert!(!loaded.enabled);
        assert_eq!(loaded.keyboard_shortcut, "Ctrl+Alt+D");
        assert_eq!(loaded.microphone_device_id, Some("device_123".to_string()));
        assert_eq!(loaded.model_name, "medium");
        assert!(!loaded.show_floating_widget);
        assert!(!loaded.audio_feedback_enabled);
    }

    #[test]
    fn test_concurrent_state_access() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let settings = Arc::new(Mutex::new(DictationSettings::default()));
        let mut handles = vec![];

        for i in 0..5 {
            let settings_clone = Arc::clone(&settings);
            let handle = thread::spawn(move || {
                let mut s = settings_clone.lock().unwrap();
                s.model_name = format!("model_{}", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Just verify thread safety worked without panics
        let final_settings = settings.lock().unwrap();
        assert!(final_settings.model_name.starts_with("model_"));
    }
}
