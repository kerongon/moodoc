<template>
  <div>
    <div class="dashboard-full hide-scrollbar bg-white dark:bg-gray-900 min-h-screen w-full p-0">
      <!-- Modern macOS-style header (similar to CreateSite.vue, no New Site button) -->
      <header class="app-header" data-tauri-drag-region>
        <div class="header-content">
          <div class="titlebar-left-controls">
            <!-- Window Controls -->
            <div class="window-controls">
              <button @click="closeWindow" class="window-control-button close-button" title="Close">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1"></circle></svg>
              </button>
              <!-- <button @click="minimizeWindow" class="window-control-button minimize-button" title="Minimize">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FFBD2E" stroke="#DEA123" stroke-width="1"></circle></svg>
              </button> -->
              <button @click="toggleMaximizeWindow" class="window-control-button maximize-button" title="Maximize">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#27C93F" stroke="#1AAB29" stroke-width="1"></circle></svg>
              </button>
            </div>
            <!-- Navigation Controls -->
            <div class="navigation-controls">
              <button @click="goBack" class="header-nav-button" title="Go Back">
                <PhArrowLeft class="button-icon" />
              </button>
              <button @click="goForward" class="header-nav-button" title="Go Forward">
                <PhArrowRight class="button-icon" />
              </button>
              <button @click="reloadPage" class="header-nav-button" title="Reload Page">
                <PhArrowClockwise class="button-icon" />
              </button>
            </div>
          </div>

          <!-- Logo and app title -->
          <div class="app-branding">
            <div class="logo-container">
              <PhList class="logo-icon" weight="fill" />
            </div>
            <div class="title-container">
              <h1 class="app-title">Moodle Docker Manager</h1>
            </div>
          </div>
          <!-- Header actions -->
          <div class="header-actions">
            <DockerStatus class="docker-status-button" />
            <button @click="openSettings" class="settings-button" title="Settings">
              <PhGear class="button-icon" />
            </button>
          </div>
        </div>
      </header>
      
      <div class="main-content px-8 pb-8 hide-scrollbar">
        <!-- Breadcrumbs -->
        <div class="flex py-2 mb-2">
          <nav class="text-sm text-gray-600 dark:text-gray-400">
            <router-link to="/" class="hover:underline">Home</router-link>
            <span class="mx-2">›</span>
            <span>Site Backups</span>
          </nav>
        </div>
        
        <div class="site-backups bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full p-4">

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
      <button @click="$router.push('/')" class="compact-action-button bg-gray-500 text-white hover:bg-gray-600">
        Back to Dashboard
      </button>
    </div>
  </div> <!-- end of .site-backups -->
  </div> <!-- end of .main-content -->
</div> <!-- end of dashboard-full -->
</div> <!-- end of root div -->
  <SettingsModal ref="settingsModal" />
</template>

<script>
import DockerStatus from './DockerStatus.vue';
import SettingsModal from './SettingsModal.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { PhArchiveBox, PhPlus, PhArrowClockwise, PhFolderNotch, PhFolderOpen, PhTrash, PhArrowLeft, PhArrowRight, PhGear, PhList } from '@phosphor-icons/vue';

export default {
  name: 'SiteBackups',
  components: {
    DockerStatus,
    SettingsModal,
    PhArchiveBox,
    PhPlus,
    PhArrowClockwise,
    PhFolderNotch,
    PhFolderOpen,
    PhTrash,
    PhArrowLeft,
    PhArrowRight,
    PhGear,
    PhList
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
    // Window control methods
    openSettings() {
      const modal = this.$refs.settingsModal;
      if (modal && typeof modal.openModal === 'function') {
        modal.openModal();
      }
    },
    goBack() {
      this.$router.go(-1);
    },
    goForward() {
      this.$router.go(1);
    },
    reloadPage() {
      window.location.reload();
    },
    minimizeWindow() {
      appWindow.minimize();
    },
    async toggleMaximizeWindow() {
      const isMaximized = await appWindow.isMaximized();
      if (isMaximized) {
        appWindow.unmaximize();
      } else {
        appWindow.maximize();
      }
    },
    closeWindow() {
      appWindow.close();
    },
    
    // Existing backup methods
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
  max-width: none;
  margin: 0;
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

/* Error and output message styling */
.error-details,
.output-details {
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

.error-title,
.output-title {
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

.error-content,
.output-content {
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

/* Modern macOS-style header CSS */
.dashboard-full {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  width: 100vw;
  background: inherit;
  padding: 0;
}

.app-header {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  padding: 0.75rem 1rem;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  transition: all 0.3s ease;
  height: 56px;
  display: flex;
  align-items: center;
}

.main-content {
  padding-top: 72px;
  flex-grow: 1;
  overflow-y: auto;
}

.dark .app-header {
  background: rgba(17, 24, 39, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  margin: 0 auto;
  padding: 0 0.5rem;
}

.titlebar-left-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-right: 12px;
}

.window-control-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: default;
}

/* Specific styles for each button to mimic macOS traffic lights */
.close-button svg circle { fill: #FF5F57; stroke: #E0443E; }
.minimize-button svg circle { fill: #FFBD2E; stroke: #DEA123; }
.maximize-button svg circle { fill: #27C93F; stroke: #1AAB29; }

/* Always display the macOS traffic-light icons */
.window-control-button svg {
  opacity: 1 !important;
  transition: opacity 0.1s ease-in-out;
}

.navigation-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.app-branding {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  pointer-events: none;
}

.logo-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: linear-gradient(135deg, #4f46e5, #7c3aed);
  box-shadow: 0 2px 5px rgba(99, 102, 241, 0.3);
}

.logo-icon {
  color: white;
  width: 18px;
  height: 18px;
}

.title-container {
  display: flex;
  flex-direction: column;
}

.app-title {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0;
  background: linear-gradient(90deg, #6366F1, #D946EF, #EC4899);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  letter-spacing: -0.02em;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.header-nav-button {
  background: rgba(33, 37, 50, 0.95);
  border: none;
  padding: 6px;
  margin: 0 2px;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  color: #FFFFFF;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}
.header-nav-button .button-icon {
  width: 16px;
  height: 16px;
}
.header-nav-button:hover {
  background: rgba(50, 56, 74, 0.95);
}

.settings-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background-color: rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
  color: #4b5563;
}

.dark .settings-button {
  background-color: rgba(255, 255, 255, 0.1);
  color: #d1d5db;
}

.settings-button:hover {
  background-color: rgba(0, 0, 0, 0.1);
  transform: rotate(15deg);
}

.dark .settings-button:hover {
  background-color: rgba(255, 255, 255, 0.15);
}

.docker-status-button {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.25rem 0.6rem;
  border-radius: 50px;
  font-weight: 500;
  font-size: 0.75rem;
  background-color: #F3F4F6;
  color: #374151;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  min-height: 24px;
  margin-right: 4px;
}

.dark .docker-status-button {
  background-color: #1F2937;
  color: #D1D5DB;
}

.button-icon {
  width: 16px;
  height: 16px;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .app-branding {
    position: static;
    transform: none;
    margin-left: 10px;
  }
  .app-title {
    font-size: 1rem;
  }
}

@media (max-width: 640px) {
  .app-header {
    padding: 0.5rem 0.75rem;
    height: 52px;
  }
  .main-content {
    padding-top: 60px;
  }
  .app-title {
    font-size: 0.9rem;
  }
  .header-actions {
    gap: 0.25rem;
  }
  .header-nav-button {
    width: 26px;
    height: 26px;
  }
  .header-nav-button .button-icon {
    width: 14px;
    height: 14px;
  }
  .window-controls {
    gap: 6px;
    margin-right: 8px;
  }
  .titlebar-left-controls {
    gap: 8px;
  }
}

/* Dark mode styles */
@media (prefers-color-scheme: dark) {
  .macos-input,
  .macos-select {
    background-color: #374151 !important;
    border-color: #4b5563 !important;
    color: white !important;
  }
  
  .macos-button-secondary {
    background-color: #374151 !important;
    border-color: #4b5563 !important;
    color: #e5e7eb !important;
  }
  
  .macos-button-primary {
    background-color: #2563eb !important;
  }
  
  .bg-gray-50 {
    background-color: #1f2937 !important;
  }
  
  .bg-gray-100 {
    background-color: #1f2937 !important;
  }
  
  .border-gray-200 {
    border-color: #374151 !important;
  }
  
  .text-gray-700 {
    color: #e5e7eb !important;
  }
  
  .text-gray-600 {
    color: #d1d5db !important;
  }
  
  .text-gray-500 {
    color: #9ca3af !important;
  }
}

/* Hide scrollbar styles */
.hide-scrollbar {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;  /* Chrome, Safari and Opera */
}
</style>
