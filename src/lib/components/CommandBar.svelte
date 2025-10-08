<script lang="ts">
	import { GlobalState } from '$lib/commands.svelte';
	import { cn } from '$lib/utils.js';
	import Icon from './Icon.svelte';
	import { settingsStore, type Settings } from '$lib/stores/settings.svelte';
	import { onMount } from 'svelte';
	import '../../app.css';
	import { listen } from '@tauri-apps/api/event';

	const api = new GlobalState();

	onMount(() => {
		listen('settings-changed', (event) => {
			settingsStore.settings = event.payload as Settings;
			settingsStore.load();
		});
		settingsStore.load();
	});
</script>

<div
	class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]"
	onclick={() => api.handleBackdropClick()}
>
	<div
		class="animate-in fade-in slide-in-from-top-4 w-full max-w-2xl duration-200"
		onclick={(e) => e.stopPropagation()}
	>
		<div
			class="bg-foreground flex max-h-[70vh] flex-col overflow-hidden rounded-xl border border-white/10 shadow-2xl backdrop-blur-xl"
			style="opacity: {settingsStore.loaded ? settingsStore.opacity : 0.8}"
		>
			<div class="flex items-center gap-3 border-b border-white/10 px-4 py-3">
				{#if api.activePlugin}
					<div class="text-primary">
						<Icon name={api.activePlugin.icon} />
					</div>
				{:else}
					<Icon name="search" class="text-muted-foreground size-5" />
				{/if}
				<input
					type="text"
					bind:value={api.query}
					onkeydown={(e) => api.handleKeyDown(e)}
					placeholder={api.activePlugin
						? `Search ${api.activePlugin.name.toLowerCase()}...`
						: 'Type a command or search...'}
					class="flex-1 bg-transparent text-base text-white outline-none placeholder:text-white/50"
					bind:this={api.inputElement}
				/>
				{#if api.isLoading}
					<div
						class="border-primary size-4 animate-spin rounded-full border-2 border-t-transparent"
					></div>
				{/if}
			</div>
			<ul
				class="scrollbar-thin scrollbar-track-transparent scrollbar-thumb-white/20 hover:scrollbar-thumb-white/30 flex-1 overflow-y-auto"
				bind:this={api.resultsElement}
			>
				{#if api.results.length === 0 && api.query.trim() && !api.isLoading}
					<div class="px-4 py-8 text-center text-sm text-white/50">No results found</div>
				{/if}

				{#if api.results.length === 0 && !api.query.trim()}
					<div class="space-y-3 px-4 py-4">
						<div class="text-xs font-medium text-white/50">Available Plugins</div>
						{#each api.plugins as plugin}
							<button
								onclick={() => api.selectPlugin(plugin)}
								class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left hover:bg-white/10"
							>
								<Icon class="w-7" name={plugin.icon} />
								<div class="flex-1">
									<div class="text-sm font-medium text-white">{plugin.name}</div>
									<div class="text-xs text-white/50">{plugin.description}</div>
								</div>
								<kbd class="rounded bg-white/10 px-2 py-1 font-mono text-xs text-white/50">
									{plugin.prefix}
								</kbd>
							</button>
						{/each}
					</div>
				{/if}

				{#each api.results as result, index}
					<li
						bind:this={api.resultElements[index]}
						onmouseenter={() => api.handleMouseEnter(index)}
						onclick={() => api.executeSelectedAction()}
						class={cn(
							'cursor-pointer border-b border-white/10 px-4 py-3 last:border-0',
							index === api.selectedIndex && 'bg-white/10'
						)}
					>
						<div class="flex items-start gap-3">
							<div class="flex h-6 w-6 flex-shrink-0 items-center justify-center">
								{#if result.icon}
									<Icon class="h-5 w-5" name={result.icon} />
								{/if}
							</div>
							<div class="min-w-0 flex-1 overflow-hidden">
								<div class="truncate text-sm font-medium text-white">{result.title}</div>
								{#if result.subtitle}
									<div class="truncate text-xs text-white/50">{result.subtitle}</div>
								{/if}
							</div>
							{#if index === api.selectedIndex && result.actions && result.actions.length > 0}
								<div class="flex flex-shrink-0 items-center gap-1">
									{#each result.actions as action}
										<button
											onclick={(e) => {
												e.stopPropagation();
												api.executeAction(result, action);
											}}
											class="rounded-md bg-white/10 px-2 py-1 text-xs font-medium text-white hover:bg-white/20"
										>
											{action.label}
										</button>
									{/each}
								</div>
							{/if}
						</div>
					</li>
				{/each}
			</ul>

			<div
				class="flex flex-shrink-0 items-center justify-between border-t border-white/10 bg-white/5 px-4 py-2 text-xs text-white/50"
			>
				{#if api.selectedResult && api.selectedResult.actions && api.selectedResult.actions.length > 0}
					<div class="flex items-center gap-2">
						{#each api.selectedResult.actions as action}
							<button
								onclick={() => api.executeAction(api.selectedResult, action)}
								class="flex items-center justify-center gap-1.5 rounded-md bg-white/10 px-2.5 py-1 text-xs font-medium text-white hover:bg-white/20"
							>
								<span>{action.label}</span>
								{#if action.shortcut}
									<kbd class="ml-1 rounded bg-white/10 px-1.5 py-0.5 font-mono text-[10px]">
										{action.shortcut}
									</kbd>
								{/if}
							</button>
						{/each}
					</div>
				{:else}
					<div class="flex items-center gap-4">
						<span class="flex items-center gap-1">
							<kbd class="rounded bg-white/10 px-1.5 py-0.5 font-mono">↑↓</kbd>
							Navigate
						</span>
						<span class="flex items-center gap-1">
							<kbd class="rounded bg-white/10 px-1.5 py-0.5 font-mono">↵</kbd>
							Select
						</span>
						<span class="flex items-center gap-1">
							<kbd class="rounded bg-white/10 px-1.5 py-0.5 font-mono">Esc</kbd>
							Close
						</span>
					</div>
				{/if}
				<span>{api.results.length} results</span>
			</div>
		</div>
	</div>
</div>
