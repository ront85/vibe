import { ytDlpConfig } from './config'
import * as fs from '@tauri-apps/plugin-fs'
import * as path from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

let cachedPlatformName: string | null = null

async function getPlatformName() {
	if (cachedPlatformName === null) {
		// Only load platform when in Tauri context
		const isTauri = '__TAURI__' in window
		if (isTauri) {
			const { platform } = await import('@tauri-apps/plugin-os')
			cachedPlatformName = platform()
		} else {
			// In browser mode, default to linux
			cachedPlatformName = 'linux'
		}
	}
	return cachedPlatformName
}

async function getBinaryPath() {
	const platformName = await getPlatformName()
	const { name } = ytDlpConfig[platformName as keyof typeof ytDlpConfig]
	const localDataPath = await path.appLocalDataDir()
	const binaryPath = await path.join(localDataPath, name)
	return binaryPath
}

export async function exists() {
	const binaryPath = await getBinaryPath()
	return await fs.exists(binaryPath)
}

export async function downloadYtDlp() {
	const platformName = await getPlatformName()
	const { url } = ytDlpConfig[platformName as keyof typeof ytDlpConfig]
	const binaryPath = await getBinaryPath()
	await invoke('download_file', { url, path: binaryPath })
}

export async function downloadAudio(url: string, inDocuments?: boolean) {
	const outPath = await invoke<string>('get_temp_path', { ext: 'm4a', inDocuments })
	await invoke<string>('download_audio', { url, outPath })
	return outPath
}
