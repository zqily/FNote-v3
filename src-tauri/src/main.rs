// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod library;
mod models;
mod player;
mod state;

use rodio::{OutputStream, Sink};
use state::AppState;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let app_state = AppState {
        songs: Vec::new(),
        current_song_id: None,
        is_playing: false,
        is_shuffled: false,
        volume: 1.0,
        sink,
        stream_handle,
        playback_start_instant: None, // Initialize new field
        elapsed_ms: 0,                // Initialize new field
    };

    let app_state_arc = Arc::new(Mutex::new(app_state));

    tauri::Builder::default()
        .manage(app_state_arc.clone())
        .setup(move |app| {
            let app_handle = app.handle();
            let state_clone = app_state_arc.clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(250)).await;
                    let mut state_guard = state_clone.lock().unwrap();

                    if state_guard.is_playing && state_guard.sink.empty() {
                         if let Err(e) = player::next_song(&mut state_guard, &app_handle) {
                             eprintln!("Error playing next song automatically: {}", e);
                         }
                    } else {
                        // This block replaces the old PlayerStatusUpdate logic
                        let song_duration_ms = state_guard.current_song_id
                            .and_then(|id| state_guard.songs.get(id))
                            .map_or(0, |s| s.duration_ms);

                        let mut current_time_ms = player::get_current_time_ms(&state_guard);
                        if song_duration_ms > 0 {
                            current_time_ms = current_time_ms.min(song_duration_ms);
                        }
                        
                         let status = models::PlayerStatusUpdate {
                            songs: state_guard.songs.clone(),
                            current_song_id: state_guard.current_song_id,
                            is_playing: state_guard.is_playing && !state_guard.sink.is_paused(),
                            volume: state_guard.volume,
                            is_shuffled: state_guard.is_shuffled,
                            current_time_ms,
                        };
                        app_handle.emit_all("player://status-update", status).unwrap();
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::upload_folder,
            commands::play_song,
            commands::toggle_playback,
            commands::next_song,
            commands::prev_song,
            commands::set_volume,
            commands::seek_to,
            commands::toggle_shuffle,
            commands::get_album_art,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
