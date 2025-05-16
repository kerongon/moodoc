<template>
  <div class="docker-status">
    <h2>Docker Status</h2>
    <div v-if="loading">Checking Docker status...</div>
    <div v-else>
      <div v-if="dockerInstalled">
        <p>Docker is installed.</p>
        <p>Status: <span :class="{'running': dockerRunning, 'stopped': !dockerRunning}">{{ dockerRunning ? 'Running' : 'Stopped' }}</span></p>
      </div>
      <div v-else>
        <p>Docker is not installed. Please install Docker to use this application.</p>
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
.docker-status {
  padding: 20px;
  border: 1px solid #ccc;
  border-radius: 5px;
  background-color: #f9f9f9;
}

.running {
  color: green;
}

.stopped {
  color: red;
}
</style>