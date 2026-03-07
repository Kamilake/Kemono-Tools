use crate::settings::{DownloadState, SettingsManager};
use futures::StreamExt;
use reqwest::header::{HeaderValue, ACCEPT, COOKIE, IF_RANGE, RANGE};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadProgress {
    pub id: String,
    pub downloaded: u64,
    pub total: u64,
    pub status: String,
    pub file_name: String,
    pub attempt: u32,
    pub max_retries: u32,
    pub retry_secs: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct QueueInner {
    queue: Vec<DownloadTask>,
    is_running: bool,
}

pub struct DownloadQueue {
    inner: Mutex<QueueInner>,
}

#[derive(Debug, Clone)]
struct DownloadTask {
    id: String,
    url: String,
    dest_path: String,
    file_name: String,
    post_id: String,
    session: String,
}

impl DownloadQueue {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(QueueInner {
                queue: Vec::new(),
                is_running: false,
            }),
        }
    }

    pub async fn enqueue(
        self: &Arc<Self>,
        id: String,
        url: String,
        dest_path: String,
        file_name: String,
        post_id: String,
        session: String,
        settings_mgr: &Arc<SettingsManager>,
        app: &AppHandle,
    ) -> Result<(), String> {
        let task = DownloadTask {
            id: id.clone(),
            url: url.clone(),
            dest_path: dest_path.clone(),
            file_name: file_name.clone(),
            post_id: post_id.clone(),
            session,
        };

        // Save to settings
        settings_mgr.update(|s| {
            s.downloads.insert(id.clone(), DownloadState {
                url,
                path: dest_path,
                downloaded: 0,
                total: 0,
                status: "pending".to_string(),
                post_id,
                file_name: file_name.clone(),
                etag: None,
            });
        })?;

        // Emit pending event so UI shows the file immediately
        let _ = app.emit("download-progress", DownloadProgress {
            id: id.clone(),
            downloaded: 0,
            total: 0,
            status: "pending".to_string(),
            file_name: task.file_name.clone(),
            attempt: 0,
            max_retries: 10,
            retry_secs: 0,
            error: None,
        });

        // Atomically enqueue and check if we need to start processing
        let should_start = {
            let mut inner = self.inner.lock().await;
            inner.queue.push(task);
            if !inner.is_running {
                inner.is_running = true;
                true
            } else {
                false
            }
        };

        if should_start {
            let queue = Arc::clone(self);
            let settings = Arc::clone(settings_mgr);
            let app_handle = app.clone();
            tokio::spawn(async move {
                queue.process_queue(&settings, &app_handle).await;
            });
        }

        Ok(())
    }

    async fn process_queue(self: &Arc<Self>, settings_mgr: &Arc<SettingsManager>, app: &AppHandle) {
        loop {
            let task = {
                let mut inner = self.inner.lock().await;
                if inner.queue.is_empty() {
                    inner.is_running = false;
                    break;
                }
                inner.queue.remove(0)
            };

            self.download_file(&task, settings_mgr, app).await;
        }
    }

    async fn download_file(&self, task: &DownloadTask, settings_mgr: &Arc<SettingsManager>, app: &AppHandle) {
        let max_stale_retries: u32 = 25; // give up after 25 consecutive failures with no progress
        let mut attempt: u32 = 0;
        let mut stale_failures: u32 = 0;

        loop {
            attempt += 1;

            // snapshot bytes on disk before this attempt
            let bytes_before = tokio::fs::metadata(&task.dest_path).await.map(|m| m.len()).unwrap_or(0);

            match self.try_download(task, attempt, max_stale_retries, settings_mgr, app).await {
                Ok(()) => break,
                Err(e) => {
                    let bytes_after = tokio::fs::metadata(&task.dest_path).await.map(|m| m.len()).unwrap_or(0);
                    let made_progress = bytes_after > bytes_before;

                    if made_progress {
                        stale_failures = 0;
                        log::info!("Download made progress ({} -> {} bytes), resetting stale counter: {} - {}",
                            bytes_before, bytes_after, task.file_name, e);
                    } else {
                        stale_failures += 1;
                        log::error!("Download failed with no progress (stale {}/{}): {} - {}",
                            stale_failures, max_stale_retries, task.file_name, e);
                    }

                    let _ = settings_mgr.update(|s| {
                        if let Some(dl) = s.downloads.get_mut(&task.id) {
                            dl.status = "failed".to_string();
                        }
                    });

                    if stale_failures >= max_stale_retries {
                        log::error!("Giving up after {} consecutive failures with no progress: {}", max_stale_retries, task.file_name);
                        let _ = app.emit("download-progress", DownloadProgress {
                            id: task.id.clone(),
                            downloaded: bytes_after,
                            total: 0,
                            status: "failed".to_string(),
                            file_name: task.file_name.clone(),
                            attempt,
                            max_retries: max_stale_retries,
                            retry_secs: 0,
                            error: Some(e.clone()),
                        });
                        break;
                    }

                    // Short wait if progress was made, longer wait if stale
                    let retry_total: u32 = if made_progress { 5 } else { 60 };
                    for remaining in (0..retry_total).rev() {
                        let _ = app.emit("download-progress", DownloadProgress {
                            id: task.id.clone(),
                            downloaded: bytes_after,
                            total: 0,
                            status: "retrying".to_string(),
                            file_name: task.file_name.clone(),
                            attempt,
                            max_retries: max_stale_retries,
                            retry_secs: remaining + 1,
                            error: Some(e.clone()),
                        });
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }

    async fn try_download(&self, task: &DownloadTask, attempt: u32, max_retries: u32, settings_mgr: &Arc<SettingsManager>, app: &AppHandle) -> Result<(), String> {
        let dest = Path::new(&task.dest_path);
        log::info!("[Download] attempt {}/{} file='{}' dest='{}'", attempt, max_retries, task.file_name, task.dest_path);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                let msg = format!("create_dir_all failed for '{}': {}", parent.display(), e);
                log::error!("[Download] {}", msg);
                msg
            })?;
        }

        // Check existing partial file size for resume
        let bytes_on_disk: u64 = match fs::metadata(dest).await {
            Ok(meta) => meta.len(),
            Err(_) => 0,
        };

        // Retrieve stored ETag for If-Range validation
        let stored_etag: Option<String> = settings_mgr.get().ok().and_then(|s| {
            s.downloads.get(&task.id).and_then(|dl| dl.etag.clone())
        });

        let client = reqwest::Client::builder()
            .gzip(true)
            .build()
            .map_err(|e| e.to_string())?;

        let mut req = client.get(&task.url)
            .header(ACCEPT, HeaderValue::from_static("*/*"));

        if !task.session.is_empty() {
            let cookie_val = format!("session={}", task.session);
            if let Ok(v) = HeaderValue::from_str(&cookie_val) {
                req = req.header(COOKIE, v);
            }
        }

        // Add Range + If-Range headers for resume
        if bytes_on_disk > 0 {
            let range_val = format!("bytes={}-", bytes_on_disk);
            log::info!("[Download] Requesting resume with Range: {}", range_val);
            if let Ok(v) = HeaderValue::from_str(&range_val) {
                req = req.header(RANGE, v);
            }
            if let Some(ref etag) = stored_etag {
                log::info!("[Download] If-Range: {}", etag);
                if let Ok(v) = HeaderValue::from_str(etag) {
                    req = req.header(IF_RANGE, v);
                }
            }
        }

        let resp = req.send().await.map_err(|e| {
            let msg = format!("Request failed: {}", e);
            log::error!("[Download] {}", msg);
            msg
        })?;

        let status = resp.status();
        let final_url = resp.url().to_string();
        log::info!("[Download] HTTP {} from '{}' (final url: '{}')", status, task.url, final_url);

        // Handle 416 Range Not Satisfiable: file may already be complete
        if status == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
            // Parse Content-Range: bytes */TOTAL to check if file is already complete
            let total_from_header = resp.headers()
                .get("content-range")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("bytes */"))
                .and_then(|s| s.parse::<u64>().ok());

            if let Some(total) = total_from_header {
                if bytes_on_disk >= total {
                    log::info!("[Download] File already complete ({} bytes on disk, server total {}), skipping download", bytes_on_disk, total);

                    // Run SHA256 verification
                    self.verify_sha256_from_url(dest, &task.url).await;

                    let _ = settings_mgr.update(|s| {
                        if let Some(dl) = s.downloads.get_mut(&task.id) {
                            dl.status = "completed".to_string();
                            dl.downloaded = bytes_on_disk;
                            dl.total = total;
                        }
                    });

                    let _ = app.emit("download-progress", DownloadProgress {
                        id: task.id.clone(),
                        downloaded: bytes_on_disk,
                        total,
                        status: "completed".to_string(),
                        file_name: task.file_name.clone(),
                        attempt,
                        max_retries,
                        retry_secs: 0,
                        error: None,
                    });

                    return Ok(());
                }
            }

            // If we can't determine total or size is wrong, delete and retry fresh
            log::warn!("[Download] Got 416 but file may be corrupted (bytes_on_disk={}), deleting and restarting", bytes_on_disk);
            let _ = fs::remove_file(dest).await;
            return Err(format!("HTTP 416, deleted partial file and will retry from scratch"));
        }

        if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(format!("HTTP {} from {}", status, final_url));
        }

        // Store ETag from response for future resume validation
        let response_etag = resp.headers().get("etag").and_then(|v| v.to_str().ok()).map(String::from);

        // Determine if we're resuming or starting fresh
        let resuming = status == reqwest::StatusCode::PARTIAL_CONTENT && bytes_on_disk > 0;
        let resume_offset = if resuming { bytes_on_disk } else { 0 };

        if resuming {
            log::info!("[Download] Resuming from byte {} (206 Partial Content)", resume_offset);
        } else if bytes_on_disk > 0 {
            log::info!("[Download] Server returned 200 (not 206), restarting from scratch (partial file will be overwritten)");
        }

        // Calculate total size
        let total_size = if resuming {
            // Content-Length in 206 is the remaining bytes
            let remaining = resp.content_length().unwrap_or(0);
            resume_offset + remaining
        } else {
            resp.content_length().unwrap_or(0)
        };
        log::info!("[Download] Total size: {} bytes (resume_offset: {})", total_size, resume_offset);

        // Update settings with ETag and status
        let _ = settings_mgr.update(|s| {
            if let Some(dl) = s.downloads.get_mut(&task.id) {
                dl.status = "downloading".to_string();
                dl.total = total_size;
                dl.downloaded = resume_offset;
                dl.etag = response_etag.clone();
            }
        });

        let _ = app.emit("download-progress", DownloadProgress {
            id: task.id.clone(),
            downloaded: resume_offset,
            total: total_size,
            status: "downloading".to_string(),
            file_name: task.file_name.clone(),
            attempt,
            max_retries,
            retry_secs: 0,
            error: None,
        });

        // Open file: append for resume, truncate for fresh start
        let mut file = if resuming {
            let f = OpenOptions::new()
                .write(true)
                .open(dest)
                .await
                .map_err(|e| {
                    let msg = format!("Failed to open file for resume '{}': {}", task.dest_path, e);
                    log::error!("[Download] {}", msg);
                    msg
                })?;
            // Seek to the end of existing content
            let mut f = f;
            f.seek(std::io::SeekFrom::Start(resume_offset)).await.map_err(|e| {
                let msg = format!("Seek error: {}", e);
                log::error!("[Download] {}", msg);
                msg
            })?;
            f
        } else {
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(dest)
                .await
                .map_err(|e| {
                    let msg = format!("Failed to open file '{}': {}", task.dest_path, e);
                    log::error!("[Download] {}", msg);
                    msg
                })?
        };

        let mut stream = resp.bytes_stream();
        let mut downloaded: u64 = resume_offset;
        let mut last_save = std::time::Instant::now();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| {
                let msg = format!("Stream error at {} bytes: {}", downloaded, e);
                log::error!("[Download] {}", msg);
                msg
            })?;
            file.write_all(&chunk).await.map_err(|e| {
                let msg = format!("Write error at {} bytes: {}", downloaded, e);
                log::error!("[Download] {}", msg);
                msg
            })?;
            downloaded += chunk.len() as u64;

            let _ = app.emit("download-progress", DownloadProgress {
                id: task.id.clone(),
                downloaded,
                total: total_size,
                status: "downloading".to_string(),
                file_name: task.file_name.clone(),
                attempt,
                max_retries,
                retry_secs: 0,
                error: None,
            });

            // Save progress every 5 seconds
            if last_save.elapsed().as_secs() >= 5 {
                let _ = settings_mgr.update(|s| {
                    if let Some(dl) = s.downloads.get_mut(&task.id) {
                        dl.downloaded = downloaded;
                    }
                });
                last_save = std::time::Instant::now();
            }
        }

        // Ensure all data is written to disk
        file.sync_all().await.map_err(|e| {
            let msg = format!("Sync error: {}", e);
            log::error!("[Download] {}", msg);
            msg
        })?;
        drop(file);

        // Verify the file exists and has content
        match fs::metadata(dest).await {
            Ok(meta) => {
                log::info!("[Download] SAVED: '{}' size={} bytes (downloaded={} bytes)", task.dest_path, meta.len(), downloaded);
                if meta.len() == 0 && total_size > 0 {
                    return Err(format!("File saved but is 0 bytes (expected {})", total_size));
                }
            }
            Err(e) => {
                let msg = format!("File verification failed - file does not exist after write: {}", e);
                log::error!("[Download] {}", msg);
                return Err(msg);
            }
        }

        // SHA256 verification (warn only) - extract hash from URL path if possible
        self.verify_sha256_from_url(dest, &task.url).await;

        // Mark completed
        let _ = settings_mgr.update(|s| {
            if let Some(dl) = s.downloads.get_mut(&task.id) {
                dl.status = "completed".to_string();
                dl.downloaded = downloaded;
            }
        });

        let _ = app.emit("download-progress", DownloadProgress {
            id: task.id.clone(),
            downloaded,
            total: total_size,
            status: "completed".to_string(),
            file_name: task.file_name.clone(),
            attempt,
            max_retries,
            retry_secs: 0,
            error: None,
        });

        Ok(())
    }

    /// Attempt to extract SHA256 hash from URL path and verify against downloaded file.
    /// Only logs a warning on mismatch — does not fail the download.
    async fn verify_sha256_from_url(&self, file_path: &Path, url: &str) {
        // URL pattern: /data/XX/YY/<sha256_hex>.ext
        let expected_hash = match url.split('/').last() {
            Some(filename) => {
                // Strip query string: "hash.ext?f=name" -> "hash.ext"
                let without_query = filename.split('?').next().unwrap_or(filename);
                // Strip extension: "hash.ext" -> "hash"
                match without_query.rsplit_once('.') {
                    Some((stem, _)) => stem.to_string(),
                    None => without_query.to_string(),
                }
            }
            None => return,
        };

        // Validate it looks like a SHA256 hex string (64 hex chars)
        if expected_hash.len() != 64 || !expected_hash.chars().all(|c| c.is_ascii_hexdigit()) {
            log::info!("[Download] SHA256 verification skipped: URL filename '{}' doesn't look like a SHA256 hash", expected_hash);
            return;
        }

        // Compute SHA256 of the downloaded file
        let mut file = match tokio::fs::File::open(file_path).await {
            Ok(f) => f,
            Err(e) => {
                log::warn!("[Download] SHA256 verification: could not open file: {}", e);
                return;
            }
        };

        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 1024 * 1024]; // 1MB buffer
        loop {
            let n = match file.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    log::warn!("[Download] SHA256 verification: read error: {}", e);
                    return;
                }
            };
            hasher.update(&buf[..n]);
        }

        let actual_hash = format!("{:x}", hasher.finalize());

        if actual_hash == expected_hash {
            log::info!("[Download] SHA256 verified OK: {}", actual_hash);
        } else {
            log::warn!("[Download] SHA256 MISMATCH for '{}': expected={}, actual={}", file_path.display(), expected_hash, actual_hash);
        }
    }
}
