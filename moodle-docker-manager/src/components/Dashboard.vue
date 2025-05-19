<template>
  <div class="dashboard p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 max-w-4xl mx-auto my-8">
    <h1 class="text-2xl font-bold mb-6 text-gray-800 dark:text-gray-100">Moodle Docker Manager Dashboard</h1>
    <div class="docker-status">
      <DockerStatus />
    </div>
    <div class="actions flex items-center justify-between">
      <router-link to="/create" class="create-button macos-button macos-button-primary">Create New Site</router-link>
      <router-link to="/settings" class="settings-button macos-button macos-button-secondary flex items-center">
        <svg class="w-4 h-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
        </svg>
        Settings
      </router-link>
    </div>
    <div class="site-list-container">
      <SiteList ref="siteList" />
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import SiteList from './SiteList.vue';
import DockerStatus from './DockerStatus.vue';

export default {
  name: 'Dashboard',
  components: {
    SiteList,
    DockerStatus
  },
  data() {
    return {
      dockerInstalled: false,
      dockerRunning: false
    };
  },
  async mounted() {
    try {
      this.dockerInstalled = await invoke('check_docker_installed');
      this.dockerRunning = await invoke('check_docker_running');
    } catch (error) {
      console.error('Error checking Docker status:', error);
    }
  }
};
</script>

<style scoped>
.dashboard {
  padding: 20px;
}

.site-list {
  margin-bottom: 20px;
}

.docker-status {
  margin-top: 20px;
}
</style>