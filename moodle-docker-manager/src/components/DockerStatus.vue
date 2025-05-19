<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 p-4 w-full">
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="mr-3">
          <!-- Docker logo (simplified) -->
          <svg class="w-6 h-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 12v3c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2v-3M3 12l3-3h12l3 3M3 12l.7-1.4M21 12l-.7-1.4" />
            <rect x="9" y="3" width="6" height="5" rx="1" />
          </svg>
        </div>
        <div>
          <h2 class="text-lg font-medium text-gray-900 dark:text-white">Docker Status</h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {{ loading ? 'Checking Docker status...' : dockerInstalled ? 'Docker is installed' : 'Docker is not installed' }}
          </p>
        </div>
      </div>
      <div v-if="!loading" class="flex items-center">
        <div v-if="dockerInstalled && dockerRunning" class="flex items-center">
          <div class="w-2.5 h-2.5 bg-green-500 rounded-full mr-1.5"></div>
          <span class="text-sm font-medium text-green-600 dark:text-green-400">Running</span>
        </div>
        <div v-else-if="dockerInstalled && !dockerRunning" class="flex items-center">
          <div class="w-2.5 h-2.5 bg-red-500 rounded-full mr-1.5"></div>
          <span class="text-sm font-medium text-red-600 dark:text-red-400">Stopped</span>
        </div>
        <div v-else class="flex items-center">
          <div class="w-2.5 h-2.5 bg-yellow-500 rounded-full mr-1.5"></div>
          <span class="text-sm font-medium text-yellow-600 dark:text-yellow-400">Not Installed</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';

export default {
  name: 'DockerStatus',
  data() {
    return {
      loading: true,
      dockerInstalled: false,
      dockerRunning: false
    };
  },
  methods: {
    async fetchDockerStatus() {
      try {
        this.dockerInstalled = await invoke('check_docker_installed');
        this.dockerRunning = await invoke('check_docker_running');
      } catch (error) {
        console.error('Error checking Docker status:', error);
      } finally {
        this.loading = false;
      }
    }
  },
  mounted() {
    this.fetchDockerStatus();
  }
};
</script>

<style scoped>
/* macOS-style component */
@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.6; }
  100% { opacity: 1; }
}

.pulse {
  animation: pulse 2s infinite ease-in-out;
}

/* Hide scrollbar when not needed */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>