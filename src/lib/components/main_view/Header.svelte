<script lang="ts">
    import { Search, MapPin, Music } from 'lucide-svelte';
</script>

<div class="space-y-4">
    <div class="flex items-center space-x-2">
        <!-- 
            The actual `icon.ico` is used for the application window icon as per tauri.conf.json.
            Displaying it directly in the webview is complex without placing it in a public directory.
            To adhere to the UI mockup, we use a similar icon from a library here.
        -->
        <Music class="w-6 h-6 text-green-500" />
        <h1 class="text-xl font-bold">FNote</h1>
    </div>
    <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" size={18} />
        <input type="text" placeholder="Search" disabled class="bg-[#282828] w-full rounded-md py-2 pl-10 pr-4 focus:outline-none cursor-not-allowed" />
    </div>
    <div class="flex items-center justify-between text-gray-400">
        <button class="hover:text-white cursor-not-allowed opacity-50">
            <MapPin size={20} />
        </button>
    </div>
</div>