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
                <!-- default red circle -->
                <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                  <circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1" />
                </svg>
                <!-- hover X icon -->
                <svg class="icon-hover" width="8" height="8" viewBox="0 0 24 24">
                  <path stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
              <button @click="minimizeWindow" class="window-control-button minimize-button" title="Minimize">
                <!-- default yellow circle -->
                <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                  <circle cx="6" cy="6" r="5.5" fill="#FFBD2E" stroke="#DEA123" stroke-width="1" />
                </svg>
                <!-- hover minus icon -->
                <svg class="icon-hover" width="8" height="2" viewBox="0 0 12 12">
                  <rect x="2" y="5" width="8" height="2" fill="white" />
                </svg>
              </button>
              <button @click="toggleMaximizeWindow" class="window-control-button maximize-button" title="Maximize">
                <!-- default green circle -->
                <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                  <circle cx="6" cy="6" r="5.5" fill="#27C93F" stroke="#1AAB29" stroke-width="1" />
                </svg>
                <PhArrowsOut class="icon-hover w-2 h-2 text-white" weight="fill" />
              </button>
            </div>
            <!-- Navigation Controls -->
            <div class="navigation-controls">
              <button @click="goBack" class="header-nav-button" title="Go Back">
                <span class="chevron">&lt;</span>
              </button>
              <button @click="goForward" class="header-nav-button" title="Go Forward">
                <span class="chevron">&gt;</span>
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
            
            <router-link to="/create" title="Create New Site" class="modern-new-site-button">
              <PhPlus class="button-icon" weight="fill" />
              <span>New Site</span>
            </router-link>
            
            <button @click="openSettings" title="Settings" class="modern-settings-button">
              <PhGear class="button-icon" />
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
import { PhList, PhPlus, PhGear, PhArrowClockwise, PhSquare, PhArrowsOut } from '@phosphor-icons/vue';
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
    PhArrowClockwise,
    PhSquare,
    PhArrowsOut
  },
  data() {
    return {
      dockerInstalled: false,
      dockerRunning: false,
      newSiteButtonStyle: {
        transform: 'translateY(0px)'
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
  position: relative;
  width: 12px;
  height: 12px;
  padding: 0;
}

.window-control-button svg.icon-hover {
  display: none;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}

.window-control-button:hover svg.icon-hover {
  display: block;
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
  gap: 6px;
}

.header-nav-button {
  background: rgba(33, 37, 50, 0.95);
  border: none;
  padding: 6px;
  margin: 0 2px;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  color: #FFFFFF;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}
.header-nav-button .button-icon {
  width: 16px;
  height: 16px;
}
.header-nav-button:hover {
  background: rgba(50, 56, 74, 0.95);
}

.chevron {
  font-size: 16px;
  font-weight: bold;
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
  transform: rotate(15deg); /* Added back rotation */
}

.dark .settings-button:hover {
  color: #d1d5db; /* This is light gray, not purple, and matches default dark mode color */
}

.docker-status-button {
  display: flex;
  align-items: center;
  gap: 0.3rem;
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

@media (max-width: 768px) {
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
}

@media (max-width: 640px) {
  .app-header {
    padding: 0.5rem 0.75rem;
    height: 52px;
  }

  .main-content {
    padding-top: 60px;
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

  .header-nav-button {
    width: 28px;
    height: 28px;
  }

  .header-nav-button .button-icon {
    width: 16px;
    height: 16px;
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