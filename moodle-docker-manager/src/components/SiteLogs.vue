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
      
      <!-- Wrap the site-logs in a container with the same padding as the header -->
      <div class="px-8">
        <div class="site-logs bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full my-4 p-4">
          <!-- Breadcrumbs -->
          <nav class="text-sm text-gray-600 dark:text-gray-400 mb-4">
            <router-link to="/" class="hover:underline">Home</router-link>
            <span class="mx-2">›</span>
            <span>Site Logs</span>
          </nav>
          
          <!-- Header - macOS style (more compact) -->
          <div class="flex items-center justify-between mb-5">
            <div class="flex items-center">
              <div class="bg-blue-600 rounded-lg p-1.5 mr-3 shadow-sm">
                <PhTerminal class="text-white w-6 h-6" weight="fill" />
              </div>
              <div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Site Logs: {{ siteName }}</h2>
                <p class="text-xs text-gray-500 dark:text-gray-400">View and monitor the container logs for this site</p>
              </div>
            </div>
            
            <!-- Open in New Window button -->
            <button @click="openLogsInNewWindow" class="open-in-window-btn flex items-center gap-1 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm">
              <PhSquaresFour class="w-4 h-4" weight="fill" />
              <span>Open in New Window</span>
            </button>
          </div>
          
          <!-- Services tabs -->
          <div class="mb-4">
            <div class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Available Services:</div>
            <div class="flex flex-wrap gap-2">
              <button 
                v-for="service in services" 
                :key="service"
                @click="openLogsInNewWindow(service)"
                class="service-btn flex items-center gap-1 px-3 py-1.5 rounded-md text-sm hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <span class="service-icon w-3 h-3 rounded-full" :class="getServiceColor(service)"></span>
                {{ service }}
              </button>
            </div>
          </div>
          
          <div class="service-description p-4 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 mb-4">
            <p class="text-sm text-gray-600 dark:text-gray-300">
              <span class="font-semibold">Logs are now displayed in a separate window</span> for improved performance. 
              Click on any service above to view its logs or click the "Open in New Window" button to see all logs.
            </p>
          </div>
          
          <div class="actions mt-3 flex gap-2">
            <button @click="$router.push({ name: 'Dashboard' })" class="compact-action-button bg-gray-500 text-white hover:bg-gray-600">Back to Dashboard</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { PhTerminal, PhList, PhPlus, PhGear, PhSquaresFour } from '@phosphor-icons/vue';
import DockerStatus from './DockerStatus.vue';
import { openLogsWindow } from '../utils/window-manager';

export default {
  name: 'SiteLogs',
  components: { 
    PhTerminal,
    PhList,
    PhPlus,
    PhGear,
    PhSquaresFour,
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
      services: ['db', 'webserver', 'mailpit', 'selenium', 'exttests'],
      selectedService: 'webserver'
    };
  },
  methods: {
    openLogsInNewWindow(service = this.selectedService) {
      // Use the utility function to open logs in a new window
      openLogsWindow(this.siteName, service);
    },
    getServiceColor(service) {
      // Return a different color class for each service
      switch(service) {
        case 'db': return 'bg-green-500';
        case 'webserver': return 'bg-blue-500';
        case 'mailpit': return 'bg-purple-500';
        case 'selenium': return 'bg-orange-500';
        case 'exttests': return 'bg-red-500';
        default: return 'bg-gray-500';
      }
    }
  },
  mounted() {
    // Automatically open logs in a new window when component is mounted
    this.openLogsInNewWindow('webserver');
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

.service-btn {
  transition: all 0.2s ease;
  border: 1px solid #e5e7eb;
}

.dark .service-btn {
  border-color: #4b5563;
}

.service-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.dark .service-btn:hover {
  box-shadow: 0 2px 4px rgba(0,0,0,0.2);
}

.service-icon {
  display: inline-block;
}

.open-in-window-btn {
  transition: all 0.15s ease;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  border-radius: 0.375rem;
}

.open-in-window-btn:hover {
  background-color: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.2);
}

.dark .open-in-window-btn:hover {
  background-color: rgba(59, 130, 246, 0.2);
  border-color: rgba(59, 130, 246, 0.3);
}

.service-description {
  font-style: italic;
}

.actions {
  display: flex;
  gap: 8px;
}

button.compact-action-button {
  background-color: #6c757d;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

button.compact-action-button:hover {
  background-color: #5a6268;
}

.site-logs li {
  margin: 5px 0;
}
</style>