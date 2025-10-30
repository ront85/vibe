import { useEffect, useState, useRef } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import WaveformBars from './WaveformBars'
import { useTranslation } from 'react-i18next'

type DictationState = 'idle' | 'recording' | 'processing' | 'error'

interface FloatingWidgetProps {
	/** Whether the widget is enabled and visible */
	enabled: boolean
	/** Keyboard shortcut text to display in tooltip (e.g., "Cmd+Shift+Space") */
	shortcut: string
}

/**
 * FloatingWidget - Floating dictation control widget
 *
 * A minimal, always-on-top widget for speech-to-text dictation control.
 * States:
 * - Idle: 5px × 30px, 50% opacity, pill-shaped
 * - Hover: 30px height, 100% opacity, shows tooltip
 * - Recording: 30px height, shows waveform and controls
 * - Processing: 30px height, shows loading animation
 *
 * Position: Center horizontally, 100px from screen bottom
 */
export default function FloatingWidget({ enabled, shortcut }: FloatingWidgetProps) {
	const [state, setState] = useState<DictationState>('idle')
	const [audioLevel, setAudioLevel] = useState(0)
	const [isHovered, setIsHovered] = useState(false)
	const [error, setError] = useState<string | null>(null)
	const unlistenStateRef = useRef<(() => void) | null>(null)
	const unlistenAudioRef = useRef<(() => void) | null>(null)
	const { t } = useTranslation()

	// Listen to backend state changes
	useEffect(() => {
		const setupListeners = async () => {
			// Listen for dictation state changes
			const unlistenState = await listen<string>('dictation_state_change', (event) => {
				const newState = event.payload as DictationState
				setState(newState)
				if (newState === 'idle') {
					setAudioLevel(0)
					setError(null)
				}
			})
			unlistenStateRef.current = unlistenState

			// Listen for audio level updates
			const unlistenAudio = await listen<number>('audio_level_update', (event) => {
				setAudioLevel(event.payload)
			})
			unlistenAudioRef.current = unlistenAudio
		}

		setupListeners()

		return () => {
			unlistenStateRef.current?.()
			unlistenAudioRef.current?.()
		}
	}, [])

	const handleClick = async () => {
		try {
			if (state === 'idle') {
				// Start recording
				await invoke('start_dictation')
				setState('recording')
			} else if (state === 'recording') {
				// Stop recording
				await invoke('stop_dictation')
				setState('processing')
			}
		} catch (err) {
			console.error('Failed to toggle dictation:', err)
			setError(err instanceof Error ? err.message : 'Unknown error')
			setState('error')
		}
	}

	const handleCancel = async (e: React.MouseEvent) => {
		e.stopPropagation()
		try {
			await invoke('cancel_dictation')
			setState('idle')
			setAudioLevel(0)
		} catch (err) {
			console.error('Failed to cancel dictation:', err)
		}
	}

	if (!enabled) {
		return null
	}

	const isExpanded = state !== 'idle' || isHovered
	const showControls = state === 'recording'

	return (
		<div
			className="fixed left-1/2 -translate-x-1/2 z-50 transition-all duration-300 ease-out"
			style={{
				bottom: '100px',
				willChange: 'height, opacity',
			}}>
			<div
				className={`
					relative flex items-center justify-center
					bg-black rounded-full cursor-pointer
					transition-all duration-300 ease-out
					${isExpanded ? 'h-[30px] w-[200px] opacity-100' : 'h-[5px] w-[30px] opacity-50'}
					${state === 'error' ? 'bg-red-600' : ''}
				`}
				onClick={handleClick}
				onMouseEnter={() => setIsHovered(true)}
				onMouseLeave={() => setIsHovered(false)}
				style={{
					willChange: 'height, width, opacity',
				}}>
				{/* Idle state with hover tooltip */}
				{state === 'idle' && isHovered && (
					<div
						className="absolute bottom-full mb-2 px-3 py-2 bg-black text-white text-sm rounded-lg whitespace-nowrap animate-in fade-in duration-200"
						style={{ pointerEvents: 'none' }}>
						{t('dictation.tooltip', {
							defaultValue: `Click or hold ${shortcut} to start dictating`,
							shortcut,
						})}
					</div>
				)}

				{/* Recording state - waveform visualization */}
				{state === 'recording' && (
					<div className="flex items-center justify-between w-full h-full px-2">
						{showControls && (
							<button
								onClick={handleCancel}
								className="flex items-center justify-center w-6 h-6 hover:bg-white/20 rounded-full transition-colors"
								aria-label="Cancel">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									className="w-4 h-4 text-white"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									strokeWidth="2"
									strokeLinecap="round"
									strokeLinejoin="round">
									<line x1="18" y1="6" x2="6" y2="18" />
									<line x1="6" y1="6" x2="18" y2="18" />
								</svg>
							</button>
						)}

						<div className="flex-1 h-full">
							<WaveformBars audioLevel={audioLevel} barCount={5} />
						</div>

						{showControls && (
							<button
								onClick={handleClick}
								className="flex items-center justify-center w-6 h-6 hover:bg-white/20 rounded-full transition-colors"
								aria-label="Stop recording">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									className="w-4 h-4 text-white"
									viewBox="0 0 24 24"
									fill="currentColor">
									<rect x="6" y="6" width="12" height="12" rx="2" />
								</svg>
							</button>
						)}
					</div>
				)}

				{/* Processing state - loading animation */}
				{state === 'processing' && (
					<div className="relative w-full h-full flex items-center justify-center overflow-hidden">
						<div className="absolute inset-0 flex items-center justify-center">
							<div className="w-[80%] h-[3px] bg-white/20 rounded-full overflow-hidden">
								<div
									className="h-full bg-white rounded-full animate-pulse"
									style={{
										width: '30%',
										animation: 'slide 1.5s ease-in-out infinite',
									}}
								/>
							</div>
						</div>
					</div>
				)}

				{/* Error state */}
				{state === 'error' && error && (
					<div className="text-white text-xs px-3 truncate">{error}</div>
				)}

				{/* Idle dots animation */}
				{state === 'idle' && isHovered && (
					<div className="flex gap-1">
						{[0, 1, 2].map((i) => (
							<div
								key={i}
								className="w-1 h-1 bg-white rounded-full animate-pulse"
								style={{
									animationDelay: `${i * 0.15}s`,
								}}
							/>
						))}
					</div>
				)}
			</div>

			{/* CSS animation for sliding bar */}
			<style>{`
				@keyframes slide {
					0% {
						transform: translateX(-100%);
					}
					50% {
						transform: translateX(250%);
					}
					100% {
						transform: translateX(-100%);
					}
				}
			`}</style>
		</div>
	)
}
