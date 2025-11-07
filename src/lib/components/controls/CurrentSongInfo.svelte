<script lang="ts">
    import { playerStore, albumArtStore } from '$lib/stores/playerStore';
    import { Music } from 'lucide-svelte';
    import placeholderUrl from '$lib/assets/placeholder.svg';

    $: currentSong = $playerStore.songs.find(
        (s) => s.id === $playerStore.current_song_id
    );
</script>

<div class="flex items-center space-x-3 w-full">
    <!-- Album Art Thumbnail -->
    <div class="w-12 h-12 flex-shrink-0 rounded overflow-hidden">
        {#if $albumArtStore}
            <img src={$albumArtStore} alt="Album Art Thumbnail" class="w-full h-full object-cover" />
        {:else}
            <div class="w-full h-full bg-[#282828] flex items-center justify-center">
                <img src={placeholderUrl} alt="No art" class="w-2/3 h-2/3 text-gray-500" />
            </div>
        {/if}
    </div>

    <!-- Song Details -->
    <div class="min-w-0 flex-grow">
        {#if currentSong}
            <p class="text-sm font-semibold truncate text-white" title={currentSong.title}>
                {currentSong.title || 'Unknown Title'}
            </p>
            <p class="text-xs text-gray-400 truncate" title={currentSong.artist}>
                {currentSong.artist || 'Unknown Artist'}
            </p>
        {:else}
            <p class="text-sm font-semibold text-gray-400">Not Playing</p>
            <p class="text-xs text-gray-500">Select a song</p>
        {/if}
    </div>
</div>