// Update management functionality
// This module contains functions for updating Moodle versions and managing update operations

use std::process::Command;
use tauri::{command, AppHandle, Manager};

use super::{SiteState, RunningUpdates, safe_emit};

#[command]
pub async fn update_moodle_version(
    app_handle: AppHandle, 
    name: String, 
    new_branch: String, 
    new_php_version: Option<String>
) -> Result<String, String> {
    let handle = app_handle.clone();
    
    // Run the blocking update function in a separate thread
    let result = tokio::task::spawn_blocking(move || {
        update_moodle_version_blocking(handle, name, new_branch, new_php_version)
    }).await.map_err(|e| format!("Task execution error: {}", e))?;
    
    result
}

fn update_moodle_version_blocking(
    app_handle: AppHandle,
    name: String,
    new_branch: String,
    new_php_version: Option<String>
) -> Result<String, String> {
    let state = app_handle.state::<SiteState>();
    let running_updates = app_handle.state::<RunningUpdates>();
    
    // Check if an update is already running for this site
    {
        let updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        if updates.get(&name) == Some(&true) {
            return Err(format!("Update is already running for site '{}'", name));
        }
    }
    
    // Mark update as running
    {
        let mut updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        updates.insert(name.clone(), true);
    }
    
    let sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
    
    // Check if site exists
    let mut site = match sites.get(&name) {
        Some(site) => site.clone(),
        None => {
            // Remove from running updates
            let mut updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
            updates.remove(&name);
            return Err(format!("Site '{}' not found", name));
        }
    };
    
    // Drop the sites lock to avoid deadlock
    drop(sites);
    
    safe_emit(&app_handle, "update_progress", format!("Starting update for site '{}' to branch '{}'", name, new_branch));
    
    // Stop the site before updating
    safe_emit(&app_handle, "update_progress", format!("Stopping site '{}'", name));
    if let Err(e) = super::site_management::stop_site(app_handle.clone(), name.clone()) {
        // Remove from running updates
        let mut updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        updates.remove(&name);
        return Err(format!("Failed to stop site before update: {}", e));
    }
    
    // Update Moodle branch
    safe_emit(&app_handle, "update_progress", format!("Updating Moodle to branch '{}'", new_branch));
    let moodle_dir = site.path.join("moodle");
    
    // Backup current branch info (in case we need to rollback)
    let current_branch_output = Command::new("git")
        .current_dir(&moodle_dir)
        .args(["branch", "--show-current"])
        .output()
        .map_err(|e| {
            // Remove from running updates
            if let Ok(mut updates) = running_updates.0.lock() {
                updates.remove(&name);
            }
            format!("Failed to get current branch: {}", e)
        })?;
    
    let _current_branch = String::from_utf8_lossy(&current_branch_output.stdout).trim().to_string();
    
    // Fetch latest changes
    safe_emit(&app_handle, "update_progress", format!("Fetching latest changes from repository"));
    let fetch_output = Command::new("git")
        .current_dir(&moodle_dir)
        .args(["fetch", "origin"])
        .output()
        .map_err(|e| {
            // Remove from running updates
            if let Ok(mut updates) = running_updates.0.lock() {
                updates.remove(&name);
            }
            format!("Failed to fetch repository: {}", e)
        })?;
    
    if !fetch_output.status.success() {
        let error = String::from_utf8_lossy(&fetch_output.stderr);
        // Remove from running updates
        if let Ok(mut updates) = running_updates.0.lock() {
            updates.remove(&name);
        }
        return Err(format!("Failed to fetch repository: {}", error));
    }
    
    // Switch to new branch
    safe_emit(&app_handle, "update_progress", format!("Switching to branch '{}'", new_branch));
    let checkout_output = Command::new("git")
        .current_dir(&moodle_dir)
        .args(["checkout", &new_branch])
        .output()
        .map_err(|e| {
            // Remove from running updates
            if let Ok(mut updates) = running_updates.0.lock() {
                updates.remove(&name);
            }
            format!("Failed to checkout branch: {}", e)
        })?;
    
    if !checkout_output.status.success() {
        let error = String::from_utf8_lossy(&checkout_output.stderr);
        // Remove from running updates
        if let Ok(mut updates) = running_updates.0.lock() {
            updates.remove(&name);
        }
        return Err(format!("Failed to checkout branch '{}': {}", new_branch, error));
    }
    
    // Pull latest changes for the new branch
    safe_emit(&app_handle, "update_progress", format!("Pulling latest changes for branch '{}'", new_branch));
    let pull_output = Command::new("git")
        .current_dir(&moodle_dir)
        .args(["pull", "origin", &new_branch])
        .output()
        .map_err(|e| {
            // Remove from running updates
            if let Ok(mut updates) = running_updates.0.lock() {
                updates.remove(&name);
            }
            format!("Failed to pull latest changes: {}", e)
        })?;
    
    if !pull_output.status.success() {
        let error = String::from_utf8_lossy(&pull_output.stderr);
        // Remove from running updates
        if let Ok(mut updates) = running_updates.0.lock() {
            updates.remove(&name);
        }
        return Err(format!("Failed to pull latest changes: {}", error));
    }
    
    // Update site configuration
    site.moodle_branch = new_branch.clone();
    if let Some(php_version) = new_php_version {
        site.php_version = php_version;
    }
    
    // Save updated site configuration
    if let Err(e) = super::save_site_config(app_handle.clone(), site.clone()) {
        // Remove from running updates
        if let Ok(mut updates) = running_updates.0.lock() {
            updates.remove(&name);
        }
        return Err(format!("Failed to save updated site configuration: {}", e));
    }
    
    // Update the in-memory state
    {
        let state = app_handle.state::<SiteState>();
        let mut sites = state.0.lock().map_err(|_| "Failed to lock sites state".to_string())?;
        sites.insert(name.clone(), site);
    }
    
    // Restart the site with updated configuration
    safe_emit(&app_handle, "update_progress", format!("Restarting site '{}' with updated configuration", name));
    if let Err(e) = super::site_management::start_site(app_handle.clone(), name.clone()) {
        // Remove from running updates
        if let Ok(mut updates) = running_updates.0.lock() {
            updates.remove(&name);
        }
        return Err(format!("Failed to restart site after update: {}", e));
    }
    
    // Run database upgrade if needed
    safe_emit(&app_handle, "update_progress", format!("Running database upgrade for site '{}'", name));
    let upgrade_output = Command::new("docker")
        .args([
            "exec", 
            &format!("{}_webserver_1", name),
            "php", 
            "/var/www/html/admin/cli/upgrade.php",
            "--non-interactive"
        ])
        .output()
        .map_err(|e| {
            // Remove from running updates
            if let Ok(mut updates) = running_updates.0.lock() {
                updates.remove(&name);
            }
            format!("Failed to run database upgrade: {}", e)
        })?;
    
    if !upgrade_output.status.success() {
        let error = String::from_utf8_lossy(&upgrade_output.stderr);
        safe_emit(&app_handle, "update_warning", format!("Database upgrade may have encountered issues: {}", error));
    }
    
    // Remove from running updates
    {
        let mut updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        updates.remove(&name);
    }
    
    safe_emit(&app_handle, "update_completed", format!("Update completed successfully for site '{}'", name));
    Ok(format!("Site '{}' updated successfully to branch '{}'", name, new_branch))
}

#[command]
pub fn cancel_update(app_handle: AppHandle, name: String) -> Result<String, String> {
    let running_updates = app_handle.state::<RunningUpdates>();
    
    // Check if an update is running for this site
    {
        let updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        if updates.get(&name) != Some(&true) {
            return Err(format!("No update is currently running for site '{}'", name));
        }
    }
    
    // For now, we'll just mark the update as cancelled
    // In a more sophisticated implementation, we could send a signal to the running task
    {
        let mut updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
        updates.remove(&name);
    }
    
    safe_emit(&app_handle, "update_cancelled", format!("Update cancelled for site '{}'", name));
    Ok(format!("Update cancelled for site '{}'", name))
}

// Helper function to check if an update is running for a site
pub fn is_update_running(app_handle: &AppHandle, name: &str) -> Result<bool, String> {
    let running_updates = app_handle.state::<RunningUpdates>();
    let updates = running_updates.0.lock().map_err(|_| "Failed to lock running updates state".to_string())?;
    Ok(updates.get(name) == Some(&true))
}
