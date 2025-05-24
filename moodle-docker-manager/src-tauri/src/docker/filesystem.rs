// Filesystem operations for Moodle Docker Manager
// This module contains functions for opening folders, files, and backup locations

use std::process::Command;
use tauri::command;

#[command]
pub fn open_backup_location(path: String) -> Result<String, String> {
    // Open the backup location in the system file manager
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open backup location: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened backup location: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open backup location: {}", error))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("explorer")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open backup location: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened backup location: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open backup location: {}", error))
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open backup location: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened backup location: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open backup location: {}", error))
        }
    }
}

#[command]
pub fn open_site_folder(path: String) -> Result<String, String> {
    // Open the site folder in the system file manager
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened site folder: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("explorer")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened site folder: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(&path)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;
        
        if output.status.success() {
            Ok(format!("Opened site folder: {}", path))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }
}

#[command]
pub fn open_in_editor(editor_name: String, path: String) -> Result<String, String> {
    // Open the path in the specified editor
    let editor_cmd = match editor_name.to_lowercase().as_str() {
        "vs code" | "vscode" | "code" => "code",
        "sublime" | "sublime text" => "subl",
        "atom" => "atom",
        "vim" => "vim",
        "nano" => "nano",
        "emacs" => "emacs",
        "phpstorm" => "phpstorm",
        "webstorm" => "webstorm",
        "intellij" => "idea",
        _ => &editor_name, // Use the editor name as-is if not recognized
    };
    
    let output = Command::new(editor_cmd)
        .arg(&path)
        .output()
        .map_err(|e| format!("Failed to open {} in {}: {}", path, editor_name, e))?;
    
    if output.status.success() {
        Ok(format!("Opened {} in {}", path, editor_name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to open {} in {}: {}", path, editor_name, error))
    }
}

// Helper function to check if a file or directory exists
pub fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// Helper function to get the parent directory of a path
pub fn get_parent_dir(path: &str) -> Option<String> {
    std::path::Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}
