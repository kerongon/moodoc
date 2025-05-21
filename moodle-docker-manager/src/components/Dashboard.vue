<template>
  <div>
    <div class="dashboard-full bg-white dark:bg-gray-900 min-h-screen w-full p-0">
      <!-- Modern macOS-style header -->
      <header class="app-header">
        <div class="header-content">
          <!-- Logo and app title -->
          <div class="app-branding">
            <div class="logo-container">
              <PhList class="logo-icon" weight="fill" />
            </div>
            <div class="title-container">
              <h1 class="app-title">Moodle Docker Manager</h1>
            </div>
          </div>
          
          <!-- Header actions -->
          <div class="header-actions">
            <DockerStatus />
            
            <router-link to="/create" class="new-site-button" title="Create New Site">
              <PhPlus class="button-icon" weight="fill" />
              <span>New Site</span>
            </router-link>
            
            <button @click="openSettings" class="settings-button" title="Settings">
              <PhGear class="button-icon" />
            </button>
          </div>
        </div>
      </header>
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

/* Modern macOS-style header */
.app-header {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  padding: 1rem 1.5rem;
  position: sticky;
  top: 0;
  z-index: 10;
  transition: all 0.3s ease;
}

.dark .app-header {
  background: rgba(17, 24, 39, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1400px;
  margin: 0 auto;
}

.app-branding {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: linear-gradient(135deg, #4f46e5, #7c3aed);
  box-shadow: 0 2px 5px rgba(99, 102, 241, 0.3);
}

.logo-icon {
  color: white;
  width: 22px;
  height: 22px;
}

.title-container {
  display: flex;
  flex-direction: column;
}

.app-title {
  font-size: 1.5rem;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(90deg, #4f46e5, #8b5cf6);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  letter-spacing: -0.02em;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.new-site-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #4f46e5;
  color: white;
  padding: 0.5rem 1.25rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.95rem;
  transition: all 0.2s ease;
  box-shadow: 0 2px 5px rgba(79, 70, 229, 0.3);
}

.new-site-button:hover {
  background-color: #4338ca;
  transform: translateY(-1px);
}

.button-icon {
  width: 18px;
  height: 18px;
}

.settings-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 42px;
  height: 42px;
  border-radius: 50%;
  background-color: rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
  color: #4b5563;
}

.dark .settings-button {
  background-color: rgba(255, 255, 255, 0.1);
  color: #d1d5db;
}

.settings-button:hover {
  background-color: rgba(0, 0, 0, 0.1);
  transform: rotate(15deg);
}

.dark .settings-button:hover {
  background-color: rgba(255, 255, 255, 0.15);
}

@media (max-width: 640px) {
  .app-header {
    padding: 0.75rem 1rem;
  }
  
  .app-title {
    font-size: 1.25rem;
  }
  
  .new-site-button span {
    display: none;
  }
  
  .new-site-button {
    width: 42px;
    height: 42px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }
  
  .header-actions {
    gap: 0.75rem;
  }
}
</style>