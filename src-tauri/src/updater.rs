//! Offline build: the auto-updater has been removed.
//! No code in this file opens a network connection, spawns curl, or downloads
//! and runs installers. The public API is kept so the rest of the app compiles.

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

const DISABLED: &str = "Updates are disabled in this offline build";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_url: String,
    pub asset_name: Option<String>,
    pub download_url: Option<String>,
    pub asset_sha256: Option<String>,
    pub asset_size: Option<u64>,
    pub install_kind: String,
    pub notes_summary: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgressEvent {
    pub stage: String,
    pub message: String,
    pub info: Option<UpdateInfo>,
}

pub fn check_latest_update() -> Result<UpdateInfo, String> {
    Err(DISABLED.to_string())
}

pub fn perform_update_install(_app: &AppHandle, _info: UpdateInfo) -> Result<(), String> {
    Err(DISABLED.to_string())
}

/// Previously polled GitHub in the background. Now does nothing.
pub fn start_background_update_checker(_app: AppHandle) {}

/// Previously opened release / OBS pages in the browser. Now does nothing.
pub fn open_url_in_browser(_url: &str) {}

/// No update information is ever fetched in the offline build.
pub fn get_cached_update_info() -> Option<UpdateInfo> {
    None
}
