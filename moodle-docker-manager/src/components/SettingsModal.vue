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
import { ref, computed, onMounted } from 'vue';
import { useSettingsStore } from '../stores/settings';
import { open } from '@tauri-apps/api/dialog';
import { listen } from '@tauri-apps/api/event';

export default {
  setup() {
    const settingsStore = useSettingsStore();
    const sitesDirectory = ref('');
    const isVisible = ref(false);
    
    const currentDirectory = computed(() => {
      return settingsStore.sitesRootDirectory || '';
    });
    
    onMounted(async () => {
      await settingsStore.loadSettings();
      sitesDirectory.value = settingsStore.sitesRootDirectory;
      
      // Listen for the open-settings event from the menu
      await listen('open-settings', () => {
        // Update directory value when modal is opened
        sitesDirectory.value = settingsStore.sitesRootDirectory;
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
      closeModal();
    };
    
    const closeModal = () => {
      isVisible.value = false;
    };
    
    return {
      settingsStore,
      sitesDirectory,
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

.browse-button {
  padding: 8px 15px;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
}

.browse-button:hover {
  background-color: #e9e9e9;
}

.current-directory {
  font-size: 12px;
  color: #666;
  margin-top: 5px;
}

.modal-footer {
  padding: 15px 20px;
  border-top: 1px solid #eaeaea;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.cancel-button {
  padding: 8px 15px;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
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

.save-button:hover {
  background-color: #0060df;
}

.save-button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}
</style>
