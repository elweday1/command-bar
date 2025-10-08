<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { settingsStore } from '$lib/stores/settings.svelte';

	onMount(() => {
		settingsStore.init();
		settingsStore.load();
	});
</script>

<div class="bg-foreground min-h-screen p-6">
	<Card class="mx-auto max-w-lg">
		<CardHeader>
			<CardTitle>Settings</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<label for="transparency" class="text-sm font-medium">
					Transparency: {Math.round(settingsStore.settings.transparency * 100)}%
				</label>
				<input
					id="transparency"
					type="range"
					min="0.1"
					max="1"
					step="0.1"
					bind:value={settingsStore.settings.transparency}
					on:input={() => settingsStore.save()}
					class="w-full"
				/>
			</div>

			<div class="space-y-3">
				<h3 class="text-sm font-medium">Plugins</h3>
				{#each settingsStore.allPlugins as plugin}
					<div class="flex items-center space-x-2">
						<input
							id="plugin-{plugin.id}"
							type="checkbox"
							checked={settingsStore.isPluginEnabled(plugin.id)}
							on:change={(e) => settingsStore.togglePlugin(plugin.id, e.currentTarget.checked)}
							class="rounded"
						/>
						<label for="plugin-{plugin.id}" class="text-sm flex items-center space-x-2">
							<span>{plugin.icon}</span>
							<span>{plugin.name}</span>
						</label>
					</div>
				{/each}
			</div>

			<div class="space-y-3">
				<h3 class="text-sm font-medium">Shortcuts</h3>
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<label class="text-sm">Toggle Window:</label>
						<input
							type="text"
							bind:value={settingsStore.settings.shortcuts.toggleWindow}
							on:blur={() => settingsStore.save()}
							class="px-2 py-1 text-xs border rounded w-24 text-center"
							placeholder="Ctrl+R"
						/>
					</div>
					<div class="flex items-center justify-between">
						<label class="text-sm">Hide Window:</label>
						<input
							type="text"
							bind:value={settingsStore.settings.shortcuts.hideWindow}
							on:blur={() => settingsStore.save()}
							class="px-2 py-1 text-xs border rounded w-24 text-center"
							placeholder="Escape"
						/>
					</div>
				</div>
			</div>
			<p class="text-sm text-muted-foreground text-center">
				Settings are saved automatically
			</p>
		</CardContent>
	</Card>
</div>
