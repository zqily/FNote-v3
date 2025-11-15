// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

// --- Data Models ---
// These structs define the shape of our application's data.
// They derive traits for serialization, debugging, cloning, etc.

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LoopMode {
    None,
    Playlist,
    Single,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    id: u32,
    title: String,
    artist: String,
    duration_secs: u32,
    album_art_url: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    id: u32,
    name: String,
    song_ids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    is_playing: bool,
    current_song_id: Option<u32>,
    progress_secs: u32,
    volume: f32, // 0.0 to 1.0
    is_shuffled: bool,
    loop_mode: LoopMode,
}

// The complete, shared application state.
// It MUST derive Serialize to be sent to the frontend and Clone to be copied out of the Mutex.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    playlists: Vec<Playlist>,
    songs: Vec<Song>,
    playback: PlaybackState,
    active_playlist_id: u32,
}

pub struct AppStateManager(Mutex<AppState>);

// --- Tauri Commands ---

/// Fetches the entire initial state for the UI.
#[tauri::command]
async fn get_initial_data(state: State<'_, AppStateManager>) -> Result<AppState, ()> {
    // Correctly dereference the MutexGuard before cloning the AppState inside.
    Ok((*state.0.lock().await).clone())
}

/// Toggles the play/pause state.
#[tauri::command]
async fn toggle_playback(state: State<'_, AppStateManager>) -> Result<bool, ()> {
    let mut app_state = state.0.lock().await;
    if app_state.playback.current_song_id.is_some() {
        app_state.playback.is_playing = !app_state.playback.is_playing;
    }
    Ok(app_state.playback.is_playing)
}

/// Cycles through loop modes: None -> Playlist -> Single -> None.
#[tauri::command]
async fn cycle_loop_mode(state: State<'_, AppStateManager>) -> Result<LoopMode, ()> {
    let mut app_state = state.0.lock().await;
    app_state.playback.loop_mode = match app_state.playback.loop_mode {
        LoopMode::None => LoopMode::Playlist,
        LoopMode::Playlist => LoopMode::Single,
        LoopMode::Single => LoopMode::None,
    };
    Ok(app_state.playback.loop_mode)
}

/// Toggles the shuffle state.
#[tauri::command]
async fn toggle_shuffle(state: State<'_, AppStateManager>) -> Result<bool, ()> {
    let mut app_state = state.0.lock().await;
    app_state.playback.is_shuffled = !app_state.playback.is_shuffled;
    Ok(app_state.playback.is_shuffled)
}

/// Sets the player volume.
#[tauri::command]
async fn set_volume(volume: f32, state: State<'_, AppStateManager>) -> Result<(), ()> {
    let mut app_state = state.0.lock().await;
    app_state.playback.volume = volume.clamp(0.0, 1.0);
    Ok(())
}

/// Placeholder for selecting a playlist.
#[tauri::command]
async fn select_playlist(id: u32, state: State<'_, AppStateManager>) -> Result<(), ()> {
    let mut app_state = state.0.lock().await;
    if app_state.playlists.iter().any(|p| p.id == id) {
        app_state.active_playlist_id = id;
    }
    Ok(())
}

/// Sets the currently active song.
#[tauri::command]
async fn select_song(id: u32, state: State<'_, AppStateManager>) -> Result<(), ()> {
    let mut app_state = state.0.lock().await;
    // Check if the song exists before setting it.
    if app_state.songs.iter().any(|s| s.id == id) {
        app_state.playback.current_song_id = Some(id);
    }
    Ok(())
}


// --- Main Application Setup ---

fn main() {
    // Create mock data for initial startup
    let mock_songs = vec![
        Song { id: 1, title: "Feel It (From \"Invincible\")".to_string(), artist: "d4vd".to_string(), duration_secs: 157, album_art_url: "https://i.scdn.co/image/ab67616d0000b27303351d33b47f07897e937397".to_string() },
        Song { id: 2, title: "Doechii - Anxiety (Visualizer)".to_string(), artist: "Doechii".to_string(), duration_secs: 185, album_art_url: "".to_string() },
        Song { id: 3, title: "void (super slowed)".to_string(), artist: "isq".to_string(), duration_secs: 240, album_art_url: "".to_string() },
        Song { id: 4, title: "Can't Hold Me Down".to_string(), artist: "Doodles, Lil Wayne, Lil Yachty, Kyle Richh...".to_string(), duration_secs: 198, album_art_url: "".to_string() },
        Song { id: 5, title: "LOVELY BASTARDS x Meet the Fro...".to_string(), artist: "GOD OF SPEED".to_string(), duration_secs: 150, album_art_url: "".to_string() },
        Song { id: 6, title: "Steve Lacy - Dark Red (Lyrics)".to_string(), artist: "R&BHype".to_string(), duration_secs: 173, album_art_url: "".to_string() },
        Song { id: 7, title: "El Paso del Gigante".to_string(), artist: "Grupo Soñador de Beto Tlahuati, Alberto Tlahu...".to_string(), duration_secs: 200, album_art_url: "".to_string() },
    ];
    let mock_playlists = vec![
        Playlist { id: 1, name: "Default".to_string(), song_ids: vec![] },
        Playlist { id: 2, name: "YT".to_string(), song_ids: vec![1, 2, 3, 4, 5, 6, 7] },
        Playlist { id: 3, name: "Phonk".to_string(), song_ids: vec![] },
    ];

    // Initialize the shared application state
    let app_state = AppStateManager(Mutex::new(AppState {
        playlists: mock_playlists,
        songs: mock_songs,
        playback: PlaybackState {
            is_playing: false,
            current_song_id: Some(1),

           progress_secs: 148,
            volume: 0.75,
            is_shuffled: false,
            loop_mode: LoopMode::None,
        },
        active_playlist_id: 2,
    }));

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_initial_data,
            toggle_playback,
            cycle_loop_mode,
            toggle_shuffle,
            set_volume,
            select_playlist,
            select_song
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
