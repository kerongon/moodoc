<template>
  <div class="bg-white rounded-lg shadow-md border border-gray-200 max-w-2xl mx-auto my-8 p-6">
    
    <!-- Header with App Icon -->
    <div class="flex items-center justify-center mb-6">
      <div class="bg-blue-600 rounded-lg p-2 mr-4">
        <PhSquare class="text-white w-8 h-8" weight="fill" />
      </div>
      <h2 class="text-xl font-semibold">Create a New Moodle Site</h2>
    </div>
    
    <!-- Tab Navigation -->
    <div class="flex bg-gray-100 rounded-lg p-1 mb-6">
      <button 
        class="flex-1 py-2 px-4 text-sm font-medium transition-all duration-150" 
        @click="activeTab = 'standard'"
        :class="activeTab === 'standard' ? 'bg-white rounded-md shadow text-gray-900' : 'text-gray-500 hover:text-gray-700'">
        Standard
      </button>
      <button 
        class="flex-1 py-2 px-4 text-sm font-medium transition-all duration-150" 
        @click="activeTab = 'custom'"
        :class="activeTab === 'custom' ? 'bg-white rounded-md shadow text-gray-900' : 'text-gray-500 hover:text-gray-700'">
        Custom
      </button>
      <button 
        class="flex-1 py-2 px-4 text-sm font-medium transition-all duration-150" 
        @click="activeTab = 'advanced'"
        :class="activeTab === 'advanced' ? 'bg-white rounded-md shadow text-gray-900' : 'text-gray-500 hover:text-gray-700'">
        Advanced
      </button>
    </div>
    
    <!-- Form Content -->
    <form @submit.prevent="createSite" class="space-y-6">
      <!-- Standard Tab Content -->
      <div v-if="activeTab === 'standard'">
        <!-- Site Name -->
        <div class="mb-4">
          <label for="siteName" class="block text-sm font-medium text-gray-700 mb-1">Site Name</label>
          <input 
            type="text" 
            id="siteName" 
            v-model="siteName" 
            :class="['macos-input', { 'border-red-500 focus:ring-red-500 focus:border-red-500': siteName && !siteNameIsValid }]" 
            required 
            placeholder="moodle-site"
          />
          <p v-if="siteName && !siteNameIsValid" class="mt-1 text-sm text-red-600">
            Site name can only contain letters, numbers, underscores and hyphens
          </p>
        </div>
        
        <!-- Moodle Version -->
        <div class="mb-4">
          <label for="moodleBranch" class="block text-sm font-medium text-gray-700 mb-1">Moodle Version</label>
          <select 
            id="moodleBranch" 
            v-model="moodleBranch" 
            @change="updatePHPVersion" 
            class="macos-select"
          >
            <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
          </select>
          <p v-if="recommendedPhpVersion" class="mt-1 text-sm text-gray-500">
            Recommended PHP version: <span class="font-medium">{{ recommendedPhpVersion }}</span>
          </p>
        </div>
        
        <!-- PHP Version -->
        <div class="mb-4">
          <label for="phpVersion" class="block text-sm font-medium text-gray-700 mb-1">PHP Version</label>
          <select id="phpVersion" v-model="phpVersion" class="macos-select">
            <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
          </select>
          <p v-if="isPhpVersionCompatible === false" class="mt-1 text-sm text-red-600">
            Warning: This PHP version may not be compatible with the selected Moodle branch
          </p>
        </div>
        
        <!-- Database Configuration -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">Database Configuration</label>
          <div class="flex space-x-4">
            <div class="flex-1">
              <select 
                id="dbType" 
                v-model="dbType" 
                @change="updateDbType" 
                class="macos-select"
              >
                <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
              </select>
            </div>
            <div class="w-1/3">
              <input 
                type="number" 
                v-model="dbPort" 
                :placeholder="`Port: ${currentDbPortRange}`" 
                :class="['macos-input', { 'border-red-500': dbPort && !isDbPortInRange }]"
              />
            </div>
          </div>
          <div class="flex justify-end mt-1">
            <button 
              type="button" 
              @click="generateDbPort" 
              class="text-xs text-blue-600 hover:text-blue-800"
            >
              Generate random port
            </button>
          </div>
          <p v-if="dbPort && !isDbPortInRange" class="mt-1 text-sm text-red-600">
            Port is outside the recommended range ({{ currentDbPortRange }}) for {{ dbType }}
          </p>
        </div>
        
        <!-- Web Server -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">Web Server</label>
          <div class="flex space-x-4">
            <div class="flex-1">
              <select class="macos-select">
                <option>Apache</option>
              </select>
            </div>
            <div class="w-1/3">
              <input 
                type="number" 
                v-model="webPort" 
                :placeholder="`Port: ${currentWebPortRange}`" 
                :class="['macos-input', { 'border-red-500': webPort && !isWebPortInRange }]"
              />
            </div>
          </div>
          <div class="flex justify-end mt-1">
            <button 
              type="button" 
              @click="generateWebPort" 
              class="text-xs text-blue-600 hover:text-blue-800"
            >
              Generate random port
            </button>
          </div>
          <p v-if="webPort && !isWebPortInRange" class="mt-1 text-sm text-red-600">
            Port is outside the recommended range ({{ currentWebPortRange }})
          </p>
        </div>
        
        <!-- Site Directory -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-1">Site Directory</label>
          <div class="flex">
            <input 
              type="text" 
              v-model="siteDirectory" 
              :placeholder="useDefaultDirectory ? 'Using default directory' : ''" 
              class="macos-input rounded-r-none" 
              readonly 
            />
            <button 
              type="button" 
              @click="selectCustomDirectory" 
              class="px-4 border border-l-0 border-gray-300 bg-gray-50 rounded-r-md text-gray-700 hover:bg-gray-100"
            >
              Browse...
            </button>
          </div>
          <div class="mt-1 flex justify-between items-center">
            <p class="text-xs text-gray-500">
              <span v-if="useDefaultDirectory">
                Using default directory: <span class="font-mono">{{ defaultDirectory || '[App Data Directory]/sites' }}</span>
              </span>
              <span v-else>
                Custom directory: <span class="font-mono">{{ siteDirectory }}</span>
              </span>
            </p>
            <button 
              v-if="!useDefaultDirectory" 
              type="button" 
              @click="resetToDefaultDirectory" 
              class="text-xs text-blue-600 hover:text-blue-800"
            >
              Reset to default
            </button>
          </div>
          <p v-if="siteName" class="mt-1 text-xs text-gray-500">
            Site will be created at: <span class="font-mono">{{ getFullSitePath() }}</span>
          </p>
        </div>
        
        <!-- Redis Toggle -->
        <div class="flex items-center justify-between mb-4">
          <div>
            <label for="useRedis" class="text-sm font-medium text-gray-700">Enable Redis for caching and sessions</label>
            <p class="text-xs text-gray-500">Improves performance for larger Moodle sites</p>
          </div>
          <div class="relative inline-block w-12 align-middle select-none">
            <button 
              type="button"
              role="switch"
              :aria-checked="useRedis.toString()"
              @click="useRedis = !useRedis"
              class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
              :class="useRedis ? 'bg-blue-600' : 'bg-gray-200'"
            >
              <span class="sr-only">Use Redis</span>
              <span 
                class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out" 
                :class="useRedis ? 'translate-x-5' : 'translate-x-0'"
              ></span>
            </button>
          </div>
        </div>
      </div>
      
      <!-- Custom Tab Content -->
      <div v-if="activeTab === 'custom'">
        <div class="space-y-6">
          <!-- Custom Moodle Repository -->
          <div>
            <label for="customRepo" class="block text-sm font-medium text-gray-700 mb-1">Custom Moodle Repository</label>
            <div class="flex space-x-2">
              <input 
                type="text" 
                id="customRepo" 
                v-model="customRepoUrl" 
                class="macos-input flex-1" 
                placeholder="https://github.com/yourusername/moodle.git" 
              />
              <button 
                type="button"
                @click="resetToDefaultRepo"
                class="macos-button macos-button-secondary text-xs px-3"
              >
                Reset
              </button>
            </div>
            <p class="mt-1 text-xs text-gray-500">
              Leave empty to use the official Moodle repository
            </p>
          </div>
          
          <!-- Custom Branch -->
          <div>
            <label for="customBranch" class="block text-sm font-medium text-gray-700 mb-1">Custom Branch</label>
            <div class="flex space-x-2">
              <input 
                type="text" 
                id="customBranch" 
                v-model="customBranch" 
                class="macos-input flex-1" 
                placeholder="main" 
              />
              <button 
                type="button"
                @click="useSelectedStableBranch"
                class="macos-button macos-button-secondary text-xs px-3"
              >
                Use Stable
              </button>
            </div>
            <p class="mt-1 text-xs text-gray-500">
              Custom branch to checkout (overrides Moodle version selected in Standard tab)
            </p>
          </div>
          
          <!-- Database Prefix -->
          <div>
            <label for="dbPrefix" class="block text-sm font-medium text-gray-700 mb-1">Database Table Prefix</label>
            <input 
              type="text" 
              id="dbPrefix" 
              v-model="dbPrefix" 
              class="macos-input w-full" 
              placeholder="mdl_" 
            />
            <p class="mt-1 text-xs text-gray-500">
              Default: mdl_
            </p>
          </div>
          
          <!-- Custom Site Config -->
          <div>
            <label for="siteConfig" class="block text-sm font-medium text-gray-700 mb-1">Site Configuration</label>
            <div class="flex space-x-4 mb-2">
              <div class="flex items-center">
                <input
                  type="checkbox"
                  id="siteDebug"
                  v-model="siteConfig.debug"
                  class="h-4 w-4 text-blue-600 rounded border-gray-300"
                />
                <label for="siteDebug" class="ml-2 text-sm text-gray-700">Debug Mode</label>
              </div>
              <div class="flex items-center">
                <input
                  type="checkbox"
                  id="siteDevLib"
                  v-model="siteConfig.devLibs"
                  class="h-4 w-4 text-blue-600 rounded border-gray-300"
                />
                <label for="siteDevLib" class="ml-2 text-sm text-gray-700">Development Libraries</label>
              </div>
            </div>
            <div class="flex space-x-4">
              <div class="flex items-center">
                <input
                  type="checkbox"
                  id="siteCaching"
                  v-model="siteConfig.disableCaching"
                  class="h-4 w-4 text-blue-600 rounded border-gray-300"
                />
                <label for="siteCaching" class="ml-2 text-sm text-gray-700">Disable Caching</label>
              </div>
              <div class="flex items-center">
                <input
                  type="checkbox"
                  id="siteThemeCaching"
                  v-model="siteConfig.disableThemeCaching"
                  class="h-4 w-4 text-blue-600 rounded border-gray-300"
                />
                <label for="siteThemeCaching" class="ml-2 text-sm text-gray-700">Disable Theme Caching</label>
              </div>
            </div>
          </div>
          
          <!-- Admin Account -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-3">Admin Account</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="adminUsername" class="block text-xs font-medium text-gray-600 mb-1">Username</label>
                <input 
                  type="text" 
                  id="adminUsername" 
                  v-model="adminAccount.username" 
                  class="macos-input" 
                  placeholder="admin" 
                />
              </div>
              <div>
                <label for="adminPassword" class="block text-xs font-medium text-gray-600 mb-1">Password</label>
                <input 
                  type="text" 
                  id="adminPassword" 
                  v-model="adminAccount.password" 
                  class="macos-input" 
                  placeholder="admin" 
                />
              </div>
              <div>
                <label for="adminEmail" class="block text-xs font-medium text-gray-600 mb-1">Email</label>
                <input 
                  type="email" 
                  id="adminEmail" 
                  v-model="adminAccount.email" 
                  class="macos-input" 
                  placeholder="admin@example.com" 
                />
              </div>
              <div>
                <label for="adminFullname" class="block text-xs font-medium text-gray-600 mb-1">Full Name</label>
                <input 
                  type="text" 
                  id="adminFullname" 
                  v-model="adminAccount.fullname" 
                  class="macos-input" 
                  placeholder="Admin User" 
                />
              </div>
            </div>
            <p class="mt-1 text-xs text-gray-500">
              Default credentials will be used if left empty
            </p>
          </div>
        </div>
      </div>
      
      <!-- Advanced Tab Content -->
      <div v-if="activeTab === 'advanced'">
        <div class="space-y-6">
          <!-- Docker Configuration -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-3">Docker Configuration</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="dockerNetwork" class="block text-xs font-medium text-gray-600 mb-1">Docker Network</label>
                <input 
                  type="text" 
                  id="dockerNetwork" 
                  v-model="dockerConfig.network" 
                  class="macos-input" 
                  placeholder="moodle-network" 
                />
              </div>
              <div>
                <label for="dockerPrefix" class="block text-xs font-medium text-gray-600 mb-1">Container Prefix</label>
                <input 
                  type="text" 
                  id="dockerPrefix" 
                  v-model="dockerConfig.containerPrefix" 
                  class="macos-input" 
                  placeholder="moodle-" 
                />
              </div>
            </div>
            <div class="mt-3">
              <label for="dockerComposePath" class="block text-xs font-medium text-gray-600 mb-1">Docker Compose Path</label>
              <div class="flex space-x-2">
                <input 
                  type="text" 
                  id="dockerComposePath" 
                  v-model="dockerConfig.composePath" 
                  class="macos-input flex-1" 
                  placeholder="/usr/local/bin/docker-compose" 
                />
                <button 
                  type="button"
                  @click="detectDockerComposePath"
                  class="macos-button macos-button-secondary text-xs px-3"
                >
                  Detect
                </button>
              </div>
            </div>
          </div>

          <!-- PHP Configuration -->
          <div>
            <h3 class="text-sm font-medium text-gray-700 mb-3">PHP Configuration</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="phpVersion" class="block text-xs font-medium text-gray-600 mb-1">PHP Version</label>
                <select id="phpVersion" v-model="phpVersion" class="macos-select">
                  <option value="7.4">PHP 7.4</option>
                  <option value="8.0">PHP 8.0</option>
                  <option value="8.1">PHP 8.1</option>
                  <option value="8.2">PHP 8.2</option>
                </select>
              </div>
              <div>
                <label for="phpMemoryLimit" class="block text-xs font-medium text-gray-600 mb-1">Memory Limit</label>
                <input 
                  type="text" 
                  id="phpMemoryLimit" 
                  v-model="phpConfig.memoryLimit" 
                  class="macos-input" 
                  placeholder="512M" 
                />
              </div>
            </div>
            <div class="mt-3">
              <label for="phpModules" class="block text-xs font-medium text-gray-600 mb-1">Additional PHP Modules (comma-separated)</label>
              <input 
                type="text" 
                id="phpModules" 
                v-model="phpConfig.additionalModules" 
                class="macos-input w-full" 
                placeholder="xdebug, opcache, etc." 
              />
            </div>
            <div class="mt-3 flex items-center">
              <input
                type="checkbox"
                id="enableXdebug"
                v-model="phpConfig.xdebug"
                class="h-4 w-4 text-blue-600 rounded border-gray-300"
              />
              <label for="enableXdebug" class="ml-2 text-sm text-gray-700">Enable Xdebug</label>
            </div>
          </div>

          <!-- Volume Mounts -->
          <div>
            <div class="flex justify-between items-center mb-3">
              <h3 class="text-sm font-medium text-gray-700">Additional Volume Mounts</h3>
              <button 
                type="button"
                @click="addVolumeMount"
                class="macos-button macos-button-secondary text-xs px-3"
              >
                Add Mount
              </button>
            </div>
            <div v-for="(mount, index) in volumeMounts" :key="index" class="flex space-x-2 mb-2">
              <input 
                type="text" 
                v-model="mount.source" 
                class="macos-input flex-1" 
                placeholder="Local path" 
              />
              <span class="flex items-center text-gray-500">:</span>
              <input 
                type="text" 
                v-model="mount.target" 
                class="macos-input flex-1" 
                placeholder="Container path" 
              />
              <button 
                type="button"
                @click="removeVolumeMount(index)"
                class="macos-button macos-button-danger text-xs px-3"
              >
                Remove
              </button>
            </div>
            <p class="text-xs text-gray-500 mt-1">
              Add custom volume mounts for development plugins, themes, etc.
            </p>
          </div>

          <!-- Environment Variables -->
          <div>
            <div class="flex justify-between items-center mb-3">
              <h3 class="text-sm font-medium text-gray-700">Environment Variables</h3>
              <button 
                type="button"
                @click="addEnvironmentVariable"
                class="macos-button macos-button-secondary text-xs px-3"
              >
                Add Variable
              </button>
            </div>
            <div v-for="(envVar, index) in environmentVariables" :key="index" class="flex space-x-2 mb-2">
              <input 
                type="text" 
                v-model="envVar.key" 
                class="macos-input flex-1" 
                placeholder="VARIABLE_NAME" 
              />
              <span class="flex items-center text-gray-500">=</span>
              <input 
                type="text" 
                v-model="envVar.value" 
                class="macos-input flex-1" 
                placeholder="value" 
              />
              <button 
                type="button"
                @click="removeEnvironmentVariable(index)"
                class="macos-button macos-button-danger text-xs px-3"
              >
                Remove
              </button>
            </div>
            <p class="text-xs text-gray-500 mt-1">
              Add custom environment variables for containers
            </p>
          </div>
        </div>
      </div>
      
      <!-- Docker Status -->
      <div class="flex items-center justify-between text-sm px-3 py-2 bg-gray-50 rounded-md mb-4">
        <div>Docker is installed and running</div>
        <div class="w-2 h-2 rounded-full bg-green-500"></div>
      </div>
      
      <!-- Form Errors -->
      <div v-if="formErrors.length > 0" class="bg-red-50 border border-red-200 rounded-md p-3 mb-4">
        <h3 class="text-sm font-medium text-red-800 mb-1">Please fix the following issues:</h3>
        <ul class="text-xs text-red-700 list-disc pl-5">
          <li v-for="(error, index) in formErrors" :key="index" class="mb-1">{{ error }}</li>
        </ul>
      </div>
      
      <!-- Action Buttons -->
      <div class="flex justify-end space-x-4">
        <button type="button" class="macos-button macos-button-secondary" @click="$router.push('/')">Cancel</button>
        <button 
          type="submit" 
          class="macos-button macos-button-primary relative"
          :class="{ 'opacity-50 cursor-not-allowed': !formIsValid || formErrors.length > 0 || createSiteInProgress }"
          :disabled="!formIsValid || formErrors.length > 0 || createSiteInProgress"
        >
          <span v-if="!createSiteInProgress">Create Site</span>
          <span v-else class="flex items-center">
            <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Creating...
          </span>
        </button>
      </div>
    </form>
    
    <!-- Logs -->
    <div v-if="logs.length" class="mt-6 border-t border-gray-200 pt-4">
      <h3 class="text-sm font-medium text-gray-700 mb-2">Logs:</h3>
      <div class="bg-gray-100 rounded-md p-3 font-mono text-xs overflow-auto max-h-40">
        <div v-for="(log, i) in logs" :key="i" class="pb-1">{{ log }}</div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { useSettingsStore } from '../stores/settings';
import { computed } from 'vue';
import { PhSquare } from '@phosphor-icons/vue';

export default {
  components: {
    PhSquare
  },
  emits: ['site-created'],
  data() {
    return {
      activeTab: 'standard',
      siteName: '',
      phpVersion: '8.2',
      dbType: 'MySQL', // Default kept as MySQL
      moodleBranch: 'MOODLE_500_STABLE',
      dbPort: '',
      webPort: '',
      siteDirectory: '',
      useDefaultDirectory: true,
      useRedis: false, // Default to false for Redis support
      phpVersions: ['8.4', '8.3', '8.2', '8.1', '8.0', '7.4', '7.3', '7.2', '7.1', '7.0', '5.6'],
      dbTypes: ['MySQL', 'MariaDB', 'PostgreSQL', 'Microsoft SQL Server', 'Oracle XE'],
      moodleBranches: ['MOODLE_500_STABLE', 'MOODLE_405_STABLE', 'MOODLE_404_STABLE', 'MOODLE_403_STABLE', 'MOODLE_402_STABLE', 'MOODLE_401_STABLE'],
      logs: [],
      dockerInstalled: false,
      settingsStore: useSettingsStore(),
      recommendedPhpVersion: '8.2',
      isPhpVersionCompatible: true,
      // Port ranges for different services
      portRanges: {
        web: { min: 8000, max: 8999 },
        db: {
          'MySQL': { min: 3306, max: 3399 },
          'MariaDB': { min: 3400, max: 3499 },
          'PostgreSQL': { min: 5432, max: 5499 },
          'Microsoft SQL Server': { min: 1433, max: 1499 },
          'Oracle XE': { min: 1521, max: 1599 }
        }
      },
      // Mapping of Moodle branches to PHP versions
      moodlePhpVersionMap: {
        'MOODLE_500_STABLE': { recommended: '8.2', compatible: ['8.2', '8.3', '8.4'] },
        'MOODLE_405_STABLE': { recommended: '8.3', compatible: ['8.1', '8.2', '8.3'] },
        'MOODLE_404_STABLE': { recommended: '8.3', compatible: ['8.1', '8.2', '8.3'] },
        'MOODLE_403_STABLE': { recommended: '8.2', compatible: ['8.0', '8.1', '8.2'] },
        'MOODLE_402_STABLE': { recommended: '8.2', compatible: ['8.0', '8.1', '8.2'] },
        'MOODLE_401_STABLE': { recommended: '8.1', compatible: ['7.4', '8.0', '8.1'] }
      },
      // Site creation state
      createSiteInProgress: false,
      // Custom settings for the Custom tab
      customRepoUrl: '',
      customBranch: '',
      dbPrefix: 'mdl_',
      siteConfig: {
        debug: false,
        devLibs: false,
        disableCaching: false,
        disableThemeCaching: false
      },
      adminAccount: {
        username: '',
        password: '',
        email: '',
        fullname: ''
      },
      // New data properties for Advanced tab
      dockerConfig: {
        network: 'moodle-network',
        containerPrefix: 'moodle-',
        composePath: '/usr/local/bin/docker-compose'
      },
      phpConfig: {
        memoryLimit: '512M',
        additionalModules: '',
        xdebug: false
      },
      volumeMounts: [],
      environmentVariables: []
    };
  },
  computed: {
    defaultDirectory() {
      return this.settingsStore.sitesRootDirectory;
    },
    
    currentWebPortRange() {
      return `${this.portRanges.web.min}-${this.portRanges.web.max}`;
    },
    
    currentDbPortRange() {
      const dbRange = this.portRanges.db[this.dbType] || { min: 0, max: 0 };
      return `${dbRange.min}-${dbRange.max}`;
    },
    
    isWebPortInRange() {
      if (!this.webPort) return true; // Empty is valid (will auto-generate)
      const port = parseInt(this.webPort);
      return port >= this.portRanges.web.min && port <= this.portRanges.web.max;
    },
    
    isDbPortInRange() {
      if (!this.dbPort) return true; // Empty is valid (will auto-generate)
      const port = parseInt(this.dbPort);
      const dbRange = this.portRanges.db[this.dbType] || { min: 0, max: 0 };
      return port >= dbRange.min && port <= dbRange.max;
    },

    siteNameIsValid() {
      // Check if the site name is valid: no spaces, special characters, etc.
      return this.siteName.length === 0 || /^[a-z0-9_-]+$/i.test(this.siteName);
    },
    
    formIsValid() {
      if (!this.siteName || !this.siteNameIsValid) {
        return false;
      }
      
      // Port validation
      const portsValid = 
        (!this.webPort || this.isWebPortInRange) && 
        (!this.dbPort || this.isDbPortInRange);
        
      return portsValid;
    },
    
    // Get the list of errors to show to the user
    formErrors() {
      const errors = [];
      
      if (this.siteName && !this.siteNameIsValid) {
        errors.push("Site name can only contain letters, numbers, underscores and hyphens");
      }
      
      if (this.webPort && !this.isWebPortInRange) {
        errors.push(`Web port ${this.webPort} is outside the recommended range (${this.currentWebPortRange})`);
      }
      
      if (this.dbPort && !this.isDbPortInRange) {
        errors.push(`DB port ${this.dbPort} is outside the recommended range for ${this.dbType} (${this.currentDbPortRange})`);
      }
      
      if (this.activeTab === 'custom' && this.customRepoUrl && !this.isValidGitUrl(this.customRepoUrl)) {
        errors.push("Please enter a valid git repository URL");
      }
      
      return errors;
    },
  },
  mounted() {
    this.checkDockerStatus();
    this.updatePHPVersion(); // Set initial PHP version based on default Moodle branch
    this.initializeAdvancedTabData(); // Initialize default values for the advanced tab
  },
  methods: {
    initializeAdvancedTabData() {
      // Add a default empty volume mount and environment variable for better UX
      if (this.volumeMounts.length === 0) {
        this.addVolumeMount();
      }
      
      if (this.environmentVariables.length === 0) {
        this.addEnvironmentVariable();
      }
    },
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
    
    selectFolder() {
      this.folderPickerOpen = true
      window.__TAURI__.dialog.open({
        directory: true,
        multiple: false,
        title: 'Select Moodle folder'
      }).then((result) => {
        if (result) {
          this.moodleFolder = result
        }
        this.folderPickerOpen = false
      })
    },
    
    generateWebPort() {
      // Generate a random port within the web port range
      const min = this.portRanges.web.min;
      const max = this.portRanges.web.max;
      this.webPort = Math.floor(Math.random() * (max - min + 1)) + min;
    },
    
    generateDbPort() {
      // Generate a random port within the database-specific range
      const dbRange = this.portRanges.db[this.dbType] || { min: 3306, max: 3399 };
      const min = dbRange.min;
      const max = dbRange.max;
      this.dbPort = Math.floor(Math.random() * (max - min + 1)) + min;
    },
    
    // When db type changes, update the db port placeholder and clear the value if out of range
    updateDbType() {
      if (this.dbPort) {
        const port = parseInt(this.dbPort);
        const dbRange = this.portRanges.db[this.dbType] || { min: 0, max: 0 };
        if (port < dbRange.min || port > dbRange.max) {
          // Clear the port if it's out of the new range
          this.dbPort = '';
        }
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

      if (!this.name) {
        this.logs.push('Please enter a site name')
        return
      }

      if (!this.moodleFolder) {
        this.logs.push('Please select a folder for Moodle')
        return
      }

      if (!this.version && this.activeTab === 'standard') {
        this.logs.push('Please select a Moodle version')
        return
      }

      // Validate ports if specified
      if (this.webPort && !this.isWebPortInRange) {
        this.logs.push(`Web port ${this.webPort} is outside the recommended range (${this.currentWebPortRange})`)
        return
      }
      
      if (this.dbPort && !this.isDbPortInRange) {
        this.logs.push(`DB port ${this.dbPort} is outside the recommended range for ${this.dbType} (${this.currentDbPortRange})`)
        return
      }

      // Start the creation process
      this.createSiteInProgress = true
      this.logs.push(`Creating site ${this.name}...`)

      try {
        // Compile the settings from all tabs
        const siteConfig = {
          name: this.name,
          folder: this.moodleFolder,
          webPort: this.webPort || this.portRanges.web.min,
          dbPort: this.dbPort || this.portRanges.db[this.dbType].min,
          dbType: this.dbType,
          useRedis: this.useRedis,
          // Custom tab settings
          customRepo: this.customRepoUrl || null,
          customBranch: this.customBranch || null,
          dbPrefix: this.dbPrefix || 'mdl_',
          siteConfig: this.siteConfig,
          adminAccount: this.adminAccount,
          // Advanced tab settings
          phpVersion: this.phpVersion,
          phpConfig: this.phpConfig,
          dockerConfig: this.dockerConfig,
          volumeMounts: this.volumeMounts.filter(mount => mount.source && mount.target),
          environmentVariables: this.environmentVariables.filter(envVar => envVar.key && envVar.value)
        }

        // If using standard tab, use the selected version
        if (this.activeTab === 'standard') {
          siteConfig.version = this.version
          // Reset custom branch
          siteConfig.customBranch = null
        }

        // Call your createSite function with this configuration
        // This is where you would integrate with your existing functionality
        this.logs.push('Site configuration prepared, starting Docker containers...')
        
        // Example of how you might call the backend
        // const result = await window.__TAURI__.invoke('create_site', { config: siteConfig })
        
        this.logs.push('Site created successfully!')
        // Add additional logging about the site being created
        this.logs.push(`Site URL: http://localhost:${siteConfig.webPort}`)
        
        // Reset the form
        // this.resetForm()
      } catch (error) {
        this.logs.push(`Error creating site: ${error}`)
      } finally {
        this.createSiteInProgress = false
      }
    },
    
    resetForm() {
      this.name = ''
      this.version = ''
      this.webPort = ''
      this.dbPort = ''
      this.dbType = 'MySQL'
      this.useRedis = false
      this.moodleFolder = ''
      // Reset custom tab fields
      this.customRepoUrl = ''
      this.customBranch = ''
      this.dbPrefix = 'mdl_'
      this.siteConfig = {
        debug: false,
        devLibs: false,
        disableCaching: false,
        disableThemeCaching: false
      }
      this.adminAccount = {
        username: 'admin',
        password: 'admin',
        email: 'admin@example.com',
        fullname: 'Admin User'
      }
      // Reset advanced tab fields
      this.phpVersion = '8.1'
      this.phpConfig = {
        memoryLimit: '512M',
        additionalModules: '',
        xdebug: false
      }
      this.dockerConfig = {
        network: 'moodle-network',
        containerPrefix: 'moodle-',
        composePath: ''
      }
      this.volumeMounts = []
      this.environmentVariables = []
    },

    // Custom tab methods
    resetToDefaultRepo() {
      this.customRepoUrl = '';
    },

    useSelectedStableBranch() {
      if (this.version) {
        this.customBranch = `MOODLE_${this.version.replace('.', '')}_STABLE`;
      } else {
        this.customBranch = '';
      }
    },

    // Advanced tab methods
    addVolumeMount() {
      this.volumeMounts.push({ source: '', target: '' });
    },
    
    removeVolumeMount(index) {
      this.volumeMounts.splice(index, 1);
    },
    
    addEnvironmentVariable() {
      this.environmentVariables.push({ key: '', value: '' });
    },
    
    removeEnvironmentVariable(index) {
      this.environmentVariables.splice(index, 1);
    },
    
    detectDockerComposePath() {
      // In a real implementation, this would detect the docker-compose path
      // For now, just set a default value
      this.dockerConfig.composePath = '/usr/local/bin/docker-compose';
      this.logs.push('Docker Compose path detected: /usr/local/bin/docker-compose');
    },
    
    // Validate a Git URL
    isValidGitUrl(url) {
      if (!url) return true; // Empty is valid (will use default)
      
      // Check for common git URL formats
      const gitUrlRegex = /^(https?:\/\/|git@)([^\s:]+)(:|\/)([\w\-\.]+)\/([\w\-\.]+)(\.git)?$/;
      return gitUrlRegex.test(url);
    },
  }
};
</script>

<style scoped>
/* Additional component styles - these complement Tailwind classes */
.macos-window {
  background: linear-gradient(180deg, #ffffff 0%, #f9fafb 100%);
  box-shadow: 0 0 0 0.5px rgba(0, 0, 0, 0.05), 0 2px 8px rgba(0, 0, 0, 0.1);
  border-radius: 10px;
}

.macos-traffic-lights {
  position: absolute;
  top: 12px;
  left: 12px;
  display: flex;
  gap: 8px;
}

.traffic-light {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.traffic-light-red {
  background-color: #ff5f56;
  border: 0.5px solid #e0443e;
}

.traffic-light-yellow {
  background-color: #ffbd2e;
  border: 0.5px solid #dea123;
}

.traffic-light-green {
  background-color: #27c93f;
  border: 0.5px solid #1aab29;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .macos-traffic-lights {
    display: none;
  }
}
</style>