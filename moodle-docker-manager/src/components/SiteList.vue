<template>
  <div class="site-list-container bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full p-3 dark:text-gray-100 flex flex-col">
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
    <div class="site-list flex-1 overflow-x-auto">
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
            <button v-if="site.status !== 'running'" @click="startSite(site.name)" class="icon-action-btn bg-green-500 hover:bg-green-600" title="Start">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M5 3v18l15-9L5 3z"/></svg>
            </button>
            <button v-else @click="stopSite(site.name)" class="icon-action-btn bg-red-500 hover:bg-red-600" title="Stop">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="6" y="6" width="12" height="12" rx="2"/></svg>
            </button>
            <button @click="viewLogs(site.name)" class="icon-action-btn bg-blue-500 hover:bg-blue-600" title="Logs">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="4" y="4" width="16" height="16" rx="2"/><path d="M8 8h8M8 12h8M8 16h4"/></svg>
            </button>
            <button @click="viewBackups(site.name)" class="icon-action-btn bg-gray-500 hover:bg-gray-600" title="Backups">
              <PhArchiveBox class="w-5 h-5" />
            </button>
            <button @click="updateSite(site.name)" class="icon-action-btn bg-yellow-500 hover:bg-yellow-600" title="Update">
              <PhArrowClockwise class="w-5 h-5" />
            </button>
            <button @click="removeSite(site.name)" class="icon-action-btn bg-gray-800 hover:bg-red-700" title="Remove">
              <PhTrash class="w-5 h-5 text-red-400" />
            </button>
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
import { PhList, PhArchiveBox, PhArrowClockwise, PhTrash } from '@phosphor-icons/vue';

export default {
  name: 'SiteList',
  components: {
    PhList,
    PhArchiveBox,
    PhArrowClockwise,
    PhTrash
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
    
    // View backups for a site
    const viewBackups = (name) => {
      router.push({ name: 'SiteBackups', params: { siteName: name } });
    };
    
    // Update a site to a newer Moodle version
    const updateSite = (name) => {
      router.push({ name: 'UpdateSite', params: { siteName: name } });
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
      viewBackups,
      updateSite,
      formatPath
    };
  }
};
</script>

<style scoped>
.icon-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 0.5rem;
  color: #fff;
  margin-right: 2px;
  transition: background 0.15s, color 0.15s;
  font-size: 1rem;
  border: none;
  outline: none;
  cursor: pointer;
  position: relative;
  z-index: 10;
}

.icon-action-btn:last-child {
  margin-right: 0;
}


.site-list {
  width: 100%;
  overflow-x: auto;
}

.site-list h2 {
  margin-bottom: 20px;
}

table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 20px;
  font-size: 0.95rem;
}

/* Alternating row hover effect */
tbody tr {
  transition: background 0.15s;
}
tbody tr:nth-child(even) {
  background-color: #f9fafb;
}
tbody tr:hover {
  background-color: #e0e7ef;
}

.dark tbody tr:nth-child(even) {
  background-color: #23272f;
}
.dark tbody tr:hover {
  background-color: #2d3748;
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
  /* Ensure the actions cell does not cause unnecessary wrapping or overflow that could trigger scrollbars */
  white-space: nowrap;
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

/* More dropdown styling */

.dropdown:hover .dropdown-content {
  display: block;
}

.dropdown-content {
  min-width: 8rem; /* Or adjust as needed */
  /* Consider setting a max-width if content can be very long */
  /* max-width: 15rem; */ 
  background-color: white;
  border-radius: 0.375rem;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(209, 213, 219);
  /* overflow: hidden; */ /* This was present, ensure it's intended. If it causes issues with dropdown visibility, review. */
  position: absolute; /* Ensure dropdown is positioned correctly relative to the button */
  right: 0; /* Align to the right of the button */
  z-index: 50; /* Increased z-index to ensure it's above other elements */
}

.compact-dropdown-item {
  display: flex;
  align-items: center;
  width: 100%;
  text-align: left;
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
  transition-property: color, background-color;
  transition-duration: 150ms;
}

.compact-dropdown-item:hover {
  background-color: #f3f4f6;
}

.compact-dropdown-item.text-red-600 {
  color: #dc2626;
}

.compact-dropdown-item.text-red-600:hover {
  background-color: #fef2f2;
}

@media (prefers-color-scheme: dark) {
  .dropdown-content {
    background-color: #374151;
    border-color: rgba(75, 85, 99);
  }
  
  .compact-dropdown-item {
    color: #e5e7eb;
  }
  
  .compact-dropdown-item:hover {
    background-color: #4B5563;
  }
  
  .compact-dropdown-item.text-red-600 {
    color: #f87171;
  }
  
  .compact-dropdown-item.text-red-600:hover {
    background-color: rgba(248, 113, 113, 0.1);
  }
}
</style>
