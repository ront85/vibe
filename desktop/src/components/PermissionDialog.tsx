import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/core'

interface PermissionResult {
  status: 'Granted' | 'Denied' | 'NotDetermined' | 'NotApplicable'
  message?: string
}

interface PermissionDialogProps {
  onClose: () => void
  onPermissionsGranted?: () => void
}

export function PermissionDialog({ onClose, onPermissionsGranted }: PermissionDialogProps) {
  const [accessibilityStatus, setAccessibilityStatus] = useState<PermissionResult | null>(null)
  const [microphoneStatus, setMicrophoneStatus] = useState<PermissionResult | null>(null)
  const [accessibilityInstructions, setAccessibilityInstructions] = useState<string>('')
  const [microphoneInstructions, setMicrophoneInstructions] = useState<string>('')
  const [showAccessibilityInstructions, setShowAccessibilityInstructions] = useState(false)
  const [showMicrophoneInstructions, setShowMicrophoneInstructions] = useState(false)

  useEffect(() => {
    checkPermissions()
    loadInstructions()
  }, [])

  const checkPermissions = async () => {
    try {
      const accessibilityResult = await invoke<PermissionResult>(
        'check_dictation_accessibility_permission'
      )
      const microphoneResult = await invoke<PermissionResult>('check_dictation_microphone_permission')

      setAccessibilityStatus(accessibilityResult)
      setMicrophoneStatus(microphoneResult)

      // If both are granted, notify parent
      if (
        accessibilityResult.status === 'Granted' &&
        (microphoneResult.status === 'Granted' || microphoneResult.status === 'NotApplicable')
      ) {
        onPermissionsGranted?.()
      }
    } catch (error) {
      console.error('Error checking permissions:', error)
    }
  }

  const loadInstructions = async () => {
    try {
      const accessibilityInstr = await invoke<string>('get_dictation_accessibility_instructions')
      const microphoneInstr = await invoke<string>('get_dictation_microphone_instructions')

      setAccessibilityInstructions(accessibilityInstr)
      setMicrophoneInstructions(microphoneInstr)
    } catch (error) {
      console.error('Error loading instructions:', error)
    }
  }

  const needsAccessibility =
    accessibilityStatus?.status === 'Denied' || accessibilityStatus?.status === 'NotDetermined'
  const needsMicrophone =
    microphoneStatus?.status === 'Denied' || microphoneStatus?.status === 'NotDetermined'

  const allGranted =
    accessibilityStatus?.status === 'Granted' &&
    (microphoneStatus?.status === 'Granted' || microphoneStatus?.status === 'NotApplicable')

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="modal-box max-w-2xl">
        <h3 className="text-lg font-bold">Dictation Permissions Required</h3>

        <div className="py-4 space-y-4">
          {allGranted ? (
            <div className="alert alert-success">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="stroke-current shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <span>All required permissions have been granted!</span>
            </div>
          ) : (
            <>
              <p className="text-sm">
                To use dictation, Vibe needs the following permissions:
              </p>

              {/* Accessibility Permission */}
              {needsAccessibility && (
                <div className="card bg-base-200">
                  <div className="card-body">
                    <h4 className="card-title text-base flex items-center gap-2">
                      {accessibilityStatus?.status === 'Denied' ? (
                        <span className="badge badge-error">Denied</span>
                      ) : (
                        <span className="badge badge-warning">Not Determined</span>
                      )}
                      Accessibility Permission
                    </h4>
                    <p className="text-sm">
                      Required to paste text at your cursor and register global keyboard shortcuts.
                    </p>
                    <div className="card-actions">
                      <button
                        className="btn btn-sm btn-primary"
                        onClick={() => setShowAccessibilityInstructions(!showAccessibilityInstructions)}
                      >
                        {showAccessibilityInstructions ? 'Hide' : 'Show'} Instructions
                      </button>
                    </div>
                    {showAccessibilityInstructions && (
                      <div className="mt-2 p-3 bg-base-300 rounded-lg">
                        <pre className="text-xs whitespace-pre-wrap">{accessibilityInstructions}</pre>
                      </div>
                    )}
                  </div>
                </div>
              )}

              {/* Microphone Permission */}
              {needsMicrophone && (
                <div className="card bg-base-200">
                  <div className="card-body">
                    <h4 className="card-title text-base flex items-center gap-2">
                      {microphoneStatus?.status === 'Denied' ? (
                        <span className="badge badge-error">Denied</span>
                      ) : (
                        <span className="badge badge-warning">Not Determined</span>
                      )}
                      Microphone Permission
                    </h4>
                    <p className="text-sm">Required to record your voice for transcription.</p>
                    <div className="card-actions">
                      <button
                        className="btn btn-sm btn-primary"
                        onClick={() => setShowMicrophoneInstructions(!showMicrophoneInstructions)}
                      >
                        {showMicrophoneInstructions ? 'Hide' : 'Show'} Instructions
                      </button>
                    </div>
                    {showMicrophoneInstructions && (
                      <div className="mt-2 p-3 bg-base-300 rounded-lg">
                        <pre className="text-xs whitespace-pre-wrap">{microphoneInstructions}</pre>
                      </div>
                    )}
                  </div>
                </div>
              )}
            </>
          )}
        </div>

        <div className="modal-action">
          <button className="btn btn-sm" onClick={checkPermissions}>
            Re-check Permissions
          </button>
          <button className="btn btn-sm btn-primary" onClick={onClose}>
            {allGranted ? 'Continue' : 'I\'ll Grant Permissions Later'}
          </button>
        </div>
      </div>
    </div>
  )
}
