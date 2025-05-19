<template>
  <div class="site-logs bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto my-4 p-4">
    <!-- Header - macOS style (more compact) -->
    <div class="flex items-center mb-5">
      <div class="bg-blue-600 rounded-lg p-1.5 mr-3 shadow-sm">
        <PhTerminal class="text-white w-6 h-6" weight="fill" />
      </div>
      <div>
        <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Site Logs: {{ siteName }}</h2>
        <p class="text-xs text-gray-500 dark:text-gray-400">View and monitor the container logs for this site</p>
      </div>
    </div>

    <div v-if="loading" class="loading text-xs">
      <p>Loading logs...</p>
    </div>
    <div v-else-if="error">
      <p class="error text-xs">{{ error }}</p>
    </div>
    <div v-else-if="!logs">
      <p class="text-xs">No logs available for this site.</p>
    </div>
    <div v-else>
      <pre class="logs-container text-xs">{{ logs }}</pre>
    </div>
    <div class="actions mt-3 flex gap-2">
      <button @click="fetchLogs" class="compact-action-button bg-blue-500 text-white hover:bg-blue-600">Refresh Logs</button>
      <button @click="$router.push('/dashboard')" class="compact-action-button bg-gray-500 text-white hover:bg-gray-600">Back to Dashboard</button>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { PhTerminal } from '@phosphor-icons/vue';

export default {
  name: 'SiteLogs',
  components: {
    PhTerminal
  },
  props: {
    siteName: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      logs: null,
      loading: false,
      error: null
    };
  },
  methods: {
    async fetchLogs() {
      this.loading = true;
      this.error = null;
      
      try {
        // Construct container name based on site name
        const containerName = `${this.siteName}_moodle`;
        
        // Call the Rust function to get logs
        this.logs = await invoke('get_docker_logs', { containerName });
      } catch (err) {
        this.error = `Failed to fetch logs: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
      }
    }
  },
  mounted() {
    this.fetchLogs();
  }
};
</script>

<style scoped>
.site-logs {
  max-width: 800px;
  margin: 0 auto;
}

.logs-container {
  background-color: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  padding: 10px;
  max-height: 450px;
  overflow-y: auto;
  white-space: pre-wrap;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
  margin-bottom: 15px;
  font-size: 0.7rem;
  line-height: 1.3;
}

.dark .logs-container {
  background-color: #1f2937;
  border-color: #374151;
}

.loading {
  background-color: #e3f2fd;
  padding: 8px;
  border-radius: 4px;
}

.dark .loading {
  background-color: rgba(59, 130, 246, 0.1);
}

.error {
  color: #dc3545;
  font-weight: 500;
  background-color: #fee2e2;
  padding: 8px;
  border-radius: 4px;
}

.dark .error {
  background-color: rgba(185, 28, 28, 0.2);
  color: #fca5a5;
}

.actions {
  display: flex;
  gap: 8px;
}

button {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

button:hover {
  background-color: #45a049;
}

button:last-child {
  background-color: #6c757d;
}

button:last-child:hover {
  background-color: #5a6268;
}
.site-logs li {
  margin: 5px 0;
}
</style>