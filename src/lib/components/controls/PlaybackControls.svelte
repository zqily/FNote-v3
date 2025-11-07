<script lang="ts">
	import { Shuffle, SkipBack, Play, Pause, SkipForward } from 'lucide-svelte';
	import { playerStore } from '$lib/stores/playerStore';
	import { invoke } from '@tauri-apps/api/tauri';

	$: isPlaying = $playerStore.is_playing;
	$: isShuffled = $playerStore.is_shuffled;
</script>

<div class="flex items-center justify-center space-x-6">
	<button
		on:click={() => invoke('toggle_shuffle')}
		class="text-gray-400 hover:text-white"
		class:text-green-500={isShuffled}
		title="Shuffle"
	>
		<Shuffle size={20} />
	</button>
	<button on:click={() => invoke('prev_song')} class="text-gray-400 hover:text-white" title="Previous">
		<SkipBack size={24} />
	</button>

	<button
		on:click={() => invoke('toggle_playback')}
		class="bg-white text-black rounded-full p-3 hover:scale-105 transition-transform"
		title={isPlaying ? 'Pause' : 'Play'}
	>
		{#if isPlaying}
			<Pause size={28} class="fill-black" />
		{:else}
			<Play size={28} class="fill-black ml-1" />
		{/if}
	</button>

	<button on:click={() => invoke('next_song')} class="text-gray-400 hover:text-white" title="Next">
		<SkipForward size={24} />
	</button>
	<button class="text-gray-400 hover:text-white" title="Repeat (Not implemented)">
		<!-- Placeholder for Repeat icon -->
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			><path d="m17 2 4 4-4 4" /><path d="M3 11v-1a4 4 0 0 1 4-4h14" /><path d="m7 22-4-4 4-4" /><path
				d="M21 13v1a4 4 0 0 1-4 4H3"
			/></svg
		>
	</button>
</div>
