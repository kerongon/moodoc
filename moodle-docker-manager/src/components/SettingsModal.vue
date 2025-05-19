<template>
  <div v-if="isVisible" class="settings-modal" @click.self="closeModal">
    <div class="settings-modal-content mac-style">
      <div class="modal-header">
        <h2>Preferences</h2>
        <button @click="closeModal" class="close-button">×</button>
      </div>
      
      <div class="settings-body">
        <div class="setting-group">
          <h3>Default Sites Directory</h3>
          <p class="setting-description">
            Choose where Moodle sites will be created by default
          </p>
          
          <div class="directory-selector">
            <input 
              type="text" 
              v-model="sitesDirectory" 
              placeholder="Select a directory" 
              readonly
            />
            <button @click="selectDirectory" class="browse-button">Choose...</button>
          </div>
          
          <div class="current-directory">
            <span v-if="currentDirectory">Current directory: {{ currentDirectory }}</span>
            <span v-else>No directory selected (will use current working directory)</span>
          </div>
        </div>
        
        <div class="setting-group">
          <h3>Appearance</h3>
          <p class="setting-description">
            Choose your preferred theme mode
          </p>
          
          <div class="theme-selector">
            <label class="theme-option" :class="{ active: darkMode === 'light' }">
              <input type="radio" name="theme" value="light" v-model="darkMode" />
              <svg class="theme-icon light-icon" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" />
              </svg>
              <span>Light</span>
            </label>
            
            <label class="theme-option" :class="{ active: darkMode === 'dark' }">
              <input type="radio" name="theme" value="dark" v-model="darkMode" />
              <svg class="theme-icon dark-icon" viewBox="0 0 20 20" fill="currentColor">
                <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
              </svg>
              <span>Dark</span>
            </label>
            
            <label class="theme-option" :class="{ active: darkMode === 'system' }">
              <input type="radio" name="theme" value="system" v-model="darkMode" />
              <svg class="theme-icon system-icon" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 5a2 2 0 012-2h10a2 2 0 012 2v8a2 2 0 01-2 2h-2.22l.123.489.804.804A1 1 0 0113 18H7a1 1 0 01-.707-1.707l.804-.804L7.22 15H5a2 2 0 01-2-2V5zm5.771 7H5V5h10v7H8.771z" clip-rule="evenodd" />
              </svg>
              <span>System</span>
            </label>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button @click="closeModal" class="cancel-button">Cancel</button>
        <button 
          @click="saveSettings" 
          class="save-button"
          :disabled="settingsStore.isLoading"
        >
          Save
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, watch } from 'vue';
import { useSettingsStore } from '../stores/settings';
import { open } from '@tauri-apps/api/dialog';
import { listen } from '@tauri-apps/api/event';

export default {
  setup() {
    const settingsStore = useSettingsStore();
    const sitesDirectory = ref('');
    const darkMode = ref('system');
    const isVisible = ref(false);
    
    const currentDirectory = computed(() => {
      return settingsStore.sitesRootDirectory || '';
    });
    
    // Watch for dark mode changes and apply them immediately
    watch(darkMode, (newMode) => {
      settingsStore.setDarkMode(newMode);
    });
    
    onMounted(async () => {
      await settingsStore.loadSettings();
      sitesDirectory.value = settingsStore.sitesRootDirectory;
      darkMode.value = settingsStore.darkMode;
      
      // Listen for the open-settings event from the menu
      await listen('open-settings', () => {
        // Update values when modal is opened
        sitesDirectory.value = settingsStore.sitesRootDirectory;
        darkMode.value = settingsStore.darkMode;
        isVisible.value = true;
      });
    });
    
    const selectDirectory = async () => {
      try {
        const selected = await open({
          directory: true,
          multiple: false,
          title: 'Select Default Sites Directory'
        });
        
        if (selected) {
          sitesDirectory.value = selected;
        }
      } catch (error) {
        console.error('Error selecting directory:', error);
      }
    };
    
    const saveSettings = async () => {
      await settingsStore.setSitesRootDirectory(sitesDirectory.value);
      await settingsStore.setDarkMode(darkMode.value);
      closeModal();
    };
    
    const closeModal = () => {
      isVisible.value = false;
    };
    
    return {
      settingsStore,
      sitesDirectory,
      darkMode,
      currentDirectory,
      isVisible,
      selectDirectory,
      saveSettings,
      closeModal
    };
  }
};
</script>

<style scoped>
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.settings-modal-content {
  width: 500px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 23px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

.dark .settings-modal-content {
  background-color: #1f2937;
  box-shadow: 0 4px 23px rgba(0, 0, 0, 0.5);
  color: #e5e7eb;
}

.mac-style {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.modal-header {
  padding: 15px 20px;
  border-bottom: 1px solid #eaeaea;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dark .modal-header {
  border-bottom: 1px solid #374151;
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 500;
}

.close-button {
  background: none;
  border: none;
  font-size: 22px;
  cursor: pointer;
  color: #999;
}

.dark .close-button {
  color: #d1d5db;
}

.settings-body {
  padding: 20px;
  max-height: 60vh;
  overflow-y: auto;
}

.setting-group {
  margin-bottom: 20px;
}

.setting-group h3 {
  font-size: 16px;
  margin-top: 0;
  margin-bottom: 10px;
  font-weight: 500;
}

.setting-description {
  color: #666;
  margin-bottom: 15px;
  font-size: 14px;
}

.dark .setting-description {
  color: #9ca3af;
}

.directory-selector {
  display: flex;
  gap: 10px;
  margin-bottom: 10px;
}

.directory-selector input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #f5f5f5;
  font-size: 14px;
}

.dark .directory-selector input {
  background-color: #374151;
  border-color: #4b5563;
  color: #e5e7eb;
}

.browse-button {
  padding: 8px 15px;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.dark .browse-button {
  background-color: #374151;
  border-color: #4b5563;
  color: #e5e7eb;
}

.browse-button:hover {
  background-color: #e9e9e9;
}

.dark .browse-button:hover {
  background-color: #4b5563;
}

.current-directory {
  font-size: 12px;
  color: #666;
  margin-top: 5px;
}

.theme-selector {
  display: flex;
  gap: 15px;
  margin-top: 15px;
}

.theme-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px;
  border: 1px solid #eaeaea;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  flex: 1;
}

.theme-option:hover {
  background-color: #f5f5f5;
}

.dark .theme-option {
  border-color: #4b5563;
  color: #e5e7eb;
}

.dark .theme-option:hover {
  background-color: #374151;
}

.theme-option.active {
  border-color: #0070f3;
  background-color: #f0f7ff;
}

.dark .theme-option.active {
  border-color: #3b82f6;
  background-color: #1e40af33;
}

.theme-option input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.theme-icon {
  width: 24px;
  height: 24px;
  margin-bottom: 8px;
}

.light-icon {
  color: #ff9800;
}

.dark-icon {
  color: #555;
}

.system-icon {
  color: #0070f3;
}

.theme-option span {
  font-size: 14px;
}

.modal-footer {
  padding: 15px 20px;
  border-top: 1px solid #eaeaea;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.dark .modal-footer {
  border-top: 1px solid #374151;
}

.cancel-button {
  padding: 8px 15px;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.dark .cancel-button {
  background-color: #374151;
  border-color: #4b5563;
  color: #e5e7eb;
}

.save-button {
  padding: 8px 15px;
  background-color: #0070f3;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.dark .save-button {
  background-color: #3b82f6;
}

.save-button:hover {
  background-color: #0060df;
}

.dark .save-button:hover {
  background-color: #2563eb;
}

.save-button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.dark .save-button:disabled {
  background-color: #6b7280;
}
</style>
