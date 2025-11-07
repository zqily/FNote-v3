<script lang="ts">
	import { playerStore } from '$lib/stores/playerStore';
	import type { Song } from '$lib/types';
	import { invoke } from '@tauri-apps/api/tauri';

	let progress = 0;
	let isSeeking = false;

	$: currentSong = $playerStore.songs.find((s) => s.id === $playerStore.current_song_id);
	$: duration = currentSong?.duration_ms || 0;

	$: if (!isSeeking) {
		progress = $playerStore.current_time_ms;
	}

	function formatTime(ms: number) {
		if (isNaN(ms)) ms = 0;
		const totalSeconds = Math.floor(ms / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}:${seconds.toString().padStart(2, '0')}`;
	}

	function handleSeek(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		const newPosition = Number(input.value);
		invoke('seek_to', { positionMs: newPosition });
		isSeeking = false;
	}

	function handleInput(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		progress = Number(input.value);
	}
</script>

<div class="flex items-center justify-center w-full max-w-xl space-x-2">
	<span class="text-xs text-gray-400 w-10 text-right">{formatTime(progress)}</span>
	<input
		type="range"
		min="0"
		max={duration}
		value={progress}
		class="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-green-500"
		on:mousedown={() => (isSeeking = true)}
		on:input={handleInput}
		on:change={handleSeek}
	/>
	<span class="text-xs text-gray-400 w-10 text-left">{formatTime(duration)}</span>
</div>
