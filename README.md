<p align="center">
  <img src="assaset/logoo.png" alt="RTX YTDownloader Logo" width="800px">
</p>

# 🚀 RTX YTDownloader Pro v3.0
![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)

**RTX YTDownloader Pro** is a high-performance, lightweight YouTube media downloader designed for Linux. Built with **Rust** and **Tauri**, it offers a seamless experience for extracting high-quality audio with a modern, futuristic interface.

---

## 💎 Why RTX YTDownloader?

Most YouTube downloaders require you to manually install complex tools like FFmpeg or yt-dlp. **RTX YTDownloader Pro comes with everything built-in.**

* ✅ **No FFmpeg Installation Required:** The encoding engine is bundled inside the app.
* ✅ **No yt-dlp Installation Required:** The extraction core is pre-configured as a sidecar.
* ✅ **Zero Dependency for Users:** Just download the AppImage and start downloading. No terminal commands needed for setup!

---

## ✨ Key Features

- **Built-in Engines:** Integrated `yt-dlp` and `ffmpeg` sidecars for out-of-the-box functionality.
- **Batch Downloading:** Add multiple YouTube links using the `+` button and process them all at once.
- **Glassmorphic UI:** A futuristic 2026-style interface with real-time progress tracking.
- **Integrated Terminal:** Monitor background processes and system logs through the built-in console.
- **Resource Efficient:** Built with Rust to ensure minimal RAM and CPU usage compared to Electron apps.
- **Fluid & Responsive:** The layout adapts perfectly to any window size.

---

## 🛠️ Tech Stack & Libraries

This project leverages cutting-edge technology to ensure stability and performance:

* **Backend:** [Rust](https://www.rust-lang.org/) (Core Logic & Process Management)
* **Frontend:** HTML5, Modern CSS3 (Glassmorphism), Vanilla JavaScript
* **Framework:** [Tauri v2.0](https://v2.tauri.app/) (Desktop Bridge)
* **Core Libraries (Rust):**
    * `tauri-plugin-shell`: To manage internal sidecar processes.
    * `tauri-plugin-dialog`: For native folder selection.
    * `serde`: For high-speed data serialization.
* **Internal Engines (Sidecars):**
    * `yt-dlp`: Bundled high-performance media extraction engine.
    * `ffmpeg`: Bundled industry-standard multimedia framework for high-quality encoding.

---

## 📦 How to Build from Source

If you want to compile the project yourself on Linux, follow these steps:

### 1. Prerequisites
Ensure you have the Rust toolchain and the necessary system libraries installed:

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```
### 2. Sidecar Setup:
* **Place static binaries in src-tauri/binaries/ and rename them:**

* `yt-dlp-x86_64-unknown-linux-gnu`
* `ffmpeg-x86_64-unknown-linux-gnu`

`(Set permissions: chmod +x src-tauri/binaries/*)`

### 3. Build the AppImage:
```bash
git clone https://github.com/00sanoj00/RTX-YTDownloader-Pro.git
cd RTX-YTDownloader-Pro
cargo tauri build
```
## ⚠️ Current Limitations

* `Audio Optimization:` Specifically optimized for high-quality MP3 extraction.
* `Platform:` Primarily optimized for Linux (AppImage).

## 🗺️ Roadmap

* **Support for 4K/8K Video (.mp4)**
* **Support for full Playlists**
* **Dark/Light mode toggle**

## 📜 License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) file for the full license text.

**Developed with ❤️ by 00sanoj00**
