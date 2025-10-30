import { ModifyState, cx } from '~/lib/utils'
import SettingsPage from '~/pages/settings/Page'
import { useEffect, useState } from 'react'
import { isTauri } from '@tauri-apps/api/core'

interface SettingsModalProps {
	visible: boolean
	setVisible: ModifyState<boolean>
}
export default function SettingsModal({ visible, setVisible }: SettingsModalProps) {
	const [platform, setPlatform] = useState<string | null>(null)

	useEffect(() => {
		const detectPlatform = async () => {
			if (isTauri()) {
				const os = await import('@tauri-apps/plugin-os')
				setPlatform(os.platform())
			}
		}
		detectPlatform()
	}, [])

	if (visible) {
		// Don't use transparent background on Linux since the backdrop doesn't work!
		const isDarkTransparent = platform === null ? false : platform !== 'linux'
		return (
			<div className={cx('modal modal-open backdrop-blur-3xl !bg-base-100 overflow-y-auto', isDarkTransparent && 'dark:!bg-transparent')}>
				<SettingsPage setVisible={setVisible} />
			</div>
		)
	}
}
