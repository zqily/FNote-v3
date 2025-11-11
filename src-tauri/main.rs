// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

// The state enum for our player's loop mode.
// It derives traits needed for serialization (to cross the IPC bridge),
// debugging, and easy copying.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // Use camelCase for frontend compatibility
pub enum LoopMode {
    None,
    Playlist,
    Single,
}

// The shared application state struct.
// We use a Mutex to ensure thread-safe access from different commands.
pub struct AppState {
    loop_mode: Mutex<LoopMode>,
}

// --- Tauri Commands ---
// These functions are exposed to the frontend and can be called via `invoke`.

/// Gets the current loop mode from the shared state.
#[tauri::command]
async fn get_loop_mode(state: State<'_, AppState>) -> Result<LoopMode, ()> {
    // Lock the mutex to get read-only access and return the value.
    Ok(*state.loop_mode.lock().await)
}

/// Sets the loop mode in the shared state.
#[tauri::command]
async fn set_loop_mode(mode: LoopMode, state: State<'_, AppState>) -> Result<(), ()> {
    // Lock the mutex to get write access and update the value.
    let mut loop_mode = state.loop_mode.lock().await;
    *loop_mode = mode;
    // In a real app, you might emit an event here to notify all windows of the change.
    Ok(())
}


fn main() {
    // Initialize our shared state with a default value.
    let app_state = AppState {
        loop_mode: Mutex::new(LoopMode::None),
    };

    tauri::Builder::default()
        // Make the AppState managed by Tauri
        .manage(app_state)
        // Register our commands so the frontend can call them
        .invoke_handler(tauri::generate_handler![
            get_loop_mode,
            set_loop_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}