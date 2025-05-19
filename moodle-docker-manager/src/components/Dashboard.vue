<template>
  <div class="dashboard">
    <h1>Moodle Docker Manager Dashboard</h1>
    <div class="docker-status">
      <DockerStatus />
    </div>
    <div class="actions">
      <router-link to="/create" class="create-button">Create New Site</router-link>
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