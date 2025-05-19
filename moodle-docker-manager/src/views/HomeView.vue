<template>
  <div class="home-view">
    <h1>Moodle Docker Manager</h1>
    <DockerStatus />
    <CreateSite @site-created="refreshSiteList" />
    <SiteList ref="siteList" />
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