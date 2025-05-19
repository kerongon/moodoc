<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto my-6 p-6 dark:text-gray-100">
    <!-- Header - macOS style -->
    <div class="flex items-center mb-8">
      <div class="bg-blue-600 rounded-lg p-2 mr-4 shadow-sm">
        <PhList class="text-white w-8 h-8" weight="fill" />
      </div>
      <div>
        <h2 class="text-xl font-medium text-gray-900 dark:text-gray-100">Configured Sites</h2>
        <p class="text-sm text-gray-500 dark:text-gray-400">Manage your Moodle development environments</p>
      </div>
    </div>
    <div class="site-list">
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
          <th>Directory</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="site in sites" :key="site.name">
          <td>{{ site.name }}</td>
          <td>{{ site.php_version }}</td>
          <td>{{ site.db_type }}</td>
          <td class="directory-path" :title="site.path">
            {{ formatPath(site.path) }}
          </td>
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
    </div> <!-- closes site-list -->
  </div> <!-- closes macos-window -->
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { PhList } from '@phosphor-icons/vue';

export default {
  name: 'SiteList',
  components: {
    PhList
  },
  setup(props, { expose }) {
    const sites = ref([]);
    const loading = ref(true);
    const error = ref(null);
    const router = useRouter();

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

    // Stop a site
    const stopSite = async (name) => {
      try {
        loading.value = true;
        await invoke('stop_site', { name });
        await loadSites(); // Refresh site list
      } catch (err) {
        console.error(`Failed to stop site ${name}:`, err);
        alert(`Failed to stop site: ${err}`);
      } finally {
        loading.value = false;
      }
    };

    // Remove a site
    const removeSite = async (name) => {
      if (confirm(`Are you sure you want to remove the site '${name}'? This will delete all containers and data.`)) {
        try {
          loading.value = true;
          await invoke('remove_site', { name });
          await loadSites(); // Refresh site list
        } catch (err) {
          console.error(`Failed to remove site ${name}:`, err);
          alert(`Failed to remove site: ${err}`);
        } finally {
          loading.value = false;
        }
      }
    };

    // View logs for a site
    const viewLogs = (name) => {
      router.push({ name: 'SiteLogs', params: { siteName: name } });
    };

    // Format long paths for display
    const formatPath = (path) => {
      if (!path) return 'N/A';
      
      // If path is too long, truncate it
      const maxLength = 30;
      if (path.length <= maxLength) return path;
      
      // Extract directory name and show ellipsis
      const parts = path.split('/');
      const filename = parts.pop();
      return `.../${parts.slice(-2).join('/')}/${filename}`;
    };

    onMounted(() => {
      loadSites();
    });
    
    // Expose the loadSites method to parent components
    expose({ loadSites });

    return {
      sites,
      loading,
      error,
      startSite,
      stopSite,
      removeSite,
      viewLogs,
      formatPath
    };
  }
};
</script>

<style scoped>
.site-list {
  width: 100%;
  max-width: 800px;
  margin: auto;
}

.site-list h2 {
  margin-bottom: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 20px;
}

th, td {
  padding: 12px 15px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  background-color: #f2f2f2;
  font-weight: bold;
}

.actions {
  display: flex;
  gap: 8px;
}

.action-button {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0,0,0,0.12);
}

.action-button.start {
  background-color: #10B981;
  color: white;
}

.action-button.start:hover {
  background-color: #059669;
}

.action-button.stop {
  background-color: #EF4444;
  color: white;
}

.action-button.stop:hover {
  background-color: #DC2626;
}

.action-button.logs {
  background-color: #3B82F6;
  color: white;
}

.action-button.logs:hover {
  background-color: #2563EB;
}

.action-button.remove {
  background-color: #F59E0B;
  color: white;
}

.action-button.remove:hover {
  background-color: #D97706;
}

.status-running {
  color: #4CAF50;
  font-weight: bold;
}

.status-stopped {
  color: #f44336;
}

.loading, .error, .no-sites {
  padding: 20px;
  text-align: center;
  border-radius: 4px;
  margin-top: 20px;
}

.loading {
  background-color: #e3f2fd;
}

.error {
  background-color: #ffebee;
  color: #f44336;
}

.no-sites {
  background-color: #f5f5f5;
}

.create-button {
  display: inline-block;
  margin-top: 10px;
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  text-decoration: none;
  border-radius: 4px;
}

.directory-path {
  font-family: monospace;
  font-size: 0.9em;
  color: #666;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 200px;
  white-space: nowrap;
  cursor: default;
}
</style>
