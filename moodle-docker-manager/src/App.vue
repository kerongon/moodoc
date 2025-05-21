<template>
  <div id="app">
    <router-view />
    <SettingsModal />
    <!-- Global modals that persist across route changes -->
    <div id="global-modals">
      <SiteCreationProgress 
        v-if="showCreationProgress"
        :siteName="creationParams.siteName"
        :phpVersion="creationParams.phpVersion"
        :dbType="creationParams.dbType"
        :moodleBranch="creationParams.moodleBranch"
        :webPort="creationParams.webPort"
        :dbPort="creationParams.dbPort"
        :directory="creationParams.directory"
        @close="hideSiteCreation"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, onBeforeUnmount, ref, provide } from 'vue';
import { listen } from '@tauri-apps/api/event';
import SettingsModal from './components/SettingsModal.vue';
import SiteCreationProgress from './components/SiteCreationProgress.vue';
import { useSettingsStore } from './stores/settings';
import { eventBus } from './utils/event-bus';

export default defineComponent({
  name: 'App',
  components: {
    SettingsModal,
    SiteCreationProgress
  },
  setup() {
    const settingsStore = useSettingsStore();
    const showCreationProgress = ref(false);
    
    // Define interface for creation parameters
    interface CreationParams {
      siteName: string;
      phpVersion: string;
      dbType: string;
      moodleBranch: string;
      webPort: number;
      dbPort: number;
      directory: string;
    }
    
    // Creation progress state
    const creationParams = ref<CreationParams>({
      siteName: '',
      phpVersion: '',
      dbType: '',
      moodleBranch: '',
      webPort: 0,
      dbPort: 0,
      directory: ''
    });
    
    // Method to show the creation progress
    const startSiteCreation = (params: CreationParams) => {
      console.log('Starting site creation with params:', params);
      creationParams.value = params;
      showCreationProgress.value = true;
      // Force update to ensure modal is displayed
      setTimeout(() => {
        if (!showCreationProgress.value) {
          console.warn('Modal not showing, forcing display');
          showCreationProgress.value = true;
        }
      }, 100);
    };
    
    // Method to hide the creation progress
    const hideSiteCreation = () => {
      console.log('Hiding site creation modal');
      showCreationProgress.value = false;
    };
    
    // Provide the methods to show/hide the creation progress
    provide('startSiteCreation', startSiteCreation);
    provide('hideSiteCreation', hideSiteCreation);
    
    // Store unlisten function reference
    let unlistenCreation: (() => void) | null = null;
    
    onMounted(async () => {
      // Load settings when app starts
      await settingsStore.loadSettings();
      
      // Listen for site creation events using eventBus
      unlistenCreation = await eventBus.listenForSiteCreation((params) => {
        console.log('Site creation event received:', params);
        startSiteCreation(params as CreationParams);
      });
    });
    
    // Clean up event listeners when component is unmounted
    onBeforeUnmount(() => {
      if (unlistenCreation) unlistenCreation();
    });
    
    return {
      showCreationProgress,
      creationParams,
      hideSiteCreation,
      startSiteCreation
    };
  }
});
</script>

<style>
/* The Tailwind classes will be applied from index.css */
#app {
  @apply font-sans antialiased text-gray-800 bg-gray-50 dark:text-gray-100 dark:bg-gray-900;
  height: 100vh; /* Fixed height instead of min-height */
  width: 100%;
  transition: background-color 0.3s ease, color 0.3s ease;
  overflow: hidden; /* Prevent both horizontal and vertical scrolling */
  display: flex;
  flex-direction: column;
  position: fixed; /* Ensure it stays fixed within the window */
  top: 0;
  left: 0;
}
</style>