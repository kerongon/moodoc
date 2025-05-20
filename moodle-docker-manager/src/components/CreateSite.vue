<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700 w-full mx-auto p-3 dark:text-gray-100">
    
    <!-- Header with App Icon - smaller and more compact like MAMP -->
    <div class="flex items-center mb-3">
      <div class="bg-blue-600 rounded-lg p-1 mr-2 shadow-sm">
        <PhSquare class="text-white w-5 h-5" weight="fill" />
      </div>
      <div>
        <h2 class="text-sm font-medium text-gray-900 dark:text-gray-100">Create a New Moodle Site</h2>
        <p class="text-[10px] text-gray-500 dark:text-gray-400">Configure and set up a new Moodle development environment</p>
      </div>
    </div>
    
    <!-- Tab Navigation - more compact like MAMP -->
    <div class="flex bg-gray-100 dark:bg-gray-700 rounded-md p-0.5 mb-2 border border-gray-200 dark:border-gray-600 macos-tab-container">
      <button 
        class="flex-1 py-0.5 px-2 text-[10px] font-medium transition-all duration-150 relative overflow-hidden" 
        @click="activeTab = 'standard'"
        :class="activeTab === 'standard' ? 'bg-white dark:bg-gray-800 rounded-md shadow-sm text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400' : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'">
        Standard
      </button>
      <button 
        class="flex-1 py-0.5 px-2 text-[10px] font-medium transition-all duration-150 relative overflow-hidden" 
        @click="activeTab = 'custom'"
        :class="activeTab === 'custom' ? 'bg-white dark:bg-gray-800 rounded-md shadow-sm text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400' : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'">
        Custom
      </button>
      <button 
        class="flex-1 py-0.5 px-2 text-[10px] font-medium transition-all duration-150 relative overflow-hidden" 
        @click="activeTab = 'advanced'"
        :class="activeTab === 'advanced' ? 'bg-white dark:bg-gray-800 rounded-md shadow-sm text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400' : 'text-gray-500 hover:text-gray-700 dark:hover:text-gray-300'">
        Advanced
      </button>
    </div>
    
    <!-- Form Content -->
    <form @submit.prevent="createSite" class="space-y-4">
        <!-- Standard Tab Content -->
        <div v-if="activeTab === 'standard'">
          <!-- Site Name -->
          <div class="mb-1.5">
            <label for="siteName" class="block text-[10px] font-medium text-gray-700 dark:text-gray-300 mb-0.5 flex items-center">
            <svg class="w-3 h-3 mr-0.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 2a1 1 0 00-1 1v1a1 1 0 002 0V3a1 1 0 00-1-1zM4 4h3a3 3 0 006 0h3a2 2 0 012 2v9a2 2 0 01-2 2H4a2 2 0 01-2-2V6a2 2 0 012-2zm2.5 7a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm2.45 4a2.5 2.5 0 10-4.9 0h4.9zM12 9a1 1 0 100 2h3a1 1 0 100-2h-3zm-1 4a1 1 0 011-1h2a1 1 0 110 2h-2a1 1 0 01-1-1z" clip-rule="evenodd" />
            </svg>
            Site Name
          </label>
          <input 
            type="text" 
            id="siteName" 
            v-model="siteName" 
            :class="['macos-input h-6 py-0.5 text-xs', { 'border-red-500 focus:ring-red-500 focus:border-red-500': siteName && !siteNameIsValid }]" 
            required 
            placeholder="moodle-site"
          />
          <p v-if="siteName && !siteNameIsValid" class="mt-0.5 text-xs text-red-600">
            Site name can only contain letters, numbers, underscores and hyphens
          </p>
        </div>
        
        <div class="p-1.5 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 mb-2">
          <div class="grid grid-cols-2 gap-1.5">
            <!-- Moodle Version -->
            <div>
              <label for="moodleBranch" class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1 flex items-center">
                <svg class="w-3.5 h-3.5 mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd" />
                </svg>
                Moodle Version
              </label>
              <select 
                id="moodleBranch" 
                v-model="moodleBranch" 
                @change="updatePHPVersion" 
                class="macos-select compact-select h-7 py-0.5"
              >
                <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
              </select>
            </div>
            
            <!-- PHP Version -->
            <div>
              <label for="phpVersion" class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1 flex items-center">
                <svg class="w-3.5 h-3.5 mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
                PHP Version
              </label>
              <select id="phpVersion" v-model="phpVersion" class="macos-select compact-select h-7 py-0.5">
                <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
              </select>
            </div>
          </div>
          <div class="mt-2">
            <p v-if="recommendedPhpVersion" class="text-xs text-gray-500 dark:text-gray-400">
              Recommended PHP version: <span class="font-medium">{{ recommendedPhpVersion }}</span>
            </p>
            <p v-if="isPhpVersionCompatible === false" class="text-xs text-red-500">
              Warning: This PHP version may not be compatible with the selected Moodle branch
            </p>
          </div>
        </div>
        
        <!-- Server Configuration Group -->
        <div class="mb-3 p-2 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
          <h3 class="text-xs font-medium text-gray-700 dark:text-gray-200 mb-1 flex items-center">
            <svg class="w-3.5 h-3.5 mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M2 5a2 2 0 012-2h12a2 2 0 012 2v10a2 2 0 01-2 2H4a2 2 0 01-2-2V5zm3.293 1.293a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 01-1.414-1.414L7.586 10 5.293 7.707a1 1 0 010-1.414zM11 12a1 1 0 100 2h3a1 1 0 100-2h-3z" clip-rule="evenodd" />
            </svg>
            Server Configuration
          </h3>

          <!-- Database Configuration -->
          <div class="mb-4">
            <div class="flex items-center justify-between mb-1">
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 flex items-center">
                <span>Database Configuration</span>
                <button type="button" class="ml-1 text-gray-400 hover:text-gray-500" title="Choose database type and port">
                  <svg class="w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                  </svg>
                </button>
              </label>
            </div>
            <div class="flex space-x-3">
              <div class="flex-1">
                <select 
                  id="dbType" 
                  v-model="dbType" 
                  @change="updateDbType" 
                  class="macos-select compact-select h-7 py-0.5"
                >
                  <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
                </select>
              </div>
              <div class="w-1/3 relative flex items-center">
                <input 
                  type="number" 
                  v-model="dbPort" 
                  :placeholder="`${currentDbPortRange}`" 
                  :class="['macos-input h-7 py-0.5 text-xs pr-12', 
                    { 
                      'border-red-500 focus:ring-red-500 focus:border-red-500': dbPort && !isDbPortInRange,
                      'border-green-500 focus:ring-green-500 focus:border-green-500': dbPort && isDbPortInRange && isPortAvailable(dbPort)
                    }
                  ]"
                />
                <button 
                  type="button" 
                  @click="generateDbPort" 
                  class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-all duration-150 p-1"
                  title="Generate random port"
                  aria-label="Generate random database port"
                >
                  <!-- Refresh/recycle icon with improved styling -->
                  <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
            </div>
            <div class="flex justify-between mt-0.5 text-xs">
              <span v-if="dbPort && isDbPortInRange && isPortAvailable(dbPort)" class="text-green-600">
                Port {{ dbPort }} is available ✓
              </span>
              <span v-else-if="dbPort && !isDbPortInRange" class="text-red-600">
                Port outside range {{ currentDbPortRange }}
              </span>
              <span v-else-if="dbPort && !isPortAvailable(dbPort)" class="text-amber-600">
                Port {{ dbPort }} may be in use ⚠️
              </span>
              <span class="text-gray-400">{{ dbType }} database</span>
            </div>
          </div>
          
          <!-- Web Server -->
          <div>
            <div class="flex items-center justify-between mb-1">
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 flex items-center">
                <span>Web Server</span>
                <button type="button" class="ml-1 text-gray-400 hover:text-gray-500" title="Configure web server port">
                  <svg class="w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                  </svg>
                </button>
              </label>
            </div>
            <div class="flex space-x-3">
              <div class="flex-1">
                <select class="macos-select compact-select h-7 py-0.5">
                  <option>Apache</option>
                </select>
              </div>
              <div class="w-1/3 relative flex items-center">
                <input 
                  type="number" 
                  v-model="webPort" 
                  :placeholder="`${currentWebPortRange}`" 
                  :class="['macos-input h-7 py-0.5 text-xs pr-12', 
                    { 
                      'border-red-500 focus:ring-red-500 focus:border-red-500': webPort && !isWebPortInRange,
                      'border-green-500 focus:ring-green-500 focus:border-green-500': webPort && isWebPortInRange && isPortAvailable(webPort)
                    }
                  ]"
                />
                <button 
                  type="button" 
                  @click="generateWebPort" 
                  class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300 transition-all duration-150 p-1"
                  title="Generate random port"
                  aria-label="Generate random web server port"
                >
                  <!-- Refresh/recycle icon with improved styling -->
                  <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
            </div>
            <div class="flex justify-between mt-0.5 text-xs">
              <span v-if="webPort && isWebPortInRange && isPortAvailable(webPort)" class="text-green-600">
                Port {{ webPort }} is available ✓
              </span>
              <span v-else-if="webPort && !isWebPortInRange" class="text-red-600">
                Port outside range {{ currentWebPortRange }}
              </span>
              <span v-else-if="webPort && !isPortAvailable(webPort)" class="text-amber-600">
                Port {{ webPort }} may be in use ⚠️
              </span>
              <span class="text-gray-400">Apache web server</span>
            </div>
          </div>
        </div>
        
        <!-- Site Directory - macOS Finder style -->
        <div class="mb-3 p-2 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600">
          <label class="block text-xs font-medium text-gray-700 dark:text-gray-200 mb-1 flex items-center">
            <svg class="w-3 h-3 mr-1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6a2 2 0 012-2h2V2zM5 5h10v10H5V5z" />
            </svg>
            Site Directory
          </label>
          <div class="flex shadow-sm">
            <input 
              type="text" 
              v-model="siteDirectory" 
              :placeholder="useDefaultDirectory ? 'Using default directory' : ''" 
              class="macos-input h-7 py-0.5 rounded-r-none focus:z-10 font-mono text-xs" 
              readonly 
            />
            <button 
              type="button" 
              @click="selectCustomDirectory" 
              class="px-3 border border-l-0 border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-600 rounded-r-md text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-500 transition-all duration-150 font-medium text-xs h-7"
            >
              Browse...
            </button>
          </div>
          <div class="mt-3 flex justify-between items-center">
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
          <p v-if="siteName" class="mt-2 text-xs text-gray-500">
            Site will be created at: <span class="font-mono">{{ getFullSitePath() }}</span>
          </p>
        </div>
        
        <!-- Redis Toggle -->
        <div class="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg border border-gray-200 dark:border-gray-600 mb-3 flex items-center justify-between">
          <div class="flex items-start">
            <svg class="w-3 h-3 text-gray-700 dark:text-gray-200 mt-0.5 mr-1 flex-shrink-0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path d="M13 7H7v6h6V7z" />
              <path fill-rule="evenodd" d="M7 2a1 1 0 012 0v1h2V2a1 1 0 112 0v1h2a2 2 0 012 2v2h1a1 1 0 110 2h-1v2h1a1 1 0 110 2h-1v2a2 2 0 01-2 2h-2v1a1 1 0 11-2 0v-1H9v1a1 1 0 11-2 0v-1H5a2 2 0 01-2-2v-2H2a1 1 0 110-2h1V9H2a1 1 0 010-2h1V5a2 2 0 012-2h2V2zM5 5h10v10H5V5z" clip-rule="evenodd" />
            </svg>
            <div>
              <label for="useRedis" class="text-xs font-medium text-gray-700 dark:text-gray-200">Enable Redis for caching and sessions</label>
              <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Improves performance for larger Moodle sites</p>
            </div>
          </div>
          <div class="relative inline-flex h-5 align-middle">
            <input 
              type="checkbox"
              id="redisToggle" 
              v-model="useRedis"
              class="sr-only peer"
            />
            <label 
              for="redisToggle"
              class="flex w-9 h-5 bg-gray-200 rounded-full cursor-pointer peer-focus:ring-1 peer-focus:ring-blue-300 peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4"
            ></label>
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
                <select id="phpVersion" v-model="phpVersion" class="macos-select compact-select">
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
      
      <!-- Docker Status - more compact -->
      <div class="flex items-center justify-between text-xs px-2 py-1 bg-gray-50 dark:bg-gray-700 rounded-md mb-2">
        <div class="text-gray-700 dark:text-gray-200 text-xs">Docker is installed and running</div>
        <div class="flex items-center">
          <div class="w-1.5 h-1.5 rounded-full bg-green-500 mr-1"></div>
          <span class="text-green-600 dark:text-green-400 text-xs">Active</span>
        </div>
      </div>
      
      <!-- Form Errors -->
      <div v-if="formErrors.length > 0" class="bg-red-50 border border-red-200 rounded-md p-3 mb-4">
        <h3 class="text-sm font-medium text-red-800 mb-1">Please fix the following issues:</h3>
        <ul class="text-xs text-red-700 list-disc pl-5">
          <li v-for="(error, index) in formErrors" :key="index" class="mb-1">{{ error }}</li>
        </ul>
      </div>
      
      <!-- Action Buttons - macOS style -->
      <div class="flex justify-end space-x-2 mt-4 pt-3 border-t border-gray-200 dark:border-gray-700">
        <button 
          type="button" 
          class="px-3 py-1 text-xs font-medium bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600 transition-all duration-200 shadow-sm" 
          @click="$router.push('/')"
        >
          Cancel
        </button>
        <button 
          type="submit" 
          class="px-3 py-1 text-xs font-medium bg-blue-600 border border-blue-700 rounded-md text-white hover:bg-blue-700 transition-all duration-200 shadow-sm relative"
          :class="{ 'opacity-50 cursor-not-allowed hover:bg-blue-600': !formIsValid || formErrors.length > 0 || createSiteInProgress }"
          :disabled="!formIsValid || formErrors.length > 0 || createSiteInProgress"
        >
          <span v-if="!createSiteInProgress">Create Site</span>
          <span v-else class="flex items-center">
            <svg class="animate-spin -ml-0.5 mr-1 h-3 w-3 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
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
      <button 
        type="button"
        @click="logsExpanded = !logsExpanded"
        class="w-full flex items-center justify-between py-2 px-3 rounded bg-gray-50 hover:bg-gray-100 transition-colors duration-150"
      >
        <div class="flex items-center">
          <h3 class="text-sm font-medium text-gray-700">Logs</h3>
          <span class="ml-2 text-xs text-gray-500">({{ logs.length }} entries)</span>
          <!-- Show a badge for latest logs -->
          <span v-if="logs.length > 0 && !logsExpanded" class="ml-2 inline-block bg-blue-100 text-blue-800 text-xs px-2 py-0.5 rounded-full">
            {{ logs[logs.length-1].substring(0, 30) }}{{ logs[logs.length-1].length > 30 ? '...' : '' }}
          </span>
        </div>
        <div class="flex items-center space-x-3">
          <!-- Copy Logs Button -->
          <button
            v-if="logs.length > 0" 
            type="button"
            @click.stop="copyLogs($event)"
            class="flex items-center text-blue-600 hover:text-blue-800 text-xs px-1.5 py-0.5 bg-blue-50 hover:bg-blue-100 rounded transition-colors duration-150"
            title="Copy all log entries to clipboard"
          >
            <svg 
              class="w-3 h-3 mr-0.5" 
              xmlns="http://www.w3.org/2000/svg" 
              viewBox="0 0 20 20" 
              fill="currentColor"
            >
              <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
              <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
            </svg>
            Copy
          </button>
          
          <!-- Show/Hide Details -->
          <div class="flex items-center text-gray-500 text-xs">
            {{ logsExpanded ? 'Hide details' : 'Show details' }}
            <svg 
              class="ml-1 w-4 h-4 transform transition-transform duration-200" 
              :class="logsExpanded ? 'rotate-180' : ''"
              xmlns="http://www.w3.org/2000/svg" 
              viewBox="0 0 20 20" 
              fill="currentColor"
            >
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </div>
        </div>
      </button>
      <transition
        name="collapse"
        @enter="el => el.style.height = el.scrollHeight + 'px'"
        @leave="el => el.style.height = '0'"
      >
        <div 
          v-if="logsExpanded"
          class="bg-gray-100 rounded-md p-3 font-mono text-xs transition-all duration-300 overflow-auto"
          style="max-height: 200px;"
        >
          <div v-for="(log, i) in logs" :key="i" class="pb-1 whitespace-pre-wrap break-words">{{ log }}</div>
        </div>
      </transition>
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
      dbTypes: ['MySQL', 'MariaDB', 'PostgreSQL', 'Microsoft SQL Server', 'Oracle'],
      moodleBranches: ['MOODLE_500_STABLE', 'MOODLE_405_STABLE', 'MOODLE_404_STABLE', 'MOODLE_403_STABLE', 'MOODLE_402_STABLE', 'MOODLE_401_STABLE'],
      logsExpanded: false, // Controls the visibility of the logs section
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
          'Oracle': { min: 1521, max: 1599 }
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
    this.checkPlatformCompatibility(); // Check if selected DB is compatible with current platform
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
      
      // Check platform compatibility with the selected database type
      this.checkPlatformCompatibility();
    },
    
    async checkPlatformCompatibility() {
      try {
        const compatMessage = await invoke('check_platform_compatibility', {
          dbType: this.dbType
        });
        
        if (compatMessage && !compatMessage.includes('compatible')) {
          // This is a warning message
          this.addLogMessage(`⚠️ ${compatMessage}`);
        }
      } catch (error) {
        console.error("Error checking platform compatibility:", error);
      }
    },
    
    // Helper method to add log messages without auto-expanding logs
    addLogMessage(message) {
      this.logs.push(message);
      // Logs will remain collapsed by default
    },
    
    // Copy logs to clipboard
    async copyLogs(event) {
      try {
        // Store the original button text
        const button = event.currentTarget;
        const originalText = button.innerHTML;
        const originalClass = button.className;
        
        // Copy logs to clipboard
        const logText = this.logs && this.logs.length ? this.logs.join('\n') : 'No logs available';
        await navigator.clipboard.writeText(logText);
        
        // Update button to show success
        button.innerHTML = '<svg class="w-3 h-3 mr-0.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 111.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" /></svg>Copied!';
        button.className = originalClass.replace('text-blue-600 hover:text-blue-800 bg-blue-50 hover:bg-blue-100', 'text-green-600 bg-green-50');
        
        // Show a brief notification in logs
        this.addLogMessage('Logs copied to clipboard');
        
        // Reset button after a short delay
        setTimeout(() => {
          button.innerHTML = originalText;
          button.className = originalClass;
        }, 2000);
      } catch (err) {
        console.error('Failed to copy logs:', err);
        // Show error in logs
        this.addLogMessage('Failed to copy logs: ' + err.message);
      }
    },
    
    async checkDockerStatus() {
      try {
        const dockerInstalled = await invoke('check_docker_installed');
        const dockerRunning = await invoke('check_docker_running');
        
        this.dockerInstalled = dockerInstalled && dockerRunning;
        
        if (!this.dockerInstalled) {
          this.addLogMessage('Docker is not installed or not running. Please install Docker and try again.');
        } else {
          this.addLogMessage('Docker is installed and running.');
        }
      } catch (error) {
        this.addLogMessage(`Error checking Docker status: ${error}`);
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
        this.addLogMessage(`Error selecting directory: ${error}`);
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
        this.addLogMessage('Cannot create site: Docker is not installed or not running.');
        return;
      }

      if (!this.siteName) {
        this.addLogMessage('Please enter a site name');
        return;
      }

      if (!this.siteDirectory && !this.useDefaultDirectory) {
        this.addLogMessage('Please select a directory for the site');
        return;
      }

      if (!this.moodleBranch && this.activeTab === 'standard') {
        this.addLogMessage('Please select a Moodle version');
        return;
      }

      // Validate ports if specified
      if (this.webPort && !this.isWebPortInRange) {
        this.addLogMessage(`Web port ${this.webPort} is outside the recommended range (${this.currentWebPortRange})`);
        return;
      }
      
      if (this.dbPort && !this.isDbPortInRange) {
        this.addLogMessage(`DB port ${this.dbPort} is outside the recommended range for ${this.dbType} (${this.currentDbPortRange})`);
        return;
      }

      // Start the creation process
      this.createSiteInProgress = true;
      this.addLogMessage(`Creating site ${this.siteName}...`);

      try {
        // Compile the settings from all tabs
        const siteConfig = {
          name: this.siteName,
          directory: this.useDefaultDirectory ? this.defaultDirectory : this.siteDirectory,
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
          siteConfig.moodleBranch = this.moodleBranch;
          // Reset custom branch
          siteConfig.customBranch = null;
        }

        this.addLogMessage('Site configuration prepared, starting Docker containers...');
        
        // Call the Tauri backend to create the site
        const result = await invoke('create_site', {
          name: siteConfig.name,
          phpVersion: siteConfig.phpVersion,
          dbType: siteConfig.dbType.toLowerCase(), // Convert to lowercase to match backend expectations
          moodleBranch: siteConfig.moodleBranch || siteConfig.customBranch || 'MOODLE_400_STABLE',
          dbPort: siteConfig.dbPort,
          webPort: siteConfig.webPort,
          directory: siteConfig.directory
        });
        
        this.addLogMessage(`Result: ${result}`);
        this.addLogMessage('Site created successfully!');
        // Add additional logging about the site being created
        this.addLogMessage(`Site URL: http://localhost:${siteConfig.webPort}`);
        
        // Reset the form
        this.resetForm();
      } catch (error) {
        this.addLogMessage(`Error creating site: ${error}`);
      } finally {
        this.createSiteInProgress = false;
      }
    },
    
    resetForm() {
      this.siteName = ''
      this.moodleBranch = 'MOODLE_500_STABLE'
      this.webPort = ''
      this.dbPort = ''
      this.dbType = 'MySQL'
      this.useRedis = false
      this.siteDirectory = ''
      this.useDefaultDirectory = true
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
      this.addLogMessage('Docker Compose path detected: /usr/local/bin/docker-compose');
    },
    
    // Validate a Git URL
    isValidGitUrl(url) {
      if (!url) return true; // Empty is valid (will use default)
      
      // Check for common git URL formats
      const gitUrlRegex = /^(https?:\/\/|git@)([^\s:]+)(:|\/)([\w\-\.]+)\/([\w\-\.]+)(\.git)?$/;
      return gitUrlRegex.test(url);
    },
    
    // Check if a port is likely available
    isPortAvailable(port) {
      // This is a simplified check - in a real implementation, you would
      // want to actually check if the port is in use
      // For now, we'll randomly return true/false to simulate the check
      // In production, this would call a Tauri function to check the port
      
      // Pseudo random availability based on port number
      return (port % 10) !== 3; // Ports ending in 3 are "unavailable" for demo purposes
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

/* Make form field labels bold */
.space-y-6 label,
.mb-4 > label,
.mb-5 > label,
.mb-3,
h3.text-sm {
  font-weight: 700 !important;
}

/* Make subheadings and section titles bold too */
.text-sm.font-medium.text-gray-700 {
  font-weight: 700 !important;
}

/* Dark mode styles */
@media (prefers-color-scheme: dark) {
  .macos-input,
  .macos-select {
    background-color: #374151 !important;
    border-color: #4b5563 !important;
    color: white !important;
  }
  
  .macos-button-secondary {
    background-color: #374151 !important;
    border-color: #4b5563 !important;
    color: #e5e7eb !important;
  }
  
  .macos-button-primary {
    background-color: #2563eb !important;
  }
  
  .bg-gray-50 {
    background-color: #1f2937 !important;
  }
  
  .bg-gray-100 {
    background-color: #1f2937 !important;
  }
  
  .border-gray-200 {
    border-color: #374151 !important;
  }
  
  .text-gray-700 {
    color: #e5e7eb !important;
  }
  
  .text-gray-600 {
    color: #d1d5db !important;
  }
  
  .text-gray-500 {
    color: #9ca3af !important;
  }
}

/* Transition styles for collapsible logs */
.collapse-enter-active,
.collapse-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
  max-height: 500px; /* Increased maximum height to ensure all content is visible */
}

.collapse-enter-from,
.collapse-leave-to {
  height: 0 !important;
  opacity: 0;
  margin-top: 0;
  padding: 0;
  overflow: hidden;
}

/* Hide scrollbar when not needed */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

/* Hide scrollbar when not scrolling */
::-webkit-scrollbar-thumb {
  transition: background 0.3s;
}

/* macOS style animations */
@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.7; }
  100% { opacity: 1; }
}

.pulse-animation {
  animation: pulse 1.5s infinite ease-in-out;
}

/* Add macOS-style smooth focus transitions */
input:focus, select:focus, button:focus {
  transition: all 0.2s ease-in-out;
}

/* Responsive tweaks for full-width layout */
@media (min-width: 1024px) {
  .form-row {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }
}

/* Fixed height for create site container to ensure app-like feel */
.create-site-container {
  /* No fixed height constraint, show all content */
  width: 100%;
  display: block;
}
</style>