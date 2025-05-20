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

    <!-- Success message notification -->
    <div v-if="showSuccessMessage" class="mb-4 p-3 bg-green-100 border border-green-200 rounded-md">
      <div class="flex items-start">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-green-800">{{ successMessage }}</p>
        </div>
        <div class="ml-auto pl-3">
          <button @click="showSuccessMessage = false" class="inline-flex text-green-500 hover:text-green-700">
            <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
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
      <div class="error text-xs text-red-500 p-3 bg-red-50 border border-red-100 rounded-md overflow-auto max-h-60" v-html="error"></div>
    </div>
    <div v-else>
      <!-- Actions -->
      <div class="mb-4 flex justify-between items-center">
        <div class="flex space-x-2">
          <button @click="createBackup" 
                  :disabled="isProcessing" 
                  class="compact-action-button bg-green-500 text-white hover:bg-green-600 disabled:bg-gray-300 disabled:cursor-not-allowed">
            <PhPlus class="w-4 h-4 mr-1" />
            Create Backup
          </button>
          
          <button @click="openBackupsFolder" 
                  :disabled="isProcessing" 
                  class="compact-action-button bg-gray-500 text-white hover:bg-gray-600 disabled:bg-gray-300 disabled:cursor-not-allowed">
            <PhFolderOpen class="w-4 h-4 mr-1" />
            Open Folder
          </button>
        </div>
        
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
                  <button 
                    @click="openInFinder(backup.timestamp)" 
                    :disabled="isProcessing" 
                    class="compact-mini-button bg-green-500 hover:bg-green-600 text-white"
                    title="Show in Finder">
                    <PhFolderOpen class="w-3 h-3" />
                  </button>
                  <button 
                    @click="confirmDelete(backup.timestamp)" 
                    :disabled="isProcessing" 
                    class="compact-mini-button bg-red-500 hover:bg-red-600 text-white"
                    title="Delete this backup">
                    <PhTrash class="w-3 h-3" />
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
    
    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 flex items-center justify-center z-50 bg-black bg-opacity-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-5 w-96 mx-3">
        <div class="mb-4">
          <h3 class="text-lg font-medium text-gray-900 dark:text-gray-100">Confirm Delete</h3>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            Are you sure you want to delete this backup from {{ formatDate(confirmingDeleteTimestamp) }}?
          </p>
        </div>
        <div class="text-xs text-red-500 font-medium mb-3">
          <p>⚠️ This action cannot be undone.</p>
        </div>
        <div class="flex justify-end space-x-3">
          <button 
            @click="showDeleteConfirm = false" 
            class="compact-action-button bg-gray-300 text-gray-700 hover:bg-gray-400">
            Cancel
          </button>
          <button 
            @click="deleteBackup" 
            :disabled="isProcessing" 
            class="compact-action-button bg-red-500 text-white hover:bg-red-600 disabled:bg-gray-300">
            {{ isProcessing ? 'Deleting...' : 'Delete Backup' }}
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
import { PhArchiveBox, PhPlus, PhArrowClockwise, PhFolderNotch, PhFolderOpen, PhTrash } from '@phosphor-icons/vue';

export default {
  name: 'SiteBackups',
  components: {
    PhArchiveBox,
    PhPlus,
    PhArrowClockwise,
    PhFolderNotch,
    PhFolderOpen,
    PhTrash
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
      showRestoreModal: false,
      selectedBackupTimestamp: null,
      confirmingDeleteTimestamp: null,
      showDeleteConfirm: false,
      loadingMessage: 'Loading backups...',
      showSuccessMessage: false,
      successMessage: '',
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
      this.loadingMessage = 'Creating backup... This might take a few minutes depending on your site size';
      this.error = null;
      
      try {
        const result = await invoke('backup_site', { 
          name: this.siteName 
        });
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
      } catch (err) {
        const errorMsg = String(err);
        console.error(`Backup error:`, errorMsg);
        
        // Format error message for display
        if (errorMsg.includes("Error output:") || errorMsg.includes("Standard output:")) {
          this.error = this.formatErrorMessage(errorMsg);
        } else {
          this.error = `Failed to create backup: ${errorMsg}`;
        }
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
      this.loadingMessage = 'Restoring backup... This might take a few minutes';
      this.error = null;
      
      try {
        const result = await invoke('restore_site_backup', { 
          name: this.siteName,
          backupTimestamp: this.selectedBackupTimestamp
        });
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
      } catch (err) {
        const errorMsg = String(err);
        if (errorMsg.includes("Error output:") || errorMsg.includes("Standard output:")) {
          this.error = this.formatErrorMessage(errorMsg);
        } else {
          this.error = `Failed to restore backup: ${errorMsg}`;
        }
        console.error(err);
      } finally {
        this.loading = false;
        this.isProcessing = false;
      }
    },
    
    confirmDelete(timestamp) {
      this.confirmingDeleteTimestamp = timestamp;
      this.showDeleteConfirm = true;
    },
    
    async deleteBackup() {
      if (!this.confirmingDeleteTimestamp) return;
      
      this.isProcessing = true;
      this.showDeleteConfirm = false;
      this.loading = true;
      this.loadingMessage = 'Deleting backup...';
      this.error = null;
      
      try {
        const result = await invoke('delete_site_backup', { 
          name: this.siteName,
          backupTimestamp: this.confirmingDeleteTimestamp
        });
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
      } catch (err) {
        this.error = `Failed to delete backup: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
        this.isProcessing = false;
        this.confirmingDeleteTimestamp = null;
      }
    },
    
    async openInFinder(timestamp) {
      try {
        const result = await invoke('open_backup_location', { 
          name: this.siteName,
          backupTimestamp: timestamp
        });
        console.log(result);
      } catch (err) {
        this.error = `Failed to open in Finder: ${err}`;
        console.error(err);
      }
    },
    
    async openBackupsFolder() {
      try {
        const result = await invoke('open_backup_location', { 
          name: this.siteName
        });
        console.log(result);
      } catch (err) {
        this.error = `Failed to open backups folder: ${err}`;
        console.error(err);
      }
    },
    
    // Enhanced error message formatter with better detail extraction
    formatErrorMessage(errorMsg) {
      let formattedMsg = 'Operation failed:';
      
      // Extract error details
      const errorOutputMatch = errorMsg.match(/Error output:([\s\S]*?)(?=\n\nStandard output:|$)/);
      const stdOutputMatch = errorMsg.match(/Standard output:([\s\S]*?)$/);
      
      // Extract common database errors
      const dbErrorMatch = errorMsg.match(/mysqldump: (.*?)$/m) || errorMsg.match(/pg_dumpall: (.*?)$/m);
      
      if (dbErrorMatch && dbErrorMatch[1]) {
        formattedMsg += `\n\n<div class="error-box">
          <div class="error-title">Database Error Detected:</div>
          <div class="error-message">${dbErrorMatch[1]}</div>
          <div class="error-hint">This appears to be a database connection issue. The backup might be missing the database dump.</div>
        </div>`;
      }
      
      if (errorOutputMatch && errorOutputMatch[1]) {
        const errorDetails = errorOutputMatch[1].trim();
        formattedMsg += `\n\n<div class="error-details">
          <div class="error-title">Error Details:</div>
          <pre class="error-content">${errorDetails}</pre>
        </div>`;
      }
      
      if (stdOutputMatch && stdOutputMatch[1]) {
        const stdOutput = stdOutputMatch[1].trim();
        if (stdOutput) {
          formattedMsg += `\n\n<div class="output-details">
            <div class="output-title">Command Output:</div>
            <pre class="output-content">${stdOutput}</pre>
          </div>`;
        }
      }
      
      return formattedMsg;
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

<style>
.site-backups {
  max-width: 800px;
  margin: 0 auto;
}

.compact-action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem 0.75rem;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: 0.375rem;
  transition: color 0.15s ease, background-color 0.15s ease;
}

.table-action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: 0.25rem;
  transition: color 0.15s ease, background-color 0.15s ease;
}

.error-details, .output-details {
  margin-top: 10px;
  border: 1px solid #fee2e2;
  border-radius: 6px;
  padding: 8px;
  background-color: #fef2f2;
}

.error-box {
  margin-top: 10px;
  border: 1px solid #f87171;
  border-radius: 6px;
  padding: 12px;
  background-color: #fee2e2;
}

.error-title, .output-title {
  font-weight: 500;
  color: #dc2626;
  margin-bottom: 4px;
}

.error-message {
  font-weight: 500;
  color: #b91c1c;
  padding: 6px;
  background-color: #fecaca;
  border-radius: 4px;
  margin: 6px 0;
}

.error-hint {
  font-size: 0.7rem;
  color: #7f1d1d;
  font-style: italic;
  margin-top: 4px;
}

.output-title {
  color: #2563eb;
}

.error-content, .output-content {
  font-family: monospace;
  font-size: 0.75rem;
  white-space: pre-wrap;
  overflow-x: auto;
  padding: 6px;
  background-color: #fff;
  border-radius: 4px;
  border: 1px solid #f3f4f6;
}

.compact-mini-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.15rem 0.3rem;
  border-radius: 0.25rem;
  transition: all 0.15s ease;
}
</style>
