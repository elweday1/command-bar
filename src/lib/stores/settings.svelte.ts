import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Plugin } from '$lib/plugins';

export interface Settings {
	transparency: number;
	enabledPlugins: Record<string, boolean>;
	shortcuts: {
		toggleWindow: string;
		hideWindow: string;
	};
}

class SettingsStore {
	settings = $state<Settings>({ 
		transparency: 0.8, 
		enabledPlugins: {},
		shortcuts: {
			toggleWindow: 'Ctrl+R',
			hideWindow: 'Escape'
		}
	});
	loaded = $state(false);
	allPlugins = $state<Plugin[]>([]);
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
					this.updateShortcuts();
				}, 500);
			}
		});
	}

	async load() {
		try {
			const settings = await invoke('get_settings') as any;
			this.settings = {
				transparency: settings.transparency || 0.8,
				enabledPlugins: settings.enabledPlugins || {},
				shortcuts: settings.shortcuts || {
					toggleWindow: 'Ctrl+R',
					hideWindow: 'Escape'
				}
			};
			this.allPlugins = await invoke('list_plugins');
			this.loaded = true;
		} catch (error) {
			console.error('Failed to load settings:', error);
			this.settings = { 
				transparency: 0.8, 
				enabledPlugins: {},
				shortcuts: {
					toggleWindow: 'Ctrl+R',
					hideWindow: 'Escape'
				}
			};
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

	isPluginEnabled(pluginId: string): boolean {
		return this.settings.enabledPlugins[pluginId] !== false;
	}

	togglePlugin(pluginId: string, enabled: boolean) {
		this.settings.enabledPlugins[pluginId] = enabled;
		this.save();
	}

	async updateShortcuts() {
		try {
			await invoke('update_shortcuts');
		} catch (error) {
			console.error('Failed to update shortcuts:', error);
		}
	}
}

export const settingsStore = new SettingsStore();