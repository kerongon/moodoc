<template>
  <div class="create-site">
    <h2>Create a New Site</h2>
    <form @submit.prevent="createSite">
      <div>
        <label for="siteName">Site Name:</label>
        <input type="text" v-model="siteName" required />
      </div>
      <div>
        <label for="phpVersion">PHP Version:</label>
        <select v-model="phpVersion">
          <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
        </select>
      </div>
      <div>
        <label for="dbType">Database Type:</label>
        <select v-model="dbType">
          <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
        </select>
      </div>
      <div>
        <label for="moodleBranch">Moodle Branch Version:</label>
        <select v-model="moodleBranch">
          <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
        </select>
      </div>
      <div>
        <label for="dbPort">DB Port:</label>
        <input type="number" v-model="dbPort" placeholder="Auto-generated if left blank" />
      </div>
      <div>
        <label for="webPort">Web Port:</label>
        <input type="number" v-model="webPort" placeholder="Auto-generated if left blank" />
      </div>
      <button type="submit">Create Site</button>
    </form>
    <div v-if="logs.length">
      <h3>Logs:</h3>
      <pre>{{ logs.join('\n') }}</pre>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      siteName: '',
      phpVersion: '8.0',
      dbType: 'MySQL',
      moodleBranch: 'MOODLE_40_STABLE',
      dbPort: '',
      webPort: '',
      phpVersions: ['7.4', '8.0', '8.1'],
      dbTypes: ['MySQL', 'PostgreSQL'],
      moodleBranches: ['MOODLE_39_STABLE', 'MOODLE_40_STABLE'],
      logs: [],
      dockerInstalled: false
    };
  },
  mounted() {
    this.checkDockerStatus();
  },
  methods: {
    async checkDockerStatus() {
      try {
        const dockerInstalled = await invoke('check_docker_installed');
        const dockerRunning = await invoke('check_docker_running');
        
        this.dockerInstalled = dockerInstalled && dockerRunning;
        
        if (!this.dockerInstalled) {
          this.logs.push('Docker is not installed or not running. Please install Docker and try again.');
        } else {
          this.logs.push('Docker is installed and running.');
        }
      } catch (error) {
        this.logs.push(`Error checking Docker status: ${error}`);
      }
    },
    async createSite() {
      if (!this.dockerInstalled) {
        this.logs.push('Cannot create site: Docker is not installed or not running.');
        return;
      }

      try {
        this.logs.push(`Creating site: ${this.siteName}`);
        
        const result = await invoke('create_site', {
          name: this.siteName,
          php_version: this.phpVersion,
          db_type: this.dbType,
          moodle_branch: this.moodleBranch,
          db_port: this.dbPort ? parseInt(this.dbPort) : null,
          web_port: this.webPort ? parseInt(this.webPort) : null
        });
        
        this.logs.push(result);
      } catch (error) {
        this.logs.push(`Error creating site: ${error}`);
      }
    }
  }
};
</script>

<style scoped>
.create-site {
  max-width: 400px;
  margin: auto;
}
.create-site h2 {
  text-align: center;
}
.create-site form {
  display: flex;
  flex-direction: column;
}
.create-site div {
  margin-bottom: 1em;
}
.create-site label {
  margin-bottom: 0.5em;
}
.create-site button {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 10px;
  cursor: pointer;
}
.create-site button:hover {
  background-color: #45a049;
}
</style>