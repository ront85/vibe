/// Linux keyboard hook implementation (X11/Wayland)
///
/// This module provides global keyboard shortcut support for Linux using
/// X11 (XGrabKey) with best-effort Wayland support.
///
/// Default shortcut: Ctrl+Alt+S
///
/// Note: Wayland support is limited due to security model restrictions.
/// Full functionality requires X11 or compositor-specific protocols.

use super::{KeyboardEvent, KeyboardEventCallback, KeyboardHookManager};
use eyre::{Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Linux keyboard hook manager
pub struct LinuxKeyboardHookManager {
    registered_shortcut: Option<String>,
    callback: Option<KeyboardEventCallback>,
    is_key_down: Arc<AtomicBool>,
}

impl LinuxKeyboardHookManager {
    /// Create a new Linux keyboard hook manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            registered_shortcut: None,
            callback: None,
            is_key_down: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Check if running on X11 or Wayland
    fn get_display_server() -> &'static str {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            "wayland"
        } else if std::env::var("DISPLAY").is_ok() {
            "x11"
        } else {
            "unknown"
        }
    }

    /// Parse Linux modifiers from shortcut string
    #[allow(dead_code)]
    fn parse_linux_modifiers(modifiers: &[String]) -> u32 {
        let mut flags = 0u32;

        for modifier in modifiers {
            match modifier.as_str() {
                "Ctrl" | "Control" => flags |= 0x0004, // ControlMask in X11
                "Alt" => flags |= 0x0008,              // Mod1Mask in X11
                "Shift" => flags |= 0x0001,            // ShiftMask in X11
                _ => {}
            }
        }

        flags
    }

    /// Get X11 keysym for a key string
    #[allow(dead_code)]
    fn get_keysym(key: &str) -> Option<u32> {
        match key.to_lowercase().as_str() {
            "space" => Some(0x0020), // XK_space
            "s" => Some(0x0073),     // XK_s
            "escape" | "esc" => Some(0xFF1B), // XK_Escape
            // Add more keys as needed
            c if c.len() == 1 => {
                let ch = c.chars().next()?;
                if ch.is_ascii_lowercase() {
                    Some(ch as u32)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl KeyboardHookManager for LinuxKeyboardHookManager {
    fn register_hotkey(&mut self, shortcut: &str, callback: KeyboardEventCallback) -> Result<()> {
        tracing::info!("Registering Linux hotkey: {}", shortcut);

        // Check display server
        let display_server = Self::get_display_server();
        tracing::info!("Display server: {}", display_server);

        // Parse the shortcut
        let (modifiers, key) = super::parse_shortcut(shortcut)?;
        super::validate_shortcut(shortcut)?;

        // For now, we'll provide a stub implementation
        // Full X11 implementation requires x11-rs or xcb crate
        // Wayland implementation requires compositor-specific protocols

        match display_server {
            "x11" => {
                tracing::warn!("X11 keyboard hooks not yet fully implemented");
                tracing::warn!("This is a placeholder implementation");

                // TODO: Implement X11 XGrabKey
                // let modifier_flags = Self::parse_linux_modifiers(&modifiers);
                // let keysym = Self::get_keysym(&key)
                //     .ok_or_else(|| eyre::eyre!("Unsupported key: {}", key))?;

                // Store the callback for now
                self.registered_shortcut = Some(shortcut.to_string());
                self.callback = Some(callback);

                tracing::info!("Linux hotkey registration placeholder complete");
                Ok(())
            }
            "wayland" => {
                tracing::warn!("Wayland global hotkeys have limited support");
                tracing::warn!("Consider using X11 compatibility mode or compositor-specific protocols");

                // Wayland doesn't support global hotkeys due to security model
                // Some compositors provide custom protocols
                eyre::bail!("Wayland global hotkeys not supported. Please use X11 mode.")
            }
            _ => {
                eyre::bail!("Unknown display server. Cannot register hotkeys.")
            }
        }
    }

    fn unregister_hotkey(&mut self) -> Result<()> {
        tracing::info!("Unregistering Linux hotkey");

        // TODO: Implement X11 XUngrabKey

        self.is_key_down.store(false, Ordering::SeqCst);
        self.registered_shortcut = None;
        self.callback = None;

        Ok(())
    }

    fn is_registered(&self) -> bool {
        self.registered_shortcut.is_some()
    }

    fn current_shortcut(&self) -> Option<String> {
        self.registered_shortcut.clone()
    }
}

impl Drop for LinuxKeyboardHookManager {
    fn drop(&mut self) {
        let _ = self.unregister_hotkey();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_manager_creation() {
        let manager = LinuxKeyboardHookManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_display_server() {
        let server = LinuxKeyboardHookManager::get_display_server();
        assert!(server == "wayland" || server == "x11" || server == "unknown");
    }

    #[test]
    fn test_parse_linux_modifiers() {
        let modifiers = vec!["Ctrl".to_string(), "Alt".to_string()];
        let flags = LinuxKeyboardHookManager::parse_linux_modifiers(&modifiers);

        // Should have both Ctrl and Alt flags
        assert_ne!(flags & 0x0004, 0); // ControlMask
        assert_ne!(flags & 0x0008, 0); // Mod1Mask (Alt)
    }

    #[test]
    fn test_get_keysym() {
        assert_eq!(LinuxKeyboardHookManager::get_keysym("space"), Some(0x0020));
        assert_eq!(LinuxKeyboardHookManager::get_keysym("s"), Some(0x0073));
        assert_eq!(LinuxKeyboardHookManager::get_keysym("S"), Some(0x0073));
        assert_eq!(LinuxKeyboardHookManager::get_keysym("escape"), Some(0xFF1B));
    }

    #[test]
    fn test_manager_initial_state() {
        let manager = LinuxKeyboardHookManager::new().unwrap();
        assert!(!manager.is_registered());
        assert_eq!(manager.current_shortcut(), None);
    }

    // Note: Full integration tests require X11 connection
    // Manual testing is required to verify actual hotkey behavior
}
