import { invoke } from '@tauri-apps/api/core';
import { loadPlugins, executePluginAction, type Plugin, type PluginResult } from '$lib/plugins'

import { listen, TauriEvent } from '@tauri-apps/api/event';

export const preventDefault = <T extends Event>(fn: (e: T) => void): ((e: T) => void) => {
	return (e: T) => {
		e.preventDefault();
		fn(e);
	};
};

export class GlobalState {
	query = $state('')
	results = $state<PluginResult[]>([])
	selectedIndex = $state(0)
	isLoading = $state(false)
	activePlugin = $state<Plugin | null>(null)
	plugins = $state<Plugin[]>([]);
	inputElement: HTMLInputElement | undefined = $state(undefined);
	resultsElement: HTMLUListElement | undefined = $state(undefined);
	resultElements: (HTMLLIElement | undefined)[] = $state([]);

	constructor() {
		loadPlugins().then(plugins => {
			this.plugins = plugins;
		});

		listen(TauriEvent.WINDOW_BLUR, async () => {
			await invoke('set_is_window_shown', { shown: false });
		}, {
			target: {
				kind: "WebviewWindow",
				label: "main"
			}
		});


		$effect(() => {
			if (this.detectedPlugin && this.detectedPlugin !== this.activePlugin) {
				this.activePlugin = this.detectedPlugin;
				this.detectedPlugin.onPrefixActivate?.();
			} else if (!this.detectedPlugin && this.activePlugin) {
				this.activePlugin = null;
			}
		});

		// Search across plugins
		$effect(() => {
			if (!this.query.trim()) {
				this.results = [];
				this.selectedIndex = 0;
				return;
			}

			(async () => {
				this.isLoading = true;

				try {
					if (this.activePlugin) {
						// Search only in active plugin
						const pluginResults = await this.activePlugin.search(this.searchQuery);
						this.results = pluginResults;
					} else {
						// Check for built-in commands first
						const builtInResults = this.searchBuiltInCommands(this.query);

						// Search across all plugins
						const allResults = await Promise.all(this.plugins.map((plugin) => plugin.search(this.query)));
						const flatResults = allResults.flat();

						// Combine built-in and plugin results
						const combinedResults = [...builtInResults, ...flatResults];

						// Add Google search as fallback if no results
						if (combinedResults.length === 0) {
							const googlePlugin = this.plugins.find(p => p.id === 'google');
							if (googlePlugin) {
								const googleResults = await googlePlugin.search(this.query);
								this.results = googleResults;
							} else {
								this.results = [];
							}
						} else {
							this.results = combinedResults;
						}
					}
					this.selectedIndex = 0;
				} catch (error) {
					console.error('[v0] Search error:', error);
					this.results = [];
				} finally {
					this.isLoading = false;
				}
			})()
		});

		// Focus input when available
		$effect(() => {
			if (this.inputElement) {
				setTimeout(() => this.inputElement?.focus(), 0);
			}
		});

	}

	// Detect prefix and activate plugin
	private detectedPlugin = $derived.by(() => {
		const words = this.query.trim().split(' ');
		if (words.length > 0) {
			const potentialPrefix = words[0].toLowerCase();
			return this.plugins.find((p) => p.prefix === potentialPrefix) || null;
		}
		return null;
	});

	// Extract search query without prefix
	private searchQuery = $derived.by(() => {
		if (this.detectedPlugin) {
			const words = this.query.trim().split(' ');
			return words.slice(1).join(' ');
		}
		return this.query;
	});

	// Get selected result
	selectedResult = $derived.by(() => {
		return this.results[this.selectedIndex] || null;
	});


	// Keyboard navigation
	handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			this.selectedIndex = Math.min(this.selectedIndex + 1, this.results.length - 1);
			this.scrollToSelected();
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
			this.scrollToSelected();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			this.executeSelectedAction();
		} else if (e.key === 'Escape') {
			e.preventDefault();
			this.handleBackdropClick();
		}
	}

	// Scroll to selected item
	scrollToSelected() {
		const selectedElement = this.resultElements[this.selectedIndex];
		if (selectedElement && this.resultsElement) {
			selectedElement.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		}
	}

	selectPlugin(plugin: Plugin) {
		this.query = `${plugin.prefix} `;
		setTimeout(() => {
			this.inputElement?.focus();
			if (this.inputElement) {
				this.inputElement.setSelectionRange(
					this.inputElement.value.length,
					this.inputElement.value.length
				);
			}
		}, 0);

	}



	// Search built-in commands
	searchBuiltInCommands(query: string) {
		const commands = [
			{
				id: 'settings',
				title: 'Settings',
				subtitle: 'Open Command Bar settings',
				icon: 'settings',
				actions: [{ id: 'open', label: 'Open' }]
			}
		];

		return commands.filter(cmd =>
			cmd.title.toLowerCase().includes(query.toLowerCase()) ||
			cmd.subtitle.toLowerCase().includes(query.toLowerCase())
		);
	}

	// Execute selected action
	async executeSelectedAction() {
		const selected = this.results[this.selectedIndex];
		if (selected && (selected.actions?.length ?? 0) > 0) {
			const primaryAction = selected.actions?.[0];
			if (primaryAction) {
				// Handle built-in commands
				if (selected.id === 'settings' && primaryAction.id === 'open') {
					try {
						await invoke('open_settings_window');
						await this.handleBackdropClick();
					} catch (error) {
						console.error('Failed to open settings:', error);
					}
					return;
				}

				// Determine plugin ID - use active plugin or find by result
				let pluginId = this.activePlugin?.id;
				if (!pluginId) {
					// Find plugin that generated this result
					for (const plugin of this.plugins) {
						const pluginResults = await plugin.search(this.query);
						if (pluginResults.some(r => r.id === selected.id)) {
							pluginId = plugin.id;
							break;
						}
					}
				}

				if (pluginId) {
					try {
						await executePluginAction(pluginId, selected.id, primaryAction.id);
					} catch (error) {
						console.error('Action execution failed:', error);
					}
				}
			}
		}
	}

	// Mouse hover handler
	handleMouseEnter(index: number) {
		this.selectedIndex = index;
	}



	// Handle backdrop click
	async handleBackdropClick() {
		await invoke('set_is_window_shown', { shown: false });
		this.query = '';
		this.selectedIndex = 0;
		this.activePlugin = null;
	}

	// Execute plugin action
	async executePluginAction(pluginId: string, resultId: string, actionId: string) {
		return await executePluginAction(pluginId, resultId, actionId);
	}

	// Execute specific action
	async executeAction(result: PluginResult, action: any) {
		const pluginId = this.activePlugin?.id;
		if (pluginId) {
			try {
				await executePluginAction(pluginId, result.id, action.id);
			} catch (error) {
				console.error('Action execution failed:', error);
			}
		}
	}

}
