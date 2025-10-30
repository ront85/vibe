/// Platform-specific text input simulation for dictation feature
///
/// Provides text pasting at cursor location without modifying clipboard.
///
/// Platform-specific implementations:
/// - macOS: AXUIElement or CGEvent text input (preserves clipboard)
/// - Windows: SendInput API with KEYEVENTF_UNICODE (preserves clipboard)
/// - Linux: X11 XTest or Wayland input simulation
///
/// Key features:
/// - Direct text input preserves clipboard contents
/// - Detects active application for history metadata
/// - Gracefully handles cursor not in text field
/// - Maintains focus in target application

use eyre::Result;

/// Result of text pasting operation
#[derive(Debug, Clone, PartialEq)]
pub struct PasteResult {
    /// Whether the paste was successful
    pub success: bool,
    /// Name of the destination application
    pub destination_app: Option<String>,
    /// Error message if paste failed
    pub error: Option<String>,
}

/// Text input manager interface
///
/// This trait abstracts platform-specific text input implementations
pub trait TextInputManager: Send + Sync {
    /// Paste text at cursor location
    ///
    /// # Arguments
    /// * `text` - Text to paste
    ///
    /// # Returns
    /// * PasteResult with success status and destination app name
    fn paste_text(&self, text: &str) -> Result<PasteResult>;

    /// Get the name of the currently focused application
    ///
    /// # Returns
    /// * Application name or None if unable to detect
    fn get_active_app_name(&self) -> Result<Option<String>>;

    /// Check if text input is available (e.g., cursor in text field)
    ///
    /// # Returns
    /// * true if text can be pasted, false otherwise
    fn can_paste_text(&self) -> Result<bool>;
}

/// Shared text input manager
pub struct TextInput {
    manager: Box<dyn TextInputManager>,
}

impl TextInput {
    /// Create a new text input manager for the current platform
    pub fn new() -> Result<Self> {
        let manager = create_platform_manager()?;
        Ok(Self { manager })
    }

    /// Paste text at cursor location
    pub fn paste(&self, text: &str) -> Result<PasteResult> {
        self.manager.paste_text(text)
    }

    /// Get the name of the currently focused application
    pub fn get_active_app(&self) -> Result<Option<String>> {
        self.manager.get_active_app_name()
    }

    /// Check if text can be pasted
    pub fn can_paste(&self) -> Result<bool> {
        self.manager.can_paste_text()
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new().expect("Failed to create text input manager")
    }
}

/// Create the appropriate text input manager for the current platform
fn create_platform_manager() -> Result<Box<dyn TextInputManager>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::MacOSTextInputManager::new()?))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(windows::WindowsTextInputManager::new()?))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(linux::LinuxTextInputManager::new()?))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        eyre::bail!("Text input not supported on this platform")
    }
}

// Platform-specific modules
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_input_creation() {
        // Platform manager should be created successfully
        let result = TextInput::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_paste_result_success() {
        let result = PasteResult {
            success: true,
            destination_app: Some("TextEdit".to_string()),
            error: None,
        };
        assert!(result.success);
        assert_eq!(result.destination_app, Some("TextEdit".to_string()));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_paste_result_failure() {
        let result = PasteResult {
            success: false,
            destination_app: None,
            error: Some("Cursor not in text field".to_string()),
        };
        assert!(!result.success);
        assert!(result.destination_app.is_none());
        assert_eq!(result.error, Some("Cursor not in text field".to_string()));
    }
}
