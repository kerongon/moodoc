<template>
  <div>
    <div class="dashboard-full bg-white dark:bg-gray-800 min-h-screen w-full p-0">
      <div class="flex items-center justify-between px-8 pt-8 pb-2">
      <div class="flex items-center gap-3">
        <PhList class="w-10 h-10 text-blue-500 drop-shadow" weight="fill" />
        <h1 class="text-4xl font-extrabold bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent tracking-tight flex items-center">
          Moodle Docker Manager
        </h1>
      </div>
      <div class="flex items-center gap-4">
        <div class="docker-status-top">
          <DockerStatus />
        </div>
        <router-link to="/create" class="create-site-btn flex items-center gap-2 text-white font-semibold rounded-lg px-4 py-2 text-base shadow transition-all duration-150" title="Create New Site">
          <PhPlus class="w-5 h-5" weight="fill" />
          <span class="sm:inline">New Site</span>
        </router-link>
        <button @click="openSettings" class="settings-btn flex items-center justify-center bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-200 rounded-full p-2 transition-all duration-150" title="Settings">
          <PhGear class="w-5 h-5" />
        </button>
      </div>
      </div>
      <div class="px-8 pb-8">
        <SiteList ref="siteList" />
      </div>
    </div>
    <!-- Settings modal popup -->
    <SettingsModal ref="settingsModal"></SettingsModal>
  </div>
</template>

<script>

import { invoke } from '@tauri-apps/api/tauri';
import { PhList, PhPlus, PhGear } from '@phosphor-icons/vue';
import SiteList from './SiteList.vue';
import DockerStatus from './DockerStatus.vue';
import SettingsModal from './SettingsModal.vue';

export default {
  name: 'Dashboard',
  components: {
    SiteList,
    DockerStatus,
    SettingsModal,
    PhList,
    PhPlus,
    PhGear
  },
  data() {
    return {
      dockerInstalled: false,
      dockerRunning: false
    };
  },
  methods: {
    openSettings() {
      const modal = this.$refs.settingsModal;
      if (modal && typeof modal.openModal === 'function') {
        modal.openModal();
      }
    }
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

.dashboard-full {
  min-height: 100vh;
  width: 100vw;
  background: inherit;
  padding: 0;
}

.docker-status-top {
  min-width: 120px;
  display: flex;
  align-items: center;
  justify-content: flex-end;
}

.create-site-btn {
  min-width: 44px;
  min-height: 44px;
  font-size: 1rem;
  background-color: #2563eb;
  color: white;
}

.settings-btn {
  min-width: 44px;
  min-height: 44px;
}
</style>