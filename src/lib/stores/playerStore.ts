import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import type { PlayerStatusUpdate } from '$lib/types';

const initialState: PlayerStatusUpdate = {
  songs: [],
  current_song_id: null,
  is_playing: false,
  volume: 1.0,
  is_shuffled: false,
  repeat_mode: 'Off',
  current_time_ms: 0,
};

export const playerStore = writable<PlayerStatusUpdate>(initialState);

// This store will hold the base64 string of the current album art
export const albumArtStore = writable<string | null>(null);

export async function setupListeners() {
  await listen<PlayerStatusUpdate>('player://status-update', (event) => {
    playerStore.set(event.payload);
  });

  await listen<string>('player://album-art-update', (event) => {
    albumArtStore.set(event.payload);
  });

  await listen<string>('player://error', (event) => {
    console.error('Backend Error:', event.payload);
    // Here you could show a toast notification to the user
  });
}
