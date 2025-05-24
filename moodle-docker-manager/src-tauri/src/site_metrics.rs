use crate::docker::SiteState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use tauri::{command, AppHandle, Manager};

// Struct to hold site size information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteSizeInfo {
    pub total_size: u64,
    pub db_size: u64,
    pub files_size: u64,
    pub moodle_size: u64,
    pub moodledata_size: u64,
    pub other_files_size: u64,
}

// Struct to hold storage information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

#[command]
pub fn get_site_size(app_handle: AppHandle, name: String) -> Result<SiteSizeInfo, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => {
            return Err(format!("Site '{}' not found", name));
        }
    };

    // Get the database container name (modern Docker Compose uses hyphens)
    let db_container = format!("{}-db-1", name);

    // Get database size using docker exec
    let db_size = get_database_size(&db_container)?;

    // Calculate different file size components
    let mut moodle_size = 0u64;
    let mut moodledata_size = 0u64;
    let mut other_files_size = 0u64;

    // Main moodle directory
    let moodle_path = site.path.join("moodle");
    if moodle_path.exists() {
        moodle_size = get_directory_size(&moodle_path)?;
    }

    // Moodle data directory (often the largest component)
    let moodledata_path = site.path.join("moodledata");
    if moodledata_path.exists() {
        moodledata_size = get_directory_size(&moodledata_path)?;
    }

    // Include other common directories
    let additional_paths = [
        "backup",  // Backup directory
        "uploads", // Additional uploads
        "plugins", // Custom plugins
        "themes",  // Custom themes
        "config",  // Configuration files
    ];

    for dir_name in &additional_paths {
        let dir_path = site.path.join(dir_name);
        if dir_path.exists() {
            other_files_size += get_directory_size(&dir_path)?;
        }
    }

    // Also check for any docker-compose related files
    let docker_compose_path = site.path.join("docker-compose.yml");
    if docker_compose_path.exists() {
        other_files_size += get_directory_size(&docker_compose_path)?;
    }

    let env_path = site.path.join(".env");
    if env_path.exists() {
        other_files_size += get_directory_size(&env_path)?;
    }

    // Calculate totals
    let files_size = moodle_size + moodledata_size + other_files_size;
    let total_size = db_size + files_size;

    let result = SiteSizeInfo {
        total_size,
        db_size,
        files_size,
        moodle_size,
        moodledata_size,
        other_files_size,
    };

    Ok(result)
}

#[command]
pub fn get_storage_info() -> Result<StorageInfo, String> {
    // Get storage information for the current directory
    let output = Command::new("df")
        .args(&["-k", "."]) // -k for kilobytes, . for current directory
        .output()
        .map_err(|e| format!("Failed to execute df command: {}", e))?;

    if !output.status.success() {
        return Err("df command failed".to_string());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<&str> = output_str.lines().collect();

    if lines.len() < 2 {
        return Err("Unexpected df output format".to_string());
    }

    // Parse the second line (first line is headers)
    let parts: Vec<&str> = lines[1].split_whitespace().collect();

    if parts.len() < 4 {
        return Err("Unable to parse df output".to_string());
    }

    let total = parts[1]
        .parse::<u64>()
        .map_err(|_| "Failed to parse total size")?
        * 1024; // Convert KB to bytes
    let used = parts[2]
        .parse::<u64>()
        .map_err(|_| "Failed to parse used size")?
        * 1024; // Convert KB to bytes
    let available = parts[3]
        .parse::<u64>()
        .map_err(|_| "Failed to parse available size")?
        * 1024; // Convert KB to bytes

    let result = StorageInfo {
        total,
        available,
        used,
    };

    Ok(result)
}

// Helper function to get database size
fn get_database_size(db_container: &str) -> Result<u64, String> {
    // First, check if container exists and is running
    let container_check = Command::new("docker")
        .args(&[
            "ps",
            "--filter",
            &format!("name={}", db_container),
            "--format",
            "{{.Names}}",
        ])
        .output();

    match container_check {
        Ok(output) => {
            let container_output = String::from_utf8_lossy(&output.stdout);
            let container_name = container_output.trim();
            if container_name.is_empty() {
                return Ok(0);
            }

            // Try to get database size from running container
            let size_output = Command::new("docker")
                .args(&[
                    "exec",
                    db_container,
                    "bash",
                    "-c",
                    "du -sb /var/lib/mysql 2>/dev/null | cut -f1 || echo 0",
                ])
                .output();

            match size_output {
                Ok(output) => {
                    if output.status.success() {
                        let size_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        let size = size_str.parse::<u64>().unwrap_or(0);
                        Ok(size)
                    } else {
                        Ok(0)
                    }
                }
                Err(_) => Ok(0),
            }
        }
        Err(_) => Ok(0),
    }
}

// Helper function to get directory size recursively
fn get_directory_size(path: &PathBuf) -> Result<u64, String> {
    if !path.exists() {
        return Ok(0);
    }

    // Check if it's a directory
    if !path.is_dir() {
        // If it's a file, get its size
        match std::fs::metadata(path) {
            Ok(metadata) => return Ok(metadata.len()),
            Err(_) => return Ok(0),
        }
    }

    // Try using du command first (more efficient for large directories)
    match get_directory_size_with_du(path) {
        Ok(size) if size > 0 => Ok(size),
        _ => {
            // Fallback to manual traversal if du fails or returns 0
            get_directory_size_manual(path)
        }
    }
}

// Helper function to get directory size using du command
fn get_directory_size_with_du(path: &PathBuf) -> Result<u64, String> {
    let path_str = path.to_str().ok_or("Invalid path encoding")?;

    let output = Command::new("du")
        .args(&["-sb", path_str])
        .output()
        .map_err(|e| format!("Failed to execute du command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("du command failed: {}", stderr));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Ok(0);
    }

    parts[0]
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse size: {}", e))
}

// Manual directory traversal as fallback
fn get_directory_size_manual(path: &PathBuf) -> Result<u64, String> {
    use std::fs;

    let mut total_size = 0u64;

    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {}: {}", path.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let entry_path = entry.path();

        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to get metadata for {}: {}", entry_path.display(), e))?;

        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            // Recursive call for subdirectories
            match get_directory_size_manual(&entry_path) {
                Ok(sub_size) => total_size += sub_size,
                Err(_) => {
                    // Skip directories we can't read (permissions, etc.)
                    continue;
                }
            }
        }
    }

    Ok(total_size)
}
