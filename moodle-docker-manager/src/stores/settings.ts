import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir, exists } from '@tauri-apps/api/fs';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    sitesRootDirectory: '',
    isLoading: false,
    error: null
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
        } else {
          // Create the config directory if it doesn't exist
          await createDir(configDir, { recursive: true });
          
          // Create default settings
          const defaultSettings = { sitesRootDirectory: '' };
          await writeTextFile(settingsPath, JSON.stringify(defaultSettings));
        }
      } catch (error) {
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
          JSON.stringify({ sitesRootDirectory: this.sitesRootDirectory })
        );
      } catch (error) {
        console.error('Failed to save settings:', error);
        this.error = `Failed to save settings: ${error}`;
      } finally {
        this.isLoading = false;
      }
    },
    
    async setSitesRootDirectory(directory) {
      this.sitesRootDirectory = directory;
      await this.saveSettings();
    }
  }
});
