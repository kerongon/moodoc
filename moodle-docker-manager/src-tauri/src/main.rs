#![cfg_attr(
  all(not(debug_assertions), target_os = "macos"),
  windows_subsystem = "windows"
)]

mod docker;

use tauri::{generate_handler, CustomMenuItem, Menu, MenuItem, Submenu, WindowUrl, WindowBuilder};
use tauri::Manager;
use std::collections::HashMap;
use std::sync::Mutex;

// Function to open a logs window from Rust
#[tauri::command]
async fn open_logs_window(window: tauri::Window, site_name: String, service: String) -> Result<(), String> {
    // Create a unique label for the window
    let label = format!("logs-{}", site_name.replace(" ", "-"));
    
    // Get the app handle
    let app_handle = window.app_handle();
    
    // Check if the window already exists
    if let Some(existing_window) = app_handle.get_window(&label) {
        existing_window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    
    // Create the window URL with query parameters
    let url = WindowUrl::App(format!("/#/log-window?siteName={}&service={}", 
                                     urlencoding::encode(&site_name), 
                                     urlencoding::encode(&service)).into());
    
    // Build new window
    WindowBuilder::new(&app_handle, label, url)
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
    // Create the application menu (macOS style)
    let app_menu = Submenu::new("Moodle Docker Manager", Menu::new()
        .add_native_item(MenuItem::About(
            "Moodle Docker Manager".to_string(),
            tauri::AboutMetadata::default()
        ))
        .add_item(CustomMenuItem::new("preferences".to_string(), "Preferences...").accelerator("Cmd+,"))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit)
    );

    // Create edit menu with standard actions
    let edit_menu = Submenu::new("Edit", Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll)
    );

    // Combine all menus
    let menu = Menu::new()
        .add_submenu(app_menu)
        .add_submenu(edit_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "preferences" => {
                    let window = event.window();
                    // Emit event to show the settings modal
                    window.emit("open-settings", {}).unwrap();
                }
                _ => {}
            }
        })
        .manage(docker::SiteState(Mutex::new(HashMap::new())))
        .manage(docker::RunningUpdates::default())
        .setup(|app| {
            // Use a simple timer to show main window and close splash
            let handle = app.handle();
            std::thread::spawn(move || {
                // Wait a moment for splash to display
                std::thread::sleep(std::time::Duration::from_millis(1500));
                
                // Get window references after the delay
                if let Some(main_window) = handle.get_window("main") {
                    let _ = main_window.show();
                }
                
                if let Some(splash_window) = handle.get_window("splashscreen") {
                    let _ = splash_window.close();
                }
            });
            // Load any existing site configurations
            if let Err(err) = docker::load_site_configs(app) {
                eprintln!("Failed to load site configurations: {}", err);
            }
            Ok(())
        })
        .invoke_handler(generate_handler![
            docker::check_docker_installed,
            docker::check_docker_running,
            docker::check_platform_compatibility,
            docker::validate_environment,
            docker::create_site,
            docker::list_sites,
            docker::start_site,
            docker::stop_site,
            docker::remove_site,
            docker::get_docker_logs,
            docker::check_site_status,
            docker::backup_site,
            docker::list_site_backups,
            docker::restore_site_backup,
            docker::delete_site_backup,
            docker::open_backup_location,
            docker::update_moodle_version,
            docker::open_site_folder,
            docker::cancel_update,
            open_logs_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}