<template>
  <div class="home-view">
    <h1>Moodle Docker Manager</h1>
    <DockerStatus />
    <CreateSite />
    <Dashboard />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import DockerStatus from '../components/DockerStatus.vue';
import CreateSite from '../components/CreateSite.vue';
import Dashboard from '../components/Dashboard.vue';

export default defineComponent({
  name: 'HomeView',
  components: {
    DockerStatus,
    CreateSite,
    Dashboard,
  },
});
</script>

<style scoped>
.home-view {
  padding: 20px;
}
</style>