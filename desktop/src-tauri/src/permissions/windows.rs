/// Windows-specific permission handling

use super::{PermissionResult, PermissionStatus};

/// Check if accessibility permissions are granted
/// On Windows, most accessibility features don't require explicit permissions
pub fn check_accessibility_permission() -> PermissionResult {
    tracing::info!("Checking accessibility permission on Windows");

    // Windows doesn't require accessibility permissions for SendInput
    // Low-level keyboard hooks may require admin rights, but not for basic hotkeys
    PermissionResult::granted()
}

/// Check if microphone permission is granted
/// On Windows 10+, microphone access is controlled by Privacy settings
pub fn check_microphone_permission() -> PermissionResult {
    tracing::info!("Checking microphone permission on Windows");

    // On Windows, we can't easily check microphone permission without WinRT APIs
    // The system will prompt when cpal tries to access the microphone
    // Return NotDetermined to let the system handle it
    PermissionResult::not_determined()
}

/// Request accessibility permission
/// On Windows, this is not typically required
pub fn request_accessibility_permission() -> bool {
    tracing::info!("Requesting accessibility permission on Windows");

    // No action needed on Windows for basic input simulation
    true
}

/// Get instructions for enabling accessibility permission
pub fn get_accessibility_remediation_instructions() -> String {
    format!(
        "On Windows, Vibe doesn't require special accessibility permissions.\n\n\
        If you're experiencing issues with dictation:\n\n\
        1. Try running Vibe as administrator (right-click > Run as administrator)\n\
        2. Check Windows Defender or antivirus isn't blocking Vibe\n\
        3. Ensure no other applications are interfering with global hotkeys\n\n\
        Note: Some antivirus software may flag keyboard simulation as suspicious behavior."
    )
}

/// Get instructions for enabling microphone permission
pub fn get_microphone_remediation_instructions() -> String {
    format!(
        "To enable dictation, Vibe needs microphone access:\n\n\
        Windows 10/11:\n\
        1. Open Settings (Win + I)\n\
        2. Go to Privacy > Microphone\n\
        3. Enable 'Allow apps to access your microphone'\n\
        4. Scroll down and ensure Vibe is allowed\n\
        5. If prompted, allow microphone access when you first use dictation\n\n\
        This permission allows Vibe to:\n\
        • Record your voice for transcription\n\
        • Show real-time audio levels"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_accessibility_permission_windows() {
        let result = check_accessibility_permission();
        // Windows should grant accessibility by default
        assert_eq!(result.status, PermissionStatus::Granted);
    }

    #[test]
    fn test_check_microphone_permission_windows() {
        let result = check_microphone_permission();
        // Should return NotDetermined since system handles it
        assert_eq!(result.status, PermissionStatus::NotDetermined);
    }

    #[test]
    fn test_accessibility_instructions_not_empty() {
        let instructions = get_accessibility_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("Windows"));
    }

    #[test]
    fn test_microphone_instructions_not_empty() {
        let instructions = get_microphone_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("Settings"));
        assert!(instructions.contains("Microphone"));
    }
}
