<template>
  <div v-if="show" class="fixed inset-0 z-50 overflow-y-auto hide-scrollbar">
    <div class="flex items-center justify-center min-h-screen px-4">
      <!-- Modal backdrop -->
      <div class="fixed inset-0 bg-black/50 transition-opacity" @click="closeModal"></div>
      
      <!-- Modal panel -->
      <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[850px] overflow-hidden">
        <!-- Modal content -->
        <div class="flex flex-col h-full">
          <!-- Header -->
          <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
            <div class="flex items-center">
              <div class="bg-blue-600 rounded-lg p-1 mr-2 shadow-sm">
                <PhWrench class="text-white w-5 h-5" weight="fill" />
              </div>
              <div>
                <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">System Diagnostics</h2>
                <p class="text-xs text-gray-500 dark:text-gray-400">Diagnose production environment issues</p>
              </div>
            </div>
            <button @click="closeModal" class="text-gray-400 hover:text-gray-500">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          
          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-4">
            <!-- Loading state -->
            <div v-if="loading" class="flex items-center justify-center py-8">
              <div class="flex items-center space-x-3">
                <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
                <span class="text-gray-600 dark:text-gray-400">Running diagnostics...</span>
              </div>
            </div>
            
            <!-- Error state -->
            <div v-else-if="error" class="text-center py-8">
              <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
                <PhWarningCircle class="mx-auto h-10 w-10 text-red-500 mb-2" />
                <h3 class="text-base font-medium text-red-900 dark:text-red-400 mb-2">Diagnostic Failed</h3>
                <p class="text-sm text-red-700 dark:text-red-300">{{ error }}</p>
              </div>
            </div>
            
            <!-- Diagnostic results -->
            <div v-else-if="diagnosticInfo" class="space-y-4">
              <!-- Docker Status Section -->
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
                <h3 class="text-base font-semibold text-gray-900 dark:text-gray-100 mb-2 flex items-center">
                  <PhDocker class="w-4 h-4 mr-2 text-blue-600" />
                  Docker Status
                </h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                  <div class="space-y-1.5">
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Docker Version:</span>
                      <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.docker_version || 'Not available' }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Running:</span>
                      <span class="text-xs font-medium" :class="diagnosticInfo.docker_running ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
                        {{ diagnosticInfo.docker_running ? 'Yes' : 'No' }}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Socket Accessible:</span>
                      <span class="text-xs font-medium" :class="diagnosticInfo.socket_accessible ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
                        {{ diagnosticInfo.socket_accessible ? 'Yes' : 'No' }}
                      </span>
                    </div>
                  </div>
                  <div class="space-y-1.5">
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Containers:</span>
                      <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.containers?.length || 0 }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Networks:</span>
                      <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.networks?.length || 0 }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-xs text-gray-600 dark:text-gray-400">Compose Version:</span>
                      <span class="text-xs font-medium text-gray-900 dark:text-gray-100" :title="diagnosticInfo.compose_version">
                        {{ (diagnosticInfo.compose_version && diagnosticInfo.compose_version.length > 50) ? 
                           diagnosticInfo.compose_version.substring(0, 50) + '...' : 
                           (diagnosticInfo.compose_version || 'Not available') }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- System Environment Section (Accordion) -->
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg overflow-hidden">
                <button 
                  @click="toggleAccordion('system')"
                  class="w-full p-3 text-left flex items-center justify-between hover:bg-gray-100 dark:hover:bg-gray-600/50 transition-colors"
                >
                  <div class="flex items-center">
                    <PhGear class="w-5 h-5 mr-2 text-gray-600" />
                    <span class="text-base font-semibold text-gray-900 dark:text-gray-100">System Environment</span>
                  </div>
                  <svg 
                    class="w-4 h-4 text-gray-500 transition-transform duration-200"
                    :class="{ 'rotate-180': activeAccordion === 'system' }"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>
                <div 
                  v-show="activeAccordion === 'system'"
                  class="px-3 pb-3 border-t border-gray-200 dark:border-gray-600"
                >
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-3 mt-3">
                    <div class="space-y-1.5">
                      <div class="flex justify-between">
                        <span class="text-xs text-gray-600 dark:text-gray-400">Platform:</span>
                        <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.platform || 'Unknown' }}</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-xs text-gray-600 dark:text-gray-400">Current User:</span>
                        <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.current_user || 'Unknown' }}</span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-xs text-gray-600 dark:text-gray-400">User Groups:</span>
                        <span class="text-xs font-medium text-gray-900 dark:text-gray-100">{{ diagnosticInfo.user_groups?.join(', ') || 'None' }}</span>
                      </div>
                    </div>
                    <div class="space-y-1.5">
                      <div class="flex justify-between">
                        <span class="text-xs text-gray-600 dark:text-gray-400">Docker Group:</span>
                        <span class="text-xs font-medium" :class="diagnosticInfo.docker_groups?.length > 0 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
                          {{ diagnosticInfo.docker_groups?.length > 0 ? 'Member' : 'Not member' }}
                        </span>
                      </div>
                      <div class="flex justify-between">
                        <span class="text-xs text-gray-600 dark:text-gray-400">Working Directory:</span>
                        <span class="text-xs font-medium text-gray-900 dark:text-gray-100" :title="diagnosticInfo.current_dir">
                          {{ formatPath(diagnosticInfo.current_dir) }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Directory Status Section (Accordion) -->
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg overflow-hidden">
                <button 
                  @click="toggleAccordion('directories')"
                  class="w-full p-3 text-left flex items-center justify-between hover:bg-gray-100 dark:hover:bg-gray-600/50 transition-colors"
                >
                  <div class="flex items-center">
                    <PhFolder class="w-5 h-5 mr-2 text-yellow-600" />
                    <span class="text-base font-semibold text-gray-900 dark:text-gray-100">Directory Status</span>
                  </div>
                  <svg 
                    class="w-4 h-4 text-gray-500 transition-transform duration-200"
                    :class="{ 'rotate-180': activeAccordion === 'directories' }"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>
                <div 
                  v-show="activeAccordion === 'directories'"
                  class="px-3 pb-3 border-t border-gray-200 dark:border-gray-600"
                >
                  <div class="space-y-2 mt-3">
                    <div v-for="(dir, index) in diagnosticInfo.directory_checks" :key="index" class="flex items-center justify-between p-2 bg-white dark:bg-gray-800 rounded-md border">
                      <div class="flex items-center">
                        <PhFolder class="w-3 h-3 mr-2 text-gray-500" />
                        <span class="text-xs font-medium text-gray-900 dark:text-gray-100" :title="dir.path">{{ formatPath(dir.path) }}</span>
                      </div>
                      <div class="flex items-center space-x-2">
                        <span class="text-xs px-2 py-1 rounded" :class="dir.exists ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' : 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300'">
                          {{ dir.exists ? 'Exists' : 'Missing' }}
                        </span>
                        <span v-if="dir.exists" class="text-xs px-2 py-1 rounded" :class="dir.writable ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300' : 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300'">
                          {{ dir.writable ? 'Writable' : 'Read-only' }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Environment Variables Section (Accordion) -->
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg overflow-hidden">
                <button 
                  @click="toggleAccordion('environment')"
                  class="w-full p-3 text-left flex items-center justify-between hover:bg-gray-100 dark:hover:bg-gray-600/50 transition-colors"
                >
                  <div class="flex items-center">
                    <PhCode class="w-5 h-5 mr-2 text-purple-600" />
                    <span class="text-base font-semibold text-gray-900 dark:text-gray-100">Environment Variables</span>
                  </div>
                  <svg 
                    class="w-4 h-4 text-gray-500 transition-transform duration-200"
                    :class="{ 'rotate-180': activeAccordion === 'environment' }"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>
                <div 
                  v-show="activeAccordion === 'environment'"
                  class="px-3 pb-3 border-t border-gray-200 dark:border-gray-600"
                >
                  <div class="space-y-1 max-h-32 overflow-y-auto mt-3">
                    <div v-for="[key, value] in Object.entries(diagnosticInfo.environment_vars || {})" :key="key" class="flex justify-between py-1 border-b border-gray-200 dark:border-gray-600 last:border-b-0">
                      <span class="text-xs font-medium text-gray-600 dark:text-gray-400">{{ key }}:</span>
                      <span class="text-xs text-gray-900 dark:text-gray-100 max-w-xs truncate" :title="value">{{ value }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Port Availability Section (Accordion) -->
              <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg overflow-hidden">
                <button 
                  @click="toggleAccordion('ports')"
                  class="w-full p-3 text-left flex items-center justify-between hover:bg-gray-100 dark:hover:bg-gray-600/50 transition-colors"
                >
                  <div class="flex items-center">
                    <PhPlugs class="w-5 h-5 mr-2 text-green-600" />
                    <span class="text-base font-semibold text-gray-900 dark:text-gray-100">Port Availability</span>
                  </div>
                  <svg 
                    class="w-4 h-4 text-gray-500 transition-transform duration-200"
                    :class="{ 'rotate-180': activeAccordion === 'ports' }"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>
                <div 
                  v-show="activeAccordion === 'ports'"
                  class="px-3 pb-3 border-t border-gray-200 dark:border-gray-600"
                >
                  <div class="grid grid-cols-3 md:grid-cols-6 gap-2 mt-3">
                    <div v-for="port in diagnosticInfo.port_checks" :key="port.port" class="text-center p-2 rounded-md border" :class="port.available ? 'bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-800' : 'bg-red-50 border-red-200 dark:bg-red-900/20 dark:border-red-800'">
                      <div class="text-sm font-medium" :class="port.available ? 'text-green-800 dark:text-green-300' : 'text-red-800 dark:text-red-300'">
                        {{ port.port }}
                      </div>
                      <div class="text-xs" :class="port.available ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
                        {{ port.available ? 'Free' : 'In use' }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Errors Section -->
              <div v-if="diagnosticInfo.errors && diagnosticInfo.errors.length > 0" class="bg-red-50 dark:bg-red-900/20 rounded-lg p-3">
                <h3 class="text-base font-semibold text-red-900 dark:text-red-400 mb-2 flex items-center">
                  <PhWarningCircle class="w-4 h-4 mr-2" />
                  Errors Found
                </h3>
                <div class="space-y-2">
                  <div v-for="(error, index) in diagnosticInfo.errors" :key="index" class="p-2 bg-red-100 dark:bg-red-900/30 rounded-md">
                    <p class="text-xs text-red-800 dark:text-red-300">{{ error }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Footer -->
          <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-3 flex justify-between">
            <button @click="runDiagnostics" :disabled="loading" class="flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed">
              <PhArrowClockwise class="w-4 h-4 mr-2" :class="{ 'animate-spin': loading }" />
              {{ loading ? 'Running...' : 'Refresh Diagnostics' }}
            </button>
            <button @click="closeModal" class="px-4 py-2 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-400 dark:hover:bg-gray-500">
              Close
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { 
  PhWrench, 
  PhWarningCircle, 
  PhArrowClockwise, 
  PhGear, 
  PhFolder, 
  PhCode, 
  PhPlugs 
} from '@phosphor-icons/vue';

// Create a simple Docker icon component
const PhDocker = {
  template: `
    <svg viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
      <path d="M13.983 11.078h2.119a.186.186 0 00.186-.185V9.006a.186.186 0 00-.186-.186h-2.119a.185.185 0 00-.185.185v1.888c0 .102.083.185.185.185m-2.954-5.43h2.118a.186.186 0 00.186-.186V3.574a.186.186 0 00-.186-.185h-2.118a.185.185 0 00-.185.185v1.888c0 .102.082.185.185.185m0 2.716h2.118a.187.187 0 00.186-.186V6.29a.186.186 0 00-.186-.185h-2.118a.185.185 0 00-.185.185v1.888c0 .102.082.185.185.185m-2.93 0h2.12a.186.186 0 00.185-.186V6.29a.185.185 0 00-.185-.185H8.1a.185.185 0 00-.185.185v1.888c0 .102.083.185.185.185m-2.964 0h2.119a.186.186 0 00.185-.186V6.29a.185.185 0 00-.185-.185H5.136a.186.186 0 00-.186.185v1.888c0 .102.084.185.186.185m5.893 2.715h2.118a.186.186 0 00.186-.185V9.006a.186.186 0 00-.186-.186h-2.118a.185.185 0 00-.185.186v1.887c0 .102.082.185.185.185m-2.93 0h2.12a.185.185 0 00.185-.185V9.006a.185.185 0 00-.185-.186H8.1a.185.185 0 00-.185.186v1.887c0 .102.083.185.185.185m-2.964 0h2.119a.185.185 0 00.185-.185V9.006a.185.185 0 00-.185-.186H5.136a.186.186 0 00-.186.186v1.887c0 .102.084.185.186.185M23.763 9.89c-.065-.051-.672-.51-1.954-.51-.338 0-.676.03-1.01.09-.219-1.322-.943-2.484-2.145-3.446l-.297-.248-.248.297c-.457.536-.681 1.241-.681 1.961 0 .467.094.909.265 1.332a3.7 3.7 0 01-1.704.42H.608a.607.607 0 00-.61.61c-.061 1.538.069 3.119.388 4.669.343 1.672.856 3.248 1.521 4.684 1.161 2.529 2.681 4.293 4.523 5.25 1.91.993 4.134 1.498 6.618 1.498 1.208 0 2.411-.105 3.577-.312 1.409-.25 2.729-.655 3.905-1.211 2.573-1.218 4.594-2.992 5.994-5.267 1.5-2.44 2.335-5.312 2.476-8.537v-.361l-.248-.248zm-14.05 2.628h2.118a.186.186 0 00.186-.186v-1.887a.186.186 0 00-.186-.186H9.712a.185.185 0 00-.185.186v1.887c0 .102.083.186.185.186"/>
    </svg>
  `
};

export default {
  name: 'DiagnosticModal',
  components: {
    PhWrench,
    PhWarningCircle,
    PhArrowClockwise,
    PhGear,
    PhFolder,
    PhCode,
    PhPlugs,
    PhDocker
  },
  props: {
    show: {
      type: Boolean,
      default: false
    }
  },
  emits: ['close'],
  setup(props, { emit }) {
    const loading = ref(false);
    const error = ref(null);
    const diagnosticInfo = ref(null);
    const activeAccordion = ref(null); // Track which accordion is open

    const closeModal = () => {
      emit('close');
    };

    const toggleAccordion = (section) => {
      // If clicking the same section, close it; otherwise open the new section
      activeAccordion.value = activeAccordion.value === section ? null : section;
    };

    const formatPath = (path) => {
      if (!path) return 'N/A';
      const maxLength = 40;
      if (path.length <= maxLength) return path;
      
      const parts = path.split('/');
      const filename = parts.pop();
      return `.../${parts.slice(-2).join('/')}/${filename}`;
    };

    const getDirectoryDescription = (path) => {
      if (path.includes('/Users/') || path === diagnosticInfo.value?.current_user) {
        return 'User Home Directory';
      } else {
        return 'Directory';
      }
    };

    const runDiagnostics = async () => {
      loading.value = true;
      error.value = null;
      
      try {
        console.log('Running diagnostics...');
        const result = await invoke('run_diagnostics');
        console.log('Diagnostic result:', result);
        diagnosticInfo.value = result;
      } catch (err) {
        console.error('Failed to run diagnostics:', err);
        error.value = String(err);
      } finally {
        loading.value = false;
      }
    };

    // Auto-run diagnostics when modal opens
    watch(() => props.show, (newShow) => {
      if (newShow && !diagnosticInfo.value) {
        runDiagnostics();
      }
    });

    return {
      loading,
      error,
      diagnosticInfo,
      activeAccordion,
      closeModal,
      runDiagnostics,
      toggleAccordion,
      formatPath,
      getDirectoryDescription
    };
  }
};
</script>

<style scoped>
.hide-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;
}
</style>
