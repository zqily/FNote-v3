// src-tauri/src/player/mod.rs

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use tauri::Manager;

use crate::models::{PlaybackStatus, Track};

/// Defines the commands that can be sent to the player thread.
pub enum PlayerCommand {
    Play(Track),
    Pause,
    Resume,
}

/// The main function for the audio player thread.
/// It sets up an audio stream and then enters a loop to listen for commands.
pub fn run_player(app_handle: tauri::AppHandle, command_receiver: Receiver<PlayerCommand>) {
    // `rodio`'s `OutputStream` is the connection to the audio device.
    // It must be created and kept alive for the duration of the program.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // The `Sink` is the component that actually plays sounds. We can append sounds to it.
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut current_track: Option<Track> = None;
    let mut playback_start_time: Option<Instant> = None;
    let mut paused_at: f32 = 0.0;

    // This is the player's main loop. It checks for two things:
    // 1. New commands from the main application (e.g., from a Tauri command).
    // 2. The passage of time, to emit playback progress updates.
    loop {
        // Try to receive a command without blocking.
        if let Ok(command) = command_receiver.try_recv() {
            match command {
                PlayerCommand::Play(track) => {
                    // When a new track is played, stop the current one, clear the sink,
                    // and start playing the new file.
                    sink.stop();
                    let file = BufReader::new(File::open(&track.path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    sink.append(source);

                    current_track = Some(track.clone());
                    playback_start_time = Some(Instant::now());
                    paused_at = 0.0;
                    sink.play();
                }
                PlayerCommand::Pause => {
                    if !sink.is_paused() && playback_start_time.is_some() {
                        sink.pause();
                        paused_at = playback_start_time.unwrap().elapsed().as_secs_f32();
                    }
                }
                PlayerCommand::Resume => {
                    if sink.is_paused() {
                        sink.play();
                        // Adjust start time to account for the pause duration
                        playback_start_time = Some(Instant::now() - Duration::from_secs_f32(paused_at));
                    }
                }
            }
        }

        // Emit playback status updates every 250ms.
        let elapsed_secs = if sink.is_paused() {
            paused_at
        } else if let Some(start_time) = playback_start_time {
            if sink.empty() {
                // Track finished, reset state
                current_track = None;
                playback_start_time = None;
                paused_at = 0.0;
                0.0
            } else {
                start_time.elapsed().as_secs_f32()
            }
        } else {
            0.0
        };

        let status = PlaybackStatus {
            is_playing: sink.is_paused() == false && current_track.is_some(),
            current_track_id: current_track.as_ref().map(|t| t.id.clone()),
            duration_secs: current_track
                .as_ref()
                .map_or(0, |t| t.duration_secs),
            elapsed_secs,
        };

        // `app_handle.emit_all` sends an event to the frontend.
        // This is the primary way the backend pushes updates to the UI.
        app_handle.emit_all("playback_status_changed", status).unwrap();

        // Sleep to prevent this loop from consuming 100% CPU.
        std::thread::sleep(Duration::from_millis(250));
    }
}