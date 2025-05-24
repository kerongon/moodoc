<template>
  <div class="logs-viewer bg-white dark:bg-gray-800 min-h-screen w-full p-4">
    <!-- Minimal header with just the essential controls -->
    <div class="title-bar flex items-center justify-between mb-4" data-tauri-drag-region>
      <div class="flex items-center gap-2">
        <div class="bg-blue-600 rounded-lg p-1.5 shadow-sm">
          <PhTerminal class="text-white w-5 h-5" weight="fill" />
        </div>
        <div>
          <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Site Logs: {{ siteName }}</h2>
          <p class="text-xs text-gray-500 dark:text-gray-400">Service: {{ selectedService }}</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <!-- Service selector -->
        <select v-model="selectedService" @change="fetchLogs"
          class="text-xs p-1 border rounded bg-white dark:bg-gray-700 dark:border-gray-600">
          <option v-for="svc in services" :key="svc" :value="svc">{{ svc }}</option>
        </select>
        
        <!-- Copy to clipboard button -->
        <button @click="copyToClipboard" class="icon-action-btn bg-blue-600 hover:bg-blue-700" title="Copy to clipboard">
          <PhCopy class="w-4 h-4" weight="fill" />
        </button>
        
        <!-- Refresh button -->
        <button @click="fetchLogs" class="icon-action-btn bg-green-600 hover:bg-green-700" title="Refresh logs">
          <PhArrowClockwise class="w-4 h-4" weight="fill" />
        </button>
      </div>
    </div>
    
    <!-- Logs content -->
    <div class="logs-container-wrapper bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden flex-grow h-[calc(100vh-100px)]">
      <div v-if="loading" class="loading-spinner flex items-center justify-center h-full">
        <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-500"></div>
      </div>
      <div v-else-if="error" class="error-message p-4 text-red-600 dark:text-red-400">
        {{ error }}
      </div>
      <div v-else-if="!logs" class="empty-logs p-4 text-gray-400 dark:text-gray-500 text-center">
        No logs available for this service.
      </div>
      <pre v-else class="logs-content text-xs h-full overflow-auto p-4 font-mono whitespace-pre-wrap">{{ logs }}</pre>
    </div>
    
    <!-- Auto-refresh toggle -->
    <div class="mt-2 flex items-center justify-between">
      <div class="flex items-center">
        <input type="checkbox" id="autoRefresh" v-model="autoRefresh" class="h-4 w-4 text-blue-600 rounded border-gray-300">
        <label for="autoRefresh" class="ml-2 text-xs text-gray-700 dark:text-gray-300">Auto-refresh (10s)</label>
      </div>
      
      <!-- Service tabs for quick switching -->
      <div class="service-tabs">
        <button 
          v-for="service in services" 
          :key="service"
          @click="switchService(service)"
          class="service-tab px-2 py-1 text-xs rounded-md mr-1"
          :class="selectedService === service ? 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-300' : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'"
        >
          {{ service }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { PhTerminal, PhCopy, PhArrowClockwise } from '@phosphor-icons/vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWindow = getCurrentWebviewWindow()

// Reactive references
const logs = ref(null);
const loading = ref(false);
const error = ref(null);
const services = ref(['db', 'webserver', 'mailpit', 'selenium', 'exttests']);
const selectedService = ref('webserver');
const autoRefresh = ref(false);
const siteName = ref('');

// For interval management
let refreshInterval = null;

// Parse URL parameters to get siteName and initial service
const initParams = () => {
  console.log('Initializing parameters');
  
  // Get the current URL hash part (everything after the #)
  const hash = window.location.hash; // e.g., "#/log-window?siteName=test&service=webserver"
  console.log('Current hash:', hash);
  
  // Extract the query string part
  const queryStringMatch = hash.match(/\?(.+)$/);
  if (queryStringMatch && queryStringMatch[1]) {
    const urlParams = new URLSearchParams(queryStringMatch[1]);
    
    const siteNameParam = urlParams.get('siteName');
    console.log('Site name from URL:', siteNameParam);
    if (siteNameParam) {
      siteName.value = decodeURIComponent(siteNameParam);
      console.log('Set site name to:', siteName.value);
    }
    
    const serviceParam = urlParams.get('service');
    console.log('Service from URL:', serviceParam);
    if (serviceParam && services.value.includes(serviceParam)) {
      selectedService.value = decodeURIComponent(serviceParam);
      console.log('Set service to:', selectedService.value);
    }
  } else {
    console.warn('No query params found in URL');
    
    // Fallback to window label (the Tauri window label often contains the site name)
    try {
      const windowLabel = appWindow.label;
      console.log('Window label:', windowLabel);
      if (windowLabel && windowLabel.startsWith('logs-')) {
        // Extract site name from the window label
        const extractedName = windowLabel.substring(5).replace(/-/g, ' ');
        if (extractedName) {
          siteName.value = extractedName;
          console.log('Set site name from window label:', siteName.value);
        }
      }
    } catch (err) {
      console.error('Error getting window label:', err);
    }
  }
};

// Methods
const fetchLogs = async () => {
  loading.value = true;
  error.value = null;
  
  if (!siteName.value) {
    console.error('Cannot fetch logs: Site name is empty');
    error.value = 'Site name is missing. Please check the URL parameters.';
    loading.value = false;
    return;
  }
  
  console.log(`Fetching logs for site: "${siteName.value}", service: "${selectedService.value}"`);
  
  try {
    logs.value = await invoke('get_docker_logs', {
      name: siteName.value,
      service: selectedService.value
    });
    console.log(`Successfully fetched ${logs.value ? logs.value.length : 0} bytes of logs`);
  } catch (err) {
    error.value = `Failed to fetch logs: ${err}`;
    console.error('Error fetching logs:', err);
  } finally {
    loading.value = false;
  }
};

const copyToClipboard = async () => {
  if (!logs.value) return;
  try {
    await writeText(logs.value);
    const old = logs.value;
    logs.value = "Logs copied to clipboard!";
    setTimeout(() => { logs.value = old; }, 1000);
  } catch (err) {
    console.error("Failed to copy logs:", err);
  }
};

const switchService = (service) => {
  selectedService.value = service;
  // No need to call fetchLogs as the watcher will handle it
};

// Watch for auto-refresh changes
watch(autoRefresh, (val) => {
  if (val) {
    refreshInterval = setInterval(fetchLogs, 10000);
  } else if (refreshInterval) {
    clearInterval(refreshInterval);
  }
});

// Watch for service changes
watch(selectedService, (newService) => {
  if (siteName.value) {
    fetchLogs();
  } else {
    console.warn('Not fetching logs after service change because site name is missing');
  }
  appWindow.setTitle(`Logs: ${siteName.value || 'Unknown'} - ${newService}`);
});

// Lifecycle hooks
onMounted(async () => {
  console.log('LogWindow component mounted');
  
  // Initialize parameters from URL
  initParams();
  
  // Check if we have a site name
  if (!siteName.value) {
    // Try to get it from the window URL once more after a short delay
    console.log('No site name found, trying to get it after a short delay');
    setTimeout(async () => {
      initParams();
      
      if (siteName.value) {
        console.log('Successfully got site name after delay:', siteName.value);
        await fetchLogs();
        appWindow.setTitle(`Logs: ${siteName.value} - ${selectedService.value}`);
      } else {
        console.error('Still no site name after delay');
        error.value = 'Could not determine which site to show logs for. Please try opening logs again.';
      }
    }, 500);
  } else {
    // Fetch logs immediately if we have a site name
    await fetchLogs();
  }
  
  // Set window title
  appWindow.setTitle(`Logs: ${siteName.value || 'Unknown'} - ${selectedService.value}`);
  
  // Apply theme from localStorage
  const savedTheme = localStorage.getItem('theme') || 'system';
  if (window.ThemeManager) {
    window.ThemeManager.applyTheme(savedTheme);
  }
});

onBeforeUnmount(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval);
  }
});
</script>

<style scoped>
.logs-viewer {
  display: flex;
  flex-direction: column;
}

.title-bar {
  user-select: none;
}

.logs-content {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  line-height: 1.3;
}

.icon-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  color: white;
  transition: background-color 0.15s ease;
  cursor: pointer;
}

/* Style the scrollbars in the logs container */
.logs-content::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.logs-content::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.logs-content::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 4px;
}

.logs-content::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}

.dark .logs-content::-webkit-scrollbar-track {
  background: #1e293b;
}

.dark .logs-content::-webkit-scrollbar-thumb {
  background: #475569;
}

.dark .logs-content::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}

.service-tab {
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.service-tab:hover {
  border-color: #e2e8f0;
}

.dark .service-tab:hover {
  border-color: #4b5563;
}
</style>