// src-tauri/src/state.rs

use std::sync::{mpsc::Sender, Mutex};

use crate::{models::Track, player::PlayerCommand};

/// This struct holds the application's shared state.
/// Tauri will manage access to this state for us.
pub struct AppState {
    // The music library, a list of all discovered tracks.
    pub library: Mutex<Vec<Track>>,
    // A channel sender to communicate with the dedicated audio player thread.
    // This allows our commands (which run on a separate thread pool) to send
    // instructions like "Play" or "Pause" to the audio thread without blocking.
    pub player_command_sender: Mutex<Sender<PlayerCommand>>,
}

impl AppState {
    pub fn new(player_command_sender: Sender<PlayerCommand>) -> Self {
        Self {
            library: Mutex::new(Vec::new()),
            player_command_sender: Mutex::new(player_command_sender),
        }
    }
}