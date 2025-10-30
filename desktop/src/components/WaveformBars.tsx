import { useEffect, useRef } from 'react'

interface WaveformBarsProps {
	/** Audio level (0.0 to 1.0) */
	audioLevel: number
	/** Number of bars to display (3-5) */
	barCount?: number
}

/**
 * WaveformBars - Visual audio level indicator with animated vertical bars
 *
 * Displays 3-5 white vertical bars that animate based on real-time audio levels.
 * Uses requestAnimationFrame for smooth 60fps animations with hardware acceleration.
 */
export default function WaveformBars({ audioLevel, barCount = 5 }: WaveformBarsProps) {
	const barsRef = useRef<(HTMLDivElement | null)[]>([])
	const animationFrameRef = useRef<number>()
	const previousLevelsRef = useRef<number[]>(new Array(barCount).fill(0))

	useEffect(() => {
		const animate = () => {
			// Generate slightly different levels for each bar for visual variety
			const baseLevels = previousLevelsRef.current

			barsRef.current.forEach((bar, index) => {
				if (!bar) return

				// Create variation between bars (each bar responds slightly differently)
				const variation = Math.sin((Date.now() / 100) + index) * 0.15
				const targetLevel = Math.max(0.1, Math.min(1, audioLevel + variation))

				// Smooth interpolation between previous and target level
				const smoothLevel = baseLevels[index] * 0.7 + targetLevel * 0.3
				previousLevelsRef.current[index] = smoothLevel

				// Convert level to height percentage (min 10%, max 100%)
				const heightPercent = Math.max(10, smoothLevel * 100)
				bar.style.height = `${heightPercent}%`
			})

			animationFrameRef.current = requestAnimationFrame(animate)
		}

		animate()

		return () => {
			if (animationFrameRef.current) {
				cancelAnimationFrame(animationFrameRef.current)
			}
		}
	}, [audioLevel, barCount])

	return (
		<div className="flex items-center justify-center gap-[3px] h-full px-2">
			{Array.from({ length: barCount }).map((_, index) => (
				<div
					key={index}
					ref={(el) => (barsRef.current[index] = el)}
					className="w-[3px] bg-white rounded-full transition-all duration-75 ease-out"
					style={{
						height: '10%',
						willChange: 'height',
					}}
				/>
			))}
		</div>
	)
}
