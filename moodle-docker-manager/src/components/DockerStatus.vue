<template>
  <div class="docker-status-card flex items-center gap-2" 
       style="display: flex; align-items: center; gap: 0.5rem;"
       :title="loading ? 'Checking Docker status...' : 
               dockerInstalled && dockerRunning ? 'Docker is installed and running' : 
               dockerInstalled ? 'Docker is installed but not running' : 
               'Docker is not installed'">
    <!-- Status indicator -->
    <div v-if="!loading" class="status-indicator-compact" style="display: flex; align-items: center;">
      <div v-if="dockerInstalled && dockerRunning" class="status-dot running" 
           style="width: 6px; height: 6px; border-radius: 50%; background-color: #22c55e; box-shadow: 0 0 4px rgba(34, 197, 94, 0.6);"></div>
      <div v-else-if="dockerInstalled && !dockerRunning" class="status-dot stopped"
           style="width: 6px; height: 6px; border-radius: 50%; background-color: #ef4444; box-shadow: 0 0 4px rgba(239, 68, 68, 0.6);"></div>
      <div v-else class="status-dot not-installed"
           style="width: 6px; height: 6px; border-radius: 50%; background-color: #f59e0b; box-shadow: 0 0 4px rgba(245, 158, 11, 0.6);"></div>
    </div>

    <!-- Text content -->
    <div>
      <p class="status-text" style="font-size: 0.7rem; font-weight: 500; margin: 0;">
        {{ loading ? '...' : dockerRunning ? 'Running' : 'Stopped' }}
      </p>
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
    console.log('DockerStatus component mounted');
    this.fetchDockerStatus();
    
    // Re-check status every 10 seconds
    setInterval(() => {
      this.fetchDockerStatus();
    }, 10000);
  }
};
</script>

<style scoped>
/* Modern macOS-style component - more compact version */
.docker-status-card {
  background: linear-gradient(to bottom, #ffffff, #f8f9fb);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05), 0 0 0 1px rgba(0, 0, 0, 0.02);
  border-radius: 50px;
  padding: 3px 8px;
  transition: all 0.2s ease;
}

.dark .docker-status-card {
  background: linear-gradient(to bottom, #1f2937, #1a2332);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(255, 255, 255, 0.05);
}

.docker-status-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08), 0 0 0 1px rgba(0, 0, 0, 0.04);
}

.dark .docker-status-card:hover {
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(255, 255, 255, 0.07);
}

.status-indicator-compact {
  display: flex;
  align-items: center;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  position: relative;
}

.status-dot::after {
  content: '';
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  border-radius: 50%;
  opacity: 0.4;
  z-index: -1;
}

.status-text {
  font-size: 0.7rem;
  font-weight: 500;
  color: #374151;
  margin: 0;
}

.dark .status-text {
  color: #e5e7eb;
}

/* Status variants */
.status-dot.running {
  background-color: #22c55e;
  box-shadow: 0 0 4px rgba(34, 197, 94, 0.6);
  animation: pulse-green 2s infinite;
}

.status-dot.running::after {
  background-color: #22c55e;
  animation: pulse-green-outer 2s infinite;
}

.status-dot.stopped {
  background-color: #ef4444;
  box-shadow: 0 0 4px rgba(239, 68, 68, 0.6);
}

.status-dot.not-installed {
  background-color: #f59e0b;
  box-shadow: 0 0 4px rgba(245, 158, 11, 0.6);
}

/* Pulse animations for status indicators */
@keyframes pulse-green {
  0% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.6); }
  50% { box-shadow: 0 0 8px rgba(34, 197, 94, 0.8); }
  100% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.6); }
}

@keyframes pulse-green-outer {
  0% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.2); }
  100% { opacity: 0.3; transform: scale(1); }
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .docker-status-card {
    padding: 3px 8px;
  }
  
  .status-text {
    font-size: 0.75rem;
  }
}
</style>