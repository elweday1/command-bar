import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface Settings {
	transparency: number;
}

class SettingsStore {
	settings = $state<Settings>({ transparency: 0.8 });
	loaded = $state(false);
	private saveTimeout: NodeJS.Timeout | null = null;
	private initialized = false;

	init() {
		if (this.initialized) return;
		this.initialized = true;

		// Listen for settings changes from other windows
		listen('settings-changed', (event) => {
			this.settings = event.payload as Settings;
			this.load()
		});

		// Auto-save when settings change (debounced)
		$effect(() => {
			if (this.loaded) {
				if (this.saveTimeout) {
					clearTimeout(this.saveTimeout);
				}
				this.saveTimeout = setTimeout(() => {
					this.save();
				}, 500);
			}
		});
	}

	async load() {
		try {
			this.settings = await invoke('get_settings');
			this.loaded = true;
		} catch (error) {
			console.error('Failed to load settings:', error);
			this.settings = { transparency: 0.8 };
			this.loaded = true;
		}
	}

	async save() {
		try {
			await invoke('set_settings', { settings: this.settings });
		} catch (error) {
			console.error('Failed to save settings:', error);
			throw error;
		}
	}

	get opacity() {
		return this.settings.transparency;
	}
}

export const settingsStore = new SettingsStore();