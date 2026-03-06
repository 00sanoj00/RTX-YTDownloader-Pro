use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_shell::ShellExt;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::Duration;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    id: usize,
    percentage: String,
    filename: String,
    thumbnail: String,
}

#[derive(Serialize)]
struct FolderEntry {
    name: String,
    path: String,
}

fn sanitize_filename(name: String) -> String {
    name.replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
        .replace("'", "_")
        .replace("&", "_")
        .replace("#", "_")
}

// =============================
// Binary extract path
// =============================

fn get_bin_extract_path() -> PathBuf {
    let home = home::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    home.join(".local").join("share").join("rtx-ytdown").join("bins")
}

// =============================
// Ensure binaries extracted
// =============================

async fn ensure_binaries(
    app: &tauri::AppHandle,
    main_window: &tauri::WebviewWindow,
) -> Result<(String, String), String> {

    let extract_dir = get_bin_extract_path();
    let ffmpeg_bin = extract_dir.join("ffmpeg").join("ffmpeg");
    let ytdlp_bin = extract_dir.join("yt-dlp");

    // දෙකම තියෙනවා නම් skip
    if ffmpeg_bin.exists() && ytdlp_bin.exists() {
        let _ = main_window.emit("log-event", "✅ Binaries already extracted, skipping...");
        return Ok((
            ffmpeg_bin.to_string_lossy().to_string(),
            ytdlp_bin.to_string_lossy().to_string(),
        ));
    }

    let _ = main_window.emit("log-event", "📦 First run - extracting binaries...");

    let zip_path = app.path()
        .resolve("resources/bins.zip", tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("zip not found: {}", e))?;

    fs::create_dir_all(&extract_dir)
        .map_err(|e| format!("dir create error: {}", e))?;

    let zip_file = fs::File::open(&zip_path)
        .map_err(|e| format!("zip open error: {}", e))?;

    let mut archive = zip::ZipArchive::new(zip_file)
        .map_err(|e| format!("zip read error: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("zip entry error: {}", e))?;

        let fname = file.name().to_string();

        // folder entries skip කරනවා
        if fname.ends_with('/') {
            continue;
        }

        // bins/ffmpeg/* සියල්ල extract කරනවා
        // bins/yt-dlp extract කරනවා
        let out_path = if fname.starts_with("bins/ffmpeg/") {
            // bins/ffmpeg/model/xxx → extract_dir/ffmpeg/model/xxx
            let relative = fname.strip_prefix("bins/").unwrap();
            Some(extract_dir.join(relative))

        } else if fname == "bins/yt-dlp" {
            Some(ytdlp_bin.clone())

        } else {
            None
        };

        if let Some(out) = out_path {
            // parent dirs හදනවා
            if let Some(parent) = out.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("mkdir error: {}", e))?;
            }

            let mut outfile = fs::File::create(&out)
                .map_err(|e| format!("file create error: {}", e))?;

            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("extract error: {}", e))?;
        }
    }

    // verify
    if !ffmpeg_bin.exists() {
        return Err("❌ ffmpeg extract failed".into());
    }
    if !ytdlp_bin.exists() {
        return Err("❌ yt-dlp extract failed".into());
    }

    // permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&ffmpeg_bin, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("ffmpeg chmod error: {}", e))?;
        fs::set_permissions(&ytdlp_bin, fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("yt-dlp chmod error: {}", e))?;
    }

    let _ = main_window.emit("log-event", "✅ Binaries extracted!");

    Ok((
        ffmpeg_bin.to_string_lossy().to_string(),
        ytdlp_bin.to_string_lossy().to_string(),
    ))
}

// =============================
// Download Media
// =============================

#[tauri::command]
async fn download_media(
    id: usize,
    url: String,
    save_path: String,
    format_type: String,
    app: AppHandle,
    window: Window
) -> Result<String, String> {

    // cache path එකෙන් binary paths ගන්නවා
    let bin_dir = get_bin_extract_path();
    let ffmpeg_loc = bin_dir.join("ffmpeg").join("ffmpeg").to_string_lossy().to_string();
    let ytdlp_loc = bin_dir.join("yt-dlp").to_string_lossy().to_string();

    let shell = app.shell();

    // title + thumbnail ගන්නවා
    let info_cmd = shell.command(&ytdlp_loc)
        .args(&["--get-filename", "--get-thumbnail", "-o", "%(title)s", &url]);

    let output = info_cmd.output().await.map_err(|e| e.to_string())?;
    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let mut video_title = format!("Video {}", id + 1);
    let mut thumb_url = String::new();

    for line in out_str.lines() {
        if line.starts_with("http") {
            thumb_url = line.to_string();
        } else if !line.trim().is_empty() {
            video_title = line.to_string();
        }
    }

    video_title = sanitize_filename(video_title);

    let output_template = format!("{}/{}.%(ext)s", save_path, video_title);

    let mut args: Vec<String> = vec![
        "--ffmpeg-location".into(),
        ffmpeg_loc.clone(),
        "--newline".into(),
        "--progress-template".into(),
        "PROG:%(progress._percent_str)s".into(),
        "-o".into(),
        output_template,
    ];

    match format_type.as_str() {
        "mp3" => {
            args.push("-x".into());
            args.push("--audio-format".into());
            args.push("mp3".into());
            args.push("--audio-quality".into());
            args.push("0".into());
        }
        "mp4" => {
            args.push("-f".into());
            args.push("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]".into());
            args.push("--merge-output-format".into());
            args.push("mp4".into());
        }
        "mkv" => {
            args.push("-f".into());
            args.push("bestvideo+bestaudio".into());
            args.push("--merge-output-format".into());
            args.push("mkv".into());
        }
        "avi" => {
            args.push("-f".into());
            args.push("bestvideo+bestaudio".into());
            args.push("--merge-output-format".into());
            args.push("avi".into());
        }
        _ => return Err("Unsupported format".into()),
    }

    args.push(url.clone());

    // yt-dlp direct path spawn කරනවා
    let (mut rx, _child) = shell.command(&ytdlp_loc)
        .args(&args)
        .spawn()
        .map_err(|e| format!("Spawn error: {}", e))?;

    let title_clone = video_title.clone();
    let thumb_clone = thumb_url.clone();

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::process::CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes).to_string();
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
                }
                tauri_plugin_shell::process::CommandEvent::Terminated(_) => {
                    let _ = window.emit("download-finished", ProgressPayload {
                        id,
                        percentage: "100".into(),
                        filename: title_clone.clone(),
                        thumbnail: thumb_clone.clone(),
                    });
                }
                _ => {}
            }
        }
    });

    Ok(video_title)
}

// =============================
// Folder Commands
// =============================

#[tauri::command]
async fn list_folders(current_path: Option<String>) -> Result<Vec<FolderEntry>, String> {

    let target_path = if let Some(p) = current_path {
        PathBuf::from(p)
    } else {
        home::home_dir().ok_or("Home directory not found")?
    };

    let mut folders = Vec::new();

    if let Ok(entries) = fs::read_dir(target_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') { continue; }
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    folders.push(FolderEntry {
                        name,
                        path: entry.path().to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(folders)
}

#[tauri::command]
async fn create_new_folder(parent_path: String, name: String) -> Result<String, String> {
    let new_path = Path::new(&parent_path).join(name);
    fs::create_dir(&new_path).map_err(|e| e.to_string())?;
    Ok(new_path.to_string_lossy().to_string())
}

#[tauri::command]
fn get_parent_dir(current_path: String) -> Result<String, String> {
    let path = Path::new(&current_path);
    path.parent()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Already at root".to_string())
}

// =============================
// App Entry
// =============================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())

        .invoke_handler(tauri::generate_handler![
            download_media,
            list_folders,
            get_parent_dir,
            create_new_folder
        ])

        .setup(|app| {

            let handle = app.handle().clone();
            let splash_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();

            tauri::async_runtime::spawn(async move {

                let _ = main_window.emit("log-event", "--- RTX SYSTEM INITIALIZING ---");

                // ✅ Splash තියෙද්දීම binaries extract කරනවා
                match ensure_binaries(&handle, &main_window).await {
                    Ok((ffmpeg, ytdlp)) => {
                        let _ = main_window.emit("log-event", format!("✅ ffmpeg: {}", ffmpeg));
                        let _ = main_window.emit("log-event", format!("✅ yt-dlp: {}", ytdlp));
                    }
                    Err(e) => {
                        let _ = main_window.emit("log-event", format!("❌ BINARY INIT ERROR: {}", e));
                    }
                }

                //std::thread::sleep(std::time::Duration::from_secs(3));
                tokio::time::sleep(Duration::from_secs(3)).await;

                let _ = splash_window.close();
                let _ = main_window.show();
                let _ = main_window.emit("log-event", "🚀 RTX CORE READY");
            });

            Ok(())
        })

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}