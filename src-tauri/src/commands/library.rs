// src-tauri/src/commands/library.rs

use crate::{models::Track, state::AppState};
use std::path::Path;
use tauri::State;
use walkdir::WalkDir;

/// A Tauri command that scans a directory for music files, extracts metadata,
/// and returns a list of `Track` objects.
///
/// # Arguments
/// * `path` - The directory path to scan.
/// * `app_state` - The Tauri-managed application state.
///
/// # Returns
/// A `Result` containing either the vector of tracks or an error string.
#[tauri::command]
pub async fn scan_music_directory(
    path: String,
    app_state: State<'_, AppState>,
) -> Result<Vec<Track>, String> {
    let mut tracks = Vec::new();
    let supported_extensions = ["mp3", "wav", "flac"];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore entries with errors
    {
        let path = entry.path();
        if path.is_file() {
            let extension = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_lowercase();

            if supported_extensions.contains(&extension.as_str()) {
                if let Some(track) = parse_metadata(path) {
                    tracks.push(track);
                }
            }
        }
    }

    // Lock the state to update the library. The lock is released when `library` goes out of scope.
    let mut library = app_state.library.lock().unwrap();
    *library = tracks.clone();

    Ok(tracks)
}

/// Helper function to parse metadata from a single audio file.
fn parse_metadata(path: &Path) -> Option<Track> {
    let path_str = path.to_string_lossy().to_string();
    let id = blake3::hash(path_str.as_bytes()).to_hex().to_string();

    // Default values
    let mut title = path.file_stem()?.to_string_lossy().to_string();
    let mut artist = "Unknown Artist".to_string();
    let mut album = "Unknown Album".to_string();
    let mut duration_secs = 0;

    // Try to parse ID3 tags for MP3 files
    if let Ok(tag) = id3::Tag::read_from_path(path) {
        title = tag.title().unwrap_or(&title).to_string();
        artist = tag.artist().unwrap_or(&artist).to_string();
        album = tag.album().unwrap_or(&album).to_string();
        duration_secs = tag.duration().unwrap_or(0);
    } else if let Ok(reader) = hound::WavReader::open(path) {
        // Fallback for WAV file duration
        let spec = reader.spec();
        duration_secs = reader.duration() / spec.sample_rate;
    }
    // Note: You would add other metadata parsers here (e.g., for FLAC).

    Some(Track { id, path: path_str, title, artist, album, duration_secs })
}