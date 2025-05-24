// Filesystem operations for Moodle Docker Manager
// This module contains functions for opening folders, files, and backup locations

use super::SiteState;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn open_backup_location(
    app_handle: AppHandle,
    name: String,
    backup_timestamp: Option<String>,
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let sites = state
        .0
        .lock()
        .map_err(|_| "Failed to lock sites state".to_string())?;

    // Check if site exists
    let site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => return Err(format!("Site '{}' not found", name)),
    };

    // Determine which path to open - specific backup or backups directory
    let target_path = if let Some(ref timestamp) = backup_timestamp {
        // Open specific backup folder
        let backup_name = format!("backup_{}_{}", name, timestamp);
        site.path.join("backups").join(backup_name)
    } else {
        // Open the general backups folder
        site.path.join("backups")
    };

    // Check if path exists
    if !target_path.exists() {
        if let Some(ref timestamp) = backup_timestamp {
            return Err(format!("Backup '{}' not found", timestamp));
        } else {
            // Create backups directory if it doesn't exist
            fs::create_dir_all(&target_path)
                .map_err(|e| format!("Failed to create backups directory: {}", e))?;
        }
    }

    // Use open command on macOS to open the directory in Finder
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory in Finder: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory in Finder: {}", error));
        }

        Ok(format!("Opened in Finder: {}", target_path.display()))
    }

    // For Windows
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("explorer")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory in Explorer: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory in Explorer: {}", error));
        }

        Ok(format!("Opened in Explorer: {}", target_path.display()))
    }

    // For Linux
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(target_path.to_str().unwrap_or(""))
            .output()
            .map_err(|e| format!("Failed to open directory: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open directory: {}", error));
        }

        Ok(format!("Opened directory: {}", target_path.display()))
    }

    // Fallback for unsupported platforms
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("Platform not supported for opening directories".to_string())
    }
}

#[command]
pub fn open_site_folder(path: String) -> Result<String, String> {
    // Check if the path exists
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // First, try to find the Moodle code directory
    let moodle_dir = path_buf.join("moodle");
    
    // If the standard "moodle" directory exists, use it
    let target_path = if moodle_dir.exists() && moodle_dir.is_dir() {
        moodle_dir
    } else {
        // Try to find any directory that might contain Moodle code
        // Look for index.php and version.php which are core Moodle files
        let mut found_dir = None;
        
        // Try to read directory entries in the path
        if let Ok(entries) = std::fs::read_dir(&path_buf) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        // Check for index.php and version.php in this directory
                        let index_php = entry_path.join("index.php");
                        let version_php = entry_path.join("version.php");
                        
                        if index_php.exists() && version_php.exists() {
                            found_dir = Some(entry_path);
                            break;
                        }
                    }
                }
            }
        }
        
        // If we found a directory with Moodle files, use it; otherwise use the original path
        found_dir.unwrap_or_else(|| path_buf.clone())
    };
    
    let target_path_str = target_path.to_str().unwrap_or(&path);

    // Open the site folder in the system file manager
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("open")
            .arg(target_path_str)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;

        if output.status.success() {
            Ok(format!("Opened site folder: {}", target_path_str))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, use "explorer" command and replace forward slashes with backslashes
        let windows_path = target_path_str.replace("/", "\\");
        let output = Command::new("explorer")
            .arg(&windows_path)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;

        if output.status.success() {
            Ok(format!("Opened site folder: {}", target_path_str))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("xdg-open")
            .arg(target_path_str)
            .output()
            .map_err(|e| format!("Failed to open site folder: {}", e))?;

        if output.status.success() {
            Ok(format!("Opened site folder: {}", target_path_str))
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to open site folder: {}", error))
        }
    }

    // Fallback for unsupported platforms
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("Platform not supported for opening directories".to_string())
    }
}

#[command]
pub fn open_in_editor(editor_name: String, path: String) -> Result<String, String> {
    use std::path::PathBuf;

    // Check if the path exists
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // First, try to find the Moodle code directory
    let moodle_dir = path_buf.join("moodle");

    // If the standard "moodle" directory exists, use it
    let target_path = if moodle_dir.exists() && moodle_dir.is_dir() {
        moodle_dir
    } else {
        // Try to find any directory that might contain Moodle code
        // Look for index.php or version.php which are core Moodle files
        let mut found_dir = None;

        // Try to read directory entries in the path
        if let Ok(entries) = fs::read_dir(&path_buf) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        // Check for index.php or version.php in this directory
                        let index_php = entry_path.join("index.php");
                        let version_php = entry_path.join("version.php");

                        if index_php.exists() && version_php.exists() {
                            found_dir = Some(entry_path);
                            break;
                        }
                    }
                }
            }
        }

        // If we found a directory with Moodle files, use it; otherwise use the original path
        found_dir.unwrap_or_else(|| path_buf.clone())
    };

    let target_path_str = target_path.to_str().unwrap_or(&path);

    #[cfg(target_os = "macos")]
    {
        // On macOS, use "open -a" command
        let output = Command::new("open")
            .arg("-a")
            .arg(&editor_name)
            .arg(target_path_str)
            .output()
            .map_err(|e| format!("Failed to open in editor: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open in editor: {}", error));
        }
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, try to open with the editor
        // This is a simple implementation that assumes the editor name is the executable
        let windows_path = target_path_str.replace("/", "\\"); // Replace forward slashes with backslashes
        let output = Command::new(&editor_name)
            .arg(&windows_path)
            .output()
            .map_err(|e| format!("Failed to open in editor: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open in editor: {}", error));
        }
    }

    #[cfg(target_os = "linux")]
    {
        // On Linux, try to run the editor directly
        let output = Command::new(&editor_name)
            .arg(target_path_str)
            .output()
            .map_err(|e| format!("Failed to open in editor: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(format!("Failed to open in editor: {}", error));
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        return Err("Opening in editor is not supported on this platform".to_string());
    }

    Ok(format!("Opened Moodle code in editor: {}", target_path_str))
}
