<script lang="ts">
    import { ListMusic, Settings } from 'lucide-svelte';
</script>

<div class="space-y-2 text-gray-400 text-sm">
    <h3 class="px-2 font-semibold text-xs uppercase tracking-wider">Playlists</h3>
    <a href="/" class="flex items-center space-x-3 p-2 rounded-md hover:bg-gray-800 cursor-not-allowed opacity-50">
        <ListMusic size={20} />
        <span>My Playlist #1</span>
    </a>
    <a href="/" class="flex items-center space-x-3 p-2 rounded-md hover:bg-gray-800 cursor-not-allowed opacity-50">
        <ListMusic size={20} />
        <span>Favorites</span>
    </a>

    <div class="pt-4">
        <a href="/" class="flex items-center space-x-3 p-2 rounded-md hover:bg-gray-800 cursor-not-allowed opacity-50">
            <Settings size={20} />
            <span>Settings</span>
        </a>
    </div>
</div>