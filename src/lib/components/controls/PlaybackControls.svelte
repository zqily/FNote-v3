<script lang="ts">
	import { Shuffle, SkipBack, Play, Pause, SkipForward, Repeat } from 'lucide-svelte';
	import { playerStore } from '$lib/stores/playerStore';
	import { invoke } from '@tauri-apps/api/tauri';
	import { fade } from 'svelte/transition';

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
		class="relative flex h-6 w-6 items-center justify-center text-gray-400 hover:text-white"
		class:text-green-500={repeatMode !== 'Off'}
		title={'Repeat: ' + repeatMode}
	>
		<div
			class="flex items-center justify-center"
			on:animationend={onAnimationEnd}
			class:animate-spin-cw={animateClass === 'animate-spin-cw'}
			class:animate-spin-ccw={animateClass === 'animate-spin-ccw'}
		>
			<Repeat size={20} />
		</div>

		{#if repeatMode === 'Playlist'}
			<span
				transition:fade={{ duration: 150 }}
				class="pointer-events-none absolute -bottom-1.5 left-1/2 h-1.5 w-1.5 -translate-x-1/2 rounded-full bg-green-500"
			/>
		{/if}

		{#if repeatMode === 'Single'}
			<div
				transition:fade={{ duration: 150 }}
				class="pointer-events-none absolute -right-1.5 -top-1 flex h-3.5 w-3.5 items-center justify-center rounded-full bg-green-500 text-black"
			>
				<svg viewBox="0 0 24 24" fill="currentColor" class="h-full w-full">
					<text
						x="50%"
						y="53%"
						dominant-baseline="middle"
						text-anchor="middle"
						font-size="16"
						font-weight="bold">1</text
					>
				</svg>
			</div>
		{/if}
	</button>
</div>

<style>
	@keyframes spin-cw {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(180deg);
		}
	}
	@keyframes spin-ccw {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(-180deg);
		}
	}
	.animate-spin-cw {
		animation: spin-cw 0.4s ease-in-out;
	}
	.animate-spin-ccw {
		animation: spin-ccw 0.4s ease-in-out;
	}
</style>
