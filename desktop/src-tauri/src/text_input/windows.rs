/// Windows text input implementation using SendInput API
///
/// This implementation uses KEYEVENTF_UNICODE to send text directly
/// without modifying the clipboard.

use super::{PasteResult, TextInputManager};
use eyre::{Context, Result};
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_UNICODE, KEYEVENTF_KEYUP,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
};

pub struct WindowsTextInputManager;

impl WindowsTextInputManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Get the active window and its process name
    fn get_active_app_impl(&self) -> Result<Option<String>> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == 0 {
                return Ok(None);
            }

            // Get window title (application name)
            let mut buffer = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buffer);

            if len == 0 {
                return Ok(None);
            }

            let title = String::from_utf16_lossy(&buffer[..len as usize]);
            Ok(Some(title))
        }
    }

    /// Paste text using SendInput with Unicode events
    ///
    /// This sends KEYEVENTF_UNICODE events for each character, which:
    /// - Preserves clipboard contents
    /// - Works in most text fields
    /// - Handles multi-byte Unicode characters correctly
    fn paste_text_impl(&self, text: &str) -> Result<PasteResult> {
        tracing::debug!("Pasting text on Windows: {} chars", text.len());

        // Get destination app before pasting
        let destination_app = self.get_active_app_impl().ok().flatten();

        // Convert text to UTF-16 for Windows
        let utf16_text: Vec<u16> = OsStr::new(text)
            .encode_wide()
            .collect();

        // Create INPUT structures for each character (key down and key up)
        let mut inputs = Vec::with_capacity(utf16_text.len() * 2);

        for &ch in &utf16_text {
            // Key down
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });

            // Key up
            inputs.push(INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            });
        }

        // Send all input events
        unsafe {
            let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
            if sent != inputs.len() as u32 {
                tracing::warn!(
                    "SendInput sent {} events, expected {}",
                    sent,
                    inputs.len()
                );
            }
        }

        Ok(PasteResult {
            success: true,
            destination_app,
            error: None,
        })
    }
}

impl TextInputManager for WindowsTextInputManager {
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
        // On Windows, we'll assume we can paste if there's a foreground window
        // We can't reliably detect if cursor is in a text field without
        // additional API calls
        Ok(self.get_active_app_impl()?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let manager = WindowsTextInputManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_active_app() {
        let manager = WindowsTextInputManager::new().unwrap();
        let app_name = manager.get_active_app_name().unwrap();
        // Should return Some app name or None
        tracing::debug!("Active app: {:?}", app_name);
    }

    #[test]
    fn test_can_paste() {
        let manager = WindowsTextInputManager::new().unwrap();
        let can_paste = manager.can_paste_text().unwrap();
        // Should be able to paste if there's a foreground window
        tracing::debug!("Can paste: {}", can_paste);
    }

    #[test]
    fn test_paste_empty_text() {
        let manager = WindowsTextInputManager::new().unwrap();
        let result = manager.paste_text("");
        assert!(result.is_ok());
        let paste_result = result.unwrap();
        assert!(!paste_result.success);
        assert_eq!(paste_result.error, Some("Empty text".to_string()));
    }

    // Note: Testing actual paste requires a real text field and user interaction
    // Skipping automated paste test to avoid interfering with user's system
}
