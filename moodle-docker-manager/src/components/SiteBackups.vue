<template>
  <div class="site-backups bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto my-4 p-4">
    <!-- Header - macOS style (more compact) -->
    <div class="flex items-center mb-5">
      <div class="bg-green-600 rounded-lg p-1.5 mr-3 shadow-sm">
        <PhArchiveBox class="text-white w-6 h-6" weight="fill" />
      </div>
      <div>
        <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Site Backups: {{ siteName }}</h2>
        <p class="text-xs text-gray-500 dark:text-gray-400">Manage backups for this Moodle site</p>
      </div>
    </div>

    <div v-if="loading" class="loading text-xs">
      <div class="flex items-center justify-center space-x-2">
        <div class="h-4 w-4 bg-blue-600 rounded-full animate-bounce" style="animation-delay: 0.0s"></div>
        <div class="h-4 w-4 bg-blue-600 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
        <div class="h-4 w-4 bg-blue-600 rounded-full animate-bounce" style="animation-delay: 0.4s"></div>
      </div>
      <p class="text-center mt-2">{{ loadingMessage }}</p>
    </div>
    <div v-else-if="error">
      <p class="error text-xs text-red-500">{{ error }}</p>
    </div>
    <div v-else>
      <!-- Actions -->
      <div class="mb-4 flex justify-between items-center">
        <button @click="createBackup" 
                :disabled="isProcessing" 
                class="compact-action-button bg-green-500 text-white hover:bg-green-600 disabled:bg-gray-300 disabled:cursor-not-allowed">
          <PhPlus class="w-4 h-4 mr-1" />
          Create Backup
        </button>
        
        <button @click="refreshBackups" 
                :disabled="isProcessing" 
                class="compact-action-button bg-blue-500 text-white hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed">
          <PhArrowClockwise class="w-4 h-4 mr-1" />
          Refresh
        </button>
      </div>
      
      <!-- Backups List -->
      <div v-if="backups.length === 0" class="text-center text-gray-500 dark:text-gray-400 my-8">
        <PhFolderNotch class="w-10 h-10 mx-auto mb-2" />
        <p>No backups available</p>
        <p class="text-xs mt-1">Create your first backup using the button above</p>
      </div>
      
      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-700">
            <tr>
              <th scope="col" class="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-300 text-left">Date</th>
              <th scope="col" class="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-300 text-left">Moodle Version</th>
              <th scope="col" class="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-300 text-left">PHP Version</th>
              <th scope="col" class="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-300 text-left">Database</th>
              <th scope="col" class="px-4 py-2 text-xs font-medium text-gray-500 dark:text-gray-300 text-left">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-for="backup in backups" :key="backup.timestamp" class="hover:bg-gray-50 dark:hover:bg-gray-700">
              <td class="px-4 py-2 text-xs text-gray-900 dark:text-gray-100">
                {{ formatDate(backup.timestamp) }}
              </td>
              <td class="px-4 py-2 text-xs text-gray-900 dark:text-gray-100">{{ backup.moodle_branch }}</td>
              <td class="px-4 py-2 text-xs text-gray-900 dark:text-gray-100">{{ backup.php_version }}</td>
              <td class="px-4 py-2 text-xs text-gray-900 dark:text-gray-100">{{ backup.db_type }}</td>
              <td class="px-4 py-2 text-xs">
                <div class="flex space-x-1">
                  <button 
                    @click="restoreBackup(backup.timestamp)" 
                    :disabled="isProcessing" 
                    class="compact-mini-button bg-blue-500 hover:bg-blue-600 text-white"
                    title="Restore this backup">
                    <PhArrowClockwise class="w-3 h-3" />
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    
    <!-- Restore Confirmation Modal -->
    <div v-if="showRestoreModal" class="fixed inset-0 flex items-center justify-center z-50 bg-black bg-opacity-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-5 w-96 mx-3">
        <div class="mb-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100">Confirm Restore</h3>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            Are you sure you want to restore this backup? This will overwrite the current site content and database.
          </p>
        </div>
        <div class="text-xs text-red-500 font-medium mb-3">
          <p>⚠️ This action cannot be undone.</p>
        </div>
        <div class="flex justify-end space-x-3">
          <button 
            @click="showRestoreModal = false" 
            class="compact-action-button bg-gray-300 text-gray-700 hover:bg-gray-400">
            Cancel
          </button>
          <button 
            @click="confirmRestore" 
            :disabled="isProcessing" 
            class="compact-action-button bg-red-500 text-white hover:bg-red-600 disabled:bg-gray-300">
            {{ isProcessing ? 'Restoring...' : 'Restore Backup' }}
          </button>
        </div>
      </div>
    </div>
    
    <div class="actions mt-3 flex gap-2">
      <button @click="$router.push('/dashboard')" class="compact-action-button bg-gray-500 text-white hover:bg-gray-600">
        Back to Dashboard
      </button>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { PhArchiveBox, PhPlus, PhArrowClockwise, PhFolderNotch } from '@phosphor-icons/vue';

export default {
  name: 'SiteBackups',
  components: {
    PhArchiveBox,
    PhPlus,
    PhArrowClockwise,
    PhFolderNotch
  },
  props: {
    siteName: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      backups: [],
      loading: false,
      isProcessing: false,
      error: null,
      loadingMessage: 'Loading backups...',
      showRestoreModal: false,
      selectedBackupTimestamp: null
    };
  },
  methods: {
    async refreshBackups() {
      this.loading = true;
      this.error = null;
      this.loadingMessage = 'Loading backups...';
      
      try {
        this.backups = await invoke('list_site_backups', { 
          name: this.siteName 
        });
      } catch (err) {
        this.error = `Failed to fetch backups: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
      }
    },
    
    async createBackup() {
      this.isProcessing = true;
      this.loading = true;
      this.loadingMessage = 'Creating backup...';
      this.error = null;
      
      try {
        const result = await invoke('backup_site', { 
          name: this.siteName 
        });
        
        console.log(result);
        await this.refreshBackups();
      } catch (err) {
        this.error = `Failed to create backup: ${err}`;
        console.error(err);
        this.loading = false;
      } finally {
        this.isProcessing = false;
      }
    },
    
    restoreBackup(timestamp) {
      this.selectedBackupTimestamp = timestamp;
      this.showRestoreModal = true;
    },
    
    async confirmRestore() {
      if (!this.selectedBackupTimestamp) return;
      
      this.isProcessing = true;
      this.showRestoreModal = false;
      this.loading = true;
      this.loadingMessage = 'Restoring backup...';
      this.error = null;
      
      try {
        const result = await invoke('restore_site_backup', { 
          name: this.siteName,
          backupTimestamp: this.selectedBackupTimestamp
        });
        
        console.log(result);
      } catch (err) {
        this.error = `Failed to restore backup: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
        this.isProcessing = false;
      }
    },
    
    formatDate(timestamp) {
      if (!timestamp) return '';
      
      // Format like: "May 20, 2025 at 14:30"
      try {
        // Parse YYYYMMDD_HHMMSS format
        const year = timestamp.substring(0, 4);
        const month = timestamp.substring(4, 6);
        const day = timestamp.substring(6, 8);
        const hour = timestamp.substring(9, 11);
        const minute = timestamp.substring(11, 13);
        
        const date = new Date(`${year}-${month}-${day}T${hour}:${minute}:00`);
        
        return new Intl.DateTimeFormat('en-US', {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit'
        }).format(date);
      } catch (e) {
        return timestamp;
      }
    }
  },
  mounted() {
    this.refreshBackups();
  }
};
</script>

<style scoped>
.site-backups {
  max-width: 800px;
  margin: 0 auto;
}

.compact-action-button {
  @apply flex items-center justify-center px-3 py-1 text-xs font-medium rounded-md transition-colors duration-150;
}

.compact-mini-button {
  @apply flex items-center justify-center p-1 text-xs font-medium rounded transition-colors duration-150;
}
</style>
