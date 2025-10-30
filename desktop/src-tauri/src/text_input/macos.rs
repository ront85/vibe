/// macOS text input implementation using CGEvent text input API
///
/// This implementation preserves the clipboard by using direct text event simulation
/// instead of clipboard-based paste operations.

use super::{PasteResult, TextInputManager};
use core_graphics_helmer_fork::event::CGEvent;
use core_graphics_helmer_fork::event_source::{CGEventSource, CGEventSourceStateID};
use eyre::Result;

pub struct MacOSTextInputManager;

impl MacOSTextInputManager {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Paste text using CGEvent keyboard event simulation
    ///
    /// This sends individual character events to simulate typing, which:
    /// - Preserves clipboard contents
    /// - Works in most text fields
    /// - Respects text field character limits
    fn paste_text_impl(&self, text: &str) -> Result<PasteResult> {
        tracing::debug!("Pasting text on macOS: {} chars", text.len());

        // Create event source for this paste operation
        let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| eyre::eyre!("Failed to create CGEventSource"))?;

        // For each character in the text, create a keyboard event
        for ch in text.chars() {
            self.send_character(&event_source, ch)?;
        }

        Ok(PasteResult {
            success: true,
            destination_app: None, // We'll add app detection later if needed
            error: None,
        })
    }

    /// Send a single character using CGEvent
    fn send_character(&self, event_source: &CGEventSource, ch: char) -> Result<()> {
        // Create a Unicode string event
        let event = CGEvent::new_keyboard_event(event_source.clone(), 0, true)
            .map_err(|_| eyre::eyre!("Failed to create keyboard event"))?;

        // Set the Unicode string for this event
        let unicode_string: Vec<u16> = vec![ch as u16];
        event.set_string_from_utf16_unchecked(&unicode_string);

        // Post the key down event
        event.post(core_graphics_helmer_fork::event::CGEventTapLocation::HID);

        // Small delay between characters (500us) to avoid overwhelming the input system
        std::thread::sleep(std::time::Duration::from_micros(500));

        Ok(())
    }
}

impl TextInputManager for MacOSTextInputManager {
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
        // For now, return None. We can add NSWorkspace integration later if needed
        // This requires additional Obj-C bindings which are complex
        Ok(None)
    }

    fn can_paste_text(&self) -> Result<bool> {
        // On macOS, we'll assume we can always paste
        // The actual paste will fail gracefully if there's no text field
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_manager() {
        let manager = MacOSTextInputManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_active_app() {
        let manager = MacOSTextInputManager::new().unwrap();
        let app_name = manager.get_active_app_name().unwrap();
        // Currently returns None
        assert_eq!(app_name, None);
    }

    #[test]
    fn test_can_paste() {
        let manager = MacOSTextInputManager::new().unwrap();
        let can_paste = manager.can_paste_text().unwrap();
        // Should always return true on macOS
        assert!(can_paste);
    }

    #[test]
    fn test_paste_empty_text() {
        let manager = MacOSTextInputManager::new().unwrap();
        let result = manager.paste_text("");
        assert!(result.is_ok());
        let paste_result = result.unwrap();
        assert!(!paste_result.success);
        assert_eq!(paste_result.error, Some("Empty text".to_string()));
    }

    // Note: Testing actual paste requires a real text field and user interaction
    // Skipping automated paste test to avoid interfering with user's system
}
