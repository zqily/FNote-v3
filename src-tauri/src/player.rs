use crate::{
    library,
    models::{PlayerStatusUpdate, Song},
    state::AppState,
};
use rand::seq::SliceRandom;
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use tauri::{AppHandle, Manager};

fn emit_status_update(app_handle: &AppHandle, state: &AppState) {
    let status = PlayerStatusUpdate {
        songs: state.songs.clone(),
        current_song_id: state.current_song_id,
        is_playing: state.is_playing && !state.sink.is_paused(),
        volume: state.volume,
        is_shuffled: state.is_shuffled,
        current_time_ms: 0, // rodio 0.17 doesn't support get_pos on Sink
    };
    app_handle.emit_all("player://status-update", status).unwrap();
}

fn emit_album_art_update(app_handle: &AppHandle, song: &Song) {
    match library::get_album_art_data(&song.path) {
        Ok(Some(art)) => {
            app_handle
                .emit_all("player://album-art-update", art)
                .unwrap();
        }
        _ => {
            app_handle
                .emit_all("player://album-art-update", Option::<String>::None)
                .unwrap();
        }
    }
}

fn play_song_internal(id: usize, state: &mut AppState) -> Result<(), String> {
    let song = state
        .songs
        .get(id)
        .ok_or_else(|| "Song ID out of bounds".to_string())?
        .clone();

    let file = File::open(&song.path).map_err(|e| e.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    state.sink.stop();
    let new_sink = Sink::try_new(&state.stream_handle).map_err(|e| e.to_string())?;
    state.sink = new_sink;
    state.sink.set_volume(state.volume);
    state.sink.append(source);

    state.current_song_id = Some(id);
    state.is_playing = true;
    state.sink.play();

    Ok(())
}

pub fn play_song(
    id: usize,
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    play_song_internal(id, state)?;
    if let Some(song) = state.songs.get(id) {
        emit_album_art_update(app_handle, song);
    }
    emit_status_update(app_handle, state);
    Ok(())
}

pub fn toggle_playback(state: &mut AppState, app_handle: &AppHandle) {
    if state.current_song_id.is_some() {
        if state.sink.is_paused() {
            state.sink.play();
            state.is_playing = true;
        } else {
            state.sink.pause();
            state.is_playing = false;
        }
    }
    emit_status_update(app_handle, state);
}

fn get_next_song_id(state: &AppState) -> Option<usize> {
    if state.songs.is_empty() {
        return None;
    }

    if state.is_shuffled {
        return (0..state.songs.len())
            .collect::<Vec<usize>>()
            .choose(&mut rand::thread_rng())
            .cloned();
    }

    state
        .current_song_id
        .map(|id| (id + 1) % state.songs.len())
        .or(Some(0))
}

fn get_prev_song_id(state: &AppState) -> Option<usize> {
    if state.songs.is_empty() {
        return None;
    }

    if state.is_shuffled {
        return get_next_song_id(state);
    }

    state
        .current_song_id
        .map(|id| {
            if id == 0 {
                state.songs.len() - 1
            } else {
                id - 1
            }
        })
        .or(Some(state.songs.len() - 1))
}

pub fn next_song(state: &mut AppState, app_handle: &AppHandle) -> Result<(), String> {
    if let Some(next_id) = get_next_song_id(state) {
        play_song(next_id, state, app_handle)?;
    } else {
        // Stop playback if no next song
        state.sink.stop();
        state.is_playing = false;
        state.current_song_id = None;
        emit_status_update(app_handle, state);
    }
    Ok(())
}

pub fn prev_song(state: &mut AppState, app_handle: &AppHandle) -> Result<(), String> {
    if let Some(prev_id) = get_prev_song_id(state) {
        play_song(prev_id, state, app_handle)?;
    }
    Ok(())
}

pub fn set_volume(volume: f32, state: &mut AppState, app_handle: &AppHandle) {
    state.volume = volume.clamp(0.0, 1.0);
    state.sink.set_volume(state.volume);
    emit_status_update(app_handle, state);
}

pub fn seek_to(
    _position_ms: u64,
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    // Seeking and getting playback position is not directly supported on rodio v0.17's Sink.
    // A major refactor would be needed to manage sources to allow for seeking.
    // For now, this is a no-op that just updates the state.
    emit_status_update(app_handle, state);
    Ok(())
}

pub fn toggle_shuffle(state: &mut AppState, app_handle: &AppHandle) {
    state.is_shuffled = !state.is_shuffled;
    emit_status_update(app_handle, state);
}
