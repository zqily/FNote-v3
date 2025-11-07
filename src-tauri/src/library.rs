// Handles file system operations: scanning, metadata parsing, etc.
use crate::models::Song;
use anyhow::Result;
use lofty::{Probe, TaggedFileExt};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_directory(path: &Path) -> Result<Vec<Song>> {
    let mut songs = Vec::new();
    let supported_extensions = ["mp3", "flac", "wav", "m4a", "ogg"];

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                    if let Ok(Some(song)) = parse_metadata(path, songs.len()) {
                        songs.push(song);
                    }
                }
            }
        }
    }
    Ok(songs)
}

fn parse_metadata(path: &Path, id: usize) -> Result<Option<Song>> {
    let tagged_file = Probe::open(path)?.read()?;
    let properties = tagged_file.properties();
    let duration_ms = properties.duration().as_millis() as u64;

    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());

    let (title, artist, album) = if let Some(t) = tag {
        (
            t.title().as_deref().unwrap_or("Unknown Title").to_string(),
            t.artist().as_deref().unwrap_or("Unknown Artist").to_string(),
            t.album().as_deref().unwrap_or("Unknown Album").to_string(),
        )
    } else {
        (
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            "Unknown Artist".to_string(),
            "Unknown Album".to_string(),
        )
    };

    Ok(Some(Song {
        id,
        path: path.to_string_lossy().to_string(),
        title,
        artist,
        album,
        duration_ms,
    }))
}

pub fn get_album_art_data(path_str: &str) -> Result<Option<String>> {
    let path = PathBuf::from(path_str);
    let tagged_file = Probe::open(path)?.read()?;

    if let Some(picture) = tagged_file.primary_tag().and_then(|t| t.pictures().get(0)) {
        let mime_type = picture.mime_type().to_string();
        let data = base64::encode(picture.data());
        let base64_string = format!("data:{};base64,{}", mime_type, data);
        return Ok(Some(base64_string));
    }

    Ok(None)
}
