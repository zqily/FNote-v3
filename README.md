# FNote (v3)

A fast, cross-platform, local music player built using Tauri (Rust + SvelteKit).

[![Release Status](https://img.shields.io/badge/status-Alpha%20Development-red)](https://github.com/zqily/FNote-v3/tree/svelte-test)
[![Backend](https://img.shields.io/badge/backend-Rust%20(Tauri)-orange.svg)](https://www.rust-lang.org/)
[![Frontend](https://img.shields.io/badge/frontend-SvelteKit%20%7C%20Tailwind-ff3e00.svg)](https://svelte.dev/)
[![Project Type](https://img.shields.io/badge/type-Solo%20Hobby%20Project-blue)]()

## 🚀 Overview

FNote is a hobby project dedicated to creating a highly optimized and stable desktop music player for local libraries. This is the third, and most promising, iteration of the FNote series, leveraging the performance and efficiency of **Rust** via the **Tauri** framework alongside a modern **SvelteKit** frontend.

The application is currently in active **Alpha development**.

## 📸 Screenshot

The core interface, SUBJECT TO A COMPLETE OVERHAUL:

![FNote User Interface Screenshot](/github/ui.webp)

## ✨ Features

FNote currently supports core functionality required for local playback:

*   **Library Management:** Scan local folders for supported audio files (MP3, FLAC, WAV, M4A, OGG).
*   **Metadata Support:** Reads and displays song title, artist, album, and duration.
*   **Playback Controls:** Play, Pause, Next, Previous, and Volume adjustment.
*   **Seeking:** Accurate seeking within the current track.
*   **Playback Modes:** Shuffle, and Repeat (Off, Playlist, Single).
*   **Album Art:** Extracts and displays embedded album artwork.

## 🛠️ Getting Started (Alpha Testing)

Since FNote is still in Alpha, there are no stable release binaries. To test the application, you must build it from the source code.

### Prerequisites

1.  **Rust:** Install Rust and Cargo.
2.  **Node.js:** Install Node.js (which includes npm).
3.  **Tauri Dependencies:** Ensure you have the necessary system dependencies for Tauri builds on your specific OS (check the [Tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites) for details).

### Installation & Run

Development is taking place on the `svelte-test` branch.

1.  **Clone the Repository:**
    ```bash
    git clone https://github.com/zqily/FNote-v3.git
    cd FNote-v3
    git checkout svelte-test
    ```

2.  **Install Frontend Dependencies:**
    ```bash
    npm install
    ```

3.  **Run in Development Mode:**
    ```bash
    npm run tauri dev
    ```
This command will compile the Rust backend and launch the SvelteKit frontend, running the application window for testing.

## ⚠️ Project History & Disclaimer

FNote v3 represents a full restart and refactoring effort. Previous versions (FNote v1 and FNote v2) are **highly discouraged** for use. They are extremely unstable, poorly optimized, severely outdated, and are maintained only for historical context. Source code for those versions is unavailable.

*   [FNote v1 (Historical)](https://github.com/zqily/FNote-v1)
*   [FNote v2 (Historical & Highly Unstable)](https://github.com/zqily/FNote-v2)

This third iteration aims to finally become a stable, high-performance desktop player.
