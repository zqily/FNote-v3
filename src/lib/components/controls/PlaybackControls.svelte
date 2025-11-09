<script lang="ts">
	import { Shuffle, SkipBack, Play, Pause, SkipForward, Repeat } from 'lucide-svelte';
	import { playerStore } from '$lib/stores/playerStore';
	import { invoke } from '@tauri-apps/api/tauri';

	$: isPlaying = $playerStore.is_playing;
	$: isShuffled = $playerStore.is_shuffled;
	$: repeatMode = $playerStore.repeat_mode;

	let previousRepeatMode = repeatMode;
	let animateClass = '';

	$: {
		// This reactive block triggers when repeatMode changes
		if (previousRepeatMode === 'Off' && repeatMode === 'Playlist') {
			animateClass = 'animate-spin-cw';
		} else if (previousRepeatMode === 'Single' && repeatMode === 'Off') {
			animateClass = 'animate-spin-ccw';
		}
		// Update previousRepeatMode for the next change detection
		previousRepeatMode = repeatMode;
	}

	function onAnimationEnd() {
		// Reset the animation class so it can be re-applied on future state changes
		animateClass = '';
	}
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
	<button
		on:click={() => invoke('toggle_repeat_mode')}
		class="text-gray-400 hover:text-white"
		class:text-green-500={repeatMode !== 'Off'}
		title={'Repeat: ' + repeatMode}
	>
		<div
			class="relative w-5 h-5 flex items-center justify-center"
			on:animationend={onAnimationEnd}
			class:animate-spin-cw={animateClass === 'animate-spin-cw'}
			class:animate-spin-ccw={animateClass === 'animate-spin-ccw'}
		>
			<Repeat size={20} />

			{#if repeatMode === 'Playlist'}
				<span
					class="absolute -bottom-1.5 left-1/2 -translate-x-1/2 w-1.5 h-1.5 bg-green-500 rounded-full"
				/>
			{/if}

			{#if repeatMode === 'Single'}
				<span
					class="absolute -top-1 -right-1.5 flex items-center justify-center w-3.5 h-3.5 text-[10px] font-bold bg-green-500 text-black rounded-full leading-none"
					>1</span
				>
			{/if}
		</div>
	</button>
</div>

<style>
	@keyframes spin-cw {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
	@keyframes spin-ccw {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(-360deg);
		}
	}
	.animate-spin-cw {
		animation: spin-cw 0.4s ease-in-out;
	}
	.animate-spin-ccw {
		animation: spin-ccw 0.4s ease-in-out;
	}
</style>
