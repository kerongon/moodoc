<template>
  <div>
    <div class="dashboard-full hide-scrollbar bg-white dark:bg-gray-900 min-h-screen w-full p-0">
      <!-- Modern macOS-style header -->
      <header class="app-header" data-tauri-drag-region>
        <div class="header-content">
          <div class="titlebar-left-controls">
            <!-- Window Controls -->
            <div class="window-controls">
              <button @click="closeWindow" class="window-control-button close-button" title="Close">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1"></circle></svg>
              </button>
              <button @click="toggleMaximizeWindow" class="window-control-button maximize-button" title="Maximize">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#27C93F" stroke="#1AAB29" stroke-width="1"></circle></svg>
              </button>
            </div>
            <!-- Navigation Controls -->
            <div class="navigation-controls">
              <button @click="goBack" class="header-nav-button" title="Go Back">
                <PhArrowLeft class="button-icon" />
              </button>
              <button @click="goForward" class="header-nav-button" title="Go Forward">
                <PhArrowRight class="button-icon" />
              </button>
              <button @click="reloadPage" class="header-nav-button" title="Reload Page">
                <PhArrowClockwise class="button-icon" />
              </button>
            </div>
          </div>

          <!-- Logo and app title -->
          <div class="app-branding">
            <div class="logo-container">
              <PhSquare class="logo-icon text-white" weight="fill" />
            </div>
            <div class="title-container">
              <h1 class="app-title">Moodle Docker Manager</h1>
            </div>
          </div>
          
          <!-- Header actions -->
          <div style="display: flex; align-items: center; gap: 0.75rem;">
            <DockerStatus />
            
            <router-link to="/create" title="Create New Site"
                     class="modern-new-site-button"
                     style="display: flex; align-items: center; gap: 0.5rem; background-color: #6366F1; color: white; padding: 0.5rem 1.25rem; border-radius: 8px; font-weight: 600; font-size: 1rem; transition: all 0.2s ease; box-shadow: 0 2px 4px rgba(79, 70, 229, 0.2);">
              <PhPlus style="width: 20px; height: 20px;" weight="fill" />
              <span>New Site</span>
            </router-link>
            
            <button @click="openSettings" title="Settings"
                   class="modern-settings-button"
                   style="display: flex; align-items: center; justify-content: center; width: 40px; height: 40px; border-radius: 8px; background-color: transparent; transition: all 0.2s ease; color: #9ca3af; border: none;">
              <PhGear style="width: 22px; height: 22px;" />
            </button>
          </div>
        </div>
      </header>
      <div class="main-content px-8 pb-8 hide-scrollbar">
        <SiteList ref="siteList" />
      </div>
    </div>
    <!-- Settings modal popup -->
    <SettingsModal ref="settingsModal"></SettingsModal>
  </div>
</template>

<script>
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { PhList, PhPlus, PhGear, PhArrowLeft, PhArrowRight, PhArrowClockwise, PhSquare } from '@phosphor-icons/vue';
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
    PhGear,
    PhArrowLeft,
    PhArrowRight,
    PhArrowClockwise,
    PhSquare
  },
  data() {
    return {
      dockerInstalled: false,
      dockerRunning: false,
      newSiteButtonStyle: {
        transform: 'translateY(0px)'
      },
      settingsButtonStyle: {
        transform: 'rotate(0deg)'
      }
    };
  },
  methods: {
    openSettings() {
      const modal = this.$refs.settingsModal;
      if (modal && typeof modal.openModal === 'function') {
        modal.openModal();
      }
    },
    goBack() {
      this.$router.go(-1);
    },
    goForward() {
      this.$router.go(1);
    },
    reloadPage() {
      window.location.reload();
    },
    minimizeWindow() {
      appWindow.minimize();
    },
    async toggleMaximizeWindow() {
      const isMaximized = await appWindow.isMaximized();
      if (isMaximized) {
        appWindow.unmaximize();
      } else {
        appWindow.maximize();
      }
    },
    closeWindow() {
      appWindow.close();
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
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  width: 100vw;
  background: inherit;
  padding: 0;
}

/* Modern macOS-style header */
.app-header {
  background: rgba(17, 24, 39, 0.9);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding: 0.75rem 1rem;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  transition: all 0.3s ease;
  height: 56px;
  display: flex;
  align-items: center;
}

.main-content {
  padding-top: 72px; /* Height of header + some margin */
  flex-grow: 1;
  overflow-y: auto;
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
  width: 100%; /* Changed from max-width to width 100% */
  margin: 0 auto;
  padding: 0 0.5rem; /* Reduced horizontal padding inside content */
}

.titlebar-left-controls {
  display: flex;
  align-items: center;
  gap: 12px; /* Gap between window controls and nav controls */
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-right: 12px; /* Space after window controls */
}

.window-control-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: default; /* Changed from pointer to default for native feel */
}

/* Specific styles for each button to mimic macOS traffic lights */
.close-button svg circle { fill: #FF5F57; stroke: #E0443E; }
.minimize-button svg circle { fill: #FFBD2E; stroke: #DEA123; }
.maximize-button svg circle { fill: #27C93F; stroke: #1AAB29; }

/* Always display the macOS traffic-light icons */
.window-control-button svg {
  opacity: 1 !important;
  transition: opacity 0.1s ease-in-out;
}

.navigation-controls {
  display: flex;
  align-items: center;
  gap: 6px; /* Reduced gap for nav buttons */
}

.app-branding {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  /* Positioned to the right of navigation controls */
  margin-left: 12px;
  pointer-events: auto;
}

.logo-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  pointer-events: auto;
  background-color: #6366F1;
  border-radius: 6px;
}

.logo-icon {
  width: 18px;
  height: 18px;
}

.title-container {
  display: flex;
  flex-direction: column;
  pointer-events: auto;
}

.app-title {
  font-size: 1rem;
  font-weight: 600;
  margin: 0;
  letter-spacing: -0.025em;
  color: #FFFFFF;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.header-nav-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px; /* Smaller nav buttons */
  height: 28px;
  border-radius: 6px;
  background-color: rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
  color: #4b5563;
  border: none;
  padding: 0;
}

.dark .header-nav-button {
  background-color: rgba(255, 255, 255, 0.1);
  color: #d1d5db;
}

.header-nav-button:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.dark .header-nav-button:hover {
  background-color: rgba(255, 255, 255, 0.15);
}

.header-nav-button .button-icon {
  width: 16px; /* Smaller icons in nav buttons */
  height: 16px;
}

.new-site-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #6366F1;
  color: white;
  padding: 0.5rem 1.25rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 1rem;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(79, 70, 229, 0.2);
}

.new-site-button:hover {
  background-color: #4F46E5;
  transform: translateY(-1px);
}

.button-icon {
  width: 20px;
  height: 20px;
}

.settings-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background-color: transparent;
  transition: all 0.2s ease;
  color: #9ca3af;
  border: none;
}

.dark .settings-button {
  color: #d1d5db;
}

.settings-button:hover {
  color: #6366F1;
  transform: rotate(15deg);
}

.dark .settings-button:hover {
  color: #a5b4fc;
}

.docker-status-button {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0;
  border-radius: 0;
  font-weight: 500; 
  font-size: 0.875rem;
  background-color: transparent;
  color: #22c55e;
  min-height: 24px;
  margin-right: 8px;
  box-shadow: none;
}
.dark .docker-status-button {
  color: #4ade80;
}

@media (max-width: 768px) { /* Adjusted breakpoint */
  .app-branding {
    /* Hide app title on smaller screens if space is an issue */
    /* You might want to only show the logo or a shorter title */
    position: static; /* Remove absolute positioning */
    transform: none; /* Remove transform */
    margin-left: 10px; /* Add some margin if needed */
  }
  .app-title {
    font-size: 1rem;
  }
  .header-actions {
    /* Adjust layout or hide some actions if needed */
  }
}

@media (max-width: 640px) {
  .app-header {
    padding: 0.5rem 0.75rem; /* Further reduced padding */
    height: 52px; /* Slightly smaller height */
  }

  .main-content {
    padding-top: 60px; /* Adjust accordingly */
  }
  
  .app-title {
    font-size: 0.9rem; /* Smaller title */
  }
  
  .new-site-button span {
    display: none;
  }
  
  .new-site-button {
    width: 32px; /* Smaller button */
    height: 32px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
  }
  
  .header-actions {
    gap: 0.25rem; /* Tighter gap */
  }

  .header-nav-button {
    width: 26px; /* Smaller nav buttons */
    height: 26px;
  }

  .header-nav-button .button-icon {
    width: 14px; /* Smaller icons */
    height: 14px;
  }
  .window-controls {
    gap: 6px;
    margin-right: 8px;
  }
  .titlebar-left-controls {
    gap: 8px;
  }
}
</style>