<template>
  <div class="site-list-container bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full p-3 dark:text-gray-100 flex flex-col">
    <!-- Header - macOS style (more compact) -->
    <div class="flex items-center mb-2">
      <div class="bg-blue-600 rounded-lg p-1 mr-2 shadow-sm site-list-icon">
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
                <PhFileText class="w-5 h-5" weight="fill" />
              </button>
              <button @click="viewBackups(site.name)" class="icon-action-btn bg-gray-500 hover:bg-gray-600" title="Backups">
                <PhArchiveBox class="w-5 h-5" />
              </button>
              <button @click="showUpdateModal(site.name)" class="icon-action-btn bg-orange-600 text-white p-1.5 rounded-lg hover:bg-orange-700 transition-colors shadow-sm" title="Update">
                <PhRecycle class="w-5 h-5" weight="fill" />
              </button>
              <button @click="removeSite(site.name)" class="icon-action-btn bg-gray-800 hover:bg-red-700" title="Remove">
                <PhTrash class="w-5 h-5 text-red-400" />
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Error Modal -->
    <div v-if="showErrorModal" class="fixed inset-0 z-50 overflow-y-auto">
      <div class="flex items-center justify-center min-h-screen px-4">
        <!-- Modal backdrop -->
        <div class="fixed inset-0 bg-black/50 transition-opacity" @click="closeErrorModal"></div>
        
        <!-- Modal panel -->
        <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full">
          <!-- Modal content -->
          <div class="p-6">
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center">
                <div class="bg-red-600 rounded-lg p-1.5 mr-3 shadow-sm">
                  <svg class="text-white w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                  </svg>
                </div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Error</h2>
              </div>
              <button @click="closeErrorModal" class="text-gray-400 hover:text-gray-500">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <!-- Error content -->
            <div class="text-sm text-gray-800 dark:text-gray-200 overflow-auto max-h-96">
              <div v-html="errorContent"></div>
            </div>
            
            <!-- Footer -->
            <div class="mt-6 flex justify-end">
              <button @click="closeErrorModal" class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">Close</button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Update Site Modal -->
    <UpdateSite 
      :show="updateModalVisible"
      :siteName="selectedSiteName"
      @close="hideUpdateModal"
      @updated="handleSiteUpdated" 
    />
    
    <!-- Confirmation Modal for Site Removal -->
    <div v-if="showRemoveModal" class="fixed inset-0 z-50 overflow-y-auto">
      <div class="flex items-center justify-center min-h-screen px-4">
        <!-- Modal backdrop -->
        <div class="fixed inset-0 bg-black/50 transition-opacity" @click="cancelRemove"></div>
        
        <!-- Modal panel -->
        <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <!-- Modal content -->
          <div class="p-6">
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center">
                <div class="bg-red-600 rounded-lg p-1.5 mr-3 shadow-sm">
                  <PhTrash class="text-white w-6 h-6" weight="fill" />
                </div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Remove Site</h2>
              </div>
              <button @click="cancelRemove" class="text-gray-400 hover:text-gray-500">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <!-- Confirmation message -->
            <div class="mb-4 text-sm text-gray-800 dark:text-gray-200">
              <p>Are you sure you want to remove the site <span class="font-semibold">{{ siteToRemove }}</span>?</p>
              <div class="mt-2 flex items-center">
                <input type="checkbox" id="removeContainers" v-model="removeContainers" class="h-4 w-4 text-blue-600 rounded border-gray-300">
                <label for="removeContainers" class="ml-2">Also remove Docker containers and volumes</label>
              </div>
            </div>
            
            <!-- Warning message -->
            <div class="bg-amber-50 dark:bg-amber-900/20 p-3 rounded-md mb-4">
              <p class="text-xs text-amber-600 dark:text-amber-400">
                <span class="font-medium">Warning: </span>
                This action cannot be undone. Please make sure you have any necessary backups.
              </p>
            </div>
            
            <!-- Footer -->
            <div class="mt-4 flex justify-end space-x-2">
              <button @click="cancelRemove" class="flex items-center justify-center px-3 py-1.5 bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 text-sm min-w-[80px]">
                Cancel
              </button>
              <button @click="confirmRemove" class="flex items-center justify-center px-3 py-1.5 bg-red-600 text-white rounded-md hover:bg-red-700 text-sm min-w-[100px]">
                Remove Site
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div> 
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import { PhList, PhArchiveBox, PhArrowClockwise, PhTrash, PhTerminal, PhRecycle } from '@phosphor-icons/vue';
import UpdateSite from './UpdateSite.vue';

export default {
  name: 'SiteList',
  components: {
    PhList,
    PhArchiveBox,
    PhArrowClockwise,
    PhTrash,
    PhTerminal,
    PhRecycle,
    UpdateSite
  },
  setup(props, { expose }) {
    const sites = ref([]);
    const loading = ref(true);
    const error = ref(null);
    const router = useRouter();
    const updateModalVisible = ref(false);
    const selectedSiteName = ref(null);
    const showErrorModal = ref(false);
    const errorContent = ref('');
    const showRemoveModal = ref(false);
    const siteToRemove = ref('');
    const removeContainers = ref(true);

    // Initial load of site list
    const loadSites = async () => {
      loading.value = true;
      error.value = null;
      try {
        // fetch full list including statuses
        sites.value = await invoke('list_sites');
      } catch (err) {
        console.error('Failed to load sites:', err);
        error.value = `Failed to load sites: ${err}`;
      } finally {
        loading.value = false;
      }
    };

    // Show error modal
    const showError = (title, errorMessage) => {
      const formattedError = errorMessage
        .replace(/\n/g, '<br>')
        .replace(/Error output:/g, '<strong class="text-red-500">Error output:</strong>')
        .replace(/Standard output:/g, '<strong class="text-blue-500">Standard output:</strong>');
      
      errorContent.value = `<div class="font-medium mb-2">${title}</div><div>${formattedError}</div>`;
      showErrorModal.value = true;
    };

    // Close error modal
    const closeErrorModal = () => {
      showErrorModal.value = false;
      errorContent.value = '';
    };

    // Refresh only statuses without replacing list
    const refreshStatuses = async () => {
      try {
        const updated = await invoke('list_sites');
        updated.forEach(u => {
          const existing = sites.value.find(s => s.name === u.name);
          if (existing && existing.status !== u.status) {
            existing.status = u.status;
          }
        });
      } catch (err) {
        console.error('Failed to refresh site statuses:', err);
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
        showError(`Failed to start site: ${name}`, String(err));
      } finally {
        loading.value = false;
      }
    };

    // Stop a site
    const stopSite = async (name) => {
      try {
        loading.value = true;
        const result = await invoke('stop_site', { name });
        console.log(`Stop site result: ${result}`);
        await loadSites(); // Refresh site list
      } catch (err) {
        console.error(`Failed to stop site ${name}:`, err);
        showError(`Failed to stop site: ${name}`, String(err));
      } finally {
        loading.value = false;
      }
    };

    // Show remove site modal
    const removeSite = (name) => {
      siteToRemove.value = name;
      removeContainers.value = true; // Default to true
      showRemoveModal.value = true;
    };
    
    // Cancel remove operation
    const cancelRemove = () => {
      showRemoveModal.value = false;
      siteToRemove.value = '';
    };
    
    // Confirm and execute site removal
    const confirmRemove = async () => {
      try {
        loading.value = true;
        showRemoveModal.value = false;
        
        // Pass the removeContainers flag to the backend
        await invoke('remove_site', { 
          name: siteToRemove.value,
          removeContainers: removeContainers.value 
        });
        
        await loadSites(); // Refresh site list
      } catch (err) {
        console.error(`Failed to remove site ${siteToRemove.value}:`, err);
        showError(`Failed to remove site: ${siteToRemove.value}`, String(err));
      } finally {
        loading.value = false;
        siteToRemove.value = '';
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
    
    // Update site modal controls
    const showUpdateModal = (siteName) => {
      selectedSiteName.value = siteName;
      updateModalVisible.value = true;
    };
    
    const hideUpdateModal = () => {
      updateModalVisible.value = false;
      selectedSiteName.value = null;
    };
    
    const handleSiteUpdated = () => {
      loadSites();
      hideUpdateModal();
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

    let refreshTimer;
    onMounted(() => {
      loadSites();
      // Poll for status changes every 5 seconds without full re-render
      refreshTimer = setInterval(refreshStatuses, 5000);
    });
    onBeforeUnmount(() => clearInterval(refreshTimer));
    
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
      showUpdateModal,
      hideUpdateModal,
      handleSiteUpdated,
      formatPath,
      updateModalVisible,
      selectedSiteName,
      showErrorModal,
      errorContent,
      closeErrorModal,
      // New removal modal properties and methods
      showRemoveModal,
      siteToRemove,
      removeContainers,
      cancelRemove,
      confirmRemove
    };
  }
};
</script>

<style scoped>
.icon-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  color: white !important;
  margin-right: 4px;
  transition: background-color 0.15s ease;
  cursor: pointer;
}

/* Ensure action buttons are visible in light mode */
.icon-action-btn svg, 
.icon-action-btn path {
  color: white !important;
  fill: currentColor;
}

/* Explicit styling for the orange update button to ensure visibility in both modes */
.icon-action-btn.bg-orange-600 {
  background-color: #ea580c !important;
}
.icon-action-btn.bg-orange-600 svg {
  color: white !important;
  fill: currentColor;
}

/* Explicit styling for the blue site list icon to ensure visibility in both modes */
.site-list-icon {
  background-color: #2563eb !important; /* bg-blue-600 */
}
.site-list-icon svg {
  color: white !important;
  fill: currentColor;
}
.site-list-icon svg {
  color: white !important;
  fill: currentColor;
}

.compact-table {
  width: 100%;
  font-size: 0.75rem; /* text-xs */
  border-collapse: separate;
  border-spacing: 0;
}

.compact-table th {
  padding: 0.5rem 0.75rem;
  text-align: left;
  font-weight: 500;
  color: #4b5563; /* text-gray-600 */
  background-color: #f9fafb; /* bg-gray-50 */
}

.dark .compact-table th {
  background-color: #374151; /* dark:bg-gray-700 */
  color: #e5e7eb; /* dark:text-gray-300 */
}

.compact-table td {
  padding: 0.5rem 0.75rem;
  border-top: 1px solid #e5e7eb; /* border-t border-gray-200 */
}

.dark .compact-table td {
  border-top: 1px solid #374151; /* dark:border-gray-700 */
}

.compact-table tr:hover {
  background-color: #f3f4f6; /* hover:bg-gray-100 */
}

.dark .compact-table tr:hover {
  background-color: #1f2937; /* dark:hover:bg-gray-800 */
}

.compact-status-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.125rem 0.375rem;
  border-radius: 0.25rem;
  font-size: 0.75rem; /* text-xs */
}

.directory-path {
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.actions {
  white-space: nowrap;
}

.compact-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.375rem 0.75rem;
  border-radius: 0.375rem;
  font-size: 0.75rem;
  font-weight: 500;
  margin-top: 0.5rem;
  transition: background-color 0.15s ease;
}
</style>
