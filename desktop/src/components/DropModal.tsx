import * as event from '@tauri-apps/api/event'
import { basename } from '@tauri-apps/api/path'
import * as webview from '@tauri-apps/api/webview'
import { useEffect, useRef, useState } from 'react'
import { isTauri } from '@tauri-apps/api/core'
import { ReactComponent as DocumentIcon } from '~/icons/document.svg'
import { cx, formatLongString, validPath } from '~/lib/utils'

type Platform = 'linux' | 'macos' | 'windows'

interface Position {
	x: number
	y: number
}

function Document({ position, path }: { position: Position; path: string }) {
	return (
		<div
			className="absolute z-[100000] bg-base-300 p-4 -translate-x-[50%] -translate-y-[50%] rounded-2xl flex flex-col items-center justify-center gap-4"
			style={{ left: position.x, top: position.y, cursor: 'grabbing' }}>
			<DocumentIcon />
			<p className="text-md font-light font-mono">{formatLongString(path, 10)}</p>
		</div>
	)
}

export default function DropModal() {
	const [open, setOpen] = useState(false)
	const [position, setPosition] = useState<Position>({ x: 0, y: 0 })
	const listeners = useRef<event.UnlistenFn[]>([])
	const [path, setPath] = useState('')
	const [platform, setPlatform] = useState<Platform>('macos')

	async function handleDrops() {
		// Only set up drag listeners in Tauri context
		if (!isTauri()) {
			return
		}

		// Add blur
		listeners.current.push(
			await event.listen('tauri://drag-enter', () => {
				setOpen(true)
			})
		)

		// Remove blur
		listeners.current.push(
			await event.listen('tauri://drag-leave', (_event) => {
				setOpen(false)
			})
		)

		// Update positions
		listeners.current.push(
			await event.listen<{ position: Position }>('tauri://drag-over', (event) => {
				setPosition(event.payload.position)
			})
		)

		// Get dropped path
		listeners.current.push(
			await event.listen<{ paths?: string[] }>('tauri://drag-drop', async (event) => {
				// const paths = ((event.payload as any)?.paths as string[]) ?? []
				const { paths } = event.payload
				if (paths && paths.length > 0) {
					const newPath = await basename(paths[0])
					if (validPath(newPath)) {
						setPath(newPath)
						setOpen(true)
						// Focus window
						const currentWindow = webview.getCurrentWebview().window
						currentWindow.setFocus()
					}
				}
				setOpen(false)
			})
		)
	}

	useEffect(() => {
		const initDrops = async () => {
			await handleDrops()
			if (isTauri()) {
				const os = await import('@tauri-apps/plugin-os')
				setPlatform(os.platform())
			}
		}
		initDrops()
		return () => {
			listeners.current.forEach((unlisten) => unlisten())
		}
	}, [])

	return (
		<div>
			{open && platform === 'windows' && <Document path={path} position={position} />}
			<div className={cx('modal backdrop-blur-sm bg-base-100', open && 'modal-open')}></div>
		</div>
	)
}
