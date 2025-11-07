use crate::models::Song;
use rodio::{OutputStreamHandle, Sink};
use std::time::Instant;

pub struct AppState {
    pub songs: Vec<Song>,
    pub current_song_id: Option<usize>,
    pub is_playing: bool,
    pub is_shuffled: bool,
    pub volume: f32,
    pub sink: Sink,
    pub stream_handle: OutputStreamHandle,
    // New fields for time tracking
    pub playback_start_instant: Option<Instant>,
    pub elapsed_ms: u64,
}
