use rodio::{OutputStream, Sink};
use crate::models::Song;

pub struct AppState {
    pub songs: Vec<Song>,
    pub current_song_id: Option<usize>,
    pub is_playing: bool,
    pub is_shuffled: bool,
    pub volume: f32,
    pub sink: Sink,
    // Keep the stream handle to prevent it from being dropped
    pub _stream: OutputStream, 
}
