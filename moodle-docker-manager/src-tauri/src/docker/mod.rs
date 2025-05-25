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
        "/usr/local/bin/docker",                                    // Intel Macs with Homebrew
        "/opt/homebrew/bin/docker",                                 // Apple Silicon Macs with Homebrew
        "/usr/bin/docker",                                          // System installation
        "/Applications/Docker.app/Contents/Resources/bin/docker",   // Docker Desktop
        "/home/linuxbrew/.linuxbrew/bin/docker",                   // Additional Homebrew location
    ];
    
    // Add Rancher Desktop paths dynamically based on current user
    if let Ok(home) = std::env::var("HOME") {
        possible_paths.push(format!("{}/.rd/bin/docker", home));
        possible_paths.push(format!("{}/bin/docker", home));
    }
    
    // Check each path to see if Docker exists there
    for path in &possible_paths {
        if Path::new(path).exists() {
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
        "/usr/local/bin/docker-compose",                                    // Intel Macs with Homebrew
        "/opt/homebrew/bin/docker-compose",                                 // Apple Silicon Macs with Homebrew
        "/usr/bin/docker-compose",                                          // System installation
        "/Applications/Docker.app/Contents/Resources/bin/docker-compose",   // Docker Desktop
        "/home/linuxbrew/.linuxbrew/bin/docker-compose",                   // Additional Homebrew location
    ];
    
    // Add Rancher Desktop paths dynamically based on current user
    if let Ok(home) = std::env::var("HOME") {
        possible_paths.push(format!("{}/.rd/bin/docker-compose", home));
        possible_paths.push(format!("{}/bin/docker-compose", home));
    }
    
    // Check each path to see if docker-compose exists there
    for path in &possible_paths {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }
    
    // If none of the specific paths work, fall back to "docker-compose" and hope it's in PATH
    "docker-compose".to_string()
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

// Diagnostic information structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiagnosticInfo {
    pub docker_version: Option<String>,
    pub docker_running: bool,
    pub docker_socket_accessible: bool,
    pub docker_daemon_info: Option<String>,
    pub environment_variables: HashMap<String, String>,
    pub current_user: Option<String>,
    pub current_working_directory: Option<String>,
    pub home_directory: Option<String>,
    pub app_data_directory: Option<String>,
    pub platform_info: String,
    pub sites_directory_exists: bool,
    pub sites_directory_writable: bool,
    pub docker_groups: Vec<String>,
    pub running_containers: Vec<String>,
    pub docker_networks: Vec<String>,
    pub available_ports: Vec<u16>,
    pub errors: Vec<String>,
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
    let output = Command::new(&get_docker_path()).arg("info").output();

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
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
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

// Comprehensive diagnostic command
#[command]
pub async fn run_diagnostics(app_handle: AppHandle) -> Result<DiagnosticInfo, String> {
    let mut diagnostics = DiagnosticInfo {
        docker_version: None,
        docker_running: false,
        docker_socket_accessible: false,
        docker_daemon_info: None,
        environment_variables: HashMap::new(),
        current_user: None,
        current_working_directory: None,
        home_directory: None,
        app_data_directory: None,
        platform_info: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        sites_directory_exists: false,
        sites_directory_writable: false,
        docker_groups: Vec::new(),
        running_containers: Vec::new(),
        docker_networks: Vec::new(),
        available_ports: Vec::new(),
        errors: Vec::new(),
    };

    // Check Docker version
    match Command::new(&get_docker_path()).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                diagnostics.docker_version = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
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
            diagnostics.docker_socket_accessible = output.status.success();
            if output.status.success() {
                diagnostics.docker_daemon_info = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                diagnostics.errors.push(format!("Docker info failed: {}", stderr));
            }
        }
        Err(e) => {
            diagnostics.errors.push(format!("Failed to run docker info: {}", e));
        }
    }

    // Collect environment variables
    for (key, value) in std::env::vars() {
        if key.contains("DOCKER") || key == "PATH" || key == "HOME" || key == "USER" || key == "PWD" {
            diagnostics.environment_variables.insert(key, value);
        }
    }

    // Get current user
    diagnostics.current_user = std::env::var("USER").ok();

    // Get current working directory
    diagnostics.current_working_directory = std::env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()));

    // Get home directory
    diagnostics.home_directory = app_handle.path().home_dir()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()));

    // Get app data directory
    match app_handle.path().app_data_dir() {
        Ok(app_dir) => {
            diagnostics.app_data_directory = app_dir.to_str().map(|s| s.to_string());
            
            // Check if sites directory exists and is writable
            let sites_dir = app_dir.join("sites");
            diagnostics.sites_directory_exists = sites_dir.exists();
            
            if diagnostics.sites_directory_exists {
                // Test writability by trying to create a test file
                let test_file = sites_dir.join(".write_test");
                match std::fs::write(&test_file, "test") {
                    Ok(_) => {
                        diagnostics.sites_directory_writable = true;
                        let _ = std::fs::remove_file(test_file);
                    }
                    Err(e) => {
                        diagnostics.sites_directory_writable = false;
                        diagnostics.errors.push(format!("Sites directory not writable: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            diagnostics.errors.push(format!("Failed to get app data directory: {}", e));
        }
    }

    // Check Docker groups (Unix-like systems)
    #[cfg(unix)]
    {
        match Command::new("groups").output() {
            Ok(output) => {
                if output.status.success() {
                    let groups_str = String::from_utf8_lossy(&output.stdout);
                    diagnostics.docker_groups = groups_str
                        .split_whitespace()
                        .filter(|g| g.contains("docker"))
                        .map(|s| s.to_string())
                        .collect();
                }
            }
            Err(e) => {
                diagnostics.errors.push(format!("Failed to get user groups: {}", e));
            }
        }
    }

    // List running containers
    if diagnostics.docker_socket_accessible {
        match Command::new(&get_docker_path()).args(&["ps", "--format", "{{.Names}}"]).output() {
            Ok(output) => {
                if output.status.success() {
                    diagnostics.running_containers = String::from_utf8_lossy(&output.stdout)
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

        // List Docker networks
        match Command::new(&get_docker_path()).args(&["network", "ls", "--format", "{{.Name}}"]).output() {
            Ok(output) => {
                if output.status.success() {
                    diagnostics.docker_networks = String::from_utf8_lossy(&output.stdout)
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

    // Check common port availability
    let test_ports = vec![8000, 8001, 8002, 3306, 3307, 3308, 5432, 5433];
    for port in test_ports {
        if check_port_available(port) {
            diagnostics.available_ports.push(port);
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
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
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
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
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
