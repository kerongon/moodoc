import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir, exists } from '@tauri-apps/api/fs';

// Define a type for the window extension
declare global {
  interface Window {
    ThemeManager?: {
      applyTheme(mode: string): boolean;
    };
  }
}

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    sitesRootDirectory: '',
    isLoading: false,
    error: null as string | null,
    darkMode: 'dark' as 'system' | 'light' | 'dark',
  }),
  
  actions: {
    async loadSettings() {
      this.isLoading = true;
      this.error = null;
      
      try {
        // Get the app config directory from Tauri
        const configDir = await appConfigDir();
        const settingsPath = `${configDir}/settings.json`;
        
        // Check if the settings file exists
        const fileExists = await exists(settingsPath);
        
        if (fileExists) {
          // Read the settings file
          const settingsContent = await readTextFile(settingsPath);
          const settings = JSON.parse(settingsContent);
          this.sitesRootDirectory = settings.sitesRootDirectory || '';
          this.darkMode = settings.darkMode || 'system';
        } else {
          // Create the config directory if it doesn't exist
          await createDir(configDir, { recursive: true });
          
          // Create default settings with dark mode as default
          const defaultSettings = { 
            sitesRootDirectory: '',
            darkMode: 'dark'
          };
          await writeTextFile(settingsPath, JSON.stringify(defaultSettings));
        }
      } catch (error: any) {
        console.error('Failed to load settings:', error);
        this.error = `Failed to load settings: ${error}`;
      } finally {
        this.isLoading = false;
      }
    },
    
    async saveSettings() {
      this.isLoading = true;
      this.error = null;
      
      try {
        const configDir = await appConfigDir();
        await writeTextFile(
          `${configDir}/settings.json`, 
          JSON.stringify({ 
            sitesRootDirectory: this.sitesRootDirectory,
            darkMode: this.darkMode
          })
        );
      } catch (error: any) {
        console.error('Failed to save settings:', error);
        this.error = `Failed to save settings: ${error}`;
      } finally {
        this.isLoading = false;
      }
    },
    
    async setSitesRootDirectory(directory: string) {
      this.sitesRootDirectory = directory;
      await this.saveSettings();
    },
    
    async setDarkMode(mode: 'system' | 'light' | 'dark') {
      this.darkMode = mode;
      await this.saveSettings();
      
      // Use the ThemeManager to apply the theme if available
      if (window.ThemeManager) {
        window.ThemeManager.applyTheme(mode);
      } else {
        // Fall back to the old method if ThemeManager is not available
        this.applyDarkMode(true);
      }
    },
    
    applyDarkMode(force = false) {
      // Use ThemeManager if available
      if (window.ThemeManager) {
        window.ThemeManager.applyTheme(this.darkMode);
        return;
      }
      
      // Legacy implementation as fallback
      const html = document.documentElement;
      const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
      
      if (this.darkMode === 'dark' || (this.darkMode === 'system' && prefersDarkScheme)) {
        html.classList.add('dark');
        if (force) {
          document.body.style.setProperty('background-color', '#111827', 'important');
          document.body.style.setProperty('color', '#f9fafb', 'important');
        }
      } else {
        html.classList.remove('dark');
        if (force) {
          document.body.style.setProperty('background-color', '#ffffff', 'important');
          document.body.style.setProperty('color', '#111827', 'important');
        }
      }
    }
  }
});
