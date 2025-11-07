use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};
use crate::{state::AppState, library, player};
use rfd::AsyncFileDialog;

// A helper type for command results
type CommandResult<T> = Result<T, String>;

#[tauri::command]
pub async fn upload_folder(state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let folder = AsyncFileDialog::new().pick_folder().await;

    if let Some(folder_path) = folder {
        let path = folder_path.path().to_path_buf();
        
        let songs = tokio::task::spawn_blocking(move || library::scan_directory(&path))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;

        let mut state_guard = state.lock().unwrap();
        
        // Stop playback and clear queue
        state_guard.sink.stop();
        state_guard.current_song_id = None;
        state_guard.is_playing = false;

        state_guard.songs = songs;

        let status = crate::models::PlayerStatusUpdate {
            songs: state_guard.songs.clone(),
            current_song_id: None,
            is_playing: false,
            volume: state_guard.volume,
            is_shuffled: state_guard.is_shuffled,
            current_time_ms: 0,
        };
        app_handle.emit_all("player://status-update", status).unwrap();
        app_handle.emit_all("player://album-art-update", Option::<String>::None).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn play_song(id: usize, state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::play_song(id, &mut state_guard, &app_handle)
}

#[tauri::command]
pub async fn toggle_playback(state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::toggle_playback(&mut state_guard, &app_handle);
    Ok(())
}

#[tauri::command]
pub async fn next_song(state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::next_song(&mut state_guard, &app_handle)
}

#[tauri::command]
pub async fn prev_song(state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::prev_song(&mut state_guard, &app_handle)
}

#[tauri::command]
pub async fn set_volume(volume: f32, state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::set_volume(volume, &mut state_guard, &app_handle);
    Ok(())
}

#[tauri::command]
pub async fn seek_to(position_ms: u64, state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::seek_to(position_ms, &mut state_guard, &app_handle)
}

#[tauri::command]
pub async fn toggle_shuffle(state: State<'_, Arc<Mutex<AppState>>>, app_handle: AppHandle) -> CommandResult<()> {
    let mut state_guard = state.lock().unwrap();
    player::toggle_shuffle(&mut state_guard, &app_handle);
    Ok(())
}

#[tauri::command]
pub async fn get_album_art(song_id: usize, state: State<'_, Arc<Mutex<AppState>>>) -> CommandResult<Option<String>> {
    let path = {
        let state_guard = state.lock().unwrap();
        state_guard.songs.get(song_id).map(|s| s.path.clone())
    };

    if let Some(p) = path {
        tokio::task::spawn_blocking(move || library::get_album_art_data(&p))
            .await
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}
