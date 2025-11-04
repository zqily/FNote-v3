# FNote Music Player

THIS IS JUST A PLACEHOLDER README

FNote is a simple, lightweight, cross-platform music player built with Tauri, React, and Rust. It's designed to be a minimal application for scanning and playing local music files.

## Features

-   **Local Music Library**: Scan a folder on your computer to import your music library.
-   **Wide Format Support**: Plays `.mp3`, `.wav`, and `.flac` files.
-   **Core Playback Controls**: Play, pause, and resume your music.
-   **Real-time Progress**: A visual progress bar shows the current track's elapsed time.
-   **Responsive UI**: A clean and simple interface built with React and Tailwind CSS.

## Tech Stack

-   **Framework**: [Tauri](https://tauri.app/) - A toolkit for building fast, secure, and cross-platform desktop apps with a web frontend.
-   **Backend**: [Rust](https://www.rust-lang.org/)
    -   `rodio` for audio playback.
    -   `id3` & `hound` for metadata parsing.
    -   `walkdir` for efficient directory scanning.
-   **Frontend**: [React](https://react.dev/)
    -   [TypeScript](https://www.typescriptlang.org/) for type safety.
    -   [Tailwind CSS](https://tailwindcss.com/) for styling.
    -   [Zustand](https://github.com/pmndrs/zustand) for global state management.
    -   [Lucide React](https://lucide.dev/) for icons.

## How It Works

The application is split into two main parts:

1.  **`src` (Frontend)**: The React application that renders the user interface. It communicates with the Rust backend via Tauri's API.
2.  **`src-tauri` (Backend)**: The Rust core that handles all the heavy lifting.
    -   **Commands**: Exposes functions that can be called from the frontend (e.g., `scan_music_directory`, `play_track`).
    -   **Player Thread**: Audio playback is handled in a dedicated background thread to ensure the UI remains smooth and responsive.
    -   **State Management**: Manages the application's core state, like the music library.
    -   **Events**: Pushes real-time updates (like playback status) from the Rust backend to the React frontend.

## Getting Started

### Prerequisites

-   [Node.js and npm](https://nodejs.org/)
-   [Rust and Cargo](https://www.rust-lang.org/tools/install)
-   [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your specific operating system.

### Installation & Running

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd FNote
    ```

2.  **Install frontend dependencies:**
    ```bash
    npm install
    ```

3.  **Run in development mode:**
    This will start the Vite dev server and the Tauri application in a single window.
    ```bash
    npm run tauri dev
    ```

4.  **Build the application:**
    To create a distributable, production-ready binary for your platform:
    ```bash
    npm run tauri build
    ```
    The executable will be located in `src-tauri/target/release/`.
