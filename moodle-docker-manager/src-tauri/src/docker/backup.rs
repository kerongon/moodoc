// Backup management functionality
// This module contains functions for backing up, restoring, and managing Moodle site backups

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::{command, AppHandle, Manager};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::{SiteState, safe_emit};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub name: String,
    pub site_name: String,
    pub created_at: DateTime<Utc>,
    pub size: u64,
    pub path: PathBuf,
    pub backup_type: String, // "full", "files_only", "database_only"
}

#[command]
pub async fn backup_site(app_handle: AppHandle, name: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    // Create backup directory if it doesn't exist
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let backups_dir = app_dir.join("backups").join(&name);
    if !backups_dir.exists() {
        fs::create_dir_all(&backups_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    }
    
    // Generate backup ID based on timestamp
    let backup_id = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_path = backups_dir.join(&backup_id);
    
    safe_emit(&app_handle, "backup_progress", format!("Starting backup for site '{}'", name));
    
    // Create backup script
    let backup_script = format!(r#"#!/bin/bash
set -e

# Navigate to site directory
cd {}

# Set environment variables
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}

# Create backup directory
mkdir -p {}

# Backup database
echo "Backing up database..."
./bin/moodle-docker-compose exec -T db mysqldump -u moodle -pexample moodle > {}/database.sql

# Backup moodledata
echo "Backing up moodledata..."
docker cp ${{COMPOSE_PROJECT_NAME}}_webserver_1:/var/www/moodledata {}/moodledata

# Backup Moodle files (excluding .git)
echo "Backing up Moodle files..."
tar --exclude='.git' -czf {}/moodle_files.tar.gz -C {} moodle

echo "Backup completed successfully"
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    site.path.join("moodle").to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    site.path.to_str().unwrap_or("")
    );
    
    let script_path = site.path.join("backup_temp.sh");
    fs::write(&script_path, backup_script)
        .map_err(|e| format!("Failed to create backup script: {}", e))?;
    
    // Make script executable
    Command::new("chmod")
        .args(["+x", script_path.to_str().unwrap_or("backup_temp.sh")])
        .output()
        .map_err(|e| format!("Failed to make backup script executable: {}", e))?;
    
    // Execute backup script
    let output = Command::new(script_path.to_str().unwrap_or("./backup_temp.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&script_path);
            format!("Failed to execute backup script: {}", e)
        })?;
    
    // Clean up script
    let _ = fs::remove_file(script_path);
    
    if output.status.success() {
        // Save backup metadata
        let backup_info = BackupInfo {
            id: backup_id.clone(),
            name: format!("Backup_{}", backup_id),
            site_name: name.clone(),
            created_at: Utc::now(),
            size: calculate_backup_size(&backup_path)?,
            path: backup_path,
            backup_type: "full".to_string(),
        };
        
        save_backup_metadata(&app_handle, &backup_info)?;
        
        safe_emit(&app_handle, "backup_completed", format!("Backup '{}' completed for site '{}'", backup_id, name));
        Ok(format!("Backup created successfully with ID: {}", backup_id))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        safe_emit(&app_handle, "backup_failed", format!("Backup failed for site '{}': {}", name, error));
        Err(format!("Backup failed: {}", error))
    }
}

#[command]
pub fn list_site_backups(app_handle: AppHandle, name: String) -> Result<Vec<HashMap<String, String>>, String> {
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let backups_dir = app_dir.join("backups").join(&name);
    if !backups_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut backups = Vec::new();
    
    // Read backup directories
    let entries = fs::read_dir(&backups_dir)
        .map_err(|e| format!("Failed to read backups directory: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let backup_id = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown");
                
                let mut backup_info = HashMap::new();
                backup_info.insert("id".to_string(), backup_id.to_string());
                backup_info.insert("name".to_string(), format!("Backup_{}", backup_id));
                backup_info.insert("created_at".to_string(), backup_id.to_string());
                backup_info.insert("size".to_string(), calculate_backup_size(&path).unwrap_or(0).to_string());
                
                backups.push(backup_info);
            }
        }
    }
    
    // Sort by creation date (newest first)
    backups.sort_by(|a, b| b.get("created_at").cmp(&a.get("created_at")));
    
    Ok(backups)
}

#[command]
pub fn restore_site_backup(app_handle: AppHandle, name: String, backup_id: String) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };
    
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let backup_path = app_dir.join("backups").join(&name).join(&backup_id);
    if !backup_path.exists() {
        return Err(format!("Backup '{}' not found for site '{}'", backup_id, name));
    }
    
    safe_emit(&app_handle, "restore_progress", format!("Starting restore of backup '{}' for site '{}'", backup_id, name));
    
    // Create restore script
    let restore_script = format!(r#"#!/bin/bash
set -e

# Navigate to site directory
cd {}

# Set environment variables
export COMPOSE_PROJECT_NAME={}
export MOODLE_DOCKER_WWWROOT={}

# Stop containers before restore
./bin/moodle-docker-compose down

# Restore database
echo "Restoring database..."
./bin/moodle-docker-compose up -d db
./bin/moodle-docker-wait-for-db
./bin/moodle-docker-compose exec -T db mysql -u moodle -pexample moodle < {}/database.sql

# Restore moodledata
echo "Restoring moodledata..."
./bin/moodle-docker-compose up -d webserver
docker cp {}/moodledata ${{COMPOSE_PROJECT_NAME}}_webserver_1:/var/www/

# Restore Moodle files
echo "Restoring Moodle files..."
rm -rf moodle/*
tar -xzf {}/moodle_files.tar.gz -C .

# Start all containers
./bin/moodle-docker-compose up -d

echo "Restore completed successfully"
"#, 
    site.path.to_str().unwrap_or(""),
    name,
    site.path.join("moodle").to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or(""),
    backup_path.to_str().unwrap_or("")
    );
    
    let script_path = site.path.join("restore_temp.sh");
    fs::write(&script_path, restore_script)
        .map_err(|e| format!("Failed to create restore script: {}", e))?;
    
    // Make script executable
    Command::new("chmod")
        .args(["+x", script_path.to_str().unwrap_or("restore_temp.sh")])
        .output()
        .map_err(|e| format!("Failed to make restore script executable: {}", e))?;
    
    // Execute restore script
    let output = Command::new(script_path.to_str().unwrap_or("./restore_temp.sh"))
        .output()
        .map_err(|e| {
            let _ = fs::remove_file(&script_path);
            format!("Failed to execute restore script: {}", e)
        })?;
    
    // Clean up script
    let _ = fs::remove_file(script_path);
    
    if output.status.success() {
        safe_emit(&app_handle, "restore_completed", format!("Restore of backup '{}' completed for site '{}'", backup_id, name));
        Ok(format!("Site '{}' restored from backup '{}' successfully", name, backup_id))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        safe_emit(&app_handle, "restore_failed", format!("Restore failed for site '{}': {}", name, error));
        Err(format!("Restore failed: {}", error))
    }
}

#[command]
pub async fn restore_site_backup_async(app_handle: AppHandle, name: String, backup_id: String) -> Result<String, String> {
    // Run the blocking restore function in a separate thread
    let handle = app_handle.clone();
    let name_clone = name.clone();
    let backup_id_clone = backup_id.clone();
    
    let result = tokio::task::spawn_blocking(move || {
        restore_site_backup(handle, name_clone, backup_id_clone)
    }).await.map_err(|e| format!("Task execution error: {}", e))?;
    
    result
}

#[command]
pub fn delete_site_backup(app_handle: AppHandle, name: String, backup_id: String) -> Result<String, String> {
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let backup_path = app_dir.join("backups").join(&name).join(&backup_id);
    if !backup_path.exists() {
        return Err(format!("Backup '{}' not found for site '{}'", backup_id, name));
    }
    
    // Remove backup directory
    fs::remove_dir_all(&backup_path)
        .map_err(|e| format!("Failed to delete backup: {}", e))?;
    
    Ok(format!("Backup '{}' deleted successfully", backup_id))
}

// Helper functions
fn calculate_backup_size(path: &PathBuf) -> Result<u64, String> {
    let mut total_size = 0u64;
    
    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata()
                    .map_err(|e| format!("Failed to get file metadata: {}", e))?;
                
                if metadata.is_file() {
                    total_size += metadata.len();
                } else if metadata.is_dir() {
                    total_size += calculate_backup_size(&entry.path())?;
                }
            }
        }
    }
    
    Ok(total_size)
}

fn save_backup_metadata(app_handle: &AppHandle, backup_info: &BackupInfo) -> Result<(), String> {
    let app_dir = app_handle.path_resolver().app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;
    
    let metadata_dir = app_dir.join("backup_metadata");
    if !metadata_dir.exists() {
        fs::create_dir_all(&metadata_dir)
            .map_err(|e| format!("Failed to create metadata directory: {}", e))?;
    }
    
    let metadata_file = metadata_dir.join(format!("{}_{}.json", backup_info.site_name, backup_info.id));
    let metadata_content = serde_json::to_string_pretty(backup_info)
        .map_err(|e| format!("Failed to serialize backup metadata: {}", e))?;
    
    fs::write(&metadata_file, metadata_content)
        .map_err(|e| format!("Failed to write backup metadata: {}", e))?;
    
    Ok(())
}
