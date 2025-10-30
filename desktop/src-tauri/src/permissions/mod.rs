/// Permission management for dictation feature
/// Handles accessibility and microphone permissions across platforms

use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "linux")]
pub mod linux;

/// Permission status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionStatus {
    /// Permission granted
    Granted,
    /// Permission denied
    Denied,
    /// Permission not determined yet
    NotDetermined,
    /// Permission not applicable on this platform
    NotApplicable,
}

/// Result of permission check with optional message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResult {
    pub status: PermissionStatus,
    pub message: Option<String>,
}

impl PermissionResult {
    #[allow(dead_code)]
    pub fn granted() -> Self {
        Self {
            status: PermissionStatus::Granted,
            message: None,
        }
    }

    #[allow(dead_code)]
    pub fn denied(message: impl Into<String>) -> Self {
        Self {
            status: PermissionStatus::Denied,
            message: Some(message.into()),
        }
    }

    #[allow(dead_code)]
    pub fn not_determined() -> Self {
        Self {
            status: PermissionStatus::NotDetermined,
            message: None,
        }
    }

    #[allow(dead_code)]
    pub fn not_applicable() -> Self {
        Self {
            status: PermissionStatus::NotApplicable,
            message: None,
        }
    }
}

/// Check accessibility permission status
/// Required for text pasting and keyboard hooks
pub fn check_accessibility_permission() -> PermissionResult {
    #[cfg(target_os = "macos")]
    {
        macos::check_accessibility_permission()
    }

    #[cfg(target_os = "windows")]
    {
        windows::check_accessibility_permission()
    }

    #[cfg(target_os = "linux")]
    {
        linux::check_accessibility_permission()
    }
}

/// Check microphone permission status
/// Required for audio capture
pub fn check_microphone_permission() -> PermissionResult {
    #[cfg(target_os = "macos")]
    {
        macos::check_microphone_permission()
    }

    #[cfg(target_os = "windows")]
    {
        windows::check_microphone_permission()
    }

    #[cfg(target_os = "linux")]
    {
        linux::check_microphone_permission()
    }
}

/// Request accessibility permission
/// Returns true if granted, false otherwise
#[allow(dead_code)]
pub fn request_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::request_accessibility_permission()
    }

    #[cfg(target_os = "windows")]
    {
        windows::request_accessibility_permission()
    }

    #[cfg(target_os = "linux")]
    {
        linux::request_accessibility_permission()
    }
}

/// Get remediation instructions for accessibility permission
pub fn get_accessibility_remediation_instructions() -> String {
    #[cfg(target_os = "macos")]
    {
        macos::get_accessibility_remediation_instructions()
    }

    #[cfg(target_os = "windows")]
    {
        windows::get_accessibility_remediation_instructions()
    }

    #[cfg(target_os = "linux")]
    {
        linux::get_accessibility_remediation_instructions()
    }
}

/// Get remediation instructions for microphone permission
pub fn get_microphone_remediation_instructions() -> String {
    #[cfg(target_os = "macos")]
    {
        macos::get_microphone_remediation_instructions()
    }

    #[cfg(target_os = "windows")]
    {
        windows::get_microphone_remediation_instructions()
    }

    #[cfg(target_os = "linux")]
    {
        linux::get_microphone_remediation_instructions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_result_granted() {
        let result = PermissionResult::granted();
        assert_eq!(result.status, PermissionStatus::Granted);
        assert!(result.message.is_none());
    }

    #[test]
    fn test_permission_result_denied() {
        let result = PermissionResult::denied("Access denied");
        assert_eq!(result.status, PermissionStatus::Denied);
        assert_eq!(result.message, Some("Access denied".to_string()));
    }

    #[test]
    fn test_check_accessibility_permission() {
        let result = check_accessibility_permission();
        // Should return a valid status
        assert!(matches!(
            result.status,
            PermissionStatus::Granted
                | PermissionStatus::Denied
                | PermissionStatus::NotDetermined
                | PermissionStatus::NotApplicable
        ));
    }

    #[test]
    fn test_check_microphone_permission() {
        let result = check_microphone_permission();
        // Should return a valid status
        assert!(matches!(
            result.status,
            PermissionStatus::Granted
                | PermissionStatus::Denied
                | PermissionStatus::NotDetermined
                | PermissionStatus::NotApplicable
        ));
    }

    #[test]
    fn test_get_remediation_instructions() {
        let accessibility_instructions = get_accessibility_remediation_instructions();
        let microphone_instructions = get_microphone_remediation_instructions();

        // Instructions should not be empty
        assert!(!accessibility_instructions.is_empty());
        assert!(!microphone_instructions.is_empty());
    }
}
