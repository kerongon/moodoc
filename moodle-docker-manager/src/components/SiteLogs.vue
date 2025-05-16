<template>
  <div class="site-logs">
    <h2>Site Logs: {{ siteName }}</h2>
    <div v-if="loading">
      <p>Loading logs...</p>
    </div>
    <div v-else-if="error">
      <p class="error">{{ error }}</p>
    </div>
    <div v-else-if="!logs">
      <p>No logs available for this site.</p>
    </div>
    <div v-else>
      <pre class="logs-container">{{ logs }}</pre>
    </div>
    <div class="actions">
      <button @click="fetchLogs">Refresh Logs</button>
      <button @click="$router.push('/dashboard')">Back to Dashboard</button>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';

export default {
  name: 'SiteLogs',
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
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.site-logs h2 {
  margin-bottom: 20px;
  color: #2c3e50;
}

.logs-container {
  background-color: #f5f5f5;
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 15px;
  max-height: 500px;
  overflow-y: auto;
  white-space: pre-wrap;
  font-family: monospace;
  margin-bottom: 20px;
}

.error {
  color: #dc3545;
  font-weight: bold;
}

.actions {
  display: flex;
  gap: 10px;
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