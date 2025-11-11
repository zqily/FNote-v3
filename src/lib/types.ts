export interface Song {
	id: number;
	title: string;
	artist: string;
	durationSecs: number;
	albumArtUrl: string;
}

export interface Playlist {
	id: number;
	name: string;
	songIds: number[];
}

export type LoopMode = 'none' | 'playlist' | 'single';

export interface PlaybackState {
	isPlaying: boolean;
	currentSongId: number | null;
	progressSecs: number;
	volume: number; // 0.0 to 1.0
	isShuffled: boolean;
	loopMode: LoopMode;
}

// This is the shape of the entire application state from Rust
export interface AppState {
	playlists: Playlist[];
	songs: Song[];
	playback: PlaybackState;
	activePlaylistId: number;
}