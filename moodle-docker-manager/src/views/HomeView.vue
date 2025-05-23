<template>
  <div class="home-view bg-gray-50 dark:bg-gray-900 w-full h-screen flex flex-col">
    <div class="title-bar w-full px-4 py-2 flex-none flex items-center justify-center" data-tauri-drag-region>
      <div class="mr-2">
        <PhSquare class="w-6 h-6 text-indigo-600" weight="fill" />
      </div>
      <h1 class="text-lg font-bold text-center text-indigo-500 dark:text-indigo-400">Moodle Docker Manager</h1>
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
import { PhSquare } from '@phosphor-icons/vue';

export default defineComponent({
  name: 'HomeView',
  components: {
    DockerStatus,
    CreateSite,
    SiteList,
    PhSquare
  },
  setup() {
    const siteList = ref<any>(null);
    
    const refreshSiteList = () => {
      // Refresh the site list after creating a new site
      if (siteList.value && typeof siteList.value.loadSites === 'function') {
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

.title-bar {
  user-select: none;
}
</style>