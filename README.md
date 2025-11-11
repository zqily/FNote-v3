# FNote v3: The Lightweight Desktop Music Player

A modern, performant, and feature-rich cross-platform music player built for the desktop. FNote is designed to be fast, minimal, and secure, powered by Rust and Tauri.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-24C7E7.svg)](https://tauri.app/)
[![Frontend: SvelteKit](https://img.shields.io/badge/Frontend-SvelteKit-FF3E00.svg)](https://kit.svelte.dev/)
[![Current Status: Pre-Alpha](https://img.shields.io/badge/Status-Pre--Alpha-red)](https://github.com/yourusername/fnote)

---

## 📸 User Interface Preview

FNote v3 leverages the UI/UX design of its predecessor, FNote v2, providing a beautiful, focused experience in a dedicated desktop wrapper.

![FNote v3 UI Screenshot](/github/ui.png)

---

## 📜 The FNote Saga: Coded by AI

FNote v3 is the third (and hopefully final) iteration of a passion project started at the very beginning of my coding journey. What makes this project unique is its development process: **FNote has been built almost entirely using large language models (LLMs).**

After numerous attempts and valuable lessons learned, FNote v3 represents a commitment to stability, performance, and modern architecture.

### Version History

| Version | Core Technology | UI/Frontend | Notes | Story |
| :---: | :---: | :---: | :---: | :--- |
| **FNote v1** | Pure Python | Tkinter | My first major project; simple, but functional. Source code lost. | [Read the full story here](https://zqil.net/projects/fnote-v1) |
| **FNote v2** | Python (Pywebview) | Vanilla HTML/CSS/JS | Excellent UI/UX, but suffered from extreme instability, resource heaviness, and bugs. | [Read the full story here](https://zqil.net/projects/fnote-v2) |
| **FNote v3** | Rust (Tauri) | SvelteKit / TypeScript / Tailwind | **The modern, lightweight rewrite.** Focused on performance and security to fix the flaws of v2. | Active Development |

### Why Tauri?

The jump to **Tauri** was crucial. By compiling the application core to efficient, memory-safe Rust, FNote v3 aims to solve the heavy footprint and instability issues that plagued FNote v2, while retaining a rapid development workflow on the frontend with SvelteKit.

---

## 🚧 Current Status (Pre-Alpha)

This repository currently focuses solely on the architecture and frontend presentation.

**Status Summary:**
1.  **UI Complete:** The user interface components, based heavily on the FNote v2 design, are built with SvelteKit and Tailwind CSS.
2.  **IPC Wired:** The frontend is wired up to the Rust backend using Tauri commands (e.g., `toggle_playback`, `cycle_loop_mode`).
3.  **Data Mocked:** The Rust core holds mock data and state management via a Mutex (for `AppState`).

**There is currently no fully functional audio playback or file handling.** FNote v3 aims to achieve at least feature parity with the bug-ridden FNote v2, depending on the capabilities of current and future AI assistants.

---

## ⚙️ Tech Stack

FNote v3 leverages a modern and high-performance stack:

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Backend Core** | **Rust** | Security, performance, and low-level system interaction via Tauri. |
| **Application Shell** | **Tauri** | Cross-platform desktop bundling and native API access. |
| **Frontend Framework** | **SvelteKit** | Highly reactive and lightweight UI framework. |
| **Styling** | **Tailwind CSS** | Utility-first, clean, and modern design. |

---

## 🚀 Getting Started

To run FNote v3 locally, you need to have Rust, Node.js (with npm), and the required dependencies installed.

### Prerequisites

1.  [Rust](https://www.rust-lang.org/tools/install) (with `cargo`)
2.  [Node.js](https://nodejs.org/) (LTS recommended)
3.  Tauri prerequisites for your target OS (usually system build tools).

### Development Setup

```bash
# 1. Clone the repository
git clone https://github.com/zqily/FNote-v3.git
cd fnote

# 2. Install SvelteKit dependencies
npm install

# 3. Start the Tauri development server
npm run tauri
```

This command compiles the Rust backend and launches the SvelteKit frontend in the Tauri webview, complete with hot-reloading for faster frontend development.