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
	<Card class="mx-auto max-w-md">
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
			<p class="text-sm text-muted-foreground text-center">
				Settings are saved automatically
			</p>
		</CardContent>
	</Card>
</div>
