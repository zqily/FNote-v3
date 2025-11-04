// This file defines TypeScript types that correspond to the Rust structs in `src-tauri/src/models.rs`.
// Keeping them in sync is crucial for type safety across the IPC bridge.

export interface Track {
  id: string;
  path: string;
  title: string;
  artist: string;
  album: string;
  duration_secs: number;
}

export interface PlaybackStatus {
  is_playing: boolean;
  current_track_id: string | null;
  elapsed_secs: number;
  duration_secs: number;
}