<script lang="ts">
	import { Volume2, Volume1, VolumeX } from 'lucide-svelte';
	import { playerStore } from '$lib/stores/playerStore';
	import { invoke } from '@tauri-apps/api/tauri';

	let volume = 1;
	playerStore.subscribe((state) => {
		volume = state.volume;
	});

	function handleVolumeChange(e: Event) {
		invoke('set_volume', { volume: (e.currentTarget as HTMLInputElement).valueAsNumber });
	}
</script>

<div class="flex items-center space-x-2">
	{#if volume === 0}
		<VolumeX class="text-gray-400" />
	{:else if volume < 0.5}
		<Volume1 class="text-gray-400" />
	{:else}
		<Volume2 class="text-gray-400" />
	{/if}
	<input
		type="range"
		min="0"
		max="1"
		step="0.01"
		value={volume}
		on:input={(e) => (volume = (e.currentTarget as HTMLInputElement).valueAsNumber)}
		on:change={handleVolumeChange}
		class="w-24 h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-white"
	/>
</div>
