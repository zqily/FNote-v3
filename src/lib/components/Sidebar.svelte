<script lang="ts">
	import { Search, Plus, Music, List, Settings, ListFilter } from 'lucide-svelte';
	import { store } from '$lib/store';
</script>

<aside class="w-80 flex-shrink-0 bg-zinc-950 flex flex-col">
	<!-- Top Section -->
	<div class="bg-zinc-950 rounded-lg p-4 space-y-4">
		<div class="flex items-center space-x-2">
			<img src="/fnote.png" alt="FNote Logo" class="h-8 w-8" />
			<h1 class="text-xl font-bold">FNote</h1>
		</div>
		<button
			on:click={store.importSongs}
			class="w-full bg-zinc-800 hover:bg-accent/20 hover:text-accent transition-colors text-neutral-200 font-semibold py-2 px-4 rounded-md flex items-center justify-center"
		>
			<span>Import Songs</span>
		</button>
	</div>

	<!-- Main List Section -->
	<div class="bg-zinc-950 rounded-lg p-2 flex-1 flex flex-col min-h-0 relative">
		<!-- Filter -->
		<div class="relative p-2">
			<Search class="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-500" size={18} />
			<input
				type="text"
				placeholder="Filter current list..."
				class="w-full bg-zinc-800 border-zinc-700 rounded-md pl-8 pr-8 py-1.5 text-sm focus:ring-2 focus:ring-accent focus:outline-none"
			/>
			<button class="absolute right-4 top-1/2 -translate-y-1/2 text-zinc-500 hover:text-accent">
				<ListFilter size={16} />
			</button>
		</div>

		<!-- Playlists -->
		<div class="px-2 pt-4 pb-2">
			<div class="flex justify-between items-center mb-2">
				<h2 class="text-xs font-bold uppercase text-zinc-400 tracking-wider">Playlists</h2>
				<button class="text-zinc-400 hover:text-accent transition-colors" title="Create new playlist">
					<Plus size={18} />
				</button>
			</div>
			<ul>
				{#each $store.playlists as playlist (playlist.id)}
					<li>
						<button
							class="w-full text-left px-2 py-1.5 rounded-md text-sm font-medium transition-colors flex items-center space-x-3 hover:bg-zinc-800 hover:text-accent {$store.activePlaylistId ===
							playlist.id
								? 'text-accent bg-gradient-to-r from-accent/20 to-transparent'
								: 'text-zinc-400'}"
							on:click={() => store.selectPlaylist(playlist.id)}
						>
							<Music size={16} />
							<span>{playlist.name}</span>
						</button>
					</li>
				{/each}
			</ul>
		</div>

		<!-- Divider -->
		<hr class="border-zinc-800 mx-2" />

		<!-- Songs List -->
		<div class="flex-1 flex flex-col min-h-0 pb-12">
			{#if $store.activePlaylist}
				<div class="px-2 pt-4 pb-2 flex justify-between items-center">
					<h3 class="text-xs font-bold uppercase text-zinc-400 tracking-wider">
						Songs in {$store.activePlaylist.name}
					</h3>
					<button class="text-zinc-400 hover:text-accent transition-colors" title="List options">
						<List size={18} />
					</button>
				</div>
			{/if}

			<div class="flex-1 overflow-y-auto pr-1">
				<ul>
					{#each $store.activePlaylistSongs as song (song.id)}
						<li>
							<button
								type="button"
								on:click={() => store.selectSong(song.id)}
								class={`w-full text-left px-2 py-1.5 rounded-md hover:bg-zinc-800 cursor-pointer transition-colors ${
									song.id === $store.currentSong?.id ? 'bg-accent/20' : ''
								}`}
								aria-current={song.id === $store.currentSong?.id}
							>
								<p
									class="font-medium text-sm truncate"
									class:text-accent={song.id === $store.currentSong?.id}
									class:text-neutral-200={song.id !== $store.currentSong?.id}
								>
									{song.title}
								</p>
								<p class="text-xs text-zinc-400 truncate">{song.artist}</p>
							</button>
						</li>
					{/each}
				</ul>
			</div>
		</div>

		<!-- Settings -->
		<div class="absolute bottom-0 left-0 right-0 p-2 bg-zinc-950">
			<hr class="border-zinc-800 mx-2 mb-2" />
			<button
				class="w-full flex items-center space-x-3 text-zinc-400 hover:text-accent transition-colors px-2 py-1.5 rounded-md text-sm font-medium"
			>
				<Settings size={16} />
				<span>Settings</span>
			</button>
		</div>
	</div>
</aside>
