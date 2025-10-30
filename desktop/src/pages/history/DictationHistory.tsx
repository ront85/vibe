import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { invoke } from '@tauri-apps/api/core'
import { path } from '@tauri-apps/api'
import toast from 'react-hot-toast'
import Layout from '~/components/Layout'
import DictationHistoryEntry from '~/components/DictationHistoryEntry'
import { ReactComponent as SearchIcon } from '~/icons/link.svg'

export interface DictationHistoryEntry {
	id: number
	timestamp: string
	transcription_text: string
	destination_app: string
	model_used: string
	duration_seconds: number
}

export default function DictationHistory() {
	const { t } = useTranslation()
	const [entries, setEntries] = useState<DictationHistoryEntry[]>([])
	const [filteredEntries, setFilteredEntries] = useState<DictationHistoryEntry[]>([])
	const [searchQuery, setSearchQuery] = useState('')
	const [loading, setLoading] = useState(true)
	const [dbPath, setDbPath] = useState<string | null>(null)

	// Get database path
	useEffect(() => {
		async function getDbPath() {
			try {
				const appDataDir = await path.appDataDir()
				const historyDbPath = await path.join(appDataDir, 'dictation_history.db')
				setDbPath(historyDbPath)
			} catch (error) {
				console.error('Failed to get database path:', error)
				toast.error(t('common.error-loading-history'))
			}
		}
		getDbPath()
	}, [t])

	// Load history entries
	useEffect(() => {
		if (!dbPath) return

		async function loadHistory() {
			setLoading(true)
			try {
				const historyEntries = await invoke<DictationHistoryEntry[]>('get_dictation_history', {
					dbPath,
				})
				setEntries(historyEntries)
				setFilteredEntries(historyEntries)
			} catch (error) {
				console.error('Failed to load history:', error)
				toast.error(t('common.error-loading-history'))
			} finally {
				setLoading(false)
			}
		}

		loadHistory()
	}, [dbPath, t])

	// Handle search with debounce
	useEffect(() => {
		if (!searchQuery.trim()) {
			setFilteredEntries(entries)
			return
		}

		const timer = setTimeout(async () => {
			if (!dbPath) return

			try {
				const searchResults = await invoke<DictationHistoryEntry[]>('search_dictation_history', {
					dbPath,
					query: searchQuery,
				})
				setFilteredEntries(searchResults)
			} catch (error) {
				console.error('Search failed:', error)
				// Fallback to local filtering
				const filtered = entries.filter(
					(entry) =>
						entry.transcription_text.toLowerCase().includes(searchQuery.toLowerCase()) ||
						entry.destination_app.toLowerCase().includes(searchQuery.toLowerCase())
				)
				setFilteredEntries(filtered)
			}
		}, 100) // 100ms debounce

		return () => clearTimeout(timer)
	}, [searchQuery, entries, dbPath])

	// Handle copy
	const handleCopy = async (text: string) => {
		try {
			await navigator.clipboard.writeText(text)
			toast.success(t('common.transcript-copied'))
		} catch (error) {
			console.error('Copy failed:', error)
			toast.error(t('common.error-copying'))
		}
	}

	// Handle edit
	const handleEdit = async (id: number, newText: string) => {
		if (!dbPath) return

		try {
			await invoke('update_dictation_entry', {
				dbPath,
				id,
				newText,
			})

			// Update local state
			const updatedEntries = entries.map((entry) =>
				entry.id === id ? { ...entry, transcription_text: newText } : entry
			)
			setEntries(updatedEntries)
			setFilteredEntries(
				filteredEntries.map((entry) => (entry.id === id ? { ...entry, transcription_text: newText } : entry))
			)

			toast.success(t('common.entry-updated'))
		} catch (error) {
			console.error('Edit failed:', error)
			toast.error(t('common.error-updating-entry'))
		}
	}

	// Handle delete
	const handleDelete = async (id: number) => {
		if (!dbPath) return

		// Show confirmation dialog
		const confirmed = window.confirm(t('common.confirm-delete-entry'))
		if (!confirmed) return

		try {
			await invoke('delete_dictation_entry', {
				dbPath,
				id,
			})

			// Update local state
			const updatedEntries = entries.filter((entry) => entry.id !== id)
			setEntries(updatedEntries)
			setFilteredEntries(filteredEntries.filter((entry) => entry.id !== id))

			toast.success(t('common.entry-deleted'))
		} catch (error) {
			console.error('Delete failed:', error)
			toast.error(t('common.error-deleting-entry'))
		}
	}

	return (
		<Layout>
			<div className="flex flex-col w-full max-w-[1000px] m-auto px-5">
				{/* Header */}
				<div className="text-center mb-6">
					<h2 className="text-3xl font-normal text-base-content">{t('common.dictation-history')}</h2>
				</div>

				{/* Search bar */}
				<div className="form-control mb-6">
					<div className="relative">
						<input
							type="text"
							placeholder={t('common.search-history-placeholder')}
							className="input input-bordered w-full pr-10"
							value={searchQuery}
							onChange={(e) => setSearchQuery(e.target.value)}
						/>
						<div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
							<SearchIcon className="w-5 h-5 opacity-50" />
						</div>
					</div>
				</div>

				{/* Loading state */}
				{loading && (
					<div className="flex justify-center items-center py-20">
						<span className="loading loading-spinner loading-lg text-primary"></span>
					</div>
				)}

				{/* Empty state */}
				{!loading && filteredEntries.length === 0 && !searchQuery && (
					<div className="flex flex-col items-center justify-center py-20 text-center">
						<div className="text-6xl mb-4">🎤</div>
						<h3 className="text-xl font-medium text-base-content mb-2">{t('common.no-history-yet')}</h3>
						<p className="text-base-content opacity-60">{t('common.start-dictating-to-see-history')}</p>
					</div>
				)}

				{/* No search results */}
				{!loading && filteredEntries.length === 0 && searchQuery && (
					<div className="flex flex-col items-center justify-center py-20 text-center">
						<div className="text-6xl mb-4">🔍</div>
						<h3 className="text-xl font-medium text-base-content mb-2">{t('common.no-results-found')}</h3>
						<p className="text-base-content opacity-60">{t('common.try-different-search')}</p>
					</div>
				)}

				{/* History list */}
				{!loading && filteredEntries.length > 0 && (
					<div className="space-y-4 pb-10">
						{filteredEntries.map((entry) => (
							<DictationHistoryEntry
								key={entry.id}
								entry={entry}
								onCopy={handleCopy}
								onEdit={handleEdit}
								onDelete={handleDelete}
								searchQuery={searchQuery}
							/>
						))}
					</div>
				)}
			</div>
		</Layout>
	)
}
