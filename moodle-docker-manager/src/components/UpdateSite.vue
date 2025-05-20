<template>
  <div v-if="show" class="fixed inset-0 z-50 overflow-y-auto">
    <div class="flex items-center justify-center min-h-screen px-4">
      <!-- Modal backdrop -->
      <div class="fixed inset-0 bg-black/50 transition-opacity" @click="closeModal"></div>
      
      <!-- Modal panel -->
      <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full">
        <!-- Modal content -->
        <div class="p-6">
          <!-- Header -->
          <div class="flex items-center justify-between mb-5">
            <div class="flex items-center">
              <div class="bg-orange-600 rounded-lg p-1.5 mr-3 shadow-sm">
                <PhArrowClockwise class="text-white w-6 h-6" weight="fill" />
              </div>
              <div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Update Moodle: {{ siteName }}</h2>
                <p class="text-xs text-gray-500 dark:text-gray-400">Update this Moodle site to a newer version</p>
              </div>
            </div>
            <button @click="closeModal" class="text-gray-400 hover:text-gray-500">
              <PhX class="w-5 h-5" />
            </button>
          </div>

          <div v-if="loading" class="loading text-xs">
            <div class="flex items-center justify-center space-x-2">
              <div class="h-4 w-4 bg-orange-600 rounded-full animate-bounce" style="animation-delay: 0.0s"></div>
              <div class="h-4 w-4 bg-orange-600 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
              <div class="h-4 w-4 bg-orange-600 rounded-full animate-bounce" style="animation-delay: 0.4s"></div>
            </div>
            <p class="text-center mt-2">{{ loadingMessage }}</p>
          </div>
          <div v-else-if="error">
            <p class="error text-xs text-red-500">{{ error }}</p>
          </div>
          <div v-else>
            <!-- Current Site Info -->
            <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded-md mb-4">
              <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100 mb-2">Current Configuration</h3>
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span class="text-gray-500 dark:text-gray-400">Moodle Version: </span>
                  <span class="font-medium">{{ currentVersion }}</span>
                </div>
                <div>
                  <span class="text-gray-500 dark:text-gray-400">PHP Version: </span>
                  <span class="font-medium">{{ currentPhpVersion }}</span>
                </div>
                <div>
                  <span class="text-gray-500 dark:text-gray-400">Database: </span>
                  <span class="font-medium">{{ currentDbType }}</span>
                </div>
              </div>
            </div>

            <!-- Update Form -->
            <form @submit.prevent="updateSite" class="space-y-4">
              <div class="p-3 border border-gray-200 dark:border-gray-700 rounded-md">
                <div class="mb-3">
                  <label for="moodleBranch" class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                    New Moodle Version
                  </label>
                  <select 
                    id="moodleBranch" 
                    v-model="newMoodleBranch" 
                    @change="updatePHPVersion" 
                    class="macos-select compact-select w-full h-7 py-0.5"
                    required
                  >
                    <option value="">Select Moodle Version</option>
                    <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
                  </select>
                </div>
                
                <div class="mb-3">
                  <label for="phpVersion" class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">
                    PHP Version
                  </label>
                  <select 
                    id="phpVersion" 
                    v-model="newPhpVersion" 
                    class="macos-select compact-select w-full h-7 py-0.5"
                  >
                    <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
                  </select>
                  <p v-if="recommendedPhpVersion" class="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    Recommended PHP version: <span class="font-medium">{{ recommendedPhpVersion }}</span>
                  </p>
                </div>
                
                <div class="text-xs text-amber-500 bg-amber-50 dark:bg-amber-900/20 p-2 rounded-md">
                  <p class="font-medium">⚠️ Important Notes:</p>
                  <ul class="list-disc list-inside mt-1 space-y-0.5">
                    <li>A backup will be created automatically before updating</li>
                    <li>Your database will be migrated to the new version</li>
                    <li>All site content and settings will be preserved</li>
                    <li>This process may take several minutes</li>
                  </ul>
                </div>
              </div>
              
              <div class="flex justify-end space-x-2">
                <button 
                  type="button"
                  @click="closeModal" 
                  class="compact-action-button bg-gray-300 text-gray-700 hover:bg-gray-400">
                  Cancel
                </button>
                <button 
                  type="submit" 
                  :disabled="!newMoodleBranch || isProcessing" 
                  class="compact-action-button bg-orange-500 text-white hover:bg-orange-600 disabled:bg-gray-300 disabled:cursor-not-allowed">
                  Update Moodle
                </button>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { PhArrowClockwise, PhX } from '@phosphor-icons/vue';
import { useSitesStore } from '../stores/sites';

export default {
  name: 'UpdateSite',
  components: {
    PhArrowClockwise,
    PhX
  },
  props: {
    siteName: {
      type: String,
      required: true
    },
    show: {
      type: Boolean,
      default: false
    }
  },
  emits: ['close', 'updated'],
  data() {
    return {
      moodleBranches: [
        'MOODLE_402_STABLE',
        'MOODLE_401_STABLE',
        'MOODLE_400_STABLE',
        'MOODLE_311_STABLE',
        'MOODLE_310_STABLE',
        'MOODLE_39_STABLE',
        'master'
      ],
      phpVersions: ['8.2', '8.1', '8.0', '7.4', '7.3', '7.2', '7.1'],
      currentVersion: '',
      currentPhpVersion: '',
      currentDbType: '',
      newMoodleBranch: '',
      newPhpVersion: '',
      recommendedPhpVersion: '',
      loading: false,
      isProcessing: false,
      error: null,
      loadingMessage: 'Loading site information...',
      siteStore: useSitesStore()
    };
  },
  methods: {
    closeModal() {
      this.$emit('close');
    },
    async loadSiteInfo() {
      this.loading = true;
      this.error = null;
      
      try {
        // Get site information from store
        const sites = await this.siteStore.getSites();
        const site = sites.find(s => s.name === this.siteName);
        
        if (!site) {
          throw new Error(`Site '${this.siteName}' not found`);
        }
        
        this.currentVersion = site.moodle_branch;
        this.currentPhpVersion = site.php_version;
        this.currentDbType = site.db_type;
        
        // Initialize with current values
        this.newMoodleBranch = '';
        this.newPhpVersion = site.php_version;
        
      } catch (err) {
        this.error = `Failed to load site information: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
      }
    },
    
    updatePHPVersion() {
      // Set recommended PHP version based on selected Moodle branch
      if (this.newMoodleBranch.includes('402') || this.newMoodleBranch.includes('401') || this.newMoodleBranch.includes('400') || this.newMoodleBranch === 'master') {
        this.recommendedPhpVersion = '8.0';
        this.newPhpVersion = '8.0';
      } else if (this.newMoodleBranch.includes('311') || this.newMoodleBranch.includes('310')) {
        this.recommendedPhpVersion = '7.4';
        this.newPhpVersion = '7.4';
      } else if (this.newMoodleBranch.includes('39')) {
        this.recommendedPhpVersion = '7.2';
        this.newPhpVersion = '7.2';
      } else {
        this.recommendedPhpVersion = this.currentPhpVersion;
        this.newPhpVersion = this.currentPhpVersion;
      }
    },
    
    async updateSite() {
      if (!this.newMoodleBranch) {
        this.error = 'Please select a Moodle version';
        return;
      }
      
      this.isProcessing = true;
      this.loading = true;
      this.loadingMessage = 'Updating Moodle site...';
      this.error = null;
      
      try {
        const result = await invoke('update_moodle_version', { 
          name: this.siteName,
          newBranch: this.newMoodleBranch,
          newPhpVersion: this.newPhpVersion
        });
        
        console.log(result);
        this.$emit('updated');
        this.closeModal();
      } catch (err) {
        this.error = `Failed to update site: ${err}`;
        console.error(err);
        this.loading = false;
      } finally {
        this.isProcessing = false;
      }
    }
  },
  watch: {
    show(newVal) {
      if (newVal) {
        this.loadSiteInfo();
      }
    }
  },
  mounted() {
    // Only load site info if modal is shown initially
    if (this.show) {
      this.loadSiteInfo();
    }
  }
};
</script>

<style>
/* Use regular CSS instead of @apply */
.compact-action-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem 1rem;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: 0.375rem;
  transition: color 0.15s ease, background-color 0.15s ease;
}

.macos-select {
  appearance: none;
  border: 1px solid;
  border-color: rgb(229, 231, 235);
  border-radius: 0.375rem;
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  width: 100%;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1rem 1rem;
  padding-right: 2rem;
}

.dark .macos-select {
  border-color: rgb(75, 85, 99);
  background-color: rgb(55, 65, 81);
  color: rgb(243, 244, 246);
}

.macos-select:focus {
  outline: none;
  box-shadow: 0 0 0 1px rgb(59, 130, 246);
}
</style>
