<template>
  <div class="logs-main">
    <router-view />
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWindow = getCurrentWebviewWindow()

// Apply theme when the log window is first loaded
onMounted(() => {
  const savedTheme = localStorage.getItem('theme') || 'system';
  if (window.ThemeManager) {
    window.ThemeManager.applyTheme(savedTheme);
  }
  
  // Listen for theme changes in localStorage from other windows
  window.addEventListener('storage', (event) => {
    if (event.key === 'theme' && window.ThemeManager) {
      window.ThemeManager.applyTheme(event.newValue);
    }
  });
});
</script>

<style>
.logs-main {
  min-height: 100vh;
  width: 100%;
}
</style>
