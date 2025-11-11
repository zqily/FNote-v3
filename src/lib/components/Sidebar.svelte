<script lang="ts">
	import { Search, Plus } from 'lucide-svelte';
	import { store } from '$lib/store';
</script>

<aside class="w-80 flex-shrink-0 bg-zinc-900 p-2 flex flex-col space-y-2">
	<!-- Top Section -->
	<div class="bg-zinc-950 rounded-lg p-4 space-y-4">
		<div class="flex items-center space-x-2">
			<img src="/fnote.png" alt="FNote Logo" class="h-8 w-8" />
			<h1 class="text-xl font-bold">FNote</h1>
		</div>
		<button
			on:click={store.importSongs}
			class="w-full bg-zinc-800 hover:bg-zinc-700 transition-colors text-neutral-200 font-semibold py-2 px-4 rounded-md flex items-center justify-center space-x-2"
		>
			<span>Import Songs</span>
		</button>
	</div>

	<!-- Main List Section -->
	<div class="bg-zinc-950 rounded-lg p-2 flex-1 flex flex-col min-h-0">
		<!-- Filter -->
		<div class="relative p-2">
			<Search class="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-500" size={18} />
			<input
				type="text"
				placeholder="Filter current list..."
				class="w-full bg-zinc-800 border-zinc-700 rounded-md pl-8 pr-4 py-1.5 text-sm focus:ring-2 focus:ring-yellow-500 focus:outline-none"
			/>
		</div>

		<!-- Playlists -->
		<div class="px-2 pt-4 pb-2">
			<div class="flex justify-between items-center mb-2">
				<h2 class="text-xs font-bold uppercase text-zinc-400 tracking-wider">Playlists</h2>
				<button class="text-zinc-400 hover:text-white transition-colors" title="Create new playlist">
					<Plus size={18} />
				</button>
			</div>
			<ul>
				{#each $store.playlists as playlist (playlist.id)}
					<li>
						<button
							class="w-full text-left px-2 py-1.5 rounded-md text-sm font-medium transition-colors"
							class:bg-zinc-800={$store.activePlaylistId === playlist.id}
							class:text-white={$store.activePlaylistId === playlist.id}
							class:text-zinc-400={$store.activePlaylistId !== playlist.id}
							class:hover:bg-zinc-800={$store.activePlaylistId !== playlist.id}
							on:click={() => store.selectPlaylist(playlist.id)}
						>
							{playlist.name}
						</button>
					</li>
				{/each}
			</ul>
		</div>

		<!-- Divider -->
		<hr class="border-zinc-800 mx-2" />

		<!-- Songs List -->
		<div class="flex-1 flex flex-col min-h-0">
			{#if $store.activePlaylist}
				<div class="px-2 pt-4 pb-2">
					<h3 class="text-sm font-semibold text-zinc-300">
						Songs in <span class="text-white">{$store.activePlaylist.name}</span>
					</h3>
				</div>
			{/if}

			<div class="flex-1 overflow-y-auto pr-1">
				<ul>
					{#each $store.activePlaylistSongs as song (song.id)}
						<li
							class="px-2 py-1.5 rounded-md hover:bg-zinc-800 cursor-pointer"
							class:bg-zinc-800={song.id === $store.currentSong?.id}
						>
							<p
								class="font-medium text-sm truncate"
								class:text-yellow-400={song.id === $store.currentSong?.id}
								class:text-neutral-200={song.id !== $store.currentSong?.id}
							>
								{song.title}
							</p>
							<p class="text-xs text-zinc-400 truncate">{song.artist}</p>
						</li>
					{/each}
				</ul>
			</div>
		</div>
	</div>
</aside>