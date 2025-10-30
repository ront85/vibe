import * as dialog from '@tauri-apps/plugin-dialog'
import { useNavigate } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import { ReactComponent as SettingsIcon } from '~/icons/wrench.svg'
import { ReactComponent as BatchIcon } from '~/icons/file.svg'
import { ReactComponent as HistoryIcon } from '~/icons/link.svg'

interface AppMenuProps {
	onClickSettings: () => void
	availableUpdate?: string
	updateApp?: () => void
}

export default function AppMenu({ onClickSettings, availableUpdate, updateApp }: AppMenuProps) {
	const navigate = useNavigate()
	const { t } = useTranslation()

	async function onClickUpdate() {
		if (!availableUpdate || !updateApp) return
		const confirmation = await dialog.confirm(`${t('common.ask-for-update-body', { version: availableUpdate })}`, {
			title: t('common.ask-for-update-title'),
			kind: 'info',
			okLabel: t('common.confirm-update'),
			cancelLabel: t('common.cancel-update'),
		})

		if (confirmation) {
			updateApp()
		}
	}

	return (
		<div className="absolute top-0 end-0 dropdown dropdown-end">
			<button tabIndex={0} role="button" className="btn btn-ghost btn-circle">
				<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block h-5 w-5 stroke-current">
					<path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path>
				</svg>
			</button>
			<ul tabIndex={0} className="menu dropdown-content z-[1] p-2 shadow bg-base-200 rounded-box w-52 mt-3 gap-1">
				{availableUpdate && (
					<li>
						<button onClick={onClickUpdate} className="btn btn-primary">
							{t('common.update-available')}
						</button>
					</li>
				)}
				<li>
					<button onMouseDown={() => navigate('/batch')}>
						<BatchIcon className="h-[18px] w-[18px]" />
						{t('common.transcribe-folder')}
					</button>
				</li>
				<li>
					<button onMouseDown={() => navigate('/history')}>
						<HistoryIcon className="h-[18px] w-[18px]" />
						{t('common.dictation-history')}
					</button>
				</li>
				<li>
					<button onMouseDown={onClickSettings}>
						<SettingsIcon className="h-[18px] w-[18px]" />
						{t('common.settings')}
					</button>
				</li>
			</ul>
		</div>
	)
}
