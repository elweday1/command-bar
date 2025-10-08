<script lang="ts">
	let { name, class: className = 'size-4' }: { name: string; class?: string } = $props();

	// Check if the icon is an emoji (contains non-ASCII characters)
	const isEmoji = /[^\x00-\x7F]/.test(name);
	console.log(name);
</script>

{#if isEmoji}
	<span class={className}>{name}</span>
{:else if name.startsWith('data:image/')}
	<img src={name} alt="icon" class={className} />
{:else}
	<svg
		class={className}
		xmlns="http://www.w3.org/2000/svg"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
	>
		{#if name === 'search'}
			<circle cx="11" cy="11" r="8"></circle>
			<path d="m21 21-4.3-4.3"></path>
		{:else if name === 'command'}
			<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"></path>
		{:else if name === 'package'}
			<path d="m7.5 4.27 9 5.15"></path>
			<path
				d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"
			></path>
			<path d="m3.3 7 8.7 5 8.7-5"></path>
			<path d="M12 22V12"></path>
		{:else if name === 'activity'}
			<path
				d="M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48 0L9.24 2.18a.25.25 0 0 0-.48 0l-2.35 8.36A2 2 0 0 1 4.49 12H2"
			></path>
		{:else if name === 'file'}
			<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
			<path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
		{:else if name === 'folder'}
			<path
				d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"
			></path>
		{:else if name === 'code'}
			<polyline points="16 18 22 12 16 6"></polyline>
			<polyline points="8 6 2 12 8 18"></polyline>
		{:else if name === 'file-text'}
			<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"></path>
			<path d="M14 2v4a2 2 0 0 0 2 2h4"></path>
			<path d="M10 9H8"></path>
			<path d="M16 13H8"></path>
			<path d="M16 17H8"></path>
		{:else if name === 'image'}
			<rect width="18" height="18" x="3" y="3" rx="2" ry="2"></rect>
			<circle cx="9" cy="9" r="2"></circle>
			<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"></path>
		{:else if name === 'youtube'}
			<path
				d="M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17"
			></path>
			<path d="m10 15 5-3-5-3z"></path>
		{:else if name === 'download'}
			<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
			<polyline points="7 10 12 15 17 10"></polyline>
			<line x1="12" x2="12" y1="15" y2="3"></line>
		{:else if name === 'trash-2'}
			<path d="M3 6h18"></path>
			<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
			<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
			<line x1="10" x2="10" y1="11" y2="17"></line>
			<line x1="14" x2="14" y1="11" y2="17"></line>
		{:else if name === 'refresh-cw'}
			<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
			<path d="M21 3v5h-5"></path>
			<path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
			<path d="M8 16H3v5"></path>
		{:else if name === 'x'}
			<path d="M18 6 6 18"></path>
			<path d="m6 6 12 12"></path>
		{:else if name === 'external-link'}
			<path d="M15 3h6v6"></path>
			<path d="M10 14 21 3"></path>
			<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
		{:else if name === 'share-2'}
			<circle cx="18" cy="5" r="3"></circle>
			<circle cx="6" cy="12" r="3"></circle>
			<circle cx="18" cy="19" r="3"></circle>
			<line x1="8.59" x2="15.42" y1="13.51" y2="17.49"></line>
			<line x1="15.41" x2="8.59" y1="6.51" y2="10.49"></line>
		{:else if name === 'settings'}
			<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path>
			<circle cx="12" cy="12" r="3"></circle>
		{/if}
	</svg>
{/if}
