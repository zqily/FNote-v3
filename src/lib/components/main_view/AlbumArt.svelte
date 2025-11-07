<script lang="ts">
    import { albumArtStore } from '$lib/stores/playerStore';
    import placeholderUrl from '$lib/assets/placeholder.svg';
</script>

<div class="w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96 shadow-2xl rounded-lg overflow-hidden">
    {#if $albumArtStore}
        <img src={$albumArtStore} alt="Album Art" class="w-full h-full object-cover" />
    {:else}
        <div class="w-full h-full bg-[#282828] flex items-center justify-center">
            <div class="w-1/2 h-1/2 text-gray-500">
                <img src={placeholderUrl} alt="No album art" class="w-full h-full" />
            </div>
        </div>
    {/if}
</div>
