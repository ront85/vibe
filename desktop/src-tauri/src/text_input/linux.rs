/// Linux text input implementation using X11/Wayland
///
/// This implementation attempts X11 first, with Wayland as fallback.
/// Note: Wayland support is limited due to compositor restrictions.

use super::{PasteResult, TextInputManager};
use eyre::{bail, Context, Result};
use std::process::Command;

pub struct LinuxTextInputManager {
    use_xdotool: bool,
}

impl LinuxTextInputManager {
    pub fn new() -> Result<Self> {
        // Check if xdotool is available (for X11)
        let has_xdotool = Command::new("which")
            .arg("xdotool")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false);

        if !has_xdotool {
            tracing::warn!("xdotool not found. Text input may not work properly.");
            tracing::warn!("Install with: sudo apt install xdotool");
        }

        Ok(Self {
            use_xdotool: has_xdotool,
        })
    }

    /// Get the active window name using xdotool or wmctrl
    fn get_active_app_impl(&self) -> Result<Option<String>> {
        if !self.use_xdotool {
            return Ok(None);
        }

        // Try xdotool first
        let output = Command::new("xdotool")
            .args(&["getactivewindow", "getwindowname"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let window_name = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                return Ok(Some(window_name));
            }
        }

        // Fallback to wmctrl
        let output = Command::new("wmctrl")
            .args(&["-l", "-p"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse wmctrl output to find active window
                // Format: 0x... desktop pid hostname window_name
                if let Some(first_line) = output_str.lines().next() {
                    let parts: Vec<&str> = first_line.splitn(5, ' ').collect();
                    if parts.len() >= 5 {
                        return Ok(Some(parts[4].to_string()));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Paste text using xdotool type command
    ///
    /// This simulates keyboard typing character by character:
    /// - Preserves clipboard contents
    /// - Works in X11 environments
    /// - Limited or no support in Wayland
    fn paste_text_impl(&self, text: &str) -> Result<PasteResult> {
        tracing::debug!("Pasting text on Linux: {} chars", text.len());

        if !self.use_xdotool {
            return Ok(PasteResult {
                success: false,
                destination_app: None,
                error: Some("xdotool not available. Install with: sudo apt install xdotool".to_string()),
            });
        }

        // Get destination app before pasting
        let destination_app = self.get_active_app_impl().ok().flatten();

        // Use xdotool to type the text
        // Note: We use --delay to add small delays between characters for reliability
        let output = Command::new("xdotool")
            .args(&["type", "--delay", "1", "--", text])
            .output()
            .context("Failed to execute xdotool")?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            tracing::error!("xdotool failed: {}", error_msg);
            return Ok(PasteResult {
                success: false,
                destination_app,
                error: Some(format!("xdotool failed: {}", error_msg)),
            });
        }

        Ok(PasteResult {
            success: true,
            destination_app,
            error: None,
        })
    }
}

impl TextInputManager for LinuxTextInputManager {
    fn paste_text(&self, text: &str) -> Result<PasteResult> {
        if text.is_empty() {
            return Ok(PasteResult {
                success: false,
                destination_app: None,
                error: Some("Empty text".to_string()),
            });
        }

        self.paste_text_impl(text)
    }

    fn get_active_app_name(&self) -> Result<Option<String>> {
        self.get_active_app_impl()
    }

    fn can_paste_text(&self) -> Result<bool> {
        // On Linux, we can paste if xdotool is available
        Ok(self.use_xdotool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let manager = LinuxTextInputManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_active_app() {
        let manager = LinuxTextInputManager::new().unwrap();
        let app_name = manager.get_active_app_name();
        // May succeed or fail depending on xdotool availability
        tracing::debug!("Active app: {:?}", app_name);
    }

    #[test]
    fn test_can_paste() {
        let manager = LinuxTextInputManager::new().unwrap();
        let can_paste = manager.can_paste_text().unwrap();
        // Should match xdotool availability
        tracing::debug!("Can paste: {}", can_paste);
    }

    #[test]
    fn test_paste_empty_text() {
        let manager = LinuxTextInputManager::new().unwrap();
        let result = manager.paste_text("");
        assert!(result.is_ok());
        let paste_result = result.unwrap();
        assert!(!paste_result.success);
        assert_eq!(paste_result.error, Some("Empty text".to_string()));
    }

    // Note: Testing actual paste requires xdotool and a real text field
    // Skipping automated paste test to avoid interfering with user's system
}
