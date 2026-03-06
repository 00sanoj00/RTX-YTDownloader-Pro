use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_shell::ShellExt;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    id: usize,
    percentage: String,
    filename: String,
    thumbnail: String,
}

#[tauri::command]
async fn download_mp3(id: usize, url: String, save_path: String, app: AppHandle, window: Window) -> Result<String, String> {
    let shell = app.shell();

    // 1. Title සහ Thumbnail URL එක ලබා ගැනීම
    let info_cmd = shell.sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(&["--get-filename", "--get-thumbnail", "-o", "%(title)s", &url]);
    
    let output = info_cmd.output().await.map_err(|e| e.to_string())?;
    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Warning එක නැති කිරීමට 'mut' අයින් කරන ලදී
    let lines: Vec<&str> = out_str.lines().filter(|s| !s.is_empty()).collect();
    
    let mut video_title = format!("Video {}", id + 1);
    let mut thumb_url = String::new();

    for line in lines {
        if line.starts_with("http") {
            thumb_url = line.to_string();
        } else {
            video_title = line.to_string();
        }
    }

    let ffmpeg_sidecar_path = app.path().resolve("binaries/ffmpeg-x86_64-unknown-linux-gnu", tauri::path::BaseDirectory::Resource)
    .map_err(|e| e.to_string())?;
    // 2. සැබෑ ඩවුන්ලෝඩ් එක ආරම්භ කිරීම
    let sidecar_command = shell.sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar error: {}", e))?
        .args(&[
            "-x", "--audio-format", "mp3",
            "--ffmpeg-location", &ffmpeg_sidecar_path.to_string_lossy(),
            "--newline",
            "--progress-template", "PROG:%(progress._percent_str)s",
            "-o", &format!("{}/%(title)s.%(ext)s", save_path),
            &url
        ]);

    let (mut rx, _child) = sidecar_command.spawn()
        .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

    let title_clone = video_title.clone();
    let thumb_clone = thumb_url.clone();

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes).to_string();

                                    // Terminal එකට මුළු පේළියම යවනවා
                    let _ = window.emit("log-event", line.clone());

                    if line.contains("PROG:") {
                        let pct = line.replace("PROG:", "").replace("%", "").trim().to_string();
                        let _ = window.emit("progress-event", ProgressPayload {
                            id,
                            percentage: pct,
                            filename: title_clone.clone(),
                            thumbnail: thumb_clone.clone(),
                        });
                    }
                },
                tauri_plugin_shell::process::CommandEvent::Terminated(_) => {
                    let _ = window.emit("download-finished", ProgressPayload {
                        id,
                        percentage: "100".into(),
                        filename: title_clone.clone(),
                        thumbnail: thumb_clone.clone(),
                    });
                },
                _ => {}
            }
        }
    });

    Ok(video_title)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![download_mp3])
        .setup(|app| {
            // Splash screen labels දැන් tauri.conf.json එකට සමානයි
            let splash_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(3));
                let _ = splash_window.close();
                let _ = main_window.show();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}