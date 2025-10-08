import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { loadPlugins, executePluginAction, type Plugin, type PluginResult } from '$lib/plugins'
const appWindow = getCurrentWindow();

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
						// Search across all plugins
						const allResults = await Promise.all(this.plugins.map((plugin) => plugin.search(this.query)));
						const flatResults = allResults.flat();
						
						// Add Google search as fallback if no results
						if (flatResults.length === 0) {
							const googlePlugin = this.plugins.find(p => p.id === 'google');
							if (googlePlugin) {
								const googleResults = await googlePlugin.search(this.query);
								this.results = googleResults;
							} else {
								this.results = [];
							}
						} else {
							this.results = flatResults;
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
			this.hideWindow();
		}
	}

	// Scroll to selected item
	scrollToSelected() {
		const selectedElement = this.resultElements[this.selectedIndex];
		if (selectedElement && this.resultsElement) {
			selectedElement.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		}
	}

	// Hide window
	async hideWindow() {
		try {
			await appWindow.hide();
			this.query = '';
			this.selectedIndex = 0;
			this.activePlugin = null;
		} catch (error) {
			console.error('Failed to hide window:', error);
		}
	}

	// Execute selected action
	async executeSelectedAction() {
		const selected = this.results[this.selectedIndex];
		if (selected && (selected.actions?.length ?? 0) > 0) {
			const primaryAction = selected.actions?.[0];
			if (primaryAction) {
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

	// Global keyboard shortcut
	handleGlobalKeyDown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'r') {
			e.preventDefault();
			this.toggleWindow();
		}
	}

	// Toggle window visibility
	async toggleWindow() {
		try {
			await invoke('toggle_window');
			this.query = '';
			this.selectedIndex = 0;
		} catch (error) {
			console.error('Failed to toggle window:', error);
		}
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
