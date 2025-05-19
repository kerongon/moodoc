<template>
  <div class="home-view bg-gray-50 dark:bg-gray-900 w-full h-screen flex flex-col">
    <div class="w-full px-4 py-1.5 flex-none">
      <h1 class="text-base font-bold text-center text-gray-800 dark:text-gray-100 mb-0.5">Moodle Docker Manager</h1>
    </div>
    <div class="app-container w-full flex-1 px-3 pb-2 overflow-hidden flex flex-col compact-app-container">
      <DockerStatus class="flex-none mb-2" />
      <CreateSite class="flex-none mb-2" @site-created="refreshSiteList" />
      <SiteList class="flex-1 overflow-auto min-h-0" ref="siteList" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import DockerStatus from '../components/DockerStatus.vue';
import CreateSite from '../components/CreateSite.vue';
import SiteList from '../components/SiteList.vue';

export default defineComponent({
  name: 'HomeView',
  components: {
    DockerStatus,
    CreateSite,
    SiteList,
  },
  setup() {
    const siteList = ref(null);
    
    const refreshSiteList = () => {
      // Refresh the site list after creating a new site
      if (siteList.value) {
        siteList.value.loadSites();
      }
    };
    
    return {
      siteList,
      refreshSiteList
    };
  }
});
</script>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.app-container {
  /* Ensure content doesn't cause a scrollbar on the main view */
  height: calc(100vh - 60px); /* Slightly smaller to account for header */
}
</style>