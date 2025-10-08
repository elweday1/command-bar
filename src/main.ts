import { mount } from 'svelte';
import './app.css';
import CommandBar from '$lib/components/CommandBar.svelte';
import Settings from '$lib/components/Settings.svelte';

const isSettingsPage = window.location.pathname === '/settings';

const app = mount(isSettingsPage ? Settings : CommandBar, {
	target: document.getElementById('app')!
});

export default app;
