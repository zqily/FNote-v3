export interface Song {
  id: number;
  path: string;
  title: string;
  artist: string;
  album: string;
  duration_ms: number;
}

export interface PlayerStatusUpdate {
  songs: Song[];
  current_song_id: number | null;
  is_playing: boolean;
  volume: number;
  is_shuffled: boolean;
  repeat_mode: 'Off' | 'Playlist' | 'Single';
  current_time_ms: number;
}
