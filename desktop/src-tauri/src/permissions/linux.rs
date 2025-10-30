/// Linux-specific permission handling

use super::{PermissionResult, PermissionStatus};

/// Check if accessibility permissions are granted
/// On Linux, this depends on the user being in the 'input' group
pub fn check_accessibility_permission() -> PermissionResult {
    tracing::info!("Checking accessibility permission on Linux");

    // On Linux, accessibility depends on:
    // 1. User being in 'input' group for device access
    // 2. X11 or Wayland permissions for input simulation
    // This is complex to check programmatically

    // Return NotDetermined and let user check manually
    PermissionResult::not_determined()
}

/// Check if microphone permission is granted
/// On Linux, this depends on PulseAudio/PipeWire configuration
pub fn check_microphone_permission() -> PermissionResult {
    tracing::info!("Checking microphone permission on Linux");

    // Linux audio permissions are handled by PulseAudio/PipeWire
    // No system-level permission dialog like macOS/Windows
    // If cpal can access audio, permission is granted

    PermissionResult::not_determined()
}

/// Request accessibility permission
/// On Linux, user needs to add themselves to the 'input' group manually
pub fn request_accessibility_permission() -> bool {
    tracing::info!("Requesting accessibility permission on Linux");

    // Can't programmatically grant permissions on Linux
    // User needs to run: sudo usermod -a -G input $USER
    false
}

/// Get instructions for enabling accessibility permission
pub fn get_accessibility_remediation_instructions() -> String {
    format!(
        "To enable dictation on Linux, you may need to configure permissions:\n\n\
        For X11:\n\
        1. Ensure your user is in the 'input' group:\n\
           sudo usermod -a -G input $USER\n\
        2. Log out and log back in for changes to take effect\n\
        3. Verify with: groups | grep input\n\n\
        For Wayland:\n\
        1. Wayland has limited support for global hotkeys\n\
        2. Some compositors may require additional configuration\n\
        3. Consider using X11 for full functionality\n\n\
        If you're still having issues:\n\
        • Check that xdotool is installed: sudo apt install xdotool\n\
        • Ensure no other application is capturing the same hotkey\n\
        • Try running Vibe from a terminal to see error messages"
    )
}

/// Get instructions for enabling microphone permission
pub fn get_microphone_remediation_instructions() -> String {
    format!(
        "To enable microphone access on Linux:\n\n\
        PulseAudio:\n\
        1. Check audio devices: pactl list sources short\n\
        2. Ensure your microphone is not muted: pavucontrol\n\
        3. Verify your user has audio group access: groups | grep audio\n\
        4. If needed: sudo usermod -a -G audio $USER\n\n\
        PipeWire:\n\
        1. Check audio devices: pw-cli ls Node\n\
        2. Ensure PipeWire is running: systemctl --user status pipewire\n\
        3. Check permissions with: pw-top\n\n\
        General troubleshooting:\n\
        • Restart Vibe after changing groups\n\
        • Check system audio settings\n\
        • Test microphone with: arecord -d 5 test.wav"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_accessibility_permission_linux() {
        let result = check_accessibility_permission();
        // Should return NotDetermined since it's complex to check
        assert_eq!(result.status, PermissionStatus::NotDetermined);
    }

    #[test]
    fn test_check_microphone_permission_linux() {
        let result = check_microphone_permission();
        assert_eq!(result.status, PermissionStatus::NotDetermined);
    }

    #[test]
    fn test_accessibility_instructions_not_empty() {
        let instructions = get_accessibility_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("input"));
        assert!(instructions.contains("group"));
    }

    #[test]
    fn test_microphone_instructions_not_empty() {
        let instructions = get_microphone_remediation_instructions();
        assert!(!instructions.is_empty());
        assert!(instructions.contains("PulseAudio") || instructions.contains("PipeWire"));
    }
}
