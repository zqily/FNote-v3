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
		VolumeX,
		Minimize2,
		Bookmark
	} from 'lucide-svelte';
	import { store } from '$lib/store';

	$: song = $store.currentSong;
	$: pb = $store.playback;

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
	class="h-full bg-zinc-900 m-6 rounded-lg flex flex-col items-center justify-center p-8 relative border border-zinc-800"
>
	<div class="w-full max-w-sm aspect-square bg-zinc-950 rounded-lg shadow-2xl mb-8">
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
		<h1 class="text-3xl font-bold text-white truncate max-w-2xl">
			{song?.title ?? 'No song selected'}
		</h1>
		<p class="text-lg text-zinc-400 mt-1 truncate max-w-lg">{song?.artist ?? '...'}</p>
	</div>

	<!-- Player Controls Container -->
	<div class="w-full max-w-xl mt-6 space-y-3">
		<!-- Progress Bar -->
		<div class="w-full flex items-center space-x-3 text-xs text-zinc-400">
			<span>{formatTime(pb.progressSecs)}</span>
			<input
				type="range"
				min="0"
				max={song?.durationSecs ?? 100}
				value={pb.progressSecs}
				class="progress-bar w-full h-1 bg-zinc-600 rounded-full appearance-none cursor-pointer accent-yellow-400"
			/>
			<span>{formatTime(song?.durationSecs ?? 0)}</span>
		</div>

		<!-- Buttons -->
		<div class="w-full flex items-center justify-center space-x-4 relative">
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
			<button class="text-zinc-300 hover:text-white transition-colors" title="Previous Track">
				<SkipBack size={24} />
			</button>
			<button
				on:click={store.togglePlayback}
				class="w-14 h-14 flex items-center justify-center bg-yellow-400 text-black rounded-full hover:bg-yellow-300 transition-colors shadow-lg"
				title={pb.isPlaying ? 'Pause' : 'Play'}
			>
				{#if pb.isPlaying}
					<Pause size={28} />
				{:else}
					<Play size={28} />
				{/if}
			</button>
			<button class="text-zinc-300 hover:text-white transition-colors" title="Next Track">
				<SkipForward size={24} />
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

			<!-- Right Side Controls -->
			<div class="absolute right-0 flex items-center space-x-4">
				<button class="text-zinc-400 hover:text-white transition-colors" title="Bookmark">
					<Bookmark size={20} />
				</button>
				<!-- Volume Control -->
				<div class="group flex items-center space-x-2">
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
						class="w-24 h-1 bg-zinc-600 rounded-full appearance-none cursor-pointer accent-white scale-x-0 group-hover:scale-x-100 transition-transform duration-200 origin-left"
					/>
				</div>
			</div>
		</div>
	</div>

	<!-- Minimize button -->
	<button class="absolute bottom-4 right-4 text-zinc-500 hover:text-white transition-colors">
		<Minimize2 size={18} />
	</button>
</div>

<style>
	.progress-bar::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 14px;
		height: 14px;
		background: white;
		border-radius: 50%;
	}
	.progress-bar::-moz-range-thumb {
		width: 14px;
		height: 14px;
		background: white;
		border-radius: 50%;
		border: none;
	}
</style>