import { resolveResource } from '@tauri-apps/api/path'
import * as fs from '@tauri-apps/plugin-fs'
import i18n, { LanguageDetectorAsyncModule } from 'i18next'
import { initReactI18next } from 'react-i18next/initReactI18next'
import { isTauri } from '@tauri-apps/api/core'

// See src-tauri/locales/ for the list of supported languages
// Please keep the list sorted alphabetically
export const supportedLanguages: { [key: string]: string } = {
	'en-US': 'english', // English
	'es-ES': 'spanish (ES)', // Spanish (ES)
	'es-MX': 'spanish (MX)', // Spanish (MX)
	'fr-FR': 'french', // French
	'he-IL': 'hebrew', // Hebrew
	'hi-IN': 'hindi', // Hindi
	'it-IT': 'italian', // Italian
	'ja-JP': 'japanese', // Japanese
	'ko-KR': 'korean', // Korean
	'no-NO': 'norwegian', // Norwegian
	'pl-PL': 'polish', // Polish
	'pt-BR': 'portuguese', // Portuguese (BR)
	'ru-RU': 'russian', // Russian
	'sv-SE': 'swedish', // Swedish
	'ta-IN': 'tamil', // Tamil
	'vi-VN': 'vietnamese', // Vietnamese
	'zh-CN': 'chinese', // Chinese (Simplified)
	'zh-HK': 'chinese (HK)', // Chinese (Traditional)
}
export const supportedLanguageKeys = Object.keys(supportedLanguages)
export const supportedLanguageValues = Object.values(supportedLanguages)

export function getI18nLanguageName() {
	const name = supportedLanguages[i18n.language as keyof typeof supportedLanguages]
	return name
}

const LanguageDetector: LanguageDetectorAsyncModule = {
	type: 'languageDetector',
	async: true, // If this is set to true, your detect function receives a callback function that you should call with your language, useful to retrieve your language stored in AsyncStorage for example
	detect: (callback) => {
		// Only try to detect system locale in Tauri context
		if (isTauri()) {
			// Dynamically import Tauri API to avoid loading in browser mode
			import('@tauri-apps/plugin-os')
				.then((osModule) => osModule.locale())
				.then((detectedLocale) => {
					const prefs_language = localStorage.getItem('prefs_display_language')
					if (prefs_language) {
						const locale = JSON.parse(prefs_language)
						callback(locale)
					} else {
						if (detectedLocale) {
							callback(detectedLocale)
						}
					}
				})
				.catch(() => {
					// Fallback if locale detection fails
					const prefs_language = localStorage.getItem('prefs_display_language')
					if (prefs_language) {
						const locale = JSON.parse(prefs_language)
						callback(locale)
					} else {
						callback('en-US')
					}
				})
		} else {
			// In browser mode, use stored preference or default
			const prefs_language = localStorage.getItem('prefs_display_language')
			if (prefs_language) {
				const locale = JSON.parse(prefs_language)
				callback(locale)
			} else {
				callback('en-US')
			}
		}
	},
}

// Note: isTauri function is imported from @tauri-apps/api/core above

// Custom backend to load translations
const loadResources = async (language: string) => {
	if (!supportedLanguageKeys.includes(language)) {
		return null
	}

	try {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const translations: any = {}

		// In Tauri dev mode (with vite dev server), resources are served via HTTP
		// In Tauri production, use Tauri's resolveResource
		// In browser mode, always use HTTP
		const isDev = !import.meta.env.PROD
		const shouldUseTauriResource = isTauri() && !isDev

		if (shouldUseTauriResource) {
			// Production mode: Load from Tauri bundled resources
			const resourcePath = `./locales/${language}`
			const languageDirectory = await resolveResource(resourcePath)
			const files = await fs.readDir(languageDirectory)

			await Promise.all(
				files.map(async (file) => {
					const filePath = `${languageDirectory}/${file.name}`
					const namespace = file.name.replace('.json', '')
					const content = await fs.readTextFile(filePath)
					translations[namespace] = JSON.parse(content)
				})
			)
		} else {
			// Development mode or browser mode: Load from HTTP
			const namespaces = ['common', 'language']
			await Promise.all(
				namespaces.map(async (namespace) => {
					const response = await fetch(`/locales/${language}/${namespace}.json`)
					if (response.ok) {
						translations[namespace] = await response.json()
					} else {
						console.error(`Failed to fetch ${namespace} for ${language}`)
					}
				})
			)
		}

		return translations
	} catch (error) {
		console.error(`Failed to load translations for ${language}:`, error)
		return null
	}
}

const i18nInitPromise = (async () => {
	// Detect language
	const prefs_language = localStorage.getItem('prefs_display_language')
	let detectedLanguage = 'en-US'

	if (prefs_language) {
		detectedLanguage = JSON.parse(prefs_language)
	} else if (isTauri()) {
		// Use Tauri API in production
		try {
			const { locale } = await import('@tauri-apps/plugin-os')
			const osLocale = await locale()
			if (osLocale && supportedLanguageKeys.includes(osLocale)) {
				detectedLanguage = osLocale
			}
		} catch (error) {
			// Fallback if locale detection fails
			const browserLocale = navigator.language
			if (browserLocale && supportedLanguageKeys.includes(browserLocale)) {
				detectedLanguage = browserLocale
			}
		}
	} else {
		// Use browser API in development
		const browserLocale = navigator.language
		if (browserLocale && supportedLanguageKeys.includes(browserLocale)) {
			detectedLanguage = browserLocale
		}
	}

	console.log('Detected language:', detectedLanguage)
	console.log('Running in:', isTauri() ? 'Tauri mode' : 'Browser mode')

	// Load translations for detected language
	const resources = await loadResources(detectedLanguage)

	if (!resources) {
		console.warn(`Failed to load ${detectedLanguage}, falling back to en-US`)
		const fallbackResources = await loadResources('en-US')
		if (!fallbackResources) {
			throw new Error('Failed to load fallback translations')
		}

		return i18n.use(initReactI18next).init({
			lng: 'en-US',
			fallbackLng: 'en-US',
			debug: true,
			resources: {
				'en-US': fallbackResources
			},
			interpolation: {
				escapeValue: false,
			},
		})
	}

	return i18n.use(initReactI18next).init({
		lng: detectedLanguage,
		fallbackLng: 'en-US',
		debug: true,
		resources: {
			[detectedLanguage]: resources
		},
		interpolation: {
			escapeValue: false,
		},
	})
})()

// Helper to dynamically load additional languages
export async function changeLanguage(language: string) {
	if (!supportedLanguageKeys.includes(language)) {
		console.error(`Language ${language} is not supported`)
		return
	}

	// Check if already loaded
	if (i18n.hasResourceBundle(language, 'common')) {
		await i18n.changeLanguage(language)
		return
	}

	// Load new language
	const resources = await loadResources(language)
	if (resources) {
		Object.keys(resources).forEach(namespace => {
			i18n.addResourceBundle(language, namespace, resources[namespace])
		})
		await i18n.changeLanguage(language)
	}
}

export { i18nInitPromise }
export default i18n
