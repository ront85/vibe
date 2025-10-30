import { invoke, isTauri } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { InfoTooltip } from '~/components/InfoTooltip'
import { AudioDevice } from '~/lib/audio'
import { useDictation } from '~/providers/Dictation'
import toast from 'react-hot-toast'

type Platform = 'linux' | 'macos' | 'windows'

interface DictationSettingsProps {
	// No props needed - all state comes from DictationProvider
}

// List of available Whisper models
const WHISPER_MODELS = [
	{ value: 'tiny', label: 'Tiny', description: 'Fastest, lowest accuracy' },
	{ value: 'base', label: 'Base', description: 'Fast, basic accuracy' },
	{ value: 'small', label: 'Small', description: 'Balanced (recommended)' },
	{ value: 'medium', label: 'Medium', description: 'Slower, better accuracy' },
	{ value: 'large', label: 'Large', description: 'Slowest, best accuracy' },
]

// Common keyboard shortcuts that should trigger warnings
const CONFLICTING_SHORTCUTS = [
	'Cmd+S', // Save
	'Cmd+C', // Copy
	'Cmd+V', // Paste
	'Cmd+X', // Cut
	'Cmd+Z', // Undo
	'Cmd+A', // Select All
	'Cmd+W', // Close Window
	'Cmd+Q', // Quit
	'Cmd+T', // New Tab
	'Ctrl+S', // Save (Windows/Linux)
	'Ctrl+C', // Copy (Windows/Linux)
	'Ctrl+V', // Paste (Windows/Linux)
	'Ctrl+X', // Cut (Windows/Linux)
	'Ctrl+Z', // Undo (Windows/Linux)
	'Ctrl+A', // Select All (Windows/Linux)
	'Ctrl+W', // Close Window (Windows/Linux)
]

export default function DictationSettings({}: DictationSettingsProps) {
	const { t } = useTranslation()
	const dictation = useDictation()

	const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([])
	const [loadingDevices, setLoadingDevices] = useState(false)
	const [shortcutInput, setShortcutInput] = useState(dictation.keyboardShortcut)
	const [shortcutConflict, setShortcutConflict] = useState<string | null>(null)
	const [platform, setPlatform] = useState<Platform | null>(null)

	// Load audio devices on mount
	useEffect(() => {
		const initSettings = async () => {
			await loadAudioDevices()
			if (isTauri()) {
				const os = await import('@tauri-apps/plugin-os')
				setPlatform(os.platform())
			}
		}
		initSettings()
	}, [])

	// Check for shortcut conflicts when input changes
	useEffect(() => {
		checkShortcutConflict(shortcutInput)
	}, [shortcutInput])

	async function loadAudioDevices() {
		if (!isTauri()) {
			return
		}
		try {
			setLoadingDevices(true)
			const devices = await invoke<AudioDevice[]>('get_audio_devices')
			setAudioDevices(devices)
		} catch (error) {
			console.error('Failed to load audio devices:', error)
			toast.error(t('common.error'))
		} finally {
			setLoadingDevices(false)
		}
	}

	function checkShortcutConflict(shortcut: string) {
		const normalized = shortcut.trim()
		if (CONFLICTING_SHORTCUTS.includes(normalized)) {
			setShortcutConflict(
				`Warning: "${normalized}" conflicts with common system shortcuts and may not work reliably in all applications.`
			)
		} else {
			setShortcutConflict(null)
		}
	}

	function handleShortcutChange(value: string) {
		setShortcutInput(value)
	}

	function handleShortcutSave() {
		dictation.setKeyboardShortcut(shortcutInput)
		toast.success(t('common.saved'))
	}

	function handleMicrophoneChange(deviceId: string) {
		dictation.setMicrophoneDeviceId(deviceId || null)
		toast.success(t('common.saved'))
	}

	function handleModelChange(modelName: string) {
		dictation.setModelName(modelName)
		toast.success(t('common.saved'))
	}

	function handleToggleWidget(enabled: boolean) {
		dictation.setShowFloatingWidget(enabled)
		toast.success(enabled ? t('common.enabled') : t('common.disabled'))
	}

	function handleToggleAudioFeedback(enabled: boolean) {
		dictation.setAudioFeedbackEnabled(enabled)
		toast.success(enabled ? t('common.enabled') : t('common.disabled'))
	}

	// Get filtered input devices
	const inputDevices = audioDevices.filter((d) => d.isInput)

	// Get platform-specific default shortcut hint
	const defaultShortcut = platform === 'macos' ? 'Cmd+Shift+Space' : 'Ctrl+Alt+S'

	return (
		<div className="flex flex-col gap-6">
			{/* Microphone Selection */}
			<label className="form-control w-full">
				<div className="label">
					<span className="label-text flex items-center gap-1">
						<InfoTooltip text={t('common.info-dictation-microphone')} />
						{t('common.microphone')}
					</span>
				</div>
				<select
					value={dictation.microphoneDeviceId ?? ''}
					onChange={(e) => handleMicrophoneChange(e.target.value)}
					onFocus={loadAudioDevices}
					className="select select-bordered"
					disabled={loadingDevices}>
					<option value="">{t('common.select-device')}</option>
					{inputDevices.map((device) => (
						<option key={device.id} value={device.id}>
							{device.name} {device.isDefault ? '(Default)' : ''}
						</option>
					))}
				</select>
			</label>

			{/* Whisper Model Selection */}
			<label className="form-control w-full">
				<div className="label">
					<span className="label-text flex items-center gap-1">
						<InfoTooltip text={t('common.info-dictation-model')} />
						{t('common.dictation-model')}
					</span>
				</div>
				<select
					value={dictation.modelName}
					onChange={(e) => handleModelChange(e.target.value)}
					className="select select-bordered">
					{WHISPER_MODELS.map((model) => (
						<option key={model.value} value={model.value}>
							{model.label} - {model.description}
						</option>
					))}
				</select>
			</label>

			{/* Keyboard Shortcut Customization */}
			<div className="form-control w-full">
				<div className="label">
					<span className="label-text flex items-center gap-1">
						<InfoTooltip text={t('common.info-dictation-shortcut')} />
						{t('common.dictation-keyboard-shortcut')}
					</span>
				</div>
				<div className="flex gap-2">
					<input
						type="text"
						value={shortcutInput}
						onChange={(e) => handleShortcutChange(e.target.value)}
						onBlur={handleShortcutSave}
						onKeyDown={(e) => {
							if (e.key === 'Enter') {
								handleShortcutSave()
								e.currentTarget.blur()
							}
						}}
						placeholder={defaultShortcut}
						className="input input-bordered flex-1"
					/>
				</div>
				{shortcutConflict && (
					<div className="label">
						<span className="label-text-alt text-warning">{shortcutConflict}</span>
					</div>
				)}
				<div className="label">
					<span className="label-text-alt opacity-60">
						Default: {defaultShortcut}. Press Enter to save.
					</span>
				</div>
			</div>

			{/* Toggle Floating Widget */}
			<div className="form-control">
				<label className="label cursor-pointer">
					<span className="label-text flex items-center gap-1">
						<InfoTooltip text={t('common.info-dictation-widget')} />
						{t('common.dictation-show-widget')}
					</span>
					<input
						type="checkbox"
						className="toggle toggle-primary"
						checked={dictation.showFloatingWidget}
						onChange={(e) => handleToggleWidget(e.target.checked)}
					/>
				</label>
			</div>

			{/* Toggle Audio Feedback */}
			<div className="form-control">
				<label className="label cursor-pointer">
					<span className="label-text flex items-center gap-1">
						<InfoTooltip text={t('common.info-dictation-audio-feedback')} />
						{t('common.dictation-audio-feedback')}
					</span>
					<input
						type="checkbox"
						className="toggle toggle-primary"
						checked={dictation.audioFeedbackEnabled}
						onChange={(e) => handleToggleAudioFeedback(e.target.checked)}
					/>
				</label>
			</div>

			{/* Usage Instructions */}
			<div className="alert alert-info">
				<div className="flex flex-col gap-2">
					<p className="font-semibold">{t('common.dictation-usage-title')}</p>
					<ul className="list-disc list-inside text-sm opacity-80">
						<li>{t('common.dictation-usage-1')}</li>
						<li>{t('common.dictation-usage-2')}</li>
						<li>{t('common.dictation-usage-3')}</li>
					</ul>
				</div>
			</div>
		</div>
	)
}
