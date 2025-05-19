import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir, exists } from '@tauri-apps/api/fs';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    sitesRootDirectory: '',
    isLoading: false,
    error: null,
    darkMode: 'system', // 'system', 'light', or 'dark'
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
          
          // Create default settings
          const defaultSettings = { 
            sitesRootDirectory: '',
            darkMode: 'system'
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
      
      // Apply the dark mode
      this.applyDarkMode();
    },
    
    applyDarkMode() {
      // Get the HTML element
      const html = document.documentElement;
      
      // Apply dark mode based on the setting
      if (this.darkMode === 'dark') {
        html.classList.add('dark');
      } else if (this.darkMode === 'light') {
        html.classList.remove('dark');
      } else {
        // System preference - check the prefers-color-scheme
        const prefersDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
        if (prefersDarkMode) {
          html.classList.add('dark');
        } else {
          html.classList.remove('dark');
        }
        
        // Add a listener for system preference changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
          if (this.darkMode === 'system') {
            if (e.matches) {
              html.classList.add('dark');
            } else {
              html.classList.remove('dark');
            }
          }
        });
      }
    }
  }
});
