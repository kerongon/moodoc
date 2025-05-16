<template>
  <div class="site-list">
    <h2>Configured Sites</h2>
    <div v-if="loading" class="loading">
      Loading sites...
    </div>
    <div v-else-if="error" class="error">
      {{ error }}
    </div>
    <div v-else-if="sites.length === 0" class="no-sites">
      <p>No sites have been created yet.</p>
      <router-link to="/create" class="create-button">Create a Site</router-link>
    </div>
    <table v-else>
      <thead>
        <tr>
          <th>Site Name</th>
          <th>PHP Version</th>
          <th>Database Type</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="site in sites" :key="site.name">
          <td>{{ site.name }}</td>
          <td>{{ site.php_version }}</td>
          <td>{{ site.db_type }}</td>
          <td>
            <span :class="{'status-running': site.status === 'running', 'status-stopped': site.status !== 'running'}">
              {{ site.status }}
            </span>
          </td>
          <td class="actions">
            <button v-if="site.status !== 'running'" @click="startSite(site.name)" class="action-button start">Start</button>
            <button v-else @click="stopSite(site.name)" class="action-button stop">Stop</button>
            <button @click="viewLogs(site.name)" class="action-button logs">Logs</button>
            <button @click="removeSite(site.name)" class="action-button remove">Remove</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { ref, onMounted } from 'vue';

export default {
  name: 'SiteList',
  setup() {
    const sites = ref([]);
    const loading = ref(true);
    const error = ref(null);

    // Load sites from Rust backend
    const loadSites = async () => {
      loading.value = true;
      error.value = null;

      try {
        sites.value = await invoke('list_sites');
      } catch (err) {
        console.error('Failed to load sites:', err);
        error.value = `Failed to load sites: ${err}`;
      } finally {
        loading.value = false;
      }
    };

    // Start a site
    const startSite = async (name) => {
      try {
        loading.value = true;
        await invoke('start_site', { name });
        await loadSites(); // Refresh site list
      } catch (err) {
        console.error(`Failed to start site ${name}:`, err);
        alert(`Failed to start site: ${err}`);
      } finally {
        loading.value = false;
      }
    };
    };

    const removeSite = (id: number) => {
      store.removeSite(id);
    };

    const editSite = (id: number) => {
      store.editSite(id);
    };

    return {
      sites,
      startSite,
      stopSite,
      removeSite,
      editSite,
    };
  },
});
</script>

<style scoped>
.site-list {
  margin: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  border: 1px solid #ddd;
  padding: 8px;
}

th {
  background-color: #f2f2f2;
}

.running {
  color: green;
}

.stopped {
  color: red;
}
</style>