<template>
  <div class="docker-status-card flex items-center gap-3">
    <!-- Icon container -->
    <div class="icon-container">
      <svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M3 12v3c0 1.1.9 2 2 2h14a2 2 0 0 0 2-2v-3M3 12l3-3h12l3 3M3 12l.7-1.4M21 12l-.7-1.4" />
        <rect x="9" y="3" width="6" height="5" rx="1" />
      </svg>
    </div>

    <!-- Text content -->
    <div class="flex-grow">
      <h2 class="title">Docker Status</h2>
      <p class="description">
        {{ loading ? 'Checking Docker status...' : dockerInstalled ? 'Docker is installed' : 'Docker is not installed' }}
      </p>
    </div>

    <!-- Status indicator -->
    <div v-if="!loading" class="status-badge">
      <div v-if="dockerInstalled && dockerRunning" class="status-indicator running">
        <div class="status-dot"></div>
        <span class="status-text">Running</span>
      </div>
      <div v-else-if="dockerInstalled && !dockerRunning" class="status-indicator stopped">
        <div class="status-dot"></div>
        <span class="status-text">Stopped</span>
      </div>
      <div v-else class="status-indicator not-installed">
        <div class="status-dot"></div>
        <span class="status-text">Not Installed</span>
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
/* Modern macOS-style component */
.docker-status-card {
  background: linear-gradient(to bottom, #ffffff, #f8f9fb);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06), 0 0 0 1px rgba(0, 0, 0, 0.03);
  border-radius: 12px;
  padding: 10px 14px;
  transition: all 0.2s ease;
}

.dark .docker-status-card {
  background: linear-gradient(to bottom, #1f2937, #1a2332);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(255, 255, 255, 0.05);
}

.docker-status-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08), 0 0 0 1px rgba(0, 0, 0, 0.04);
}

.dark .docker-status-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(255, 255, 255, 0.07);
}

.icon-container {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(59, 130, 246, 0.1);
  border-radius: 10px;
  padding: 8px;
  transition: all 0.3s ease;
  width: 36px;
  height: 36px;
}

.dark .icon-container {
  background-color: rgba(59, 130, 246, 0.15);
}

.icon {
  width: 20px;
  height: 20px;
  color: #3b82f6;
}

.dark .icon {
  color: #60a5fa;
}

.title {
  font-size: 0.95rem;
  font-weight: 600;
  color: #111827;
  margin-bottom: 2px;
}

.dark .title {
  color: #f3f4f6;
}

.description {
  font-size: 0.85rem;
  color: #6b7280;
  font-weight: 400;
}

.dark .description {
  color: #9ca3af;
}

.status-badge {
  padding: 4px 10px;
  background-color: #f9fafb;
  border-radius: 12px;
  font-size: 12px;
  letter-spacing: 0.2px;
  font-weight: 500;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05), 0 0 0 1px rgba(0, 0, 0, 0.03);
  display: flex;
  align-items: center;
}

.dark .status-badge {
  background-color: rgba(31, 41, 55, 0.5);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(255, 255, 255, 0.05);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 2px 4px;
}

.status-dot {
  width: 8px;
  height: 8px;
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
  font-weight: 600;
  font-size: 12px;
}

/* Status variants */
.status-indicator.running .status-dot {
  background-color: #22c55e;
  box-shadow: 0 0 4px rgba(34, 197, 94, 0.6);
  animation: pulse-green 2s infinite;
}

.status-indicator.running .status-dot::after {
  background-color: #22c55e;
  animation: pulse-green-outer 2s infinite;
}

.status-indicator.running .status-text {
  color: #166534;
}

.dark .status-indicator.running .status-text {
  color: #4ade80;
}

.status-indicator.stopped .status-dot {
  background-color: #ef4444;
  box-shadow: 0 0 4px rgba(239, 68, 68, 0.6);
}

.status-indicator.stopped .status-text {
  color: #991b1b;
}

.dark .status-indicator.stopped .status-text {
  color: #f87171;
}

.status-indicator.not-installed .status-dot {
  background-color: #f59e0b;
  box-shadow: 0 0 4px rgba(245, 158, 11, 0.6);
}

.status-indicator.not-installed .status-text {
  color: #92400e;
}

.dark .status-indicator.not-installed .status-text {
  color: #fbbf24;
}

/* Pulse animations for status indicators */
@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.6; }
  100% { opacity: 1; }
}

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

.pulse {
  animation: pulse 2s infinite ease-in-out;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .docker-status-card {
    padding: 8px 12px;
  }
  
  .icon-container {
    width: 32px;
    height: 32px;
  }
  
  .title {
    font-size: 0.9rem;
  }
  
  .description {
    font-size: 0.8rem;
  }
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