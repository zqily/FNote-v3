<script lang="ts">
	import { onMount } from 'svelte';
	import { store } from '$lib/store';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import '../app.css';

	// When the component first mounts, fetch the initial state from Rust.
	onMount(async () => {
		await store.initializeState();
	});
</script>

<div class="h-screen w-screen bg-zinc-950 text-neutral-200 flex overflow-hidden font-sans">
	<Sidebar />

	<!-- Main content area wrapper -->
	<main class="flex-1 flex flex-col relative min-w-0">
		<!-- SvelteKit will render the page (e.g., +page.svelte) inside the <slot /> -->
		<slot />
	</main>
</div>

<style>
	:global(body) {
		font-family:
			'Inter',
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			Oxygen,
			Ubuntu,
			Cantarell,
			'Open Sans',
			'Helvetica Neue',
			sans-serif;
	}
</style>