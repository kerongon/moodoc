use std::process::Command;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager};
use crate::docker::SiteState;

// Struct to hold site size information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteSizeInfo {
    pub total_size: u64,
    pub db_size: u64,
    pub files_size: u64,
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
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => {
            return Err(format!("Site '{}' not found", name));
        },
    };
    
    // Get the database container name (modern Docker Compose uses hyphens)
    let db_container = format!("{}-db-1", name);
    
    // Get database size using docker exec
    let db_size = get_database_size(&db_container)?;
    
    // Get the moodle files size
    let moodle_path = site.path.join("moodle");
    let files_size = get_directory_size(&moodle_path)?;
    
    // Calculate total size
    let total_size = db_size + files_size;
    
    let result = SiteSizeInfo {
        total_size,
        db_size,
        files_size,
    };
    
    Ok(result)
}

#[command]
pub fn get_storage_info() -> Result<StorageInfo, String> {
    // Get storage information for the current directory
    let output = Command::new("df")
        .args(&["-k", "."])  // -k for kilobytes, . for current directory
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
    
    let total = parts[1].parse::<u64>().map_err(|_| "Failed to parse total size")? * 1024; // Convert KB to bytes
    let used = parts[2].parse::<u64>().map_err(|_| "Failed to parse used size")? * 1024; // Convert KB to bytes
    let available = parts[3].parse::<u64>().map_err(|_| "Failed to parse available size")? * 1024; // Convert KB to bytes
    
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
        .args(&["ps", "--filter", &format!("name={}", db_container), "--format", "{{.Names}}"])
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
                .args(&["exec", db_container, "bash", "-c", "du -sb /var/lib/mysql 2>/dev/null | cut -f1 || echo 0"])
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
                },
                Err(_) => Ok(0)
            }
        },
        Err(_) => Ok(0)
    }
}

// Helper function to get directory size recursively
fn get_directory_size(path: &PathBuf) -> Result<u64, String> {
    if !path.exists() {
        return Ok(0);
    }
    
    // Use du command for more efficient directory size calculation
    let output = Command::new("du")
        .args(&["-sb", path.to_str().unwrap_or("")])
        .output()
        .map_err(|e| format!("Failed to execute du command: {}", e))?;
    
    if !output.status.success() {
        return Err("du command failed".to_string());
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().split('\t').collect();
    
    if parts.is_empty() {
        return Ok(0);
    }
    
    parts[0].parse::<u64>().map_err(|e| format!("Failed to parse size: {}", e))
}

// Utility function to format bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let base = 1024_f64;
    let log = (bytes as f64).log(base).floor() as usize;
    let size = bytes as f64 / base.powi(log as i32);
    
    if log == 0 {
        format!("{} {}", bytes, UNITS[log])
    } else {
        format!("{:.1} {}", size, UNITS[log])
    }
}
