<template>
  <div class="create-site">
    <h2>Create a New Site</h2>
    <form @submit.prevent="createSite">
      <div>
        <label for="siteName">Site Name:</label>
        <input type="text" v-model="siteName" required />
      </div>
      <div>
        <label for="moodleBranch">Moodle Branch Version:</label>
        <select v-model="moodleBranch" @change="updatePHPVersion">
          <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
        </select>
        <small v-if="recommendedPhpVersion" class="version-hint">
          Recommended PHP version: {{ recommendedPhpVersion }}
        </small>
      </div>
      <div>
        <label for="phpVersion">PHP Version:</label>
        <select v-model="phpVersion">
          <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
        </select>
        <small v-if="isPhpVersionCompatible === false" class="version-warning">
          Warning: This PHP version may not be compatible with the selected Moodle branch
        </small>
      </div>
      <div>
        <label for="dbType">Database Type:</label>
        <select v-model="dbType">
          <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
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
      <div>
        <label for="siteDirectory">Site Directory:</label>
        <div class="directory-selector">
          <input 
            type="text" 
            v-model="siteDirectory" 
            placeholder="Use default directory" 
            readonly
          />
          <button type="button" @click="selectCustomDirectory">Browse...</button>
        </div>
        <div class="directory-info">
          <small v-if="useDefaultDirectory">
            Using default directory: {{ defaultDirectory || 'No default set (will use app data directory)' }}
          </small>
          <small v-else>
            Custom directory selected: <span class="directory-path">{{ siteDirectory }}</span>
          </small>
          <button 
            v-if="!useDefaultDirectory" 
            type="button"
            class="reset-button"
            @click="resetToDefaultDirectory"
          >
            Reset to default
          </button>
        </div>
        <div class="site-path-preview" v-if="siteName">
          <small>
            Site will be created at: <span class="path-highlight">{{ getFullSitePath() }}</span>
          </small>
        </div>
      </div>
      <div>
        <label for="useRedis">Use Redis:</label>
        <div class="toggle-switch">
          <input
            type="checkbox"
            id="useRedis"
            v-model="useRedis"
          />
          <label for="useRedis" class="toggle-label"></label>
        </div>
        <small>Enable Redis for caching and sessions</small>
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
import { open } from '@tauri-apps/api/dialog';
import { useSettingsStore } from '../stores/settings';
import { computed } from 'vue';

export default {
  emits: ['site-created'],      data() {
    return {
      siteName: '',
      phpVersion: '8.2',
      dbType: 'MySQL', // Default kept as MySQL
      moodleBranch: 'MOODLE_500_STABLE',
      dbPort: '',
      webPort: '',
      siteDirectory: '',
      useDefaultDirectory: true,
      useRedis: true, // Default to true for Redis support
      phpVersions: ['8.4', '8.3', '8.2', '8.1', '8.0', '7.4', '7.3', '7.2', '7.1', '7.0', '5.6'],
      dbTypes: ['MySQL', 'MariaDB', 'PostgreSQL', 'Microsoft SQL Server', 'Oracle XE'],
      moodleBranches: ['MOODLE_500_STABLE', 'MOODLE_405_STABLE', 'MOODLE_404_STABLE', 'MOODLE_403_STABLE', 'MOODLE_402_STABLE', 'MOODLE_401_STABLE'],
      logs: [],
      dockerInstalled: false,
      settingsStore: useSettingsStore(),
      recommendedPhpVersion: '8.2',
      isPhpVersionCompatible: true,
      // Mapping of Moodle branches to PHP versions
      moodlePhpVersionMap: {
        'MOODLE_500_STABLE': { recommended: '8.2', compatible: ['8.2', '8.3', '8.4'] },
        'MOODLE_405_STABLE': { recommended: '8.3', compatible: ['8.1', '8.2', '8.3'] },
        'MOODLE_404_STABLE': { recommended: '8.3', compatible: ['8.1', '8.2', '8.3'] },
        'MOODLE_403_STABLE': { recommended: '8.2', compatible: ['8.0', '8.1', '8.2'] },
        'MOODLE_402_STABLE': { recommended: '8.2', compatible: ['8.0', '8.1', '8.2'] },
        'MOODLE_401_STABLE': { recommended: '8.1', compatible: ['7.4', '8.0', '8.1'] }
      }
    };
  },
  computed: {
    defaultDirectory() {
      return this.settingsStore.sitesRootDirectory;
    }
  },
  mounted() {
    this.checkDockerStatus();
    this.updatePHPVersion(); // Set initial PHP version based on default Moodle branch
  },
  methods: {
    updatePHPVersion() {
      const branchMap = this.moodlePhpVersionMap[this.moodleBranch];
      
      if (branchMap) {
        // Set the recommended PHP version
        this.recommendedPhpVersion = branchMap.recommended;
        
        // Check if current PHP version is compatible
        this.isPhpVersionCompatible = branchMap.compatible.includes(this.phpVersion);
        
        // Auto-select the recommended PHP version
        this.phpVersion = branchMap.recommended;
      } else {
        // If no mapping exists, set a default and mark as potentially incompatible
        this.recommendedPhpVersion = '8.0';
        this.isPhpVersionCompatible = false;
      }
    },
    
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
    async selectCustomDirectory() {
      try {
        const selected = await open({
          directory: true,
          multiple: false,
          title: 'Select Site Directory'
        });
        
        if (selected) {
          this.siteDirectory = selected;
          this.useDefaultDirectory = false;
        }
      } catch (error) {
        this.logs.push(`Error selecting directory: ${error}`);
      }
    },
    
    resetToDefaultDirectory() {
      this.siteDirectory = '';
      this.useDefaultDirectory = true;
    },
    
    getFullSitePath() {
      if (!this.siteName) return '';
      
      if (this.useDefaultDirectory) {
        if (this.defaultDirectory) {
          return `${this.defaultDirectory}/${this.siteName}`;
        } else {
          return `[App Data Directory]/sites/${this.siteName}`;
        }
      } else {
        return `${this.siteDirectory}/${this.siteName}`;
      }
    },
    
    async createSite() {
      if (!this.dockerInstalled) {
        this.logs.push('Cannot create site: Docker is not installed or not running.');
        return;
      }

      try {
        this.logs.push(`Creating site: ${this.siteName}`);
        
        // Determine which directory to use
        const directory = this.useDefaultDirectory
          ? this.settingsStore.sitesRootDirectory
          : this.siteDirectory;
        
        if (!directory && this.useDefaultDirectory) {
          this.logs.push('Warning: No default directory set. Using app data directory.');
        } else if (directory) {
          this.logs.push(`Using directory: ${directory}`);
        }
        
        // Map display names to the Docker environment names
        const dbTypeMap = {
          'MySQL': 'mysql',
          'MariaDB': 'mariadb',
          'PostgreSQL': 'pgsql',
          'Microsoft SQL Server': 'mssql',
          'Oracle XE': 'oracle'
        };
        
        // Log Redis configuration
        if (this.useRedis) {
          this.logs.push('Redis support enabled for this site.');
        } else {
          this.logs.push('Redis support disabled for this site.');
        }
        
        const result = await invoke('create_site', {
          name: this.siteName,
          php_version: this.phpVersion,
          db_type: dbTypeMap[this.dbType] || this.dbType.toLowerCase(),
          moodle_branch: this.moodleBranch,
          db_port: this.dbPort ? parseInt(this.dbPort) : null,
          web_port: this.webPort ? parseInt(this.webPort) : null,
          directory: directory || null,
          use_redis: this.useRedis
        });
        
        this.logs.push(result);
        
        // After successful site creation, emit an event to refresh the site list
        this.$emit('site-created');
        
        // Clear form fields
        this.siteName = '';
        this.dbPort = '';
        this.webPort = '';
        this.resetToDefaultDirectory();
        // Keep the Redis setting as is for next site
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

.directory-selector {
  display: flex;
  gap: 10px;
  margin-bottom: 5px;
}

.directory-selector input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #f5f5f5;
}

.directory-selector button {
  padding: 8px 15px;
  background-color: #f5f5f5;
  border: 1px solid #ccc;
  border-radius: 4px;
  color: #333;
  font-size: 14px;
}

.directory-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: #718096;
  margin-bottom: 15px;
}

.reset-button {
  background: none !important;
  border: none !important;
  color: #4299e1 !important;
  cursor: pointer;
  font-size: 12px;
  padding: 0 !important;
  text-decoration: underline;
}

.site-path-preview {
  margin-top: 5px;
  font-style: italic;
}

.path-highlight {
  font-family: monospace;
  color: #4a5568;
  word-break: break-all;
}

.directory-path {
  font-family: monospace;
  word-break: break-all;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 34px;
  height: 20px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-label {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .4s;
  border-radius: 34px;
}

.toggle-label:before {
  position: absolute;
  content: "";
  height: 12px;
  width: 12px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .toggle-label {
  background-color: #4CAF50;
}

input:checked + .toggle-label:before {
  transform: translateX(14px);
}

.version-hint {
  display: block;
  color: #4a5568;
  font-size: 12px;
  margin-top: 4px;
}

.version-warning {
  display: block;
  color: #e53e3e;
  font-size: 12px;
  margin-top: 4px;
  font-weight: 500;
}
</style>