import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { ReactComponent as CopyIcon } from '~/icons/copy.svg'
import { ReactComponent as EditIcon } from '~/icons/wrench.svg'
import { ReactComponent as DeleteIcon } from '~/icons/reset.svg'

export interface DictationHistoryEntryData {
	id: number
	timestamp: string
	transcription_text: string
	destination_app: string
	model_used: string
	duration_seconds: number
}

interface DictationHistoryEntryProps {
	entry: DictationHistoryEntryData
	onCopy: (text: string) => void
	onEdit: (id: number, newText: string) => void
	onDelete: (id: number) => void
	searchQuery?: string
}

export default function DictationHistoryEntry({ entry, onCopy, onEdit, onDelete, searchQuery = '' }: DictationHistoryEntryProps) {
	const { t } = useTranslation()
	const [isEditing, setIsEditing] = useState(false)
	const [editedText, setEditedText] = useState(entry.transcription_text)

	// Format timestamp
	const formatTimestamp = (timestamp: string) => {
		try {
			const date = new Date(timestamp)
			const now = new Date()
			const diffMs = now.getTime() - date.getTime()
			const diffMins = Math.floor(diffMs / 60000)
			const diffHours = Math.floor(diffMs / 3600000)
			const diffDays = Math.floor(diffMs / 86400000)

			if (diffMins < 1) return t('common.just-now')
			if (diffMins < 60) return t('common.minutes-ago', { count: diffMins })
			if (diffHours < 24) return t('common.hours-ago', { count: diffHours })
			if (diffDays < 7) return t('common.days-ago', { count: diffDays })

			// Format as date/time for older entries
			return date.toLocaleString(undefined, {
				year: 'numeric',
				month: 'short',
				day: 'numeric',
				hour: '2-digit',
				minute: '2-digit',
			})
		} catch (error) {
			return timestamp
		}
	}

	// Format duration
	const formatDuration = (seconds: number) => {
		if (seconds < 60) {
			return `${Math.round(seconds)}s`
		}
		const mins = Math.floor(seconds / 60)
		const secs = Math.round(seconds % 60)
		return `${mins}m ${secs}s`
	}

	// Highlight search query in text
	const highlightText = (text: string, query: string) => {
		if (!query.trim()) return text

		const regex = new RegExp(`(${query})`, 'gi')
		const parts = text.split(regex)

		return parts.map((part, index) =>
			regex.test(part) ? (
				<mark key={index} className="bg-yellow-200 dark:bg-yellow-600">
					{part}
				</mark>
			) : (
				part
			)
		)
	}

	// Handle save edit
	const handleSaveEdit = () => {
		if (editedText.trim() && editedText !== entry.transcription_text) {
			onEdit(entry.id, editedText)
		}
		setIsEditing(false)
	}

	// Handle cancel edit
	const handleCancelEdit = () => {
		setEditedText(entry.transcription_text)
		setIsEditing(false)
	}

	return (
		<div className="card bg-base-200 shadow-md hover:shadow-lg transition-shadow">
			<div className="card-body p-4">
				{/* Header: Timestamp and App */}
				<div className="flex justify-between items-start mb-2">
					<div className="flex flex-col">
						<span className="text-xs text-base-content opacity-60">{formatTimestamp(entry.timestamp)}</span>
						<span className="text-sm font-medium text-base-content mt-1">
							{entry.destination_app || t('common.unknown-app')}
						</span>
					</div>
					<div className="flex gap-2">
						{/* Copy button */}
						<button
							onClick={() => onCopy(entry.transcription_text)}
							className="btn btn-ghost btn-sm btn-square"
							title={t('common.copy')}>
							<CopyIcon className="w-4 h-4" />
						</button>

						{/* Edit button */}
						<button
							onClick={() => setIsEditing(!isEditing)}
							className="btn btn-ghost btn-sm btn-square"
							title={t('common.edit')}>
							<EditIcon className="w-4 h-4" />
						</button>

						{/* Delete button */}
						<button
							onClick={() => onDelete(entry.id)}
							className="btn btn-ghost btn-sm btn-square text-error"
							title={t('common.delete')}>
							<DeleteIcon className="w-4 h-4" />
						</button>
					</div>
				</div>

				{/* Transcription text */}
				{!isEditing ? (
					<div className="text-base-content leading-relaxed whitespace-pre-wrap break-words">
						{highlightText(entry.transcription_text, searchQuery)}
					</div>
				) : (
					<div className="mt-2">
						<textarea
							className="textarea textarea-bordered w-full min-h-[100px]"
							value={editedText}
							onChange={(e) => setEditedText(e.target.value)}
							autoFocus
						/>
						<div className="flex gap-2 mt-2">
							<button onClick={handleSaveEdit} className="btn btn-primary btn-sm">
								{t('common.save')}
							</button>
							<button onClick={handleCancelEdit} className="btn btn-ghost btn-sm">
								{t('common.cancel')}
							</button>
						</div>
					</div>
				)}

				{/* Footer: Model and Duration */}
				<div className="flex justify-between items-center mt-3 pt-3 border-t border-base-300">
					<span className="text-xs text-base-content opacity-50">
						{t('common.model')}: {entry.model_used}
					</span>
					<span className="text-xs text-base-content opacity-50">
						{t('common.duration')}: {formatDuration(entry.duration_seconds)}
					</span>
				</div>
			</div>
		</div>
	)
}
