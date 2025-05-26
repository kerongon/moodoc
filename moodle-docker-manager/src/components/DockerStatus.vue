<template>
  <div class="docker-status-container" 
       @click="handleClick"
       :title="loading ? 'Checking Docker status...' : 
               dockerInstalled && dockerRunning ? 'Docker is installed and running - Click for diagnostics' : 
               dockerInstalled ? 'Docker is installed but not running - Click for diagnostics' : 
               'Docker is not installed - Click for diagnostics'">
    <!-- Status indicator dot -->
    <div v-if="!loading" class="status-dot-container">
      <div v-if="dockerInstalled && dockerRunning" class="status-dot running"></div>
      <div v-else-if="dockerInstalled && !dockerRunning" class="status-dot stopped"></div>
      <div v-else class="status-dot not-installed"></div>
    </div>
    <div v-else class="status-dot-container">
      <!-- Loading animation -->
      <div class="status-dot loading"></div>
    </div>

    <!-- Text content -->
    <div>
      <p class="status-text" 
         :class="{
           'loading-text': loading,
           'running-text': !loading && dockerRunning,
           'stopped-text': !loading && dockerInstalled && !dockerRunning,
           'not-installed-text': !loading && !dockerInstalled
         }">
        {{ loading ? '...' : dockerRunning ? 'Running' : 'Stopped' }}
      </p>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'DockerStatus',
  emits: ['click'],
  data() {
    return {
      loading: true,
      dockerInstalled: false,
      dockerRunning: false
    };
  },
  methods: {
    handleClick() {
      this.$emit('click');
    },
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
@import '../assets/dashboard-light-mode-fix.css';
/* Modern clean status indicator with enhanced animations */
.docker-status-container {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
  padding: 4px 8px;
  border-radius: 12px;
  cursor: pointer;
}

.docker-status-container:hover {
  transform: translateY(-2px);
  background-color: rgba(255, 255, 255, 0.05);
}

.status-dot-container {
  display: flex;
  align-items: center;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  transition: transform 0.3s ease;
}

.docker-status-container:hover .status-dot {
  transform: scale(1.2);
}

.status-dot.loading {
  background-color: #9ca3af;
  box-shadow: 0 0 6px rgba(156, 163, 175, 0.6);
  animation: pulse-loading 1.5s infinite ease-in-out;
}

@keyframes pulse-loading {
  0% { transform: scale(0.8); opacity: 0.7; }
  50% { transform: scale(1.2); opacity: 1; }
  100% { transform: scale(0.8); opacity: 0.7; }
}

.status-dot.running {
  background-color: #22c55e;
  box-shadow: 0 0 6px rgba(34, 197, 94, 0.6);
  animation: pulse-green 2s infinite;
  position: relative;
}

.status-dot.running::after {
  content: '';
  position: absolute;
  top: -4px;
  left: -4px;
  right: -4px;
  bottom: -4px;
  border-radius: 50%;
  background-color: #22c55e;
  opacity: 0.4;
  z-index: -1;
  animation: pulse-green-outer 2s infinite;
}

.status-dot.stopped {
  background-color: #ef4444;
  box-shadow: 0 0 6px rgba(239, 68, 68, 0.6);
  animation: pulse-red 2s infinite;
  position: relative;
}

.status-dot.stopped::after {
  content: '';
  position: absolute;
  top: -4px;
  left: -4px;
  right: -4px;
  bottom: -4px;
  border-radius: 50%;
  background-color: #ef4444;
  opacity: 0.4;
  z-index: -1;
  animation: pulse-red-outer 2s infinite;
}

.status-dot.not-installed {
  background-color: #f59e0b;
  box-shadow: 0 0 6px rgba(245, 158, 11, 0.6);
  animation: pulse-orange 2s infinite;
  position: relative;
}

.status-dot.not-installed::after {
  content: '';
  position: absolute;
  top: -4px;
  left: -4px;
  right: -4px;
  bottom: -4px;
  border-radius: 50%;
  background-color: #f59e0b;
  opacity: 0.4;
  z-index: -1;
  animation: pulse-orange-outer 2s infinite;
}

@keyframes pulse-green {
  0% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.6); }
  50% { box-shadow: 0 0 10px rgba(34, 197, 94, 0.9); }
  100% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.6); }
}

@keyframes pulse-green-outer {
  0% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.35); }
  100% { opacity: 0.3; transform: scale(1); }
}

@keyframes pulse-red {
  0% { box-shadow: 0 0 4px rgba(239, 68, 68, 0.6); }
  50% { box-shadow: 0 0 8px rgba(239, 68, 68, 0.8); }
  100% { box-shadow: 0 0 4px rgba(239, 68, 68, 0.6); }
}

@keyframes pulse-red-outer {
  0% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.2); }
  100% { opacity: 0.3; transform: scale(1); }
}

@keyframes pulse-orange {
  0% { box-shadow: 0 0 4px rgba(245, 158, 11, 0.6); }
  50% { box-shadow: 0 0 8px rgba(245, 158, 11, 0.8); }
  100% { box-shadow: 0 0 4px rgba(245, 158, 11, 0.6); }
}

@keyframes pulse-orange-outer {
  0% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.2); }
  100% { opacity: 0.3; transform: scale(1); }
}

.status-text {
  font-size: 0.875rem;
  font-weight: 500;
  margin: 0;
  transition: color 0.3s ease, transform 0.3s ease;
}

.docker-status-container:hover .status-text {
  transform: translateX(2px);
  font-weight: 600;
}

.running-text {
  color: #22c55e;
}

.stopped-text {
  color: #ef4444;
}

.not-installed-text {
  color: #f59e0b;
}

.loading-text {
  color: #9ca3af;
  animation: text-pulse 1.5s infinite;
}

@keyframes text-pulse {
  0% { opacity: 1; }
  50% { opacity: 0.6; }
  100% { opacity: 1; }
}
</style>