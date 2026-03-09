mod api;
mod downloader;
mod html_generator;
mod settings;

use api::{KemonoClient, Post};
use downloader::DownloadQueue;
use settings::{Settings, SettingsManager};
use std::sync::Arc;
use tauri::{Manager, State};
use tauri_plugin_opener::OpenerExt;

struct AppState {
    settings_mgr: Arc<SettingsManager>,
    download_queue: Arc<DownloadQueue>,
}

#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    state.settings_mgr.get()
}

#[tauri::command]
async fn save_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    state.settings_mgr.update(|s| {
        s.server = settings.server;
        s.service = settings.service;
        s.session = settings.session;
        s.download_path = settings.download_path;
        s.username = settings.username;
        s.password = settings.password;
    })?;
    Ok(())
}

#[tauri::command]
async fn login(state: State<'_, AppState>, username: String, password: String) -> Result<String, String> {
    let settings = state.settings_mgr.get()?;
    let client = KemonoClient::new(&settings.server);
    let (_, session) = client.login(&username, &password).await?;
    state.settings_mgr.update(|s| {
        s.session = session.clone();
        s.username = username;
        s.password = password;
    })?;
    Ok(session)
}

#[tauri::command]
async fn ensure_session(state: State<'_, AppState>) -> Result<String, String> {
    let settings = state.settings_mgr.get()?;
    if !settings.session.is_empty() {
        return Ok(settings.session);
    }
    if settings.username.is_empty() || settings.password.is_empty() {
        return Err("No credentials configured. Please login first.".to_string());
    }
    let client = KemonoClient::new(&settings.server);
    let (_, session) = client.login(&settings.username, &settings.password).await?;
    state.settings_mgr.update(|s| {
        s.session = session.clone();
    })?;
    Ok(session)
}

#[tauri::command]
async fn get_posts(state: State<'_, AppState>, service: String, creator_id: String) -> Result<Vec<Post>, String> {
    let settings = state.settings_mgr.get()?;
    let session = if settings.session.is_empty() {
        ensure_session(state.clone()).await?
    } else {
        settings.session.clone()
    };
    let client = KemonoClient::new(&settings.server);
    client.get_posts(&session, &service, &creator_id).await
}

#[tauri::command]
async fn get_post(state: State<'_, AppState>, service: String, creator_id: String, post_id: String) -> Result<Post, String> {
    let settings = state.settings_mgr.get()?;
    let session = if settings.session.is_empty() {
        ensure_session(state.clone()).await?
    } else {
        settings.session.clone()
    };
    let client = KemonoClient::new(&settings.server);
    client.get_post(&session, &service, &creator_id, &post_id).await
}

fn resolve_download_root(settings: &Settings, settings_path: &std::path::Path) -> std::path::PathBuf {
    let p = std::path::Path::new(&settings.download_path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        // Resolve relative paths against the current working directory
        std::env::current_dir()
            .unwrap_or_else(|_| settings_path.parent().unwrap_or(std::path::Path::new(".")).to_path_buf())
            .join(p)
    }
}

#[tauri::command]
async fn download_post_files(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    service: String,
    creator_id: String,
    post_id: String,
) -> Result<(), String> {
    let settings = state.settings_mgr.get()?;
    let session = if settings.session.is_empty() {
        ensure_session(state.clone()).await?
    } else {
        settings.session.clone()
    };

    let client = KemonoClient::new(&settings.server);

    // Fetch raw JSON (for HTML generation) and parsed Post (for file list)
    let detail_json = client.get_post_detail_raw(&session, &service, &creator_id, &post_id).await?;
    let post = client.get_post(&session, &service, &creator_id, &post_id).await?;

    // Build folder name: "번호 - 제목"
    let folder_name = html_generator::sanitize_folder_name(
        &format!("{} - {}", post_id, post.title)
    );

    let download_root = resolve_download_root(&settings, &state.settings_mgr.path);
    let base_dir = download_root
        .join(&service)
        .join(&creator_id)
        .join(&folder_name);

    // Generate post info HTML
    let html_content = html_generator::generate_post_html(&detail_json);
    let html_path = base_dir.join("post_info.html");
    tokio::fs::create_dir_all(&base_dir).await
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    tokio::fs::write(&html_path, html_content.as_bytes()).await
        .map_err(|e| format!("Failed to write post_info.html: {}", e))?;

    // Collect all files: main file + attachments
    // URL format: {server}/data{path}?f={name}
    // If server is missing, use https://kemono.cr which 302-redirects to the correct node
    let mut files_to_download: Vec<(String, String, String)> = Vec::new();

    if let Some(ref file) = post.file {
        let server = file.server.as_deref().unwrap_or("https://kemono.cr");
        let url = format!("{}/data{}?f={}", server, file.path, urlencoding::encode(&file.name));
        let dest = base_dir.join(&file.name).to_string_lossy().to_string();
        files_to_download.push((url, dest, file.name.clone()));
    }

    for att in &post.attachments {
        let server = att.server.as_deref().unwrap_or("https://kemono.cr");
        let url = format!("{}/data{}?f={}", server, att.path, urlencoding::encode(&att.name));
        let dest = base_dir.join(&att.name).to_string_lossy().to_string();
        files_to_download.push((url, dest, att.name.clone()));
    }

    for (url, dest, file_name) in files_to_download {
        let id = format!("{}_{}_{}", post_id, file_name, url);
        state.download_queue.enqueue(
            id,
            url,
            dest,
            file_name,
            post_id.clone(),
            session.clone(),
            &state.settings_mgr,
            &app,
        ).await?;
    }

    Ok(())
}

#[tauri::command]
async fn cancel_post_download(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    post_id: String,
) -> Result<(), String> {
    state.download_queue.cancel_post(&post_id, &state.settings_mgr, &app).await;
    Ok(())
}

#[tauri::command]
async fn debug_download_path(state: State<'_, AppState>) -> Result<String, String> {
    let settings = state.settings_mgr.get()?;
    let download_root = resolve_download_root(&settings, &state.settings_mgr.path);
    let root_str = download_root.display().to_string();
    let root_exists = download_root.exists();
    let canonical = download_root.canonicalize().map(|p| p.display().to_string()).unwrap_or_else(|e| format!("error: {}", e));

    Ok(format!(
        "settings.download_path: '{}'\nResolved: '{}'\nCanonical: '{}'\nExists: {}",
        settings.download_path, root_str, canonical, root_exists
    ))
}

#[tauri::command]
async fn get_resolved_download_path(state: State<'_, AppState>) -> Result<String, String> {
    let settings = state.settings_mgr.get()?;
    let download_root = resolve_download_root(&settings, &state.settings_mgr.path);
    Ok(download_root.display().to_string())
}

#[tauri::command]
async fn open_download_folder(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let settings = state.settings_mgr.get()?;
    let download_root = resolve_download_root(&settings, &state.settings_mgr.path);
    if !download_root.exists() {
        std::fs::create_dir_all(&download_root).map_err(|e| e.to_string())?;
    }
    app.opener()
        .open_path(download_root.to_string_lossy().as_ref(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .build(),
                )?;
            }

            let app_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                std::path::PathBuf::from(".")
            });

            let settings_mgr = Arc::new(SettingsManager::new(app_dir));
            let download_queue = Arc::new(DownloadQueue::new());

            app.manage(AppState {
                settings_mgr,
                download_queue,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            login,
            ensure_session,
            get_posts,
            get_post,
            download_post_files,
            cancel_post_download,
            debug_download_path,
            get_resolved_download_path,
            open_download_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
