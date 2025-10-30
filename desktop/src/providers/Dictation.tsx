import { ReactNode, createContext, useContext, useState, useEffect } from 'react'
import { useLocalStorage } from 'usehooks-ts'
import { isTauri } from '@tauri-apps/api/core'
import { ModifyState } from '~/lib/utils'

export interface DictationSettings {
	enabled: boolean
	setEnabled: ModifyState<boolean>
	showFloatingWidget: boolean
	setShowFloatingWidget: ModifyState<boolean>
	audioFeedbackEnabled: boolean
	setAudioFeedbackEnabled: ModifyState<boolean>
	keyboardShortcut: string
	setKeyboardShortcut: ModifyState<string>
	microphoneDeviceId: string | null
	setMicrophoneDeviceId: ModifyState<string | null>
	modelName: string
	setModelName: ModifyState<string>
}

// Create the context
const DictationContext = createContext<DictationSettings | null>(null)

// Custom hook to use the dictation context
export function useDictation() {
	const context = useContext(DictationContext)
	if (!context) {
		throw new Error('useDictation must be used within DictationProvider')
	}
	return context
}

// Dictation provider component
export function DictationProvider({ children }: { children: ReactNode }) {
	const [defaultShortcut, setDefaultShortcut] = useState('Ctrl+Alt+S')

	// Detect platform and set default shortcut
	useEffect(() => {
		const detectPlatform = async () => {
			if (isTauri()) {
				const { platform } = await import('@tauri-apps/plugin-os')
				const platformName = platform()
				if (platformName === 'macos') {
					setDefaultShortcut('Cmd+Shift+Space')
				}
			}
		}
		detectPlatform()
	}, [])

	const [enabled, setEnabled] = useLocalStorage('dictation_enabled', false)
	const [showFloatingWidget, setShowFloatingWidget] = useLocalStorage('dictation_show_widget', true)
	const [audioFeedbackEnabled, setAudioFeedbackEnabled] = useLocalStorage('dictation_audio_feedback', true)
	const [keyboardShortcut, setKeyboardShortcut] = useLocalStorage('dictation_keyboard_shortcut', defaultShortcut)
	const [microphoneDeviceId, setMicrophoneDeviceId] = useLocalStorage<string | null>('dictation_microphone_device', null)
	const [modelName, setModelName] = useLocalStorage('dictation_model_name', 'small')

	const settings: DictationSettings = {
		enabled,
		setEnabled,
		showFloatingWidget,
		setShowFloatingWidget,
		audioFeedbackEnabled,
		setAudioFeedbackEnabled,
		keyboardShortcut,
		setKeyboardShortcut,
		microphoneDeviceId,
		setMicrophoneDeviceId,
		modelName,
		setModelName,
	}

	return <DictationContext.Provider value={settings}>{children}</DictationContext.Provider>
}
