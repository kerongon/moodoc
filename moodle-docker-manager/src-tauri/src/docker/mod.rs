// Docker module for managing Moodle Docker sites
// This module contains functionality for creating, managing and controlling Moodle Docker containers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, create_dir_all};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use tauri::{command, AppHandle, Manager, Emitter};
use tauri_plugin_store::StoreExt;

// Helper function to get Docker executable path
// This ensures the app can find Docker even when launched from Applications folder
// where PATH environment variable might not include Docker's location
pub fn get_docker_path() -> String {
    let mut possible_paths = vec![
        // Standard system locations
        "/usr/local/bin/docker".to_string(),                                    // Homebrew (Intel), OrbStack symlinks
        "/opt/homebrew/bin/docker".to_string(),                                 // Homebrew (Apple Silicon)
        "/usr/bin/docker".to_string(),                                          // System installation
        
        // Docker Desktop
        "/Applications/Docker.app/Contents/Resources/bin/docker".to_string(),   // Docker Desktop
        
        // OrbStack installations
        "/opt/orbstack/bin/docker".to_string(),                                 // OrbStack system installation
        
        // Additional package manager locations
        "/home/linuxbrew/.linuxbrew/bin/docker".to_string(),                   // Linuxbrew
        "/opt/local/bin/docker".to_string(),                                    // MacPorts
    ];
    
    // Add user-specific paths dynamically based on current user
    if let Ok(home) = std::env::var("HOME") {
        // OrbStack user installation (prioritized over Rancher Desktop)
        possible_paths.push(format!("{}/.orbstack/bin/docker", home));
        
        // Rancher Desktop
        possible_paths.push(format!("{}/.rd/bin/docker", home));
        
        // User bin directories
        possible_paths.push(format!("{}/bin/docker", home));
        possible_paths.push(format!("{}/.local/bin/docker", home));
        
        // Homebrew user installation
        possible_paths.push(format!("{}/homebrew/bin/docker", home));
        
        // Podman Desktop (if using Docker compatibility)
        possible_paths.push(format!("{}/.local/share/containers/podman/machine/bin/docker", home));
        
        // Lima/Colima installations
        possible_paths.push(format!("{}/.lima/*/bin/docker", home));
        possible_paths.push(format!("{}/.colima/bin/docker", home));
    }
    
    // Check each path to see if Docker exists there
    for path in &possible_paths {
        // Handle glob patterns like ~/.lima/*/bin/docker
        if path.contains('*') {
            if let Ok(entries) = glob::glob(path) {
                for entry in entries.flatten() {
                    if entry.exists() {
                        return entry.to_string_lossy().to_string();
                    }
                }
            }
        } else if Path::new(path).exists() {
            return path.to_string();
        }
    }
    
    // If none of the specific paths work, fall back to "docker" and hope it's in PATH
    "docker".to_string()
}

// Helper function to get docker-compose executable path
// Similar to get_docker_path but for standalone docker-compose installations
pub fn get_docker_compose_path() -> String {
    let mut possible_paths = vec![
        // Standard system locations
        "/usr/local/bin/docker-compose".to_string(),                                    // Homebrew (Intel), OrbStack symlinks
        "/opt/homebrew/bin/docker-compose".to_string(),                                 // Homebrew (Apple Silicon)
        "/usr/bin/docker-compose".to_string(),                                          // System installation
        
        // Docker Desktop
        "/Applications/Docker.app/Contents/Resources/bin/docker-compose".to_string(),   // Docker Desktop
        
        // OrbStack installations
        "/opt/orbstack/bin/docker-compose".to_string(),                                 // OrbStack system installation
        
        // Additional package manager locations
        "/home/linuxbrew/.linuxbrew/bin/docker-compose".to_string(),                   // Linuxbrew
        "/opt/local/bin/docker-compose".to_string(),                                    // MacPorts
    ];
    
    // Add user-specific paths dynamically based on current user
    if let Ok(home) = std::env::var("HOME") {
        // OrbStack user installation (prioritized over Rancher Desktop)
        possible_paths.push(format!("{}/.orbstack/bin/docker-compose", home));
        
        // Rancher Desktop
        possible_paths.push(format!("{}/.rd/bin/docker-compose", home));
        
        // User bin directories
        possible_paths.push(format!("{}/bin/docker-compose", home));
        possible_paths.push(format!("{}/.local/bin/docker-compose", home));
        
        // Homebrew user installation
        possible_paths.push(format!("{}/homebrew/bin/docker-compose", home));
        
        // Podman Desktop (if using Docker compatibility)
        possible_paths.push(format!("{}/.local/share/containers/podman/machine/bin/docker-compose", home));
        
        // Lima/Colima installations
        possible_paths.push(format!("{}/.lima/*/bin/docker-compose", home));
        possible_paths.push(format!("{}/.colima/bin/docker-compose", home));
    }
    
    // Check each path to see if docker-compose exists there
    for path in &possible_paths {
        // Handle glob patterns like ~/.lima/*/bin/docker-compose
        if path.contains('*') {
            if let Ok(entries) = glob::glob(path) {
                for entry in entries.flatten() {
                    if entry.exists() {
                        return entry.to_string_lossy().to_string();
                    }
                }
            }
        } else if Path::new(path).exists() {
            return path.to_string();
        }
    }
    
    // If none of the specific paths work, fall back to "docker-compose" and hope it's in PATH
    "docker-compose".to_string()
}

// Helper function to detect Docker engine type
#[command]
pub fn detect_docker_engine() -> Result<String, String> {
    let docker_path = get_docker_path();
    let mut engine_info = Vec::new();
    
    // Check what Docker engine we're using
    match Command::new(&docker_path).args(&["version", "--format", "json"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_json = String::from_utf8_lossy(&output.stdout);
                
                // Parse basic version info
                if version_json.contains("orbstack") || version_json.contains("OrbStack") {
                    engine_info.push("Engine: OrbStack".to_string());
                } else if version_json.contains("rancher") || docker_path.contains(".rd/bin") {
                    engine_info.push("Engine: Rancher Desktop".to_string());
                } else if docker_path.contains("Docker.app") {
                    engine_info.push("Engine: Docker Desktop".to_string());
                } else if docker_path.contains("homebrew") || docker_path.contains("/usr/local/bin") {
                    engine_info.push("Engine: Homebrew Docker".to_string());
                } else if docker_path.contains("podman") {
                    engine_info.push("Engine: Podman Desktop".to_string());
                } else if docker_path.contains("lima") {
                    engine_info.push("Engine: Lima".to_string());
                } else if docker_path.contains("colima") {
                    engine_info.push("Engine: Colima".to_string());
                } else {
                    engine_info.push("Engine: Unknown/System Docker".to_string());
                }
                
                engine_info.push(format!("Docker Path: {}", docker_path));
                engine_info.push(format!("Docker Compose Path: {}", get_docker_compose_path()));
            }
        }
        Err(e) => {
            return Err(format!("Failed to detect Docker engine: {}", e));
        }
    }
    
    // Check Docker info for additional details
    match Command::new(&docker_path).args(&["info", "--format", "{{.ServerVersion}}"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim();
                engine_info.push(format!("Server Version: {}", version));
            }
        }
        Err(_) => {} // Don't fail if this doesn't work
    }
    
    // Check if we can access Docker socket
    match Command::new(&docker_path).args(&["ps", "-q"]).output() {
        Ok(output) => {
            if output.status.success() {
                engine_info.push("Socket Access: ✓ Working".to_string());
            } else {
                engine_info.push("Socket Access: ✗ Failed".to_string());
            }
        }
        Err(_) => {
            engine_info.push("Socket Access: ✗ Failed".to_string());
        }
    }
    
    Ok(engine_info.join("\n"))
}

// Enhanced Docker path debugging with support for all engines
#[command]
pub fn debug_docker_engine_paths() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Get HOME environment variable
    let home = std::env::var("HOME").unwrap_or_else(|_| "HOME not available".to_string());
    results.push(format!("HOME environment variable: {}", home));
    
    // Create dynamic paths
    let orbstack_user_path = format!("{}/.orbstack/bin/docker", home);
    let rancher_path = format!("{}/.rd/bin/docker", home);
    let user_bin_path = format!("{}/bin/docker", home);
    let local_bin_path = format!("{}/.local/bin/docker", home);
    let homebrew_user_path = format!("{}/homebrew/bin/docker", home);
    let podman_path = format!("{}/.local/share/containers/podman/machine/bin/docker", home);
    let colima_path = format!("{}/.colima/bin/docker", home);
    
    // Check for different Docker engine installations
    let engine_paths = vec![
        // Docker Desktop
        ("/Applications/Docker.app/Contents/Resources/bin/docker", "Docker Desktop"),
        
        // OrbStack
        ("/opt/orbstack/bin/docker", "OrbStack System"),
        (orbstack_user_path.as_str(), "OrbStack User"),
        
        // Rancher Desktop
        (rancher_path.as_str(), "Rancher Desktop"),
        
        // Homebrew installations
        ("/usr/local/bin/docker", "Homebrew (Intel) / OrbStack Symlinks"),
        ("/opt/homebrew/bin/docker", "Homebrew (Apple Silicon)"),
        
        // System installations
        ("/usr/bin/docker", "System Installation"),
        
        // Package managers
        ("/opt/local/bin/docker", "MacPorts"),
        ("/home/linuxbrew/.linuxbrew/bin/docker", "Linuxbrew"),
        
        // User directories
        (user_bin_path.as_str(), "User Bin"),
        (local_bin_path.as_str(), "Local User Bin"),
        (homebrew_user_path.as_str(), "User Homebrew"),
        
        // Podman Desktop
        (podman_path.as_str(), "Podman Desktop"),
        
        // Lima/Colima (we'll check these separately due to glob patterns)
        (colima_path.as_str(), "Colima"),
    ];
    
    results.push(format!("\n--- DOCKER ENGINE DETECTION ---"));
    
    for (path, engine_type) in engine_paths {
        let exists = Path::new(path).exists();
        results.push(format!(
            "{}: {} {}",
            engine_type,
            path,
            if exists { "✓ EXISTS" } else { "✗ NOT FOUND" }
        ));
        
        if exists {
            // Test if it's executable and get version
            match Command::new(path).arg("--version").output() {
                Ok(output) => {
                    if output.status.success() {
                        let version_output = String::from_utf8_lossy(&output.stdout);
                        let version = version_output.trim().to_string();
                        let short_version = version.lines().next().unwrap_or(&version);
                        results.push(format!("  └─ Version: {}", short_version));
                        
                        // Try to detect engine type from version
                        if version.to_lowercase().contains("orbstack") {
                            results.push(format!("  └─ 🎯 DETECTED: OrbStack Engine"));
                        } else if version.to_lowercase().contains("rancher") {
                            results.push(format!("  └─ 🐄 DETECTED: Rancher Desktop"));
                        } else if version.to_lowercase().contains("podman") {
                            results.push(format!("  └─ 🐧 DETECTED: Podman Engine"));
                        }
                    } else {
                        results.push(format!("  └─ ✗ Found but not executable"));
                    }
                }
                Err(e) => {
                    results.push(format!("  └─ ✗ Error executing: {}", e));
                }
            }
        }
    }
    
    // Check Lima installations (glob pattern)
    results.push(format!("\n--- LIMA INSTALLATIONS ---"));
    let lima_pattern = format!("{}/.lima/*/bin/docker", home);
    match glob::glob(&lima_pattern) {
        Ok(entries) => {
            let mut found_lima = false;
            for entry in entries.flatten() {
                if entry.exists() {
                    found_lima = true;
                    results.push(format!("Lima: {} ✓ EXISTS", entry.display()));
                    
                    // Test Lima docker
                    match Command::new(&entry).arg("--version").output() {
                        Ok(output) => {
                            if output.status.success() {
                                let version_output = String::from_utf8_lossy(&output.stdout);
                                let version = version_output.trim();
                                let short_version = version.lines().next().unwrap_or(&version);
                                results.push(format!("  └─ Version: {}", short_version));
                                results.push(format!("  └─ 🐧 DETECTED: Lima Engine"));
                            }
                        }
                        Err(e) => {
                            results.push(format!("  └─ ✗ Error executing: {}", e));
                        }
                    }
                }
            }
            if !found_lima {
                results.push(format!("Lima: No installations found"));
            }
        }
        Err(e) => {
            results.push(format!("Lima: Error checking installations: {}", e));
        }
    }
    
    // Test what our path resolution functions actually return
    let resolved_docker = get_docker_path();
    let resolved_compose = get_docker_compose_path();
    
    results.push(format!("\n--- PATH RESOLUTION RESULTS ---"));
    results.push(format!("get_docker_path() returns: {}", resolved_docker));
    results.push(format!("get_docker_compose_path() returns: {}", resolved_compose));
    
    // Check if resolved paths match specific engines
    if resolved_docker.contains("orbstack") {
        results.push(format!("🎯 Using OrbStack: ✓ YES"));
    } else if resolved_docker.contains(".rd/bin") {
        results.push(format!("🐄 Using Rancher Desktop: ✓ YES"));
    } else if resolved_docker.contains("Docker.app") {
        results.push(format!("🐳 Using Docker Desktop: ✓ YES"));
    } else if resolved_docker.contains("homebrew") {
        results.push(format!("🍺 Using Homebrew Docker: ✓ YES"));
    } else if resolved_docker.contains("podman") {
        results.push(format!("🐧 Using Podman Desktop: ✓ YES"));
    } else if resolved_docker.contains("lima") {
        results.push(format!("🐧 Using Lima: ✓ YES"));
    } else if resolved_docker.contains("colima") {
        results.push(format!("🦙 Using Colima: ✓ YES"));
    } else {
        results.push(format!("❓ Using: {} (Unknown engine)", resolved_docker));
    }
    
    // Test Docker functionality
    results.push(format!("\n--- FUNCTIONALITY TEST ---"));
    match Command::new(&resolved_docker).args(&["ps", "-q"]).output() {
        Ok(output) => {
            if output.status.success() {
                results.push(format!("Docker ps: ✓ Working"));
            } else {
                results.push(format!("Docker ps: ✗ Failed - {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        Err(e) => {
            results.push(format!("Docker ps: ✗ Error - {}", e));
        }
    }
    
    // Test docker-compose functionality
    match Command::new(&resolved_compose).args(&["--version"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_output = String::from_utf8_lossy(&output.stdout);
                let version = version_output.trim();
                results.push(format!("Docker Compose: ✓ Working - {}", version.lines().next().unwrap_or(&version)));
            } else {
                results.push(format!("Docker Compose: ✗ Failed"));
            }
        }
        Err(_) => {
            // Try docker compose (v2)
            match Command::new(&resolved_docker).args(&["compose", "--version"]).output() {
                Ok(output) => {
                    if output.status.success() {
                        let version_output = String::from_utf8_lossy(&output.stdout);
                        let version = version_output.trim();
                        results.push(format!("Docker Compose V2: ✓ Working - {}", version.lines().next().unwrap_or(&version)));
                    } else {
                        results.push(format!("Docker Compose: ✗ Neither legacy nor v2 working"));
                    }
                }
                Err(e) => {
                    results.push(format!("Docker Compose: ✗ Error - {}", e));
                }
            }
        }
    }
    
    Ok(results.join("\n"))
}

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
    let docker_path = get_docker_path();
    // First check if we can find a Docker binary using our enhanced path resolution
    if !docker_path.is_empty() && docker_path != "docker" {
        // We found a specific path, verify it exists and is executable
        if let Ok(metadata) = std::fs::metadata(&docker_path) {
            return metadata.is_file();
        }
    }
    
    // Fallback to checking if docker command works at all
    Command::new(&docker_path)
        .arg("--version")
        .output()
        .map_or(false, |output| output.status.success())
}

#[command]
pub fn check_docker_running() -> bool {
    let output = Command::new(&get_docker_path()).arg("info").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

// Add comprehensive Docker status check
#[derive(serde::Serialize)]
pub struct DockerStatus {
    pub installed: bool,
    pub running: bool,
    pub socket_accessible: bool,
    pub path: String,
    pub version: Option<String>,
    pub engine_type: Option<String>,
}

#[command]
pub fn get_docker_status() -> DockerStatus {
    let docker_path = get_docker_path();
    let installed = check_docker_installed();
    let running = if installed { check_docker_running() } else { false };
    
    // Check socket accessibility
    let socket_accessible = if installed && running {
        Command::new(&docker_path)
            .arg("info")
            .output()
            .map_or(false, |output| output.status.success())
    } else {
        false
    };
    
    // Get version if available
    let version = if installed {
        Command::new(&docker_path)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
    } else {
        None
    };
    
    // Detect engine type
    let engine_type = if installed {
        detect_docker_engine_type(&docker_path)
    } else {
        None
    };
    
    DockerStatus {
        installed,
        running,
        socket_accessible,
        path: docker_path,
        version,
        engine_type,
    }
}

fn detect_docker_engine_type(docker_path: &str) -> Option<String> {
    if docker_path.contains("orbstack") {
        Some("OrbStack".to_string())
    } else if docker_path.contains("rancher") || docker_path.contains(".rd") {
        Some("Rancher Desktop".to_string())
    } else if docker_path.contains("Docker.app") || docker_path.contains("docker-desktop") {
        Some("Docker Desktop".to_string())
    } else if docker_path.contains("homebrew") || docker_path.contains("brew") {
        Some("Homebrew".to_string())
    } else if docker_path.contains("lima") {
        Some("Lima".to_string())
    } else if docker_path.contains("colima") {
        Some("Colima".to_string())
    } else if docker_path.contains("podman") {
        Some("Podman Desktop".to_string())
    } else if docker_path == "docker" {
        Some("System PATH".to_string())
    } else {
        Some("Unknown".to_string())
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

// Enhanced Docker permission checking and solutions
#[command]
pub fn check_docker_permissions() -> Result<String, String> {
    let mut issues = Vec::new();
    let mut solutions = Vec::new();

    // Check if Docker socket is accessible
    match Command::new(&get_docker_path()).args(&["info"]).output() {
        Ok(output) => {
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                if error.contains("permission denied") || error.contains("Cannot connect to the Docker daemon") {
                    issues.push("Docker socket permission denied".to_string());
                    solutions.push("Solution 1: Restart Docker Desktop".to_string());
                    solutions.push("Solution 2: Check if Docker Desktop is running".to_string());
                    solutions.push("Solution 3: Try running the app with administrator privileges".to_string());
                    solutions.push("Solution 4: Verify Docker Desktop allows socket access".to_string());
                }
            }
        }
        Err(e) => {
            issues.push(format!("Cannot execute docker command: {}", e));
            solutions.push("Solution: Ensure Docker Desktop is installed and in PATH".to_string());
        }
    }

    // Check Docker compose availability
    match Command::new(&get_docker_path()).args(&["compose", "version"]).output() {
        Ok(output) => {
            if !output.status.success() {
                issues.push("Docker Compose not available".to_string());
                solutions.push("Solution: Update Docker Desktop to include Docker Compose V2".to_string());
            }
        }
        Err(_) => {
            // Try legacy docker-compose
            match Command::new(&get_docker_compose_path()).args(&["--version"]).output() {
                Ok(_) => {
                    solutions.push("Note: Using legacy docker-compose. Consider upgrading to Docker Compose V2".to_string());
                }
                Err(_) => {
                    issues.push("Neither docker compose nor docker-compose found".to_string());
                    solutions.push("Solution: Install Docker Desktop or Docker Compose".to_string());
                }
            }
        }
    }

    let result = if issues.is_empty() {
        "Docker permissions check passed".to_string()
    } else {
        format!("Issues found:\n{}\n\nSuggested solutions:\n{}", 
               issues.join("\n"), 
               solutions.join("\n"))
    };

    Ok(result)
}

// Enhanced environment validation for production
#[command]
pub fn validate_production_environment() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Check if we're in a production build
    let is_production = !cfg!(debug_assertions);
    results.push(format!("Build type: {}", if is_production { "Production" } else { "Development" }));
    
    // Check Docker accessibility with detailed error reporting
    match Command::new(&get_docker_path()).args(&["version", "--format", "{{.Server.Version}}"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim().to_string();
                results.push(format!("Docker server accessible: Yes ({})", version));
                
                // Test container operations
                match Command::new(&get_docker_path()).args(&["ps", "-q"]).output() {
                    Ok(ps_output) => {
                        if ps_output.status.success() {
                            results.push("Container operations: Accessible".to_string());
                        } else {
                            results.push("Container operations: Limited access".to_string());
                        }
                    }
                    Err(e) => {
                        results.push(format!("Container operations: Failed ({})", e));
                    }
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                results.push(format!("Docker server accessible: No ({})", error));
            }
        }
        Err(e) => {
            results.push(format!("Docker command failed: {}", e));
        }
    }
    
    // Check environment variables
    let important_vars = ["PATH", "HOME", "USER", "PWD"];
    for var in &important_vars {
        match std::env::var(var) {
            Ok(value) => {
                if var == &"PATH" && !value.contains("docker") {
                    results.push(format!("{}: Set (Docker not in PATH)", var));
                } else {
                    results.push(format!("{}: Set", var));
                }
            }
            Err(_) => {
                results.push(format!("{}: Not set", var));
            }
        }
    }
    
    Ok(results.join("\n"))
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

// Updated diagnostic structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiagnosticResult {
    pub docker_version: Option<String>,
    pub docker_running: bool,
    pub socket_accessible: bool,
    pub compose_version: Option<String>,
    pub platform: String,
    pub current_user: Option<String>,
    pub current_dir: Option<String>,
    pub user_groups: Vec<String>,
    pub docker_groups: Vec<String>,
    pub containers: Vec<String>,
    pub networks: Vec<String>,
    pub port_checks: Vec<PortCheck>,
    pub directory_checks: Vec<DirectoryCheck>,
    pub environment_vars: HashMap<String, String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortCheck {
    pub port: u16,
    pub available: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectoryCheck {
    pub path: String,
    pub exists: bool,
    pub writable: bool,
}

// Comprehensive diagnostic command
#[command]
pub async fn run_diagnostics(app_handle: AppHandle) -> Result<DiagnosticResult, String> {
    let mut diagnostics = DiagnosticResult {
        docker_version: None,
        docker_running: false,
        socket_accessible: false,
        compose_version: None,
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        current_user: std::env::var("USER").ok().or_else(|| std::env::var("USERNAME").ok()),
        current_dir: std::env::current_dir().ok().and_then(|p| p.to_str().map(String::from)),
        user_groups: Vec::new(),
        docker_groups: Vec::new(),
        containers: Vec::new(),
        networks: Vec::new(),
        port_checks: Vec::new(),
        directory_checks: Vec::new(),
        environment_vars: HashMap::new(),
        errors: Vec::new(),
    };

    // Check Docker version
    match Command::new(&get_docker_path()).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                diagnostics.docker_version = Some(version_str.trim().to_string());
            } else {
                diagnostics.errors.push("Docker version command failed".to_string());
            }
        }
        Err(e) => {
            diagnostics.errors.push(format!("Failed to run docker --version: {}", e));
        }
    }

    // Check if Docker is running
    diagnostics.docker_running = check_docker_running();

    // Check Docker socket accessibility
    match Command::new(&get_docker_path()).arg("info").output() {
        Ok(output) => {
            diagnostics.socket_accessible = output.status.success();
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                diagnostics.errors.push(format!("Docker socket not accessible: {}", stderr));
            }
        }
        Err(e) => {
            diagnostics.errors.push(format!("Failed to run docker info: {}", e));
        }
    }

    // Check Docker Compose version
    match Command::new(&get_docker_path()).args(&["compose", "version"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version_line = version_str.lines().next().unwrap_or(version_str.trim());
                diagnostics.compose_version = Some(version_line.to_string());
            } else {
                // Try legacy docker-compose
                match Command::new(&get_docker_compose_path()).args(&["--version"]).output() {
                    Ok(legacy_output) => {
                        if legacy_output.status.success() {
                            let version_str = String::from_utf8_lossy(&legacy_output.stdout);
                            diagnostics.compose_version = Some(format!("Legacy: {}", version_str.trim()));
                        }
                    }
                    Err(_) => {
                        diagnostics.errors.push("Neither 'docker compose' nor 'docker-compose' available".to_string());
                    }
                }
            }
        }
        Err(e) => {
            diagnostics.errors.push(format!("Failed to check compose version: {}", e));
        }
    }

    // Collect important environment variables
    let important_vars = ["DOCKER_HOST", "DOCKER_CERT_PATH", "DOCKER_TLS_VERIFY", "DOCKER_MACHINE_NAME", "PATH", "HOME", "USER", "PWD"];
    for var in &important_vars {
        if let Ok(value) = std::env::var(var) {
            // Truncate PATH as it can be very long
            if var == &"PATH" && value.len() > 200 {
                diagnostics.environment_vars.insert(var.to_string(), format!("{}...", &value[..200]));
            } else {
                diagnostics.environment_vars.insert(var.to_string(), value);
            }
        }
    }

    // Get user groups (Unix-like systems)
    #[cfg(unix)]
    {
        match Command::new("groups").output() {
            Ok(output) => {
                if output.status.success() {
                    let groups_str = String::from_utf8_lossy(&output.stdout);
                    diagnostics.user_groups = groups_str
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    
                    // Filter docker-related groups
                    diagnostics.docker_groups = diagnostics.user_groups
                        .iter()
                        .filter(|g| g.to_lowercase().contains("docker"))
                        .cloned()
                        .collect();
                }
            }
            Err(e) => {
                diagnostics.errors.push(format!("Failed to get user groups: {}", e));
            }
        }
    }

    // List containers and networks (only if socket is accessible)
    if diagnostics.socket_accessible {
        // List containers
        match Command::new(&get_docker_path()).args(&["ps", "-a", "--format", "{{.Names}} ({{.Status}})"]).output() {
            Ok(output) => {
                if output.status.success() {
                    diagnostics.containers = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|s| s.to_string())
                        .collect();
                }
            }
            Err(e) => {
                diagnostics.errors.push(format!("Failed to list containers: {}", e));
            }
        }

        // List networks
        match Command::new(&get_docker_path()).args(&["network", "ls", "--format", "{{.Name}} ({{.Driver}})"]).output() {
            Ok(output) => {
                if output.status.success() {
                    diagnostics.networks = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|s| s.to_string())
                        .collect();
                }
            }
            Err(e) => {
                diagnostics.errors.push(format!("Failed to list networks: {}", e));
            }
        }
    }

    // Check port availability for common Moodle ports
    let test_ports = vec![8000, 8001, 8002, 8080, 8081, 8082, 3306, 3307, 3308, 5432, 5433, 5434];
    for port in test_ports {
        diagnostics.port_checks.push(PortCheck {
            port,
            available: check_port_available(port),
        });
    }

    // Check directory accessibility
    let mut directories_to_check = vec![
        (std::env::current_dir().unwrap_or_default().to_string_lossy().to_string(), "Current Working Directory"),
        (std::env::var("HOME").unwrap_or_default(), "Home Directory"),
    ];

    for (path, _description) in directories_to_check {
        if !path.is_empty() {
            let path_obj = std::path::Path::new(&path);
            let exists = path_obj.exists();
            let writable = if exists {
                // Test writability by trying to create a test file
                let test_file = path_obj.join(".write_test");
                match std::fs::write(&test_file, "test") {
                    Ok(_) => {
                        let _ = std::fs::remove_file(test_file);
                        true
                    }
                    Err(_) => false
                }
            } else {
                false
            };

            diagnostics.directory_checks.push(DirectoryCheck {
                path: path.clone(),
                exists,
                writable,
            });
        }
    }

    Ok(diagnostics)
}

// Enhanced site status checking with detailed error reporting
#[command]
pub async fn check_site_status_detailed(
    app_handle: AppHandle,
    site_name: String,
) -> Result<(String, Vec<String>), String> {
    let mut errors = Vec::new();
    let mut status = "unknown".to_string();

    // Get site configuration
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state")?;
    
    let site = sites.get(&site_name)
        .ok_or_else(|| format!("Site '{}' not found", site_name))?;

    // Check if site directory exists
    if !site.path.exists() {
        errors.push(format!("Site directory does not exist: {}", site.path.display()));
        return Ok(("error".to_string(), errors));
    }

    // Check for docker-compose file
    let compose_file = site.path.join("docker-compose.yml");
    if !compose_file.exists() {
        errors.push("docker-compose.yml file not found".to_string());
        status = "error".to_string();
    }

    // Check for moodle-docker directory
    let moodle_docker_dir = site.path.join("moodle-docker");
    if !moodle_docker_dir.exists() {
        errors.push("moodle-docker directory not found".to_string());
        status = "error".to_string();
    }

    // If basic files exist, check Docker containers
    if errors.is_empty() {
        // Set environment and check containers
        let output = Command::new(&get_docker_path())
            .args(&["ps", "-a", "--filter", &format!("name={}", site_name), "--format", "{{.State}}"])
            .current_dir(&site.path)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let container_states = String::from_utf8_lossy(&output.stdout);
                    let states: Vec<&str> = container_states.lines().collect();
                    
                    if states.is_empty() {
                        status = "stopped".to_string();
                        errors.push("No Docker containers found for this site".to_string());
                    } else if states.iter().all(|&s| s == "running") {
                        status = "running".to_string();
                    } else if states.iter().any(|&s| s == "running") {
                        status = "partial".to_string();
                        errors.push("Some containers are not running".to_string());
                    } else {
                        status = "stopped".to_string();
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    errors.push(format!("Failed to check container status: {}", stderr));
                    status = "error".to_string();
                }
            }
            Err(e) => {
                errors.push(format!("Failed to execute docker command: {}", e));
                status = "error".to_string();
            }
        }
    }

    Ok((status, errors))
}

// Test command to verify Docker path resolution
#[command]
pub fn test_docker_path_resolution() -> Result<String, String> {
    let docker_path = get_docker_path();
    let docker_compose_path = get_docker_compose_path();
    
    // Test if Docker is accessible at the resolved path
    let docker_accessible = match Command::new(&docker_path).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim().to_string();
                format!("✓ Docker accessible at: {} ({})", docker_path, version)
            } else {
                format!("✗ Docker found but not working at: {}", docker_path)
            }
        }
        Err(e) => format!("✗ Docker not accessible at: {} ({})", docker_path, e)
    };
    
    // Test docker-compose
    let docker_compose_accessible = match Command::new(&docker_compose_path).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.trim().to_string();
                format!("✓ Docker Compose accessible at: {} ({})", docker_compose_path, version)
            } else {
                format!("✗ Docker Compose found but not working at: {}", docker_compose_path)
            }
        }
        Err(e) => format!("✗ Docker Compose not accessible at: {} ({})", docker_compose_path, e)
    };
    
    // Get current PATH for comparison
    let current_path = std::env::var("PATH").unwrap_or_else(|_| "PATH not available".to_string());
    let path_contains_docker = current_path.contains("/docker") || current_path.contains("docker");
    
    let result = format!(
        "Docker Path Resolution Test Results:\n\n{}\n{}\n\nCurrent PATH contains docker paths: {}\nPATH variable: {}\n\nThis test verifies that the app can find Docker even when PATH doesn't include Docker locations (like when launched from Applications folder).",
        docker_accessible,
        docker_compose_accessible,
        path_contains_docker,
        if current_path.len() > 200 { format!("{}...", &current_path[..200]) } else { current_path }
    );
    
    Ok(result)
}

// Debug function specifically for testing Rancher Desktop path detection
#[command]
pub fn debug_rancher_docker_paths() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Get HOME environment variable
    let home = std::env::var("HOME").unwrap_or_else(|_| "HOME not available".to_string());
    results.push(format!("HOME environment variable: {}", home));
    
    // Check for Rancher Desktop specific paths
    let rancher_docker_path = format!("{}/.rd/bin/docker", home);
    let rancher_compose_path = format!("{}/.rd/bin/docker-compose", home);
    let user_bin_docker = format!("{}/bin/docker", home);
    let user_bin_compose = format!("{}/bin/docker-compose", home);
    
    // Test Rancher Desktop docker
    let rancher_docker_exists = Path::new(&rancher_docker_path).exists();
    results.push(format!(
        "Rancher Desktop Docker: {} {}",
        rancher_docker_path,
        if rancher_docker_exists { "✓ EXISTS" } else { "✗ NOT FOUND" }
    ));
    
    if rancher_docker_exists {
        match Command::new(&rancher_docker_path).arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    results.push(format!("  └─ Version: {}", version));
                } else {
                    results.push(format!("  └─ ✗ Found but not executable"));
                }
            }
            Err(e) => {
                results.push(format!("  └─ ✗ Error executing: {}", e));
            }
        }
    }
    
    // Test Rancher Desktop docker-compose
    let rancher_compose_exists = Path::new(&rancher_compose_path).exists();
    results.push(format!(
        "Rancher Desktop Docker Compose: {} {}",
        rancher_compose_path,
        if rancher_compose_exists { "✓ EXISTS" } else { "✗ NOT FOUND" }
    ));
    
    if rancher_compose_exists {
        match Command::new(&rancher_compose_path).arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    results.push(format!("  └─ Version: {}", version));
                } else {
                    results.push(format!("  └─ ✗ Found but not executable"));
                }
            }
            Err(e) => {
                results.push(format!("  └─ ✗ Error executing: {}", e));
            }
        }
    }
    
    // Test user bin paths
    let user_docker_exists = Path::new(&user_bin_docker).exists();
    results.push(format!(
        "User bin Docker: {} {}",
        user_bin_docker,
        if user_docker_exists { "✓ EXISTS" } else { "✗ NOT FOUND" }
    ));
    
    let user_compose_exists = Path::new(&user_bin_compose).exists();
    results.push(format!(
        "User bin Docker Compose: {} {}",
        user_bin_compose,
        if user_compose_exists { "✓ EXISTS" } else { "✗ NOT FOUND" }
    ));
    
    // Test what our path resolution functions actually return
    let resolved_docker = get_docker_path();
    let resolved_compose = get_docker_compose_path();
    
    results.push(format!("\n--- PATH RESOLUTION RESULTS ---"));
    results.push(format!("get_docker_path() returns: {}", resolved_docker));
    results.push(format!("get_docker_compose_path() returns: {}", resolved_compose));
    
    // Check if resolved paths match Rancher Desktop
    let using_rancher_docker = resolved_docker == rancher_docker_path;
    let using_rancher_compose = resolved_compose == rancher_compose_path;
    
    results.push(format!("Using Rancher Desktop Docker: {}", if using_rancher_docker { "✓ YES" } else { "✗ NO" }));
    results.push(format!("Using Rancher Desktop Compose: {}", if using_rancher_compose { "✓ YES" } else { "✗ NO" }));
    
    Ok(results.join("\n"))
}

// Function to create a docker-compose wrapper script for compatibility with modern Docker installations
// Modern Docker (like OrbStack) uses 'docker compose' instead of standalone 'docker-compose'
pub fn create_docker_compose_wrapper(temp_dir: &Path) -> Result<PathBuf, String> {
    let wrapper_path = temp_dir.join("docker-compose");
    
    // Get the Docker path to ensure we're using the correct Docker installation
    let docker_path = get_docker_path();
    
    let wrapper_content = format!(
        r#"#!/bin/bash
# Temporary docker-compose wrapper for moodle-docker compatibility
# This script calls 'docker compose' (V2) instead of standalone 'docker-compose'

# Use the detected Docker path or fall back to 'docker' in PATH
DOCKER_CMD="{}"
if [ ! -x "$DOCKER_CMD" ]; then
    DOCKER_CMD="docker"
fi

# Handle version checking specially to return proper version info
if [ "$1" = "--version" ] || [ "$1" = "version" ]; then
    exec "$DOCKER_CMD" compose version
else
    # Forward all arguments to 'docker compose'
    exec "$DOCKER_CMD" compose "$@"
fi
"#,
        docker_path
    );
    
    std::fs::write(&wrapper_path, wrapper_content)
        .map_err(|e| format!("Failed to create docker-compose wrapper: {}", e))?;
    
    // Make the wrapper executable
    let chmod_output = Command::new("chmod")
        .arg("+x")
        .arg(&wrapper_path)
        .output()
        .map_err(|e| format!("Failed to make docker-compose wrapper executable: {}", e))?;
    
    if !chmod_output.status.success() {
        return Err("Failed to make docker-compose wrapper executable".to_string());
    }
    
    Ok(wrapper_path)
}

// Function to create a temporary directory with docker-compose wrapper
pub fn setup_docker_compose_environment() -> Result<PathBuf, String> {
    // Create a temporary directory for our wrapper
    let temp_dir = std::env::temp_dir().join(format!("moodle-docker-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Create the docker-compose wrapper
    create_docker_compose_wrapper(&temp_dir)?;
    
    Ok(temp_dir)
}

// Function to execute a command with docker-compose wrapper in PATH
pub fn execute_with_docker_compose_wrapper<F, R>(operation: F) -> Result<R, String>
where
    F: FnOnce(&Path) -> Result<R, String>,
{
    // Set up the docker-compose wrapper environment
    let wrapper_dir = setup_docker_compose_environment()?;
    
    // Execute the operation with the wrapper in PATH
    let result = operation(&wrapper_dir);
    
    // Clean up the temporary directory
    let _ = std::fs::remove_dir_all(&wrapper_dir);
    
    result
}

// Test Docker Compose specifically
#[command]
pub fn test_docker_compose() -> Result<String, String> {
    let mut results = Vec::new();
    
    // Test docker compose version (V2)
    match Command::new(&get_docker_path()).args(&["compose", "version"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                results.push(format!("✓ Docker Compose V2: {}", version_str.trim()));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                results.push(format!("✗ Docker Compose V2 failed: {}", stderr));
            }
        }
        Err(e) => {
            results.push(format!("✗ Docker Compose V2 error: {}", e));
        }
    }
    
    // Test legacy docker-compose
    match Command::new(&get_docker_compose_path()).args(&["--version"]).output() {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                results.push(format!("✓ Legacy docker-compose: {}", version_str.trim()));
            } else {
                results.push("✗ Legacy docker-compose not working".to_string());
            }
        }
        Err(e) => {
            results.push(format!("✗ Legacy docker-compose error: {}", e));
        }
    }
    
    // Test our wrapper functionality
    match execute_with_docker_compose_wrapper(|wrapper_dir| {
        let wrapper_path = wrapper_dir.join("docker-compose");
        Command::new(&wrapper_path)
            .arg("--version")
            .output()
            .map_err(|e| format!("Wrapper execution failed: {}", e))
    }) {
        Ok(output) => {
            if output.status.success() {
                let version_str = String::from_utf8_lossy(&output.stdout);
                results.push(format!("✓ Docker Compose wrapper: {}", version_str.trim()));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                results.push(format!("✗ Docker Compose wrapper failed: {}", stderr));
            }
        }
        Err(e) => {
            results.push(format!("✗ Docker Compose wrapper error: {}", e));
        }
    }
    
    Ok(results.join("\n"))
}
