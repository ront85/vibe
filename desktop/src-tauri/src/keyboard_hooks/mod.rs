#![allow(dead_code)]

//! Global keyboard hook system for dictation feature
//!
//! Provides cross-platform keyboard shortcut registration and push-to-talk support.
//!
//! Platform-specific implementations:
//! - macOS: Event taps via cocoa
//! - Windows: RegisterHotKey and low-level keyboard hooks
//! - Linux: X11/Wayland support
//!
//! Default keyboard shortcuts:
//! - macOS: Cmd+Shift+Space
//! - Windows/Linux: Ctrl+Alt+S
//! - All platforms: ESC to cancel recording

use eyre::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Keyboard event types
#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardEvent {
    /// Hotkey pressed down (start recording)
    HotkeyDown,
    /// Hotkey released (stop recording)
    HotkeyUp,
    /// ESC key pressed (cancel recording)
    CancelPressed,
}

/// Callback function for keyboard events
pub type KeyboardEventCallback = Arc<dyn Fn(KeyboardEvent) + Send + Sync>;

/// Keyboard hook manager interface
///
/// This trait abstracts platform-specific keyboard hook implementations
pub trait KeyboardHookManager: Send + Sync {
    /// Register a global hotkey
    ///
    /// # Arguments
    /// * `shortcut` - Keyboard shortcut string (e.g., "Cmd+Shift+Space", "Ctrl+Alt+S")
    /// * `callback` - Function to call when keyboard events occur
    ///
    /// # Returns
    /// * Result indicating success or failure
    fn register_hotkey(&mut self, shortcut: &str, callback: KeyboardEventCallback) -> Result<()>;

    /// Unregister the current hotkey
    fn unregister_hotkey(&mut self) -> Result<()>;

    /// Check if a hotkey is currently registered
    fn is_registered(&self) -> bool;

    /// Get the current registered shortcut
    fn current_shortcut(&self) -> Option<String>;
}

/// Shared keyboard hook manager
pub struct KeyboardHooks {
    manager: Arc<Mutex<Box<dyn KeyboardHookManager>>>,
}

impl KeyboardHooks {
    /// Create a new keyboard hook manager for the current platform
    pub fn new() -> Result<Self> {
        let manager = create_platform_manager()?;
        Ok(Self {
            manager: Arc::new(Mutex::new(manager)),
        })
    }

    /// Register a global hotkey with callback
    pub async fn register(&self, shortcut: &str, callback: KeyboardEventCallback) -> Result<()> {
        let mut manager = self.manager.lock().await;
        manager.register_hotkey(shortcut, callback)
    }

    /// Unregister the current hotkey
    pub async fn unregister(&self) -> Result<()> {
        let mut manager = self.manager.lock().await;
        manager.unregister_hotkey()
    }

    /// Check if a hotkey is registered
    pub async fn is_registered(&self) -> bool {
        let manager = self.manager.lock().await;
        manager.is_registered()
    }

    /// Get the current shortcut
    pub async fn current_shortcut(&self) -> Option<String> {
        let manager = self.manager.lock().await;
        manager.current_shortcut()
    }
}

impl Default for KeyboardHooks {
    fn default() -> Self {
        Self::new().expect("Failed to create keyboard hooks")
    }
}

/// Create the appropriate keyboard hook manager for the current platform
fn create_platform_manager() -> Result<Box<dyn KeyboardHookManager>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::MacOSKeyboardHookManager::new()?))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(windows::WindowsKeyboardHookManager::new()?))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(linux::LinuxKeyboardHookManager::new()?))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        eyre::bail!("Keyboard hooks not supported on this platform")
    }
}

/// Get the default keyboard shortcut for the current platform
pub fn get_default_shortcut() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "Cmd+Shift+Space"
    }

    #[cfg(not(target_os = "macos"))]
    {
        "Ctrl+Alt+S"
    }
}

/// Parse a keyboard shortcut string into modifiers and key
///
/// # Arguments
/// * `shortcut` - Shortcut string like "Cmd+Shift+Space" or "Ctrl+Alt+S"
///
/// # Returns
/// * Tuple of (modifiers, key_code)
pub fn parse_shortcut(shortcut: &str) -> Result<(Vec<String>, String)> {
    let parts: Vec<&str> = shortcut.split('+').collect();
    if parts.len() < 2 {
        eyre::bail!("Invalid shortcut format: {}", shortcut);
    }

    let key = parts.last().ok_or_else(|| eyre::eyre!("No key in shortcut"))?.to_string();
    let modifiers = parts[..parts.len() - 1].iter().map(|s| s.to_string()).collect();

    Ok((modifiers, key))
}

/// Validate that a shortcut is valid for the current platform
pub fn validate_shortcut(shortcut: &str) -> Result<()> {
    let (modifiers, key) = parse_shortcut(shortcut)?;

    // Check for platform-specific modifier requirements
    #[cfg(target_os = "macos")]
    {
        // macOS should use Cmd, Option, Shift, or Ctrl
        for modifier in &modifiers {
            match modifier.as_str() {
                "Cmd" | "Command" | "Option" | "Alt" | "Shift" | "Ctrl" | "Control" => {}
                _ => eyre::bail!("Invalid modifier for macOS: {}", modifier),
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Windows/Linux use Ctrl, Alt, Shift
        for modifier in &modifiers {
            match modifier.as_str() {
                "Ctrl" | "Control" | "Alt" | "Shift" => {}
                _ => eyre::bail!("Invalid modifier for Windows/Linux: {}", modifier),
            }
        }
    }

    // Check that we have at least one modifier
    if modifiers.is_empty() {
        eyre::bail!("Shortcut must have at least one modifier");
    }

    // Check that key is valid
    if key.is_empty() {
        eyre::bail!("Shortcut must have a key");
    }

    Ok(())
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
    fn test_get_default_shortcut() {
        let shortcut = get_default_shortcut();
        assert!(!shortcut.is_empty());

        #[cfg(target_os = "macos")]
        assert_eq!(shortcut, "Cmd+Shift+Space");

        #[cfg(not(target_os = "macos"))]
        assert_eq!(shortcut, "Ctrl+Alt+S");
    }

    #[test]
    fn test_parse_shortcut() {
        let (modifiers, key) = parse_shortcut("Cmd+Shift+Space").unwrap();
        assert_eq!(modifiers, vec!["Cmd", "Shift"]);
        assert_eq!(key, "Space");

        let (modifiers, key) = parse_shortcut("Ctrl+Alt+S").unwrap();
        assert_eq!(modifiers, vec!["Ctrl", "Alt"]);
        assert_eq!(key, "S");
    }

    #[test]
    fn test_parse_invalid_shortcut() {
        assert!(parse_shortcut("S").is_err()); // No modifiers
        assert!(parse_shortcut("").is_err()); // Empty
    }

    #[test]
    fn test_validate_shortcut() {
        #[cfg(target_os = "macos")]
        {
            assert!(validate_shortcut("Cmd+Shift+Space").is_ok());
            assert!(validate_shortcut("Cmd+S").is_ok());
            assert!(validate_shortcut("Option+Space").is_ok());
            assert!(validate_shortcut("S").is_err()); // No modifier
        }

        #[cfg(not(target_os = "macos"))]
        {
            assert!(validate_shortcut("Ctrl+Alt+S").is_ok());
            assert!(validate_shortcut("Ctrl+S").is_ok());
            assert!(validate_shortcut("Alt+Space").is_ok());
            assert!(validate_shortcut("Cmd+S").is_err()); // Invalid modifier on Windows/Linux
            assert!(validate_shortcut("S").is_err()); // No modifier
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_validate_macos_specific() {
        assert!(validate_shortcut("Command+Shift+Space").is_ok());
        assert!(validate_shortcut("Cmd+Option+S").is_ok());
        assert!(validate_shortcut("Ctrl+Shift+S").is_ok());
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn test_validate_windows_linux_specific() {
        assert!(validate_shortcut("Control+Alt+S").is_ok());
        assert!(validate_shortcut("Ctrl+Shift+S").is_ok());
    }
}
