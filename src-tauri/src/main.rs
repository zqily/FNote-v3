// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod player;
mod state;

use std::sync::mpsc;
use tauri::Manager;

fn main() {
    // Create a channel for communicating with the dedicated audio player thread.
    // `tx` is the sender (transmitter), `rx` is the receiver.
    let (tx, rx) = mpsc::channel();

    // Create the initial application state. We pass the sender `tx` to it
    // so that our Tauri commands can send messages to the player thread.
    let app_state = state::AppState::new(tx);

    tauri::Builder::default()
        // The `.setup()` hook is the ideal place to perform initialization tasks
        // like spawning threads. It runs once when the application starts.
        .setup(|app| {
            // We need a handle to the app to be able to emit events from the player thread.
            let app_handle = app.handle();

            // Spawn the player thread. `std::thread::spawn` creates a new OS thread.
            // The `move` keyword transfers ownership of `app_handle` and `rx` to the new thread.
            std::thread::spawn(move || {
                player::run_player(app_handle, rx);
            });

            // All setup hooks must return Ok.
            Ok(())
        })
        // `manage` makes our AppState available to all command functions.
        // Tauri handles the necessary synchronization (e.g., using a Mutex).
        .manage(app_state)
        // `invoke_handler` registers all our `#[tauri::command]` functions.
        // `generate_handler!` is a macro that collects all functions passed to it.
        .invoke_handler(tauri::generate_handler![
            commands::library::scan_music_directory,
            commands::playback::play_track,
            commands::playback::pause_playback,
            commands::playback::resume_playback
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}