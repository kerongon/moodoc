<template>
  <div class="site-list-container bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto p-3 dark:text-gray-100 overflow-hidden flex flex-col">
    <!-- Header - macOS style (more compact) -->
    <div class="flex items-center mb-2">
      <div class="bg-blue-600 rounded-lg p-1 mr-2 shadow-sm">
        <PhList class="text-white w-5 h-5" weight="fill" />
      </div>
      <div>
        <h2 class="text-sm font-medium text-gray-900 dark:text-gray-100">Configured Sites</h2>
        <p class="text-[10px] text-gray-500 dark:text-gray-400">Manage your Moodle development environments</p>
      </div>
    </div>
    <div class="site-list overflow-auto flex-1">
    <div v-if="loading" class="loading text-xs">
      Loading sites...
    </div>
    <div v-else-if="error" class="error text-xs">
      {{ error }}
    </div>
    <div v-else-if="sites.length === 0" class="no-sites text-xs">
      <p>No sites have been created yet.</p>
      <router-link to="/create" class="compact-button bg-blue-600 text-white hover:bg-blue-700">Create a Site</router-link>
    </div>
    <table v-else class="compact-table">
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
            <span class="compact-status-badge" :class="{'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300': site.status === 'running', 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300': site.status !== 'running'}">
              <span class="inline-block w-1.5 h-1.5 rounded-full mr-1" :class="{'bg-green-500': site.status === 'running', 'bg-gray-400': site.status !== 'running'}"></span>
              {{ site.status }}
            </span>
          </td>
          <td class="actions">
            <button v-if="site.status !== 'running'" @click="startSite(site.name)" class="compact-action-button bg-green-500 text-white hover:bg-green-600">Start</button>
            <button v-else @click="stopSite(site.name)" class="compact-action-button bg-red-500 text-white hover:bg-red-600">Stop</button>
            <button @click="viewLogs(site.name)" class="compact-action-button bg-blue-500 text-white hover:bg-blue-600">Logs</button>
            <button @click="removeSite(site.name)" class="compact-action-button bg-amber-500 text-white hover:bg-amber-600">Remove</button>
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
  font-size: 0.75rem; /* Smaller text */
}

thead {
  background-color: #f9fafb;
}

.dark thead {
  background-color: #374151;
}

th, td {
  padding: 8px 10px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

th {
  text-align: left;
  padding: 6px 8px;
  font-weight: 500;
  color: #4b5563;
  border-bottom: 1px solid #e5e7eb;
  background-color: #f9fafb;
}

.dark th {
  color: #d1d5db;
  border-bottom: 1px solid #4b5563;
  background-color: #374151;
}

td {
  padding: 5px 8px;
  border-bottom: 1px solid #f3f4f6;
}

.dark td {
  border-bottom: 1px solid #374151;
}

.actions {
  display: flex;
  gap: 8px;
}

.action-button {
  padding: 4px 8px;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s ease;
  box-shadow: 0 1px 2px rgba(0,0,0,0.1);
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

/* Status badges */
.status-running, .status-stopped {
  display: inline-flex;
  align-items: center;
  border-radius: 9999px;
  padding: 0 0.375rem;
  font-size: 0.7rem;
  font-weight: 500;
}

.status-running {
  background-color: #ecfdf5;
  color: #065f46;
  font-weight: 500;
}

.dark .status-running {
  background-color: rgba(6, 95, 70, 0.3);
  color: #34d399;
}

.status-stopped {
  background-color: #f3f4f6;
  color: #4b5563;
  font-weight: 500;
}

.dark .status-stopped {
  background-color: #374151;
  color: #d1d5db;
}

.loading, .error, .no-sites {
  padding: 15px;
  text-align: center;
  border-radius: 4px;
  margin-top: 15px;
}

.loading {
  background-color: #e3f2fd;
}

.dark .loading {
  background-color: rgba(59, 130, 246, 0.1);
}

.error {
  background-color: #fee2e2;
  color: #b91c1c;
}

.dark .error {
  background-color: rgba(185, 28, 28, 0.2);
  color: #fca5a5;
}

.no-sites {
  background-color: #f5f5f5;
}

.dark .no-sites {
  background-color: #374151;
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
