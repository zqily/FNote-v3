<script lang="ts">
	import { onMount } from 'svelte';
	import { store } from '$lib/store';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import PlayerControls from '$lib/components/PlayerControls.svelte';
	import '../app.css';

	// When the component first mounts, fetch the initial state from Rust.
	onMount(async () => {
		await store.initializeState();
	});
</script>

<div class="h-screen w-screen bg-black text-neutral-200 flex overflow-hidden font-sans">
	<Sidebar />

	<!-- Main content area wrapper -->
	<main class="flex-1 flex flex-col relative min-w-0">
		<!-- 
      This is the content area. SvelteKit will render the page 
      (e.g., +page.svelte) inside the <slot /> component.
    -->
		<div class="flex-1 min-h-0">
			<slot />
		</div>

		<!-- Player controls are part of the main layout, below the page content -->
		<PlayerControls />
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