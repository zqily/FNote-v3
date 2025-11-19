<div align="center">

  <img src="/github/fnote.png" alt="FNote Logo" width="180" height="auto" />
  
  <h1>FNote v3</h1>
  
  <p>
    <strong>Your Personal, Offline-First Music Library.</strong><br>
    Built for Creators, Curators, and You.
  </p>

  <p>
    <a href="#-user-interface-preview">View Demo</a> •
    <a href="#-getting-started">Getting Started</a> •
    <a href="#-the-fnote-saga-coded-by-ai">Read the Story</a>
  </p>

  <!-- Badges -->
  <p>
    <img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge&logo=none" alt="License" />
    <img src="https://img.shields.io/badge/Built%20with-Tauri-24C7E7.svg?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri" />
    <img src="https://img.shields.io/badge/Frontend-SvelteKit-FF3E00.svg?style=for-the-badge&logo=svelte&logoColor=white" alt="SvelteKit" />
    <img src="https://img.shields.io/badge/Status-Pre--Alpha-red?style=for-the-badge" alt="Status" />
  </p>

</div>

---

## 📖 About

**FNote** is a modern, performant, and feature-rich cross-platform music player built for the desktop. Designed to be **fast, minimal, and secure**, FNote leverages the power of Rust and the flexibility of web technologies to deliver a premium audio experience without the bloat.

### Key Features (Planned)
| ⚡️ Performance | 🔒 Security | 🎨 Experience |
| :--- | :--- | :--- |
| Native Rust backend for minimal memory footprint. | Offline-first architecture. Your data stays on your machine. | Beautiful, focused UI built with Tailwind CSS. |

---

## 📸 User Interface Preview

FNote v3 leverages the UI/UX design of its predecessor, providing a beautiful, focused experience in a dedicated desktop wrapper.

<div align="center">
  <img src="/github/fnote_screenshot.png" alt="FNote v3 UI Screenshot" width="100%" style="border-radius: 10px; box-shadow: 0 10px 30px rgba(0,0,0,0.5);">
</div>

---

## 📜 The FNote Saga: Coded by AI

> **🤖 AI-Native Development:** FNote v3 is the third iteration of a passion project started at the beginning of my coding journey. **This project has been built almost entirely using large language models (LLMs).**

After numerous attempts and valuable lessons learned, FNote v3 represents a commitment to stability, performance, and modern architecture.

### ⏳ Version History

| Version | Core Tech | Frontend | The Story |
| :--- | :--- | :--- | :--- |
| **FNote v1** | `Python` | `Tkinter` | *My first major project.* Simple, functional, but source code is lost. [Read Story](https://zqil.net/projects/fnote-v1) |
| **FNote v2** | `Python` | `Vanilla JS` | *The Designer.* Excellent UI/UX, but suffered from instability and resource heaviness. [Read Story](https://zqil.net/projects/fnote-v2) |
| **FNote v3** | `Rust` | `SvelteKit` | **The Modern Rewrite.** Focused on performance, security, and fixing the flaws of v2. |

### 🦀 Why Tauri?

The jump to **Tauri** was crucial. By compiling the application core to efficient, memory-safe **Rust**, FNote v3 aims to solve the heavy footprint and instability issues that plagued FNote v2, while retaining a rapid development workflow on the frontend with **SvelteKit**.

---

## 🚧 Current Status: Pre-Alpha

This repository currently focuses solely on the **architecture and frontend presentation**.

> [!WARNING]  
> **There is currently no fully functional audio playback or file handling.** 
> FNote v3 aims to achieve feature parity with the bug-ridden FNote v2, depending on the capabilities of current and future AI assistants.

**Development Milestones:**
- [x] **UI Complete:** Interface components based on v2 design (SvelteKit + Tailwind).
- [x] **IPC Wired:** Frontend connected to Rust backend via Tauri commands.
- [x] **Data Mocked:** Rust core holds mock data/state.
- [ ] **Audio Engine:** File streaming and playback controls.
- [ ] **Database:** Local library management.

---

## ⚙️ Tech Stack

FNote v3 leverages a modern, high-performance stack:

<div align="center">

| Layer | Technology | Icon | Purpose |
| :--- | :--- | :---: | :--- |
| **Backend** | **Rust** | <img src="https://skillicons.dev/icons?i=rust" width="20"> | Security, performance, and system interaction. |
| **Shell** | **Tauri** | <img src="https://skillicons.dev/icons?i=tauri" width="20"> | Cross-platform bundling and native APIs. |
| **Frontend** | **SvelteKit** | <img src="https://skillicons.dev/icons?i=svelte" width="20"> | Highly reactive and lightweight UI. |
| **Styling** | **Tailwind** | <img src="https://skillicons.dev/icons?i=tailwind" width="20"> | Utility-first modern design. |

</div>

---

## 🚀 Getting Started

To run FNote v3 locally, ensure you have the prerequisites installed.

### Prerequisites
*   [Rust](https://www.rust-lang.org/tools/install) (via `cargo`)
*   [Node.js](https://nodejs.org/) (LTS recommended)
*   Build tools for your OS (C++ compilers, etc.)

### Installation & Run

```bash
# 1. Clone the repository
git clone https://github.com/zqily/FNote-v3.git
cd FNote-v3

# 2. Install SvelteKit dependencies
npm install

# 3. Start the Tauri development server
# This launches the Rust backend and the SvelteKit frontend with hot-reloading
npm run tauri
```

---

<div align="center">
Made with ❤️ and 🤖 by <a href="https://github.com/zqily">Zqil</a>
</div>