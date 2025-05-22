import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { appConfigDir } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir, exists } from '@tauri-apps/api/fs';

// Define a type for the window extension
declare global {
  interface Window {
    ThemeManager: {
      applyTheme(mode: string): boolean;
      refreshTheme(): void;
    };
  }
}

// Helper function to safely access localStorage (may not exist in some contexts)
const getFromLocalStorage = (key: string, defaultValue: string): string => {
  try {
    const value = localStorage.getItem(key);
    return value !== null ? value : defaultValue;
  } catch (e) {
    return defaultValue;
  }
};

// Helper function to safely set localStorage (may not exist in some contexts)
const setToLocalStorage = (key: string, value: string): void => {
  try {
    localStorage.setItem(key, value);
  } catch (e) {
    console.error('Failed to save to localStorage:', e);
  }
};

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    sitesRootDirectory: getFromLocalStorage('sitesRootDirectory', ''),
    isLoading: false,
    error: null as string | null,
    darkMode: getFromLocalStorage('theme', 'dark') as 'system' | 'light' | 'dark',
    // Path to the user's preferred code editor application
    editorPath: getFromLocalStorage('editorPath', '') as string,
    initialized: false,
  }),
  
  actions: {
    async loadSettings() {
      this.isLoading = true;
      this.error = null;
      
      try {
        console.log('Loading settings from disk and localStorage...');
        
        // Get the app config directory from Tauri
        const configDir = await appConfigDir();
        const settingsPath = `${configDir}/settings.json`;
        
        // Check if the settings file exists
        const fileExists = await exists(settingsPath);
        
        if (fileExists) {
          // Read the settings file
          const settingsContent = await readTextFile(settingsPath);
          const settings = JSON.parse(settingsContent);
          
          console.log('Loaded settings from disk:', settings);
          
          // Update state from the saved settings
          if (settings.sitesRootDirectory) {
            this.sitesRootDirectory = settings.sitesRootDirectory;
            // Sync localStorage with the loaded setting
            setToLocalStorage('sitesRootDirectory', settings.sitesRootDirectory);
          }
          
          // Update theme mode if available in settings
          if (settings.darkMode) {
            this.darkMode = settings.darkMode;
            setToLocalStorage('theme', settings.darkMode);
          }
          // Update editor path if available in settings
          if (settings.editorPath) {
            this.editorPath = settings.editorPath;
            setToLocalStorage('editorPath', settings.editorPath);
          }
        } else {
          console.log('No settings file found, creating one with current values');
          // Create the config directory if it doesn't exist
          await createDir(configDir, { recursive: true });
          
          // Use current state values (which might be from localStorage)
          // and create the initial settings file
          const initialSettings = {
            sitesRootDirectory: this.sitesRootDirectory,
            darkMode: this.darkMode,
            editorPath: this.editorPath
          };
          
          // Save initial settings to disk
          await writeTextFile(settingsPath, JSON.stringify(initialSettings, null, 2));
          console.log('Created initial settings file with:', initialSettings);
        }
        
        // Apply the theme from our settings
        if (window.ThemeManager) {
          window.ThemeManager.applyTheme(this.darkMode);
        } else {
          this.applyDarkMode(true);
        }
        
        // Mark as initialized to prevent redundant loading
        this.initialized = true;
      } catch (error: any) {
        console.error('Failed to load settings:', error);
        this.error = `Failed to load settings: ${error}`;
        
        // Even if there was an error, try to use localStorage values as fallback
        try {
          const localTheme = getFromLocalStorage('theme', 'dark');
          const localSitesDir = getFromLocalStorage('sitesRootDirectory', '');
          
          this.darkMode = localTheme as 'system' | 'light' | 'dark';
          this.sitesRootDirectory = localSitesDir;
          
          console.log('Using localStorage fallback values:', { 
            darkMode: this.darkMode, 
            sitesRootDirectory: this.sitesRootDirectory 
          });
          
          // Apply theme from localStorage
          if (window.ThemeManager) {
            window.ThemeManager.applyTheme(this.darkMode);
          }
        } catch (e) {
          console.error('Failed to use localStorage fallback:', e);
        }
      } finally {
        this.isLoading = false;
      }
    },
    
    async saveSettings() {
      this.isLoading = true;
      this.error = null;
      
      try {
        // Make sure the config directory exists
        const configDir = await appConfigDir();
        
        // Create the directory if it doesn't exist yet
        const dirExists = await exists(configDir);
        if (!dirExists) {
          console.log(`Creating config directory: ${configDir}`);
          await createDir(configDir, { recursive: true });
        }
        
        // Prepare settings object with current values
        const settings = {
          sitesRootDirectory: this.sitesRootDirectory,
          darkMode: this.darkMode,
          editorPath: this.editorPath
        };
        
        console.log('Saving settings to disk:', settings);
        
        // Save to localStorage for quick access
        setToLocalStorage('theme', this.darkMode);
        setToLocalStorage('sitesRootDirectory', this.sitesRootDirectory);
        setToLocalStorage('editorPath', this.editorPath);
        
        // Write to the config file with pretty formatting (for debugging)
        const settingsPath = `${configDir}/settings.json`;
        await writeTextFile(
          settingsPath, 
          JSON.stringify(settings, null, 2)
        );
        
        // Verify the file was written successfully
        const fileExists = await exists(settingsPath);
        if (!fileExists) {
          throw new Error('Settings file was not created successfully');
        }
        
        console.log('Settings saved successfully to:', settingsPath);
        this.initialized = true;
      } catch (error: any) {
        console.error('Failed to save settings:', error);
        this.error = `Failed to save settings: ${error}`;
        
        // At least try to save to localStorage as fallback
        setToLocalStorage('theme', this.darkMode);
        setToLocalStorage('sitesRootDirectory', this.sitesRootDirectory);
      } finally {
        this.isLoading = false;
      }
    },
    
    async setSitesRootDirectory(directory: string) {
      this.sitesRootDirectory = directory;
      
      // Update localStorage immediately for fast access on reload
      setToLocalStorage('sitesRootDirectory', directory);
      
      // Save to persistent storage
      await this.saveSettings();
    },
    
    async setDarkMode(mode: 'system' | 'light' | 'dark') {
      this.darkMode = mode;
      
      // Update localStorage immediately for fast access on reload
      setToLocalStorage('theme', mode);
      
      // Use the ThemeManager to apply the theme if available
      if (window.ThemeManager) {
        window.ThemeManager.applyTheme(mode);
      } else {
        // Fall back to the old method if ThemeManager is not available
        this.applyDarkMode(true);
      }
      
      // Save to persistent storage
      await this.saveSettings();
    },
    
    /**
     * Set the preferred code editor application path
     */
    async setEditorPath(path: string) {
      this.editorPath = path;
      // Update localStorage immediately
      setToLocalStorage('editorPath', path);
      // Save to persistent storage
      await this.saveSettings();
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
    },

    /**
     * Reset all settings to default values and clear stored settings
     */
    async resetSettings() {
      this.isLoading = true;
      this.error = null;
      
      try {
        console.log('Resetting all settings to default values');
        
        // Reset state to default values
        this.sitesRootDirectory = '';
        this.darkMode = 'system';
        this.editorPath = '';
        
        // Clear localStorage
        try {
          localStorage.removeItem('theme');
          localStorage.removeItem('sitesRootDirectory');
          localStorage.removeItem('editorPath');
          console.log('Cleared settings from localStorage');
        } catch (e) {
          console.warn('Could not clear localStorage:', e);
        }
        
        // Delete settings file if possible
        try {
          const configDir = await appConfigDir();
          const settingsPath = `${configDir}/settings.json`;
          
          // Check if the file exists first
          const fileExists = await exists(settingsPath);
          if (fileExists) {
            // In Tauri API, there's no direct delete, so we overwrite with empty settings
            await writeTextFile(
              settingsPath,
              JSON.stringify({
                sitesRootDirectory: '',
                darkMode: 'system',
                editorPath: ''
              }, null, 2)
            );
            console.log('Reset settings file at:', settingsPath);
          }
        } catch (error) {
          console.error('Failed to reset settings file:', error);
          this.error = `Failed to reset settings file: ${error}`;
        }
        
        // Apply default theme
        if (window.ThemeManager) {
          window.ThemeManager.applyTheme('system');
        } else {
          this.applyDarkMode(true);
        }
        
        console.log('Settings reset to defaults');
      } catch (error) {
        console.error('Error during settings reset:', error);
        this.error = `Error during settings reset: ${error}`;
      } finally {
        this.isLoading = false;
      }
    }
  }
});
