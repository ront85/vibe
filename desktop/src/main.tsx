import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App'
import { i18nInitPromise } from './lib/i18n'

// Wait for i18n to initialize before rendering the app
i18nInitPromise
	.then(() => {
		console.log('i18n initialized successfully')
		ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
			<BrowserRouter>
				<App />
			</BrowserRouter>
		)
	})
	.catch((error) => {
		console.error('Failed to initialize i18n:', error)
		// Render app anyway as a fallback
		ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
			<BrowserRouter>
				<App />
			</BrowserRouter>
		)
	})
