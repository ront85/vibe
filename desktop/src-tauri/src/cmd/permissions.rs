/// Tauri commands for permission management

use crate::permissions::{
    check_accessibility_permission, check_microphone_permission,
    get_accessibility_remediation_instructions, get_microphone_remediation_instructions,
    PermissionResult,
};

/// Check accessibility permission status
#[tauri::command]
pub fn check_dictation_accessibility_permission() -> PermissionResult {
    tracing::info!("Checking dictation accessibility permission");
    check_accessibility_permission()
}

/// Check microphone permission status
#[tauri::command]
pub fn check_dictation_microphone_permission() -> PermissionResult {
    tracing::info!("Checking dictation microphone permission");
    check_microphone_permission()
}

/// Get accessibility permission remediation instructions
#[tauri::command]
pub fn get_dictation_accessibility_instructions() -> String {
    get_accessibility_remediation_instructions()
}

/// Get microphone permission remediation instructions
#[tauri::command]
pub fn get_dictation_microphone_instructions() -> String {
    get_microphone_remediation_instructions()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions::PermissionStatus;

    #[test]
    fn test_check_dictation_accessibility_permission() {
        let result = check_dictation_accessibility_permission();
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
    fn test_check_dictation_microphone_permission() {
        let result = check_dictation_microphone_permission();
        assert!(matches!(
            result.status,
            PermissionStatus::Granted
                | PermissionStatus::Denied
                | PermissionStatus::NotDetermined
                | PermissionStatus::NotApplicable
        ));
    }

    #[test]
    fn test_get_dictation_accessibility_instructions() {
        let instructions = get_dictation_accessibility_instructions();
        assert!(!instructions.is_empty());
    }

    #[test]
    fn test_get_dictation_microphone_instructions() {
        let instructions = get_dictation_microphone_instructions();
        assert!(!instructions.is_empty());
    }
}
