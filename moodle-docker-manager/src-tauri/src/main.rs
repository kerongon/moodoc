#![cfg_attr(
    all(not(debug_assertions), target_os = "macos"),
    windows_subsystem = "windows"
)]

mod docker;
mod site_metrics;

use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{generate_handler, Manager, WebviewWindowBuilder, WebviewUrl};

// Function to open a logs window from Rust
#[tauri::command]
async fn open_logs_window(
    window: tauri::Window,
    site_name: String,
    service: String,
) -> Result<(), String> {
    // Create a unique label for the window
    let label = format!("logs-{}", site_name.replace(" ", "-"));

    // Get the app handle
    let app_handle = window.app_handle();

    // Check if the window already exists
    if let Some(existing_window) = app_handle.get_webview_window(&label) {
        existing_window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create the window URL with query parameters
    let url = WebviewUrl::App(
        format!(
            "/#/log-window?siteName={}&service={}",
            urlencoding::encode(&site_name),
            urlencoding::encode(&service)
        )
        .into(),
    );

    // Build new window
    WebviewWindowBuilder::new(app_handle, label, url)
        .title(format!("Logs: {} - {}", site_name, service))
        .inner_size(900.0, 700.0)
        .min_inner_size(600.0, 400.0)
        .resizable(true)
        .decorations(true)
        .fullscreen(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // Use a timer for smoother transition between splash and main app
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                // Wait for animation to complete (7.5 seconds - longer for smoother experience)
                std::thread::sleep(std::time::Duration::from_secs(7));

                // First trigger fade-out animation in splash screen
                if let Some(splash_window) = handle.get_webview_window("splashscreen") {
                    let _ = splash_window.eval("document.body.classList.add('fade-out');");
                    // Give the fade-out animation time to complete (800ms for smoother transition)
                    std::thread::sleep(std::time::Duration::from_millis(800));
                }

                // Show main window after fade-out starts
                if let Some(main_window) = handle.get_webview_window("main") {
                    let _ = main_window.show();
                }

                // Close splash window after fade-out completes
                if let Some(splash_window) = handle.get_webview_window("splashscreen") {
                    let _ = splash_window.close();
                }
            });
            // Load any existing site configurations using the store
            if let Err(err) = docker::load_site_configs_from_store(app) {
                eprintln!("Failed to load site configurations: {}", err);
            }
            Ok(())
        })
        .manage(docker::SiteState(Mutex::new(HashMap::new())))
        .manage(docker::RunningUpdates::default())
        .manage(docker::RunningCreations::default())
        .invoke_handler(generate_handler![
            docker::check_docker_installed,
            docker::check_docker_running,
            docker::check_platform_compatibility,
            docker::validate_environment,
            docker::create_site,
            docker::cancel_site_creation,
            docker::list_sites,
            docker::start_site,
            docker::stop_site,
            docker::remove_site,
            docker::get_docker_logs,
            docker::check_site_status,
            docker::check_site_status_detailed,
            docker::run_diagnostics,
            docker::backup_site,
            docker::list_site_backups,
            docker::restore_site_backup,
            docker::restore_site_backup_async,
            docker::delete_site_backup,
            docker::open_backup_location,
            docker::update_moodle_version,
            docker::open_site_folder,
            docker::open_in_editor,
            docker::cancel_update,
            docker::check_port_available,
            site_metrics::get_site_size,
            site_metrics::get_storage_info,
            open_logs_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
