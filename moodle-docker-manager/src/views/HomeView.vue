<template>
  <div class="home-view bg-gray-50 dark:bg-gray-900 min-h-screen w-full">
    <div class="w-full px-4 py-4">
      <h1 class="text-lg font-bold text-center text-gray-800 dark:text-gray-100 mb-3">Moodle Docker Manager</h1>
      <div class="space-y-4">
        <DockerStatus />
        <CreateSite @site-created="refreshSiteList" />
        <SiteList ref="siteList" />
      </div>
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
  padding: 15px;
}
</style>