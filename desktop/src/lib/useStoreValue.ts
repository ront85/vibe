import { load } from '@tauri-apps/plugin-store'
import { isTauri } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import * as config from '~/lib/config'

export function useStoreValue<T>(key: string) {
	const [value, setValue] = useState<T | null>(null)

	const setValueWrapper = async (newValue: T) => {
		if (!isTauri()) {
			return
		}
		const store = await load(config.storeFilename)
		await store.set(key, newValue)
		await store.save()
		// Optimistic
		setValue(newValue)
	}

	async function initValue() {
		if (!isTauri()) {
			return
		}
		const store = await load(config.storeFilename)

		store.get<T>(key).then((currentValue) => {
			setValue(currentValue ?? null)
		})
	}

	useEffect(() => {
		initValue()
	}, [value])

	return [value, setValueWrapper] as const
}
