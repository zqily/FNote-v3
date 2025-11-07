<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { FolderUp } from 'lucide-svelte';

	async function handleUpload() {
		await invoke('upload_folder');
	}
</script>

<button
	on:click={handleUpload}
	class="w-full bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-lg flex items-center justify-center space-x-2 transition-colors"
>
	<FolderUp size={20} />
	<span>Upload songs/folder</span>
</button>
