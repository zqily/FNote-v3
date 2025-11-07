<script lang="ts">
    import { albumArtStore } from '$lib/stores/playerStore';
    import Placeholder from '$lib/assets/placeholder.svg?component';
</script>

<div class="w-64 h-64 md:w-80 md:h-80 lg:w-96 lg:h-96 shadow-2xl rounded-lg overflow-hidden">
    {#if $albumArtStore}
        <img src={$albumArtStore} alt="Album Art" class="w-full h-full object-cover" />
    {:else}
        <div class="w-full h-full bg-[#282828] flex items-center justify-center">
            <div class="w-1/2 h-1/2 text-gray-500">
                <Placeholder />
            </div>
        </div>
    {/if}
</div>