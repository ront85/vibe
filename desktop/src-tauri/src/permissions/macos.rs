/// macOS-specific permission handling

use super::PermissionResult;

/// Check if accessibility permissions are granted
/// Required for text input simulation and keyboard hooks
pub fn check_accessibility_permission() -> PermissionResult {
    // TODO: Implement actual AXIsProcessTrusted() check
    // This requires the cocoa crate and calling AXIsProcessTrusted()
    // For now, return NotDetermined to prompt user to check manually

    tracing::info!("Checking accessibility permission on macOS");

    // Placeholder implementation
    // In production, this would use:
    // use cocoa::appkit::NSWorkspace;
    // use cocoa::base::id;
    // let trusted = unsafe { AXIsProcessTrusted() };

    PermissionResult::not_determined()
}

/// Check if microphone permission is granted
pub fn check_microphone_permission() -> PermissionResult {
    // On macOS, microphone permission is handled by the system
    // When cpal tries to access the microphone, macOS will prompt automatically
    // We can't check the status directly without using private APIs

    tracing::info!("Checking microphone permission on macOS");

    // Assume not determined - system will prompt when needed
    PermissionResult::not_determined()
}

/// Request accessibility permission
/// Opens System Preferences to the Accessibility pane
#[allow(dead_code)]
pub fn request_accessibility_permission() -> bool {
    tracing::info!("Requesting accessibility permission on macOS");

    // TODO: Implement opening System Preferences
    // This would use NSWorkspace or open command
    // For now, return false to indicate user needs to do it manually

    false
}

/// Get instructions for enabling accessibility permission
pub fn get_accessibility_remediation_instructions() -> String {
    format!(
        "To enable dictation, Vibe needs accessibility permissions:\n\n\
        1. Open System Preferences (or System Settings on macOS 13+)\n\
        2. Go to Security & Privacy > Privacy > Accessibility\n\
        3. Click the lock icon and enter your password\n\
        4. Find 'Vibe' in the list and check the box next to it\n\
        5. If Vibe is not in the list, click '+' and add it\n\
        6. Restart Vibe\n\n\
        These permissions allow Vibe to:\n\
        • Simulate keyboard input to paste text at your cursor\n\
        • Register global keyboard shortcuts"
    )
}

/// Get instructions for enabling microphone permission
pub fn get_microphone_remediation_instructions() -> String {
    format!(
        "To enable dictation, Vibe needs microphone access:\n\n\
        1. Open System Preferences (or System Settings on macOS 13+)\n\
        2. Go to Security & Privacy > Privacy > Microphone\n\
        3. Find 'Vibe' in the list and check the box next to it\n\
        4. If prompted, allow microphone access when you first use dictation\n\n\
        This permission allows Vibe to:\n\
        • Record your voice for transcription\n\
        • Show real-time audio levels"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_accessibility_permission_macos() {
        let result = check_accessibility_permission();
        // Should return a valid status
        assert!(matches!(
            result.status,
            PermissionStatus::Granted
                | PermissionStatus::Denied
                | PermissionStatus::NotDetermined
        ));
    }

    #[test]
    fn test_check_microphone_permission_macos() {
        let result = check_microphone_permission();
        assert!(matches!(
            result.status,
            PermissionStatus::Granted
                | PermissionStatus::Denied
                | PermissionStatus::NotDetermined
        ));
    }

    #[test]
    fn test_accessibility_instructions_not_empty() {
        let instructions = get_accessibility_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("System Preferences"));
        assert!(instructions.contains("Accessibility"));
    }

    #[test]
    fn test_microphone_instructions_not_empty() {
        let instructions = get_microphone_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("Microphone"));
    }
}
