<template>
  <div class="home-view bg-gray-50 min-h-screen">
    <div class="container mx-auto px-4 py-6">
      <h1 class="text-3xl font-bold text-center text-gray-800 mb-6">Moodle Docker Manager</h1>
      <div class="space-y-6">
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
  padding: 20px;
}
</style>