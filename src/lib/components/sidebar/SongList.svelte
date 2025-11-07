<script lang="ts">
	import { playerStore } from '$lib/stores/playerStore';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Music } from 'lucide-svelte';

	$: songs = $playerStore.songs;
	$: currentSongId = $playerStore.current_song_id;

	function formatDuration(ms: number) {
		const totalSeconds = Math.floor(ms / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}:${seconds.toString().padStart(2, '0')}`;
	}

	async function playSong(id: number) {
		await invoke('play_song', { id });
	}
</script>

<div class="space-y-2 text-sm">
	{#if songs.length === 0}
		<p class="text-gray-400 text-center py-10">Upload a folder to begin.</p>
	{:else}
		{#each songs as song (song.id)}
			<button
				on:click={() => playSong(song.id)}
				class="w-full text-left p-2 rounded-md flex items-center justify-between transition-colors"
				class:bg-green-600={currentSongId === song.id}
				class:hover:bg-gray-800={currentSongId !== song.id}
			>
				<div class="flex items-center space-x-3 overflow-hidden">
					<Music size={18} class="text-gray-400 flex-shrink-0" />
					<div class="truncate">
						<p
							class="font-semibold"
							class:text-white={currentSongId === song.id}
							class:text-gray-200={currentSongId !== song.id}
						>
							{song.title || 'Unknown Title'}
						</p>
						<p
							class:text-gray-200={currentSongId === song.id}
							class:text-gray-400={currentSongId !== song.id}
						>
							{song.artist || 'Unknown Artist'}
						</p>
					</div>
				</div>
				<span class="text-gray-400">{formatDuration(song.duration_ms)}</span>
			</button>
		{/each}
	{/if}
</div>
