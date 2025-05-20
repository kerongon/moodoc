<template>
  <div> <!-- root wrapper -->
    <!-- Main header from Dashboard -->
    <div class="dashboard-full bg-white dark:bg-gray-800 min-h-screen w-full p-0">
      <div class="flex items-center justify-between px-8 pt-8 pb-2">
        <div class="flex items-center gap-3">
          <PhList class="w-10 h-10 text-blue-500 drop-shadow" weight="fill" />
          <h1 class="text-4xl font-extrabold bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 bg-clip-text text-transparent tracking-tight flex items-center">
            Moodle Docker Manager
          </h1>
        </div>
        <div class="flex items-center gap-4">
          <div class="docker-status-top">
            <DockerStatus />
          </div>
          <router-link to="/create" class="create-site-btn flex items-center gap-2 text-white font-semibold rounded-lg px-4 py-2 text-base shadow transition-all duration-150" title="Create New Site">
            <PhPlus class="w-5 h-5" weight="fill" />
            <span class="sm:inline">New Site</span>
          </router-link>
        </div>
      </div>
      
      <div class="site-logs bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto my-4 p-4">
        <!-- Breadcrumbs -->
        <nav class="text-sm text-gray-600 dark:text-gray-400 mb-4">
          <router-link to="/" class="hover:underline">Home</router-link>
          <span class="mx-2">›</span>
          <span>Site Logs</span>
        </nav>
        
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
        
        <!-- Service selector -->
        <div class="mb-4 flex items-center space-x-2">
          <label for="service-select" class="text-sm text-gray-700 dark:text-gray-300">Service:</label>
          <select id="service-select" v-model="selectedService" @change="fetchLogs"
            class="text-xs p-1 border rounded bg-white dark:bg-gray-700 dark:border-gray-600">
            <option v-for="svc in services" :key="svc" :value="svc">{{ svc }}</option>
          </select>
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
          <button @click="$router.push({ name: 'Dashboard' })" class="compact-action-button bg-gray-500 text-white hover:bg-gray-600">Back to Dashboard</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { PhTerminal, PhList, PhPlus, PhGear } from '@phosphor-icons/vue';
import DockerStatus from './DockerStatus.vue';

export default {
  name: 'SiteLogs',
  components: { 
    PhTerminal,
    PhList,
    PhPlus,
    PhGear,
    DockerStatus,
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
      error: null,
      services: ['db', 'webserver', 'mailpit', 'selenium', 'exttests'],
      selectedService: 'webserver'
    };
  },
  methods: {
    async fetchLogs() {
      this.loading = true;
      this.error = null;
      try {
        // Use selected service from dropdown
        const service = this.selectedService;
        // Call the Rust function to get logs
        this.logs = await invoke('get_docker_logs', {
          name: this.siteName,
          service: service
        });
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
  width: 100%;
  max-height: 100vh;
  margin: 0 auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

 .logs-container {
  background-color: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  padding: 10px;
  flex: 1 1 auto;
  min-height: 0;
  max-height: 50vh;
  overflow: auto;
  white-space: pre;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
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