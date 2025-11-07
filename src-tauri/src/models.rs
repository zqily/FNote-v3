use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: usize,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStatusUpdate {
    pub songs: Vec<Song>,
    pub current_song_id: Option<usize>,
    pub is_playing: bool,
    pub volume: f32,
    pub is_shuffled: bool,
    pub current_time_ms: u64,
}
