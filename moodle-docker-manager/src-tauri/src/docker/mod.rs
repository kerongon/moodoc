// Docker module for managing Moodle Docker sites
// This module contains functionality for creating, managing and controlling Moodle Docker containers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tauri::{command, AppHandle, Manager, Emitter};
use tauri_plugin_store::StoreExt;

// Re-export submodules
pub mod backup;
pub mod filesystem;
pub mod logs;
pub mod site_management;
pub mod update;

// Data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    pub name: String,
    pub php_version: String,
    pub db_type: String,
    pub moodle_branch: String,
    pub db_port: Option<u16>,
    pub web_port: Option<u16>,
    pub status: String, // "running", "stopped", or "error"
    pub path: PathBuf,
}

// State management for sites
pub struct SiteState(pub Mutex<HashMap<String, Site>>);

// Struct to track update operations in progress
pub struct RunningUpdates(pub Mutex<HashMap<String, bool>>);

impl Default for RunningUpdates {
    fn default() -> Self {
        RunningUpdates(Mutex::new(HashMap::new()))
    }
}

// Struct to track site creation operations in progress
pub struct RunningCreations(pub Mutex<HashMap<String, bool>>);

impl Default for RunningCreations {
    fn default() -> Self {
        RunningCreations(Mutex::new(HashMap::new()))
    }
}

// Helper function for safe event emission
fn safe_emit<T: serde::Serialize + Clone>(app_handle: &AppHandle, event: &str, payload: T) {
    if let Err(e) = app_handle.emit(event, payload.clone()) {
        println!("Warning: Failed to emit event {}: {}", event, e);
    }
}

// Re-export functions from submodules
pub use site_management::{
    __cmd__cancel_site_creation, __cmd__check_site_status, __cmd__create_site, __cmd__list_sites, __cmd__remove_site,
    __cmd__start_site, __cmd__stop_site, cancel_site_creation, check_site_status, create_site, list_sites, remove_site,
    start_site, stop_site,
};

pub use backup::{
    __cmd__backup_site, __cmd__delete_site_backup, __cmd__list_site_backups,
    __cmd__restore_site_backup, __cmd__restore_site_backup_async, backup_site, delete_site_backup,
    list_site_backups, restore_site_backup, restore_site_backup_async,
};

pub use filesystem::{
    __cmd__open_backup_location, __cmd__open_in_editor, __cmd__open_site_folder,
    open_backup_location, open_in_editor, open_site_folder,
};

pub use update::{
    __cmd__cancel_update, __cmd__update_moodle_version, cancel_update, update_moodle_version,
};

pub use logs::{__cmd__get_docker_logs, get_docker_logs};

// Basic Docker utility functions
#[command]
pub fn check_docker_installed() -> bool {
    Command::new("which")
        .arg("docker")
        .output()
        .map_or(false, |output| output.status.success())
}

#[command]
pub fn check_docker_running() -> bool {
    let output = Command::new("docker").arg("info").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

#[command]
pub fn check_platform_compatibility(db_type: String) -> Result<String, String> {
    // Check if we're on ARM architecture (like Apple Silicon)
    #[cfg(target_arch = "aarch64")]
    {
        // All database types that might have compatibility issues on ARM
        match db_type.to_lowercase().as_str() {
            "oracle" => {
                return Ok("Note: Oracle on ARM64 uses an experimental image. You may experience some performance issues.".to_string());
            }
            "microsoft sql server" => {
                return Ok("Note: SQL Server on ARM64 uses Azure SQL Edge, which has some feature limitations compared to SQL Server.".to_string());
            }
            _ => {
                return Ok(
                    "Your platform is compatible with the selected database type.".to_string(),
                );
            }
        }
    }

    // For non-ARM architectures, all database types should work fine
    #[cfg(not(target_arch = "aarch64"))]
    {
        return Ok("Your platform is compatible with the selected database type.".to_string());
    }
}

#[command]
pub fn check_port_available(port: u16) -> bool {
    match TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// This function will validate the Docker setup and ensure directories exist
#[command]
pub fn validate_environment(app_handle: AppHandle) -> Result<String, String> {
    // Check if Docker is installed and running
    if !check_docker_installed() {
        return Err("Docker is not installed".to_string());
    }

    if !check_docker_running() {
        return Err("Docker is installed but not running".to_string());
    }

    // Ensure app data directory exists
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    // Create sites directory
    let sites_dir = app_dir.join("sites");
    if !sites_dir.exists() {
        fs::create_dir_all(&sites_dir)
            .map_err(|e| format!("Failed to create sites directory: {}", e))?;
    }

    Ok("Environment validation successful".to_string())
}

// New store-based site configuration functions
pub fn load_site_configs_from_store(app: &tauri::App) -> Result<(), String> {
    let app_handle = app.handle();
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Create or open the store
    let store = app.handle().store("sites.json")
        .map_err(|e| format!("Failed to create store: {}", e))?;

    // Load all sites from store
    for (key, value) in store.entries() {
        if key.starts_with("site_") {
            if let Ok(site) = serde_json::from_value::<Site>(value.clone()) {
                sites.insert(site.name.clone(), site);
            }
        }
    }

    Ok(())
}

pub fn save_site_config_to_store(app_handle: AppHandle, site: Site) -> Result<(), String> {
    // Create or open the store
    let store = app_handle.store("sites.json")
        .map_err(|e| format!("Failed to create store: {}", e))?;

    // Save site to store with site_ prefix
    let site_key = format!("site_{}", site.name);
    store.set(
        site_key,
        serde_json::to_value(&site).map_err(|e| format!("Failed to serialize site: {}", e))?,
    );

    // Persist to disk
    store.save()
        .map_err(|e| format!("Failed to persist store: {}", e))?;

    // Also update the in-memory state
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;
    sites.insert(site.name.clone(), site);

    Ok(())
}

pub fn remove_site_from_store(app_handle: AppHandle, site_name: String) -> Result<(), String> {
    // Create or open the store
    let store = app_handle.store("sites.json")
        .map_err(|e| format!("Failed to create store: {}", e))?;

    // Remove site from store
    let site_key = format!("site_{}", site_name);
    store.delete(&site_key);

    // Persist to disk
    store.save()
        .map_err(|e| format!("Failed to persist store: {}", e))?;

    // Also remove from in-memory state
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;
    sites.remove(&site_name);

    Ok(())
}

// Legacy file-based loading for migration
pub fn load_site_configs(app: &tauri::App) -> Result<(), String> {
    let app_handle = app.handle();
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let sites_dir = app_dir.join("sites");
    if !sites_dir.exists() {
        return Ok(()); // No sites directory, so no sites to load
    }

    // Read all site configuration files
    let entries =
        fs::read_dir(&sites_dir).map_err(|e| format!("Failed to read sites directory: {}", e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(site) = serde_json::from_str::<Site>(&content) {
                        sites.insert(site.name.clone(), site);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn save_site_config(app_handle: AppHandle, site: Site) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let sites_dir = app_dir.join("sites");
    if !sites_dir.exists() {
        create_dir_all(&sites_dir)
            .map_err(|e| format!("Failed to create sites directory: {}", e))?;
    }

    let config_file = sites_dir.join(format!("{}.json", site.name));
    let config_content = serde_json::to_string_pretty(&site)
        .map_err(|e| format!("Failed to serialize site config: {}", e))?;

    fs::write(&config_file, config_content)
        .map_err(|e| format!("Failed to write site config: {}", e))?;

    // Also update the in-memory state
    let state = app_handle.state::<SiteState>();
    let mut sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;
    sites.insert(site.name.clone(), site);

    Ok(())
}
