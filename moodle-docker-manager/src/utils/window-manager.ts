// Functions for handling windows in the application
import { WebviewWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';

/**
 * Opens a logs window for a specific site in a separate process
 * 
 * @param {string} siteName - The name of the site to view logs for
 * @param {string} initialService - The initial service to show logs for (defaults to webserver)
 */
export async function openLogsWindow(siteName: string, initialService: string = 'webserver'): Promise<void> {
  try {
    // Use the Rust function to create the window
    await invoke('open_logs_window', {
      siteName,
      service: initialService
    });
    
    console.log(`Logs window for ${siteName} requested`);
  } catch (error) {
    console.error(`Error opening logs window: ${error}`);
    
    // Fallback to JavaScript API if the Rust function fails
    const label = `logs-${siteName.replace(/[^a-zA-Z0-9]/g, '-')}`;
    
    // Check if window already exists
    const existingWindow = WebviewWindow.getByLabel(label);
    if (existingWindow) {
      existingWindow.setFocus();
      return;
    }
    
    // Create a new window
    const logWindow = new WebviewWindow(label, {
      url: `/#/log-window?siteName=${encodeURIComponent(siteName)}&service=${encodeURIComponent(initialService)}`,
      title: `Logs: ${siteName} - ${initialService}`,
      width: 900,
      height: 700,
      minWidth: 600,
      minHeight: 400,
    });
    
    // Handle window events if needed
    logWindow.once('tauri://created', () => {
      console.log(`Logs window for ${siteName} created (fallback method)`);
    });
  }
}