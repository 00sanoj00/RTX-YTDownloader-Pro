<p align="center">
  <img src="assaset/logoo.png" alt="RTX YTDownloader Logo" width="800px">
</p>

# 🚀 RTX YTDownloader Pro

![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)
![Platform: Linux](https://img.shields.io/badge/Platform-Linux-orange.svg)
![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri%20v2-blue.svg)
![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)

**RTX YTDownloader Pro** is a high-performance, lightweight YouTube media downloader designed for Linux. Built with **Rust** and **Tauri**, it offers a seamless experience for downloading high-quality audio and video with a modern, futuristic interface.

---

## 💎 Why RTX YTDownloader?

Most YouTube downloaders require you to manually install complex tools like FFmpeg or yt-dlp. **RTX YTDownloader Pro comes with everything built-in.**

* ✅ **No FFmpeg Installation Required:** Bundled inside the app and auto-extracted on first launch.
* ✅ **No yt-dlp Installation Required:** Pre-bundled extraction core, ready out of the box.
* ✅ **Zero Dependency for Users:** Just download the AppImage or .deb and start downloading. No terminal commands needed!
* ✅ **Smart First-Run Setup:** Binaries are automatically extracted to `~/.local/share/rtx-ytdown/bins/` on first launch and reused on every subsequent run.

---

## ✨ Key Features

- **Built-in Engines:** `yt-dlp` and `ffmpeg` are bundled inside this and auto-extracted on first run.
- **Multi-Format Support:** Download as **MP3**, **MP4**, **MKV**, or **AVI** with a single click.
- **Batch Downloading:** Add multiple YouTube links using the `+` button and process them all simultaneously.
- **Smart Folder Picker:** Built-in folder navigator with the ability to create new folders directly from the UI.
- **Glassmorphic UI:** A futuristic 2026-style interface with real-time progress tracking per download.
- **Integrated Terminal:** Monitor background processes and system logs through the built-in console.
- **Resource Efficient:** Built with Rust — minimal RAM and CPU usage compared to Electron-based apps.
- **Splashscreen Initialization:** Binaries are verified and extracted during the splash screen before the main window opens.

---

## 🖥️ Supported Output Formats

| Format | Description |
|--------|-------------|
| 🎵 MP3 | Best quality audio extraction |
| 🎬 MP4 | Best quality video + audio |
| 🎥 MKV | Best quality video + audio (Matroska) |
| 📺 AVI | Best quality video + audio |

---

## 🛠️ Tech Stack & Libraries

* **Backend:** [Rust](https://www.rust-lang.org/) (Core Logic & Process Management)
* **Frontend:** HTML5, Modern CSS3 (Glassmorphism), Vanilla JavaScript
* **Framework:** [Tauri v2.0](https://v2.tauri.app/) (Desktop Bridge)
* **Core Libraries (Rust):**
    * `tauri-plugin-shell` — Direct binary process management
    * `tauri-plugin-dialog` — Native folder selection dialogs
    * `tauri-plugin-fs` — Filesystem access
    * `serde` — High-speed data serialization
    * `zip` — Runtime binary extraction from bundled zip
    * `home` — Cross-platform home directory resolution
* **Bundled Engines:**
    * `yt-dlp` — High-performance media extraction engine
    * `ffmpeg` (static build) — Industry-standard multimedia framework with full codec support

---

## 📦 How to Build from Source

### 1. Prerequisites
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
npm install -g @tauri-apps/cli
```

### 2. Binary Setup

Download a **static build** of ffmpeg (single binary, no dependencies):
```bash
wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz
tar -xf ffmpeg-release-amd64-static.tar.xz
```

Place binaries and create the zip:
```bash
mkdir -p src-tauri/resources
mkdir -p bins/ffmpeg

# Copy ffmpeg static files
cp ffmpeg-*-static/ffmpeg bins/ffmpeg/
cp ffmpeg-*-static/ffprobe bins/ffmpeg/

# Download yt-dlp
wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -O bins/yt-dlp
chmod +x bins/yt-dlp bins/ffmpeg/ffmpeg bins/ffmpeg/ffprobe

# zip
cd bins && zip -r ../src-tauri/resources/bins.zip ffmpeg/ yt-dlp
cd ..
```

### 3. Build 
```bash
git clone https://github.com/00sanoj00/RTX-YTDownloader-Pro.git
cd RTX-YTDownloader-Pro
cargo tauri build
```

---

## 🔧 Development & Testing
```bash
# Normal dev mode
cargo tauri dev

# Test mode - isolated test without detecting system ffmpeg/yt-dlp
RTX_TEST_MODE=1 cargo tauri dev

# Clear test cache.
rm -rf /tmp/rtx-test-bins
```

---

## ⚠️ Current Limitations

* `Platform:` Primarily optimized for Linux (AppImage).
* `Playlists:` Single video URLs only (playlist support coming soon).

---

## 🗺️ Roadmap

* **Playlist Support**
* **4K/8K Video Download**
* **Download Queue Management**
* **Dark/Light Mode Toggle**
* **Windows & macOS Support**
* **Download Speed Limiter**

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) file for details.

---

**Developed with ❤️ by 00sanoj00**
