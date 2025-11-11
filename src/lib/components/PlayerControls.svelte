<script lang="ts">
	import {
		Shuffle,
		SkipBack,
		Play,
		Pause,
		SkipForward,
		Repeat,
		Repeat1,
		Volume2,
		Volume1,
		VolumeX
	} from 'lucide-svelte';
	import { store } from '$lib/store';

	$: pb = $store.playback;
	$: song = $store.currentSong;

	function formatTime(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60)
			.toString()
			.padStart(2, '0');
		return `${mins}:${secs}`;
	}

	function handleVolumeChange(e: Event) {
		const target = e.target as HTMLInputElement;
		store.setVolume(parseFloat(target.value));
	}
</script>

<div
	class="h-28 flex-shrink-0 bg-zinc-950 m-2 mt-0 rounded-b-lg flex flex-col justify-center items-center p-4 space-y-2"
>
	<!-- Player Controls -->
	<div class="flex items-center space-x-6">
		<button
			on:click={store.toggleShuffle}
			class="transition-colors"
			class:text-yellow-400={pb.isShuffled}
			class:text-zinc-400={!pb.isShuffled}
			class:hover:text-white={!pb.isShuffled}
			title="Shuffle"
		>
			<Shuffle size={20} />
		</button>
		<button class="text-zinc-400 hover:text-white transition-colors" title="Previous Track">
			<SkipBack size={22} />
		</button>
		<button
			on:click={store.togglePlayback}
			class="w-12 h-12 flex items-center justify-center bg-yellow-400 text-black rounded-full hover:bg-yellow-300 transition-colors"
			title={pb.isPlaying ? 'Pause' : 'Play'}
		>
			{#if pb.isPlaying}
				<Pause size={24} />
			{:else}
				<Play size={24} class="ml-1" />
			{/if}
		</button>
		<button class="text-zinc-400 hover:text-white transition-colors" title="Next Track">
			<SkipForward size={22} />
		</button>
		<button
			on:click={store.cycleLoopMode}
			class="transition-colors"
			class:text-yellow-400={pb.loopMode !== 'none'}
			class:text-zinc-400={pb.loopMode === 'none'}
			class:hover:text-white={pb.loopMode === 'none'}
			title="Loop Mode: {pb.loopMode}"
		>
			{#if pb.loopMode === 'single'}
				<Repeat1 size={20} />
			{:else}
				<Repeat size={20} />
			{/if}
		</button>
	</div>

	<!-- Progress Bar -->
	<div class="w-full max-w-2xl flex items-center space-x-3 text-xs text-zinc-400">
		<span>{formatTime(pb.progressSecs)}</span>
		<input
			type="range"
			min="0"
			max={song?.durationSecs ?? 100}
			value={pb.progressSecs}
			class="w-full h-1 bg-zinc-700 rounded-full appearance-none cursor-pointer range-sm accent-yellow-400"
		/>
		<span>{formatTime(song?.durationSecs ?? 0)}</span>
	</div>

	<!-- Volume Control -->
	<div class="absolute right-8 bottom-10 flex items-center space-x-2">
		<button class="text-zinc-400 hover:text-white">
			{#if pb.volume > 0.5}
				<Volume2 size={20} />
			{:else if pb.volume > 0}
				<Volume1 size={20} />
			{:else}
				<VolumeX size={20} />
			{/if}
		</button>
		<input
			type="range"
			min="0"
			max="1"
			step="0.01"
			value={pb.volume}
			on:input={handleVolumeChange}
			class="w-24 h-1 bg-zinc-700 rounded-full appearance-none cursor-pointer range-sm accent-white"
		/>
	</div>
</div>