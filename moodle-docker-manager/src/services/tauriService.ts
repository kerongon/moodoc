import { invoke } from '@tauri-apps/api/core';

/**
 * Service for Tauri-specific functionality
 */
export class TauriService {
  /**
   * Opens the system terminal at the specified path
   * @param path The directory path to open the terminal in
   */
  static async openTerminalAtPath(path: string): Promise<void> {
    try {
      const moodlePath = `${path}/moodle`;
      await invoke('open_terminal_at_path', { path: moodlePath });
    } catch (error) {
      console.error('Failed to open terminal:', error);
      throw error;
    }
  }
}
