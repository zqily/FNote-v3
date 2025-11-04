// src-tauri/src/models.rs

use serde::{Deserialize, Serialize};

/// Represents a single audio track in the music library.
/// It must derive `Serialize` to be sent to the frontend and `Clone` to be easily passed around.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    // A unique ID generated from the file path's hash. Stable and reliable.
    pub id: String,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_secs: u32,
}

/// Represents the current playback status, sent from Rust to the frontend via events.
/// This is a "view model" for the UI.
#[derive(Debug, Clone, Serialize)]
pub struct PlaybackStatus {
    pub is_playing: bool,
    // The ID of the track currently playing or paused.
    pub current_track_id: Option<String>,
    // The elapsed time of the current track in seconds.
    pub elapsed_secs: f32,
    // The total duration of the current track in seconds.
    pub duration_secs: u32,
}