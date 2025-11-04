// src-tauri/src/commands/playback.rs

use crate::{player::PlayerCommand, state::AppState};
use tauri::State;

/// Command to start playing a specific track.
#[tauri::command]
pub async fn play_track(track_id: String, app_state: State<'_, AppState>) -> Result<(), String> {
    // Find the track in the library by its ID.
    let track_to_play = {
        let library = app_state.library.lock().unwrap();
        library.iter().find(|t| t.id == track_id).cloned()
    };

    if let Some(track) = track_to_play {
        // Lock the sender and send the Play command to the player thread.
        app_state
            .player_command_sender
            .lock()
            .unwrap()
            .send(PlayerCommand::Play(track))
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Track not found".to_string())
    }
}

/// Command to pause the currently playing track.
#[tauri::command]
pub async fn pause_playback(app_state: State<'_, AppState>) -> Result<(), String> {
    app_state
        .player_command_sender
        .lock()
        .unwrap()
        .send(PlayerCommand::Pause)
        .map_err(|e| e.to_string())
}

/// Command to resume the currently paused track.
#[tauri::command]
pub async fn resume_playback(app_state: State<'_, AppState>) -> Result<(), String> {
    app_state
        .player_command_sender
        .lock()
        .unwrap()
        .send(PlayerCommand::Resume)
        .map_err(|e| e.to_string())
}