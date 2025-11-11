<script lang="ts">
	import PlayerControls from './PlayerControls.svelte';
	import { store } from '$lib/store';

	$: song = $store.currentSong;
</script>

<main class="flex-1 flex flex-col relative min-w-0">
	<!-- Central Content View (Album Art Area) -->
	<div
		class="flex-1 bg-gradient-to-b from-zinc-800 to-zinc-950/50 m-2 mb-0 rounded-t-lg flex flex-col items-center justify-center p-8"
	>
		<div class="w-full max-w-sm aspect-square bg-zinc-900 rounded-lg shadow-2xl mb-8">
			{#if song?.albumArtUrl}
				<img
					src={song.albumArtUrl}
					alt="Album art for {song.title}"
					class="w-full h-full object-cover rounded-lg"
				/>
			{:else}
				<div
					class="w-full h-full bg-zinc-800 rounded-lg flex items-center justify-center text-zinc-500"
				>
					<span>No Album Art</span>
				</div>
			{/if}
		</div>
		<div class="text-center">
			<h1 class="text-3xl font-bold text-white truncate max-w-lg">
				{song?.title ?? 'No song selected'}
			</h1>
			<p class="text-lg text-zinc-400 mt-1 truncate max-w-md">{song?.artist ?? '...'}</p>
		</div>
	</div>

	<!-- Bottom Player Control Bar -->
	<PlayerControls />
</main>