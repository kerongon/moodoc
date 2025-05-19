<template>
  <div id="app">
    <router-view />
    <SettingsModal />
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import SettingsModal from './components/SettingsModal.vue';
import { useSettingsStore } from './stores/settings';

export default defineComponent({
  name: 'App',
  components: {
    SettingsModal
  },
  setup() {
    const settingsStore = useSettingsStore();
    
    onMounted(async () => {
      // Load settings when app starts
      await settingsStore.loadSettings();
    });
    
    return {};
  }
});
</script>

<style>
/* The Tailwind classes will be applied from index.css */
#app {
  @apply font-sans antialiased text-gray-800 bg-gray-50;
  min-height: 100vh;
}
</style>