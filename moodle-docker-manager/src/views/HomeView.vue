<template>
  <div class="home-view bg-gray-50 dark:bg-gray-900 w-full h-screen flex flex-col">
    <div class="title-bar w-full px-4 py-2 flex-none flex items-center justify-center" data-tauri-drag-region>
      <div class="mr-2">
        <svg class="w-6 h-6 text-indigo-600" viewBox="0 0 1024 1024" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect width="1024" height="1024" rx="224" fill="#7066C6"/>
          <g id="icon-elements">
            <!-- M Letter: Scaled and positioned to be 450px wide, 300px tall. Top at y=277 -->
            <path transform="translate(287, 227) scale(0.78125)" d="M0 64H134.4L288 344.3L441.6 64H576V448H480V204.3L326.4 448H249.6L96 204.3V448H0V64Z" fill="white"/>
            <!-- Server Rack: 450px wide, 120px tall. Top at y=627 -->
            <rect x="287" y="627" width="450" height="120" rx="20" fill="white"/>
            <line x1="307" y1="663" x2="717" y2="663" stroke="#5A52A3" stroke-width="10" stroke-linecap="round"/>
            <line x1="307" y1="703" x2="717" y2="703" stroke="#5A52A3" stroke-width="10" stroke-linecap="round"/>
          </g>
        </svg>
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

export default defineComponent({
  name: 'HomeView',
  components: {
    DockerStatus,
    CreateSite,
    SiteList
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