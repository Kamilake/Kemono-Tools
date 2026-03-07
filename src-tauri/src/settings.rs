use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadState {
    pub url: String,
    pub path: String,
    pub downloaded: u64,
    pub total: u64,
    pub status: String, // "pending", "downloading", "completed", "failed", "paused"
    pub post_id: String,
    pub file_name: String,
    #[serde(default)]
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: String,
    pub service: String,
    pub session: String,
    pub download_path: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub downloads: HashMap<String, DownloadState>,
}

impl Default for Settings {
    fn default() -> Self {
        let default_download = dirs::download_dir()
            .unwrap_or_else(|| dirs::home_dir()
                .map(|h| h.join("Downloads"))
                .unwrap_or_else(|| std::path::PathBuf::from("./downloads")))
            .join("Kemono")
            .display()
            .to_string();
        Self {
            server: "https://kemono.cr/api".to_string(),
            service: "fantia".to_string(),
            session: String::new(),
            download_path: default_download,
            username: String::new(),
            password: String::new(),
            downloads: HashMap::new(),
        }
    }
}

pub struct SettingsManager {
    pub settings: Mutex<Settings>,
    pub path: PathBuf,
}

impl SettingsManager {
    pub fn new(app_dir: PathBuf) -> Self {
        let path = app_dir.join("settings.json");
        let settings = if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Settings::default(),
            }
        } else {
            Settings::default()
        };
        Self {
            settings: Mutex::new(settings),
            path,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let settings = self.settings.lock().map_err(|e| e.to_string())?;
        let content = serde_json::to_string_pretty(&*settings).map_err(|e| e.to_string())?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(&self.path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get(&self) -> Result<Settings, String> {
        let settings = self.settings.lock().map_err(|e| e.to_string())?;
        Ok(settings.clone())
    }

    pub fn update<F>(&self, f: F) -> Result<Settings, String>
    where
        F: FnOnce(&mut Settings),
    {
        let mut settings = self.settings.lock().map_err(|e| e.to_string())?;
        f(&mut settings);
        let cloned = settings.clone();
        drop(settings);
        self.save()?;
        Ok(cloned)
    }
}
