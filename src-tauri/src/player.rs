use crate::{
    library,
    models::{PlayerStatusUpdate, RepeatMode, Song},
    state::AppState,
};
use rand::seq::SliceRandom;
use rodio::{Decoder, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

pub fn get_current_time_ms(state: &AppState) -> u64 {
    if state.is_playing {
        let elapsed_since_start = state
            .playback_start_instant
            .map_or(0, |instant| instant.elapsed().as_millis() as u64);
        state.elapsed_ms + elapsed_since_start
    } else {
        state.elapsed_ms
    }
}

fn emit_status_update(app_handle: &AppHandle, state: &AppState) {
    let song_duration_ms = state.current_song_id
        .and_then(|id| state.songs.get(id))
        .map_or(0, |s| s.duration_ms);

    let mut current_time_ms = get_current_time_ms(state);
    if song_duration_ms > 0 {
        current_time_ms = current_time_ms.min(song_duration_ms);
    }
    
    let status = PlayerStatusUpdate {
        songs: state.songs.clone(),
        current_song_id: state.current_song_id,
        is_playing: state.is_playing && !state.sink.is_paused(),
        volume: state.volume,
        is_shuffled: state.is_shuffled,
        repeat_mode: state.repeat_mode,
        current_time_ms,
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

fn start_playback_from(
    id: usize,
    position_ms: u64,
    state: &mut AppState,
) -> Result<(), String> {
    let song = state
        .songs
        .get(id)
        .ok_or_else(|| "Song ID out of bounds".to_string())?
        .clone();

    let file = File::open(&song.path).map_err(|e| e.to_string())?;
    let source_decoder = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    // Create a new source that skips to the desired position
    let source_with_offset = source_decoder.skip_duration(Duration::from_millis(position_ms));

    state.sink.stop();
    let new_sink = Sink::try_new(&state.stream_handle).map_err(|e| e.to_string())?;
    state.sink = new_sink;
    state.sink.set_volume(state.volume);
    state.sink.append(source_with_offset);

    state.current_song_id = Some(id);
    state.is_playing = true;
    state.elapsed_ms = position_ms;
    state.playback_start_instant = Some(Instant::now());
    state.sink.play();

    Ok(())
}

pub fn play_song(
    id: usize,
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    start_playback_from(id, 0, state)?;
    if let Some(song) = state.songs.get(id) {
        emit_album_art_update(app_handle, song);
    }
    emit_status_update(app_handle, state);
    Ok(())
}

pub fn toggle_playback(state: &mut AppState, app_handle: &AppHandle) {
    if state.current_song_id.is_some() {
        if state.sink.is_paused() { // Resuming playback
            state.sink.play();
            state.is_playing = true;
            state.playback_start_instant = Some(Instant::now());
        } else { // Pausing playback
            state.sink.pause();
            state.is_playing = false;
            let elapsed_since_start = state.playback_start_instant.map_or(0, |i| i.elapsed().as_millis() as u64);
            state.elapsed_ms += elapsed_since_start;
            state.playback_start_instant = None;
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
        // Stop playback and reset state if no next song
        state.sink.stop();
        state.is_playing = false;
        state.current_song_id = None;
        state.elapsed_ms = 0;
        state.playback_start_instant = None;
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
    position_ms: u64,
    state: &mut AppState,
    app_handle: &AppHandle,
) -> Result<(), String> {
    if let Some(current_id) = state.current_song_id {
        let song_duration = state.songs.get(current_id).map_or(0, |s| s.duration_ms);
        let seek_position_ms = position_ms.min(song_duration);
        let seek_duration = Duration::from_millis(seek_position_ms);

        if state.sink.try_seek(seek_duration).is_ok() {
            // If seek is successful, update our manual time tracking state
            state.elapsed_ms = seek_position_ms;
            if state.is_playing {
                // Reset the instant from which we measure elapsed time
                state.playback_start_instant = Some(Instant::now());
            }
        } else {
            // Fallback for when seek is not supported or fails
            eprintln!("Native seek failed. Falling back to recreating the audio sink.");
            let was_playing = state.is_playing;
            start_playback_from(current_id, seek_position_ms, state)?;
            if !was_playing {
                state.sink.pause();
                state.is_playing = false;
                state.playback_start_instant = None;
            }
        }
    }
    
    // Emit an update immediately to make the UI feel responsive
    emit_status_update(app_handle, state);
    Ok(())
}

pub fn toggle_shuffle(state: &mut AppState, app_handle: &AppHandle) {
    state.is_shuffled = !state.is_shuffled;
    emit_status_update(app_handle, state);
}

pub fn toggle_repeat_mode(state: &mut AppState, app_handle: &AppHandle) {
    state.repeat_mode = match state.repeat_mode {
        RepeatMode::Off => RepeatMode::Playlist,
        RepeatMode::Playlist => RepeatMode::Single,
        RepeatMode::Single => RepeatMode::Off,
    };
    emit_status_update(app_handle, state);
}

pub fn handle_track_finished(state: &mut AppState, app_handle: &AppHandle) {
    match state.repeat_mode {
        RepeatMode::Off => {
            state.sink.stop();
            state.is_playing = false;
            state.current_song_id = None;
            state.elapsed_ms = 0;
            state.playback_start_instant = None;
            emit_status_update(app_handle, state);
        }
        RepeatMode::Playlist => {
            if let Err(e) = next_song(state, app_handle) {
                eprintln!("Error playing next song automatically: {}", e);
            }
        }
        RepeatMode::Single => {
            if let Some(id) = state.current_song_id {
                if let Err(e) = play_song(id, state, app_handle) {
                    eprintln!("Error re-playing song: {}", e);
                }
            } else {
                // Behave like Off if there's no song for some reason
                state.sink.stop();
                state.is_playing = false;
                state.current_song_id = None;
                state.elapsed_ms = 0;
                state.playback_start_instant = None;
                emit_status_update(app_handle, state);
            }
        }
    }
}
