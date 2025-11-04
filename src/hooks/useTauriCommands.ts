// src/hooks/useTauriCommands.ts

import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { useAppStore } from '../store';
import { Track } from '../types';

// This hook provides typed wrappers around the Tauri `invoke` API.
// It's a central place to manage all communication *to* the Rust backend.
export const useTauriCommands = () => {
    const { setLibrary } = useAppStore();

    const selectAndScanMusicDirectory = async () => {
        const selected = await open({ directory: true, multiple: false });
        if (typeof selected === 'string') {
            const tracks = await invoke<Track[]>('scan_music_directory', { path: selected });
            setLibrary(tracks);
        }
    };

    const playTrack = (trackId: string) => invoke('play_track', { trackId });
    const pausePlayback = () => invoke('pause_playback');
    const resumePlayback = () => invoke('resume_playback');

    return {
        selectAndScanMusicDirectory,
        playTrack,
        pausePlayback,
        resumePlayback,
    };
};