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
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>