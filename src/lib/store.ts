import { writable, get, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import type { AppState, Song, Playlist, LoopMode, PlaybackState } from './types';

// --- Writable Stores ---
// These hold the raw data from the Rust backend.
const playlists = writable<Playlist[]>([]);
const songs = writable<Song[]>([]);
const playback = writable<PlaybackState>({
	isPlaying: false,
	currentSongId: null,
	progressSecs: 0,
	volume: 1,
	isShuffled: false,
	loopMode: 'none'
});
const activePlaylistId = writable<number>(0);

// --- Derived Stores ---
// These compute values based on the writable stores, making UI logic cleaner.

// Provides the full Song object for the currently playing song.
const currentSong = derived([playback, songs], ([$playback, $songs]) => {
	if ($playback.currentSongId === null) return null;
	return $songs.find((s) => s.id === $playback.currentSongId) ?? null;
});

// Provides the full Playlist object for the currently active playlist.
const activePlaylist = derived([activePlaylistId, playlists], ([$activePlaylistId, $playlists]) => {
	return $playlists.find((p) => p.id === $activePlaylistId) ?? null;
});

// Provides a list of full Song objects for the songs in the active playlist.
const activePlaylistSongs = derived([activePlaylist, songs], ([$activePlaylist, $songs]) => {
	if (!$activePlaylist) return [];
	// Create a map for quick song lookup
	const songMap = new Map($songs.map((s) => [s.id, s]));
	return $activePlaylist.songIds.map((id) => songMap.get(id)).filter(Boolean) as Song[];
});

// --- Actions ---
// Functions that interact with the Rust backend. They handle the IPC call
// and update the Svelte stores upon success.

/**
 * Fetches the entire application state from Rust and initializes the stores.
 * Should be called once when the application loads.
 */
async function initializeState() {
	try {
		const initialState: AppState = await invoke('get_initial_data');
		playlists.set(initialState.playlists);
		songs.set(initialState.songs);
		playback.set(initialState.playback);
		activePlaylistId.set(initialState.activePlaylistId);
	} catch (e) {
		console.error('Failed to initialize application state:', e);
	}
}

async function togglePlayback() {
	try {
		const newIsPlaying: boolean = await invoke('toggle_playback');
		playback.update((p) => ({ ...p, isPlaying: newIsPlaying }));
	} catch (e) {
		console.error('Failed to toggle playback:', e);
	}
}

async function cycleLoopMode() {
	try {
		const newLoopMode: LoopMode = await invoke('cycle_loop_mode');
		playback.update((p) => ({ ...p, loopMode: newLoopMode }));
	} catch (e) {
		console.error('Failed to cycle loop mode:', e);
	}
}

async function toggleShuffle() {
	try {
		const newIsShuffled: boolean = await invoke('toggle_shuffle');
		playback.update((p) => ({ ...p, isShuffled: newIsShuffled }));
	} catch (e) {
		console.error('Failed to toggle shuffle:', e);
	}
}

async function setVolume(newVolume: number) {
	try {
		await invoke('set_volume', { volume: newVolume });
		playback.update((p) => ({ ...p, volume: newVolume }));
	} catch (e) {
		console.error('Failed to set volume:', e);
	}
}

async function selectPlaylist(id: number) {
	try {
		await invoke('select_playlist', { id });
		activePlaylistId.set(id);
	} catch (e) {
		console.error(`Failed to select playlist ${id}:`, e);
	}
}

/**
 * Simulates importing songs by opening a file dialog.
 * This is a placeholder and doesn't actually process files.
 */
async function importSongs() {
	try {
		const selected = await open({
			multiple: true,
			filters: [{ name: 'Audio', extensions: ['mp3', 'wav', 'flac'] }]
		});
		if (selected) {
			console.log('Simulating import for:', selected);
			// In a real app, you would send these file paths to Rust
			// for processing and then refresh the state.
		}
	} catch (e) {
		console.error('File dialog error:', e);
	}
}

export const store = {
	// State (readable stores)
	subscribe: derived(
		[playlists, songs, playback, activePlaylistId, currentSong, activePlaylist, activePlaylistSongs],
		([$p, $s, $pb, $apId, $cs, $ap, $aps]) => ({
			playlists: $p,
			songs: $s,
			playback: $pb,
			activePlaylistId: $apId,
			currentSong: $cs,
			activePlaylist: $ap,
			activePlaylistSongs: $aps
		})
	).subscribe,

	// Actions
	initializeState,
	togglePlayback,
	cycleLoopMode,
	toggleShuffle,
	setVolume,
	selectPlaylist,
	importSongs
};