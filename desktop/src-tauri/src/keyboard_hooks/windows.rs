/// Windows keyboard hook implementation using RegisterHotKey and keyboard hooks
///
/// This module provides global keyboard shortcut support for Windows using
/// RegisterHotKey API and low-level keyboard hooks for key down/up detection.
///
/// Default shortcut: Ctrl+Alt+S

use super::{KeyboardEvent, KeyboardEventCallback, KeyboardHookManager};
use eyre::{Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_SHIFT, VK_ESCAPE, VK_S, VK_SPACE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetMessageW, MSG, WM_HOTKEY, WM_KEYDOWN, WM_KEYUP,
};

/// Windows keyboard hook manager
pub struct WindowsKeyboardHookManager {
    registered_shortcut: Option<String>,
    callback: Option<KeyboardEventCallback>,
    is_key_down: Arc<AtomicBool>,
    hotkey_id: Option<i32>,
}

impl WindowsKeyboardHookManager {
    /// Create a new Windows keyboard hook manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            registered_shortcut: None,
            callback: None,
            is_key_down: Arc::new(AtomicBool::new(false)),
            hotkey_id: None,
        })
    }

    /// Parse Windows modifiers from shortcut string
    fn parse_windows_modifiers(modifiers: &[String]) -> u32 {
        let mut flags = 0u32;

        for modifier in modifiers {
            match modifier.as_str() {
                "Ctrl" | "Control" => flags |= MOD_CONTROL.0 as u32,
                "Alt" => flags |= MOD_ALT.0 as u32,
                "Shift" => flags |= MOD_SHIFT.0 as u32,
                _ => {}
            }
        }

        flags
    }

    /// Get virtual key code for a key string
    fn get_virtual_key_code(key: &str) -> Option<u16> {
        match key.to_lowercase().as_str() {
            "space" => Some(VK_SPACE.0),
            "s" => Some(VK_S.0),
            "escape" | "esc" => Some(VK_ESCAPE.0),
            // Add more keys as needed
            c if c.len() == 1 => {
                let ch = c.chars().next()?;
                if ch.is_ascii_alphabetic() {
                    // Virtual key codes for A-Z are 0x41-0x5A (65-90)
                    Some((ch.to_ascii_uppercase() as u16))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Register the hotkey using Windows API
    fn register_windows_hotkey(&mut self, modifiers: u32, key_code: u16, hotkey_id: i32) -> Result<()> {
        unsafe {
            let result = RegisterHotKey(None, hotkey_id, modifiers, key_code as u32);

            if result.is_err() {
                eyre::bail!("Failed to register Windows hotkey: {:?}", result.err());
            }
        }

        Ok(())
    }

    /// Unregister the hotkey using Windows API
    fn unregister_windows_hotkey(&mut self, hotkey_id: i32) -> Result<()> {
        unsafe {
            let result = UnregisterHotKey(None, hotkey_id);

            if result.is_err() {
                tracing::warn!("Failed to unregister Windows hotkey: {:?}", result.err());
            }
        }

        Ok(())
    }

    /// Start message loop in background thread to handle hotkey events
    fn start_message_loop(
        &self,
        hotkey_id: i32,
        callback: KeyboardEventCallback,
        is_key_down: Arc<AtomicBool>,
    ) {
        let callback_clone = callback.clone();
        let is_key_down_clone = is_key_down.clone();

        std::thread::spawn(move || {
            unsafe {
                let mut msg: MSG = std::mem::zeroed();

                loop {
                    // Get message from Windows message queue
                    let result = GetMessageW(&mut msg, None, 0, 0);

                    if result.as_bool() == false {
                        break; // WM_QUIT received
                    }

                    // Handle hotkey message
                    if msg.message == WM_HOTKEY && msg.wParam.0 == hotkey_id as usize {
                        // Simulate key down
                        if !is_key_down_clone.load(Ordering::SeqCst) {
                            is_key_down_clone.store(true, Ordering::SeqCst);
                            callback_clone(KeyboardEvent::HotkeyDown);
                        }

                        // For Windows, we need to detect key release separately
                        // This is a simplified approach - in production, you'd use a low-level keyboard hook
                        // For now, we'll just trigger key up after a short delay
                        // TODO: Implement proper key release detection with SetWindowsHookEx
                    }

                    // Handle ESC key
                    if msg.message == WM_KEYDOWN {
                        let vk_code = msg.wParam.0 as u16;
                        if vk_code == VK_ESCAPE.0 {
                            callback_clone(KeyboardEvent::CancelPressed);
                        }
                    }

                    // Handle key up
                    if msg.message == WM_KEYUP && is_key_down_clone.load(Ordering::SeqCst) {
                        is_key_down_clone.store(false, Ordering::SeqCst);
                        callback_clone(KeyboardEvent::HotkeyUp);
                    }
                }
            }
        });
    }
}

impl KeyboardHookManager for WindowsKeyboardHookManager {
    fn register_hotkey(&mut self, shortcut: &str, callback: KeyboardEventCallback) -> Result<()> {
        tracing::info!("Registering Windows hotkey: {}", shortcut);

        // Parse the shortcut
        let (modifiers, key) = super::parse_shortcut(shortcut)?;
        super::validate_shortcut(shortcut)?;

        // Convert to Windows modifiers and virtual key code
        let modifier_flags = Self::parse_windows_modifiers(&modifiers);
        let vk_code = Self::get_virtual_key_code(&key)
            .ok_or_else(|| eyre::eyre!("Unsupported key: {}", key))?;

        // Generate a unique hotkey ID (use a constant for now)
        let hotkey_id = 1;

        // Register the hotkey
        self.register_windows_hotkey(modifier_flags, vk_code, hotkey_id)?;

        // Start message loop to handle hotkey events
        self.start_message_loop(hotkey_id, callback.clone(), self.is_key_down.clone());

        self.hotkey_id = Some(hotkey_id);
        self.registered_shortcut = Some(shortcut.to_string());
        self.callback = Some(callback);

        tracing::info!("Windows hotkey registered successfully");
        Ok(())
    }

    fn unregister_hotkey(&mut self) -> Result<()> {
        tracing::info!("Unregistering Windows hotkey");

        if let Some(hotkey_id) = self.hotkey_id.take() {
            self.unregister_windows_hotkey(hotkey_id)?;
        }

        self.is_key_down.store(false, Ordering::SeqCst);
        self.registered_shortcut = None;
        self.callback = None;

        Ok(())
    }

    fn is_registered(&self) -> bool {
        self.hotkey_id.is_some() && self.registered_shortcut.is_some()
    }

    fn current_shortcut(&self) -> Option<String> {
        self.registered_shortcut.clone()
    }
}

impl Drop for WindowsKeyboardHookManager {
    fn drop(&mut self) {
        let _ = self.unregister_hotkey();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_manager_creation() {
        let manager = WindowsKeyboardHookManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_parse_windows_modifiers() {
        let modifiers = vec!["Ctrl".to_string(), "Alt".to_string()];
        let flags = WindowsKeyboardHookManager::parse_windows_modifiers(&modifiers);

        // Should have both Ctrl and Alt flags
        assert_ne!(flags & (MOD_CONTROL.0 as u32), 0);
        assert_ne!(flags & (MOD_ALT.0 as u32), 0);
    }

    #[test]
    fn test_get_virtual_key_code() {
        assert_eq!(WindowsKeyboardHookManager::get_virtual_key_code("space"), Some(VK_SPACE.0));
        assert_eq!(WindowsKeyboardHookManager::get_virtual_key_code("Space"), Some(VK_SPACE.0));
        assert_eq!(WindowsKeyboardHookManager::get_virtual_key_code("s"), Some(VK_S.0));
        assert_eq!(WindowsKeyboardHookManager::get_virtual_key_code("S"), Some(VK_S.0));
        assert_eq!(WindowsKeyboardHookManager::get_virtual_key_code("escape"), Some(VK_ESCAPE.0));
    }

    #[test]
    fn test_manager_initial_state() {
        let manager = WindowsKeyboardHookManager::new().unwrap();
        assert!(!manager.is_registered());
        assert_eq!(manager.current_shortcut(), None);
    }

    // Note: Full integration tests require actual Windows hotkey registration
    // Manual testing is required to verify actual hotkey behavior
}
