<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { setupListeners } from '$lib/stores/playerStore';
	import PlaybackControls from '$lib/components/controls/PlaybackControls.svelte';
	import ProgressBar from '$lib/components/controls/ProgressBar.svelte';
	import VolumeBar from '$lib/components/controls/VolumeBar.svelte';
	import AlbumArt from '$lib/components/main_view/AlbumArt.svelte';
	import Header from '$lib/components/main_view/Header.svelte';
	import SongList from '$lib/components/sidebar/SongList.svelte';
	import UploadButton from '$lib/components/sidebar/UploadButton.svelte';
	import SkeletonMenus from '$lib/components/sidebar/SkeletonMenus.svelte';

	onMount(() => {
		setupListeners();
	});
</script>

<div class="bg-[#121212] text-white h-screen flex flex-col font-sans">
	<main class="flex flex-grow overflow-hidden">
		<!-- Sidebar -->
		<aside class="w-64 bg-[#000000] p-4 flex flex-col space-y-6 flex-shrink-0">
			<Header />
			<UploadButton />
			<div class="flex-grow overflow-y-auto">
				<SongList />
			</div>
			<SkeletonMenus />
		</aside>

		<!-- Main View -->
		<div class="flex-1 flex flex-col items-center justify-center p-8 bg-gradient-to-b from-[#282828] to-[#121212]">
			<AlbumArt />
		</div>
	</main>

	<!-- Footer / Player Controls -->
	<footer class="bg-[#181818] p-4 flex items-center justify-between border-t border-gray-800">
		<div class="w-1/4">
			<!-- Current Song Info Placeholder -->
		</div>
		<div class="w-1/2 flex flex-col items-center space-y-2">
			<PlaybackControls />
			<ProgressBar />
		</div>
		<div class="w-1/4 flex items-center justify-end">
			<VolumeBar />
		</div>
	</footer>
</div>

<style>
	/* Custom scrollbar for webkit browsers */
	::-webkit-scrollbar {
		width: 8px;
	}
	::-webkit-scrollbar-track {
		background: #000;
	}
	::-webkit-scrollbar-thumb {
		background: #555;
		border-radius: 4px;
	}
	::-webkit-scrollbar-thumb:hover {
		background: #777;
	}
</style>
