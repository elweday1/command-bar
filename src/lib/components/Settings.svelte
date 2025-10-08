<script lang="ts">
	import { onMount } from 'svelte';
	import { settingsStore } from '$lib/stores/settings.svelte';
	import Icon from './Icon.svelte';
	import * as Tabs from '$lib/components/ui/tabs/index.js';

	onMount(() => {
		settingsStore.init();
		settingsStore.load();
	});
</script>

<Tabs.Root value="appearance" class="bg-foreground min-h-screen text-white">
	<div class="flex h-screen">
		<!-- Sidebar -->
		<div class="w-48 bg-black/20 border-r border-white/10 flex flex-col">
			<div class="p-4 border-b border-white/10">
				<h1 class="text-lg font-semibold">Settings</h1>
				<p class="text-xs text-white/60 mt-1">Customize your command bar</p>
			</div>
			<nav class="flex-1 p-3">
				<Tabs.List class="flex flex-col h-auto w-full bg-transparent p-0 space-y-1">
					<Tabs.Trigger
						value="appearance"
						class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-white/70 transition-colors hover:bg-white/5 hover:text-white data-[state=active]:bg-white/10 data-[state=active]:text-white"
					>
						<Icon name="palette" class="h-4 w-4" />
						Appearance
					</Tabs.Trigger>
					<Tabs.Trigger
						value="shortcuts"
						class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-white/70 transition-colors hover:bg-white/5 hover:text-white data-[state=active]:bg-white/10 data-[state=active]:text-white"
					>
						<Icon name="keyboard" class="h-4 w-4" />
						Shortcuts
					</Tabs.Trigger>
					<Tabs.Trigger
						value="plugins"
						class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-white/70 transition-colors hover:bg-white/5 hover:text-white data-[state=active]:bg-white/10 data-[state=active]:text-white"
					>
						<Icon name="puzzle" class="h-4 w-4" />
						Plugins
					</Tabs.Trigger>
					<Tabs.Trigger
						value="about"
						class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-left text-sm text-white/70 transition-colors hover:bg-white/5 hover:text-white data-[state=active]:bg-white/10 data-[state=active]:text-white"
					>
						<Icon name="info" class="h-4 w-4" />
						About
					</Tabs.Trigger>
				</Tabs.List>
			</nav>
		</div>

		<!-- Main Content -->
		<div class="flex-1 p-6 overflow-y-auto">
			<Tabs.Content value="appearance">
				<div class="max-w-xl">
					<div class="mb-4">
						<h2 class="flex items-center gap-2 text-lg font-semibold text-white">
							<Icon name="palette" class="h-5 w-5" />
							Appearance
						</h2>
						<p class="mt-1 text-sm text-white/60">Customize the look and feel</p>
					</div>
					<div class="rounded-lg border border-white/10 bg-white/5 p-4">
						<div class="space-y-4">
							<div>
								<label class="mb-2 block text-sm font-medium text-white">
									Transparency: {Math.round(settingsStore.settings.transparency * 100)}%
								</label>
								<input
									type="range"
									min="0.1"
									max="1"
									step="0.1"
									bind:value={settingsStore.settings.transparency}
									oninput={() => settingsStore.save()}
									class="slider h-2 w-full cursor-pointer appearance-none rounded-lg bg-white/20"
								/>
								<p class="mt-1 text-xs text-white/50">Adjust window transparency</p>
							</div>
						</div>
					</div>
				</div>
			</Tabs.Content>
			<Tabs.Content value="shortcuts">
				<div class="max-w-xl">
					<div class="mb-4">
						<h2 class="flex items-center gap-2 text-lg font-semibold text-white">
							<Icon name="keyboard" class="h-5 w-5" />
							Shortcuts
						</h2>
						<p class="mt-1 text-sm text-white/60">Configure keyboard shortcuts</p>
					</div>
					<div class="rounded-lg border border-white/10 bg-white/5 p-4">
						<div class="space-y-3">
							<div class="flex items-center justify-between py-2">
								<div>
									<div class="text-sm font-medium text-white">Toggle Window</div>
									<div class="text-xs text-white/50">Show or hide the command bar</div>
								</div>
								<input
									type="text"
									bind:value={settingsStore.settings.shortcuts.toggleWindow}
									onblur={() => settingsStore.save()}
									class="w-24 rounded border border-white/20 bg-white/10 px-2 py-1 text-center font-mono text-xs text-white focus:ring-1 focus:ring-white/30 focus:outline-none"
									placeholder="Ctrl+R"
								/>
							</div>
							<div class="flex items-center justify-between border-t border-white/10 py-2">
								<div>
									<div class="text-sm font-medium text-white">Hide Window</div>
									<div class="text-xs text-white/50">Close the command bar</div>
								</div>
								<input
									type="text"
									bind:value={settingsStore.settings.shortcuts.hideWindow}
									onblur={() => settingsStore.save()}
									class="w-24 rounded border border-white/20 bg-white/10 px-2 py-1 text-center font-mono text-xs text-white focus:ring-1 focus:ring-white/30 focus:outline-none"
									placeholder="Escape"
								/>
							</div>
							<div class="flex items-center justify-between border-t border-white/10 py-2">
								<div>
									<div class="text-sm font-medium text-white">Open Settings</div>
									<div class="text-xs text-white/50">Open this settings window</div>
								</div>
								<input
									type="text"
									bind:value={settingsStore.settings.shortcuts.openSettings}
									onblur={() => settingsStore.save()}
									class="w-24 rounded border border-white/20 bg-white/10 px-2 py-1 text-center font-mono text-xs text-white focus:ring-1 focus:ring-white/30 focus:outline-none"
									placeholder="Ctrl+Comma"
								/>
							</div>
						</div>
					</div>
				</div>
			</Tabs.Content>
			<Tabs.Content value="plugins">
				<div class="max-w-xl">
					<div class="mb-4">
						<h2 class="flex items-center gap-2 text-lg font-semibold text-white">
							<Icon name="puzzle" class="h-5 w-5" />
							Plugins
						</h2>
						<p class="mt-1 text-sm text-white/60">Enable or disable plugins</p>
					</div>
					<div class="rounded-lg border border-white/10 bg-white/5 p-4">
						<div class="space-y-3">
							{#each settingsStore.allPlugins as plugin}
								<div
									class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 p-3 transition-colors hover:bg-white/10"
								>
									<div class="flex items-center gap-3">
										<div class="text-lg">{plugin.icon}</div>
										<div>
											<div class="text-sm font-medium text-white">{plugin.name}</div>
											<div class="text-xs text-white/50">{plugin.description}</div>
										</div>
									</div>
									<input
										type="checkbox"
										checked={settingsStore.isPluginEnabled(plugin.id)}
										onchange={(e) => settingsStore.togglePlugin(plugin.id, e.currentTarget.checked)}
										class="h-4 w-4 rounded"
									/>
								</div>
							{/each}
						</div>
					</div>
				</div>
			</Tabs.Content>
			<Tabs.Content value="about">
				<div class="max-w-xl">
					<div class="mb-4">
						<h2 class="flex items-center gap-2 text-lg font-semibold text-white">
							<Icon name="info" class="h-5 w-5" />
							About
						</h2>
						<p class="mt-1 text-sm text-white/60">Information about Command Bar</p>
					</div>
					<div class="rounded-lg border border-white/10 bg-white/5 p-6 text-center">
						<div class="space-y-3">
							<h3 class="text-xl font-bold text-white">Command Bar</h3>
							<p class="text-white/70">Version 2.0.0</p>
							<p class="mx-auto max-w-sm text-sm text-white/60">
								A powerful command palette built with Tauri 2 and Svelte 5
							</p>
							<div class="border-t border-white/10 pt-3">
								<p class="text-xs text-white/40">Settings are saved automatically</p>
							</div>
						</div>
					</div>
				</div>
			</Tabs.Content>
		</div>
	</div>
</Tabs.Root>

<style>
	.slider::-webkit-slider-thumb {
		appearance: none;
		height: 16px;
		width: 16px;
		border-radius: 50%;
		background: white;
		cursor: pointer;
		border: 2px solid rgba(255, 255, 255, 0.3);
	}

	.slider::-moz-range-thumb {
		height: 16px;
		width: 16px;
		border-radius: 50%;
		background: white;
		cursor: pointer;
		border: 2px solid rgba(255, 255, 255, 0.3);
	}
</style>
