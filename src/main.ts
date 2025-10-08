import { mount } from 'svelte';
import './app.css';
import CommandBar from '$lib/components/CommandBar.svelte';

const CommandBarApp = mount(CommandBar, {
	target: document.getElementById('app')!
});

export default CommandBarApp;
