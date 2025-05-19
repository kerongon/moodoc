<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 max-w-2xl mx-auto my-8 p-6 dark:text-gray-100">
    <div class="flex items-center justify-center mb-6">
      <div class="bg-blue-600 rounded-lg p-2 mr-4">
        <PhGear class="text-white w-8 h-8" weight="fill" />
      </div>
      <h2 class="text-xl font-semibold">Settings</h2>
    </div>
    
    <div class="space-y-6">
      <!-- Site Root Directory -->
      <div class="mb-5 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
        <label class="block text-sm font-bold text-gray-700 dark:text-gray-300 mb-2 flex items-center">
          <svg class="w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
          </svg>
          Site Root Directory
        </label>
        <div class="flex">
          <input 
            type="text" 
            id="sitesRootDirectory" 
            v-model="sitesRootDirectory" 
            placeholder="Path to store Moodle sites" 
            class="macos-input rounded-r-none flex-1"
          />
          <button 
            type="button" 
            @click="selectRootDirectory" 
            class="px-4 border border-l-0 border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 rounded-r-md text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600"
          >
            Browse...
          </button>
        </div>
        <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
          This is where all your Moodle Docker sites will be stored
        </p>
      </div>
      
      <!-- Appearance Settings -->
      <div class="mb-5 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
        <label class="block text-sm font-bold text-gray-700 dark:text-gray-300 mb-3 flex items-center">
          <svg class="w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
          Appearance
        </label>
        
        <div class="mb-3">
          <label class="block text-sm text-gray-600 dark:text-gray-400 mb-1">Theme Mode</label>
          <div class="flex space-x-3 mt-2">
            <label class="flex items-center p-2 border rounded-md cursor-pointer" :class="darkMode === 'light' ? 'bg-blue-50 border-blue-200 dark:bg-blue-900 dark:border-blue-700' : 'bg-gray-50 border-gray-200 dark:bg-gray-700 dark:border-gray-600'">
              <input type="radio" name="theme" value="light" v-model="darkMode" class="mr-2 text-blue-600" />
              <svg class="h-4 w-4 mr-1 text-yellow-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" />
              </svg>
              Light
            </label>
            <label class="flex items-center p-2 border rounded-md cursor-pointer" :class="darkMode === 'dark' ? 'bg-blue-50 border-blue-200 dark:bg-blue-900 dark:border-blue-700' : 'bg-gray-50 border-gray-200 dark:bg-gray-700 dark:border-gray-600'">
              <input type="radio" name="theme" value="dark" v-model="darkMode" class="mr-2 text-blue-600" />
              <svg class="h-4 w-4 mr-1 text-blue-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
              </svg>
              Dark
            </label>
            <label class="flex items-center p-2 border rounded-md cursor-pointer" :class="darkMode === 'system' ? 'bg-blue-50 border-blue-200 dark:bg-blue-900 dark:border-blue-700' : 'bg-gray-50 border-gray-200 dark:bg-gray-700 dark:border-gray-600'">
              <input type="radio" name="theme" value="system" v-model="darkMode" class="mr-2 text-blue-600" />
              <svg class="h-4 w-4 mr-1 text-gray-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 5a2 2 0 012-2h10a2 2 0 012 2v8a2 2 0 01-2 2h-2.22l.123.489.804.804A1 1 0 0113 18H7a1 1 0 01-.707-1.707l.804-.804L7.22 15H5a2 2 0 01-2-2V5zm5.771 7H5V5h10v7H8.771z" clip-rule="evenodd" />
              </svg>
              System
            </label>
          </div>
          <p class="mt-2 text-xs text-gray-500 dark:text-gray-400">
            Select your preferred appearance mode
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex justify-end space-x-4 mt-6">
        <button 
          type="button" 
          @click="resetSettings" 
          class="macos-button macos-button-secondary dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600"
        >
          Reset
        </button>
        <button 
          type="button" 
          @click="saveSettings" 
          class="macos-button macos-button-primary"
        >
          Save Settings
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { open } from '@tauri-apps/api/dialog';
import { PhGear } from '@phosphor-icons/vue';
import { useSettingsStore } from '../stores/settings';

const settingsStore = useSettingsStore();

// Reactive references
const sitesRootDirectory = ref('');
const darkMode = ref('system');

// Watchers
watch(darkMode, (newMode) => {
  settingsStore.setDarkMode(newMode);
});

// Methods
const selectRootDirectory = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Sites Root Directory'
    });
    
    if (selected) {
      sitesRootDirectory.value = selected;
    }
  } catch (error) {
    console.error('Error selecting directory:', error);
  }
};

const saveSettings = async () => {
  await settingsStore.setSitesRootDirectory(sitesRootDirectory.value);
  alert('Settings saved successfully!');
};

const resetSettings = () => {
  sitesRootDirectory.value = settingsStore.sitesRootDirectory;
  darkMode.value = settingsStore.darkMode;
};

// Lifecycle hooks
onMounted(async () => {
  await settingsStore.loadSettings();
  sitesRootDirectory.value = settingsStore.sitesRootDirectory;
  darkMode.value = settingsStore.darkMode;
});
</script>

<style scoped>
.macos-input {
  @apply rounded-md border border-gray-300 dark:border-gray-600 px-3 py-2 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-gray-100;
}

.macos-button-primary {
  @apply bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2;
}

.macos-button-secondary {
  @apply bg-white text-gray-700 border border-gray-300 px-4 py-2 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2;
}
</style>