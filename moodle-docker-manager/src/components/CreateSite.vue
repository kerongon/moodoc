<template>
  <div class="dashboard-full hide-scrollbar bg-gradient-to-br from-gray-900 to-gray-800 dark:from-gray-900 dark:to-black min-h-screen w-full p-0">
    <!-- Modern macOS-style header (copied from Dashboard, no New Site button) -->
    <header class="app-header" data-tauri-drag-region>
      <div class="header-content">
        <div class="titlebar-left-controls">
          <!-- Window Controls -->
          <div class="window-controls">
            <button @click="closeWindow" class="window-control-button close-button" title="Close">
              <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1"></circle></svg>
            </button>
            <!-- <button @click="minimizeWindow" class="window-control-button minimize-button" title="Minimize">
              <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FFBD2E" stroke="#DEA123" stroke-width="1"></circle></svg>
            </button> -->
            <button @click="toggleMaximizeWindow" class="window-control-button maximize-button" title="Maximize">
              <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#27C93F" stroke="#1AAB29" stroke-width="1"></circle></svg>
            </button>
          </div>
          <!-- Navigation Controls -->
          <div class="navigation-controls">
            <button @click="goBack" class="header-nav-button" title="Go Back">
              <span class="chevron">&lt;</span>
            </button>
            <button @click="goForward" class="header-nav-button" title="Go Forward">
              <span class="chevron">&gt;</span>
            </button>
            <button @click="reloadPage" class="header-nav-button" title="Reload Page">
              <PhArrowClockwise class="button-icon" />
            </button>
          </div>
        </div>

        <!-- Logo and app title -->
        <div class="app-branding">
          <div class="logo-container">
            <PhSquare class="logo-icon text-white" weight="fill" />
          </div>
          <div class="title-container">
            <h1 class="app-title">Moodle Docker Manager</h1>
          </div>
        </div>
        <!-- Header actions (no New Site button) -->
        <div style="display: flex; align-items: center; gap: 0.75rem;">
          <DockerStatus />
          <button @click="openSettings" title="Settings" class="modern-settings-button">
            <PhGear class="button-icon" />
          </button>
        </div>
      </div>
    </header>
    <div class="main-content px-4 pb-4 hide-scrollbar">
      <div class="create-site-container">
        <div class="site-header flex items-center mb-4">
          <div class="icon-container bg-gradient-to-r from-indigo-600 to-blue-500 p-2 rounded-xl mr-3 shadow-lg shadow-indigo-500/20">
            <PhPlus class="text-white w-5 h-5" weight="fill" />
          </div>
          <div>
            <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Create a New Moodle Site</h2>
            <p class="text-xs text-gray-500 dark:text-gray-400">Configure and set up a new Moodle development environment</p>
          </div>
        </div>
        
        <!-- Modern Tab Navigation -->
        <div class="modern-tabs-container">
          <button 
            class="modern-tab-button" 
            @click="activeTab = 'standard'"
            :class="{'active-tab': activeTab === 'standard'}">
            <span class="tab-icon">📋</span>
            Standard
          </button>
          <button 
            class="modern-tab-button" 
            @click="activeTab = 'custom'"
            :class="{'active-tab': activeTab === 'custom'}">
            <span class="tab-icon">⚙️</span>
            Custom
          </button>
          <button 
            class="modern-tab-button" 
            @click="activeTab = 'advanced'"
            :class="{'active-tab': activeTab === 'advanced'}">
            <span class="tab-icon">🔧</span>
            Advanced
          </button>
        </div>
    
    <!-- Form Content -->
    <form @submit.prevent="createSite" class="space-y-2">
        <!-- Tab content with transitions -->
        <transition name="tab-transition" mode="out-in">
        <!-- Standard Tab Content -->
        <div v-if="activeTab === 'standard'" key="standard" class="form-content">
          <!-- Site Name -->
          <div class="config-section">
            <label for="siteName" class="modern-form-label">
              <div class="label-icon">★</div>
              Site Name
            </label>
            <input 
              type="text" 
              id="siteName" 
              v-model="siteName" 
              class="modern-input" 
              :class="{ 'error-input': siteName && !siteNameIsValid }" 
              required 
              placeholder="moodle-site"
            />
            <p v-if="siteName && !siteNameIsValid" class="error-message">
              Site name can only contain lowercase letters, numbers, underscores and hyphens
            </p>
            <p v-if="siteName" class="info-message">
              Site will be created at: <span class="font-mono">{{ getFullSitePath() }}</span>
            </p>
          </div>
        
        <div class="config-section version-config">
          <h3 class="section-title">Version Configuration</h3>
          <div class="grid-container compact">
            <!-- Moodle Version -->
            <div class="form-group">
              <label for="moodleBranch" class="modern-form-label">
                <div class="label-icon">⚡</div>
                Moodle Version
              </label>
              <select 
                id="moodleBranch" 
                v-model="moodleBranch" 
                @change="updatePHPVersion" 
                class="modern-select"
              >
                <option v-for="branch in moodleBranches" :key="branch" :value="branch">{{ branch }}</option>
              </select>
            </div>
            
            <!-- PHP Version -->
            <div class="form-group">
              <label for="phpVersion" class="modern-form-label">
                <div class="label-icon">❯</div>
                PHP Version
              </label>
              <select id="phpVersion" v-model="phpVersion" class="modern-select">
                <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
              </select>
            </div>
          </div>
          <div class="version-info">
            <p v-if="recommendedPhpVersion" class="recommendation-message">
              Recommended PHP version: <span class="highlight">{{ recommendedPhpVersion }}</span>
            </p>
            <p v-if="isPhpVersionCompatible === false" class="warning-message">
              ⚠️ This PHP version may not be compatible with the selected Moodle branch
            </p>
          </div>
        </div>
        
        <!-- Server Configuration Group -->
        <div class="config-section server-config">
          <h3 class="section-title">
            <div class="section-icon">🖥️</div>
            Server Configuration
          </h3>

          <!-- Database Configuration -->
          <div class="config-subsection">
            <div class="subsection-header">
              <h4 class="subsection-title">Database Configuration</h4>
              <div class="info-tooltip" title="Choose database type and port">ⓘ</div>
            </div>
            
            <div class="port-config-row">
              <div class="db-type-container">
                <select 
                  id="dbType" 
                  v-model="dbType" 
                  @change="updateDbType" 
                  class="modern-select"
                >
                  <option v-for="type in dbTypes" :key="type" :value="type">{{ type }}</option>
                </select>
              </div>
              <div class="port-input-container">
                <input 
                  type="number" 
                  v-model="dbPort" 
                  :placeholder="`${currentDbPortRange}`" 
                  class="modern-input port-input"
                  :class="{ 
                    'error-input': dbPort && !isDbPortInRange,
                    'success-input': dbPort && isDbPortInRange && isPortAvailable(dbPort)
                  }"
                />
                <button 
                  type="button" 
                  @click="generateDbPort" 
                  class="generate-button"
                  title="Generate random port"
                >
                  <span>↻</span>
                </button>
              </div>
            </div>
            
            <div class="port-status">
              <span v-if="dbPort && isDbPortInRange && portAvailability[dbPort]" class="status-available">
                Port {{ dbPort }} is available ✓
              </span>
              <span v-else-if="dbPort && !isDbPortInRange" class="status-error">
                Port outside range {{ currentDbPortRange }}
              </span>
              <span v-else-if="dbPort && portAvailability[dbPort] === false" class="status-warning">
                Port {{ dbPort }} is in use ⚠️
              </span>
              <span v-else-if="dbPort && portAvailability[dbPort] === undefined" class="status-checking">
                Checking port availability...
              </span>
              <span class="db-type-label">{{ dbType }} database</span>
            </div>
          </div>
          
          <!-- Web Server -->
          <div class="config-subsection">
            <div class="subsection-header">
              <h4 class="subsection-title">Web Server</h4>
              <div class="info-tooltip" title="Configure web server port">ⓘ</div>
            </div>
            
            <div class="port-config-row">
              <div class="db-type-container">
                <select class="modern-select">
                  <option>Apache</option>
                </select>
              </div>
              <div class="port-input-container">
                <input 
                  type="number" 
                  v-model="webPort" 
                  :placeholder="`${currentWebPortRange}`" 
                  class="modern-input port-input"
                  :class="{ 
                    'error-input': webPort && !isWebPortInRange,
                    'success-input': webPort && isWebPortInRange && isPortAvailable(webPort)
                  }"
                />
                <button 
                  type="button" 
                  @click="generateWebPort" 
                  class="generate-button"
                  title="Generate random port"
                >
                  <span>↻</span>
                </button>
              </div>
            </div>
            
            <div class="port-status">
              <span v-if="webPort && isWebPortInRange && portAvailability[webPort]" class="status-available">
                Port {{ webPort }} is available ✓
              </span>
              <span v-else-if="webPort && !isWebPortInRange" class="status-error">
                Port outside range {{ currentWebPortRange }}
              </span>
              <span v-else-if="webPort && portAvailability[webPort] === false" class="status-warning">
                Port {{ webPort }} is in use ⚠️
              </span>
              <span v-else-if="webPort && portAvailability[webPort] === undefined" class="status-checking">
                Checking port availability...
              </span>
              <span class="db-type-label">Apache web server</span>
            </div>
          </div>
        </div>
        
        <!-- Site Directory -->
        <div class="config-section directory-config">
          <h3 class="section-title">
            <div class="section-icon">📁</div>
            Site Directory
          </h3>
          <div class="directory-input-row">
            <input 
              type="text" 
              v-model="siteDirectory" 
              :placeholder="useDefaultDirectory ? 'Using default directory' : ''" 
              class="modern-input directory-input" 
              readonly 
            />
            <button 
              type="button" 
              @click="selectCustomDirectory" 
              class="browse-button"
            >
              Browse...
            </button>
          </div>
          <div class="directory-info">
            <p class="directory-path">
              <span v-if="useDefaultDirectory">
                Using default directory: <span class="path-value">{{ defaultDirectory || '[App Data Directory]/sites' }}</span>
              </span>
              <span v-else>
                Custom directory: <span class="path-value">{{ siteDirectory }}</span>
              </span>
            </p>
            <button 
              v-if="!useDefaultDirectory" 
              type="button" 
              @click="resetToDefaultDirectory" 
              class="reset-button"
            >
              Reset to default
            </button>
          </div>
          <p v-if="!useDefaultDirectory && !siteDirectory" class="error-message">
            Please select an installation directory
          </p>
          <p v-else-if="useDefaultDirectory && !defaultDirectory" class="warning-message">
            Default directory not available. Please select a custom directory.
          </p>
        </div>
        
        <!-- Redis Toggle -->
        <div class="config-section toggle-section">
          <div class="toggle-container">
            <div class="toggle-content">
              <div class="toggle-icon">🔄</div>
              <div class="toggle-info">
                <h4 class="toggle-title">Enable Redis for caching and sessions</h4>
                <p class="toggle-description">Improves performance for larger Moodle sites</p>
              </div>
            </div>
            <div class="toggle-switch">
              <input 
                type="checkbox"
                id="redisToggle" 
                v-model="useRedis"
                class="sr-only peer"
              />
              <label 
                for="redisToggle"
                class="toggle-switch-label"
              ></label>
            </div>
          </div>
        </div>
      </div>
      </transition>
      
      <!-- Custom Tab Content -->
      <transition name="tab-transition" mode="out-in">
      <div v-if="activeTab === 'custom'" key="custom" class="form-content">
        <div class="space-y-3">
          <!-- Custom Moodle Repository -->
          <div class="config-section">
            <label for="customRepo" class="modern-form-label">
              <div class="label-icon">📂</div>
              Custom Moodle Repository
            </label>
            <div class="flex space-x-2">
              <input 
                type="text" 
                id="customRepo" 
                v-model="customRepoUrl" 
                class="modern-input flex-1" 
                placeholder="https://github.com/yourusername/moodle.git" 
              />
              <button 
                type="button"
                @click="resetToDefaultRepo"
                class="action-button secondary-button"
              >
                Reset
              </button>
            </div>
            <p class="info-message">
              Leave empty to use the official Moodle repository
            </p>
          </div>
          
          <!-- Custom Branch -->
          <div class="config-section">
            <label for="customBranch" class="modern-form-label">
              <div class="label-icon">🔀</div>
              Custom Branch
            </label>
            <div class="flex space-x-2">
              <input 
                type="text" 
                id="customBranch" 
                v-model="customBranch" 
                class="modern-input flex-1" 
                placeholder="main" 
              />
              <button 
                type="button"
                @click="useSelectedStableBranch"
                class="action-button secondary-button"
              >
                Use Stable
              </button>
            </div>
            <p class="info-message">
              Custom branch to checkout (overrides Moodle version selected in Standard tab)
            </p>
          </div>
          
          <!-- Database Prefix -->
          <div class="config-section">
            <label for="dbPrefix" class="modern-form-label">
              <div class="label-icon">🔤</div>
              Database Table Prefix
            </label>
            <input 
              type="text" 
              id="dbPrefix" 
              v-model="dbPrefix" 
              class="modern-input w-full" 
              placeholder="mdl_" 
            />
            <p class="info-message">
              Default: mdl_
            </p>
          </div>
          
          <!-- Custom Site Config -->
          <div class="config-section">
            <h3 class="section-title">
              <div class="section-icon">🛠️</div>
              Site Configuration
            </h3>
            <div class="modern-checkbox-grid">
              <div class="modern-checkbox-item">
                <input
                  type="checkbox"
                  id="siteDebug"
                  v-model="siteConfig.debug"
                  class="modern-checkbox"
                />
                <label for="siteDebug" class="modern-checkbox-label">Debug Mode</label>
              </div>
              <div class="modern-checkbox-item">
                <input
                  type="checkbox"
                  id="siteDevLib"
                  v-model="siteConfig.devLibs"
                  class="modern-checkbox"
                />
                <label for="siteDevLib" class="modern-checkbox-label">Development Libraries</label>
              </div>
              <div class="modern-checkbox-item">
                <input
                  type="checkbox"
                  id="siteCaching"
                  v-model="siteConfig.disableCaching"
                  class="modern-checkbox"
                />
                <label for="siteCaching" class="modern-checkbox-label">Disable Caching</label>
              </div>
              <div class="modern-checkbox-item">
                <input
                  type="checkbox"
                  id="siteThemeCaching"
                  v-model="siteConfig.disableThemeCaching"
                  class="modern-checkbox"
                />
                <label for="siteThemeCaching" class="modern-checkbox-label">Disable Theme Caching</label>
              </div>
            </div>
          </div>
          
          <!-- Admin Account -->
          <div class="config-section">
            <h3 class="section-title">
              <div class="section-icon">👤</div>
              Admin Account
            </h3>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label for="adminUsername" class="modern-form-label">Username</label>
                <input 
                  type="text" 
                  id="adminUsername" 
                  v-model="adminAccount.username" 
                  class="modern-input" 
                  placeholder="admin" 
                />
              </div>
              <div>
                <label for="adminPassword" class="modern-form-label">Password</label>
                <input 
                  type="text" 
                  id="adminPassword" 
                  v-model="adminAccount.password" 
                  class="modern-input" 
                  placeholder="admin" 
                />
              </div>
              <div>
                <label for="adminEmail" class="modern-form-label">Email</label>
                <input 
                  type="email" 
                  id="adminEmail" 
                  v-model="adminAccount.email" 
                  class="modern-input" 
                  placeholder="admin@example.com" 
                />
              </div>
              <div>
                <label for="adminFullname" class="modern-form-label">Full Name</label>
                <input 
                  type="text" 
                  id="adminFullname" 
                  v-model="adminAccount.fullname" 
                  class="modern-input" 
                  placeholder="Admin User" 
                />
              </div>
            </div>
            <p class="info-message">
              Default credentials will be used if left empty
            </p>
          </div>
        </div>
      </div>
      </transition>
      
      <!-- Advanced Tab Content -->
      <transition name="tab-transition" mode="out-in">
      <div v-if="activeTab === 'advanced'" key="advanced" class="form-content">
        <div class="space-y-3">
          <!-- Docker Configuration -->
          <div class="config-section">
            <h3 class="section-title">
              <div class="section-icon">🐳</div>
              Docker Configuration
            </h3>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label for="dockerNetwork" class="modern-form-label">Docker Network</label>
                <input 
                  type="text" 
                  id="dockerNetwork" 
                  v-model="dockerConfig.network" 
                  class="modern-input" 
                  placeholder="moodle-network" 
                />
              </div>
              <div>
                <label for="dockerPrefix" class="modern-form-label">Container Prefix</label>
                <input 
                  type="text" 
                  id="dockerPrefix" 
                  v-model="dockerConfig.containerPrefix" 
                  class="modern-input" 
                  placeholder="moodle-" 
                />
              </div>
            </div>
            <div class="mt-3">
              <label for="dockerComposePath" class="modern-form-label">Docker Compose Path</label>
              <div class="flex space-x-2">
                <input 
                  type="text" 
                  id="dockerComposePath" 
                  v-model="dockerConfig.composePath" 
                  class="modern-input flex-1" 
                  placeholder="/usr/local/bin/docker-compose" 
                />
                <button 
                  type="button"
                  @click="detectDockerComposePath"
                  class="action-button secondary-button"
                >
                  Detect
                </button>
              </div>
            </div>
          </div>

          <!-- PHP Configuration -->
          <div class="config-section">
            <h3 class="section-title">
              <div class="section-icon">❯</div>
              PHP Configuration
            </h3>
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label for="phpVersion" class="modern-form-label">PHP Version</label>
                <select id="phpVersion" v-model="phpVersion" class="modern-select">
                  <option v-for="version in phpVersions" :key="version" :value="version">{{ version }}</option>
                </select>
              </div>
              <div>
                <label for="phpMemoryLimit" class="modern-form-label">Memory Limit</label>
                <input 
                  type="text" 
                  id="phpMemoryLimit" 
                  v-model="phpConfig.memoryLimit" 
                  class="modern-input" 
                  placeholder="512M" 
                />
              </div>
            </div>
            <div class="mt-3">
              <label for="phpModules" class="modern-form-label">Additional PHP Modules</label>
              <input 
                type="text" 
                id="phpModules" 
                v-model="phpConfig.additionalModules" 
                class="modern-input w-full" 
                placeholder="xdebug, opcache, etc." 
              />
              <p class="info-message">Comma-separated list of modules to install</p>
            </div>
            <div class="toggle-container mt-3">
              <div class="toggle-content">
                <div class="toggle-icon">🐞</div>
                <div class="toggle-info">
                  <h4 class="toggle-title">Enable Xdebug</h4>
                  <p class="toggle-description">Adds PHP debugging capabilities</p>
                </div>
              </div>
              <div class="toggle-switch">
                <input 
                  type="checkbox"
                  id="enableXdebug" 
                  v-model="phpConfig.xdebug"
                  class="sr-only peer"
                />
                <label 
                  for="enableXdebug"
                  class="toggle-switch-label"
                ></label>
              </div>
            </div>
          </div>

          <!-- Volume Mounts -->
          <div class="config-section">
            <div class="section-header">
              <h3 class="section-title">
                <div class="section-icon">📁</div>
                Additional Volume Mounts
              </h3>
              <button 
                type="button"
                @click="addVolumeMount"
                class="add-item-button"
              >
                <span>+</span> Add Mount
              </button>
            </div>
            <div v-for="(mount, index) in volumeMounts" :key="index" class="mount-row">
              <input 
                type="text" 
                v-model="mount.source" 
                class="modern-input flex-1" 
                placeholder="Local path" 
              />
              <span class="mount-separator">:</span>
              <input 
                type="text" 
                v-model="mount.target" 
                class="modern-input flex-1" 
                placeholder="Container path" 
              />
              <button 
                type="button"
                @click="removeVolumeMount(index)"
                class="remove-item-button"
              >
                <span>✕</span>
              </button>
            </div>
            <p class="info-message">
              Add custom volume mounts for development plugins, themes, etc.
            </p>
          </div>

          <!-- Environment Variables -->
          <div class="config-section">
            <div class="section-header">
              <h3 class="section-title">
                <div class="section-icon">🔧</div>
                Environment Variables
              </h3>
              <button 
                type="button"
                @click="addEnvironmentVariable"
                class="add-item-button"
              >
                <span>+</span> Add Variable
              </button>
            </div>
            <div v-for="(envVar, index) in environmentVariables" :key="index" class="env-var-row">
              <input 
                type="text" 
                v-model="envVar.key" 
                class="modern-input flex-1" 
                placeholder="VARIABLE_NAME" 
              />
              <span class="env-var-separator">=</span>
              <input 
                type="text" 
                v-model="envVar.value" 
                class="modern-input flex-1" 
                placeholder="value" 
              />
              <button 
                type="button"
                @click="removeEnvironmentVariable(index)"
                class="remove-item-button"
              >
                <span>✕</span>
              </button>
            </div>
            <p class="info-message">
              Add custom environment variables for containers
            </p>
          </div>
        </div>
      </div>
      </transition>
      
      <!-- Docker Status - more compact - commented out since it's already in the header -->
      <!--
      <div class="flex items-center justify-between text-xs px-2 py-1 bg-gray-50 dark:bg-gray-700 rounded-md mb-2">
        <div class="text-gray-700 dark:text-gray-200 text-xs">Docker is installed and running</div>
        <div class="flex items-center">
          <div class="w-1.5 h-1.5 rounded-full bg-green-500 mr-1"></div>
          <span class="text-green-600 dark:text-green-400 text-xs">Active</span>
        </div>
      </div>
      -->
      
      <!-- Form Errors -->
      <div v-if="formErrors.length > 0" class="bg-red-50 border border-red-200 rounded-md p-3 mb-4">
        <h3 class="text-sm font-medium text-red-800 mb-1">Please fix the following issues:</h3>
        <ul class="text-xs text-red-700 list-disc pl-5">
          <li v-for="(error, index) in formErrors" :key="index" class="mb-1">{{ error }}</li>
        </ul>
      </div>
      
      <!-- Action Buttons - modern style -->
      <div class="action-buttons">
        <button 
          type="button" 
          class="cancel-button" 
          @click="$router.push('/')"
        >
          Cancel
        </button>
        <button 
          type="submit" 
          class="create-button"
          :class="{ 'disabled-button': !formIsValid || formErrors.length > 0 || createSiteInProgress }"
          :disabled="!formIsValid || formErrors.length > 0 || createSiteInProgress"
        >
          <span v-if="!createSiteInProgress">Create Site</span>
          <span v-else class="loading-text">
            <svg class="loading-spinner" viewBox="0 0 24 24">
              <circle class="spinner-track" cx="12" cy="12" r="10"></circle>
              <circle class="spinner-circle" cx="12" cy="12" r="10"></circle>
            </svg>
            Starting creation...
          </span>
        </button>
      </div>
    </form>
    
    <!-- Logs section hidden for now, may be deleted later -->
    <div v-if="false && logs.length" class="mt-6 border-t border-gray-200 pt-4">
      <button 
        type="button"
        @click="logsExpanded = !logsExpanded"
        class="w-full flex items-center justify-between py-2 px-3 rounded bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 transition-colors duration-150"
      >
        <div class="flex items-center">
          <h3 class="text-sm font-medium text-gray-700 dark:text-gray-100">Logs</h3>
          <span class="ml-2 text-xs text-gray-500 dark:text-gray-400">({{ logs.length }} entries)</span>
          <span v-if="logs.length > 0 && !logsExpanded" class="ml-2 inline-block bg-blue-100 text-blue-800 text-xs px-2 py-0.5 rounded-full">
            {{ logs[logs.length-1].substring(0, 30) }}{{ logs[logs.length-1].length > 30 ? '...' : '' }}
          </span>
        </div>
        <div class="flex items-center space-x-3">
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
          class="bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md p-3 font-mono text-xs dark:text-gray-200 transition-all duration-300 overflow-auto"
          style="max-height: 200px;"
        >
          <div v-for="(log, i) in logs" :key="i" class="pb-1 whitespace-pre-wrap break-words">{{ log }}</div>
        </div>
      </transition>
    </div>
      </div>
    </div>
    <SettingsModal ref="settingsModal"></SettingsModal>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { useSettingsStore } from '../stores/settings';
import { computed, watch } from 'vue';
import { PhSquare, PhList, PhGear, PhArrowClockwise } from '@phosphor-icons/vue';
import { eventBus } from '../utils/event-bus';
import { appWindow } from '@tauri-apps/api/window';
import DockerStatus from './DockerStatus.vue';
import SettingsModal from './SettingsModal.vue';

export default {
  components: {
    PhSquare,
    DockerStatus,
    SettingsModal,
    PhList,
    PhGear,
    PhArrowClockwise
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
      dockerInstalled: false,
      dockerRunning: false,
      phpVersions: ['8.4', '8.3', '8.2', '8.1', '8.0', '7.4', '7.3', '7.2', '7.1', '7.0', '5.6'],
      dbTypes: ['MySQL', 'MariaDB', 'PostgreSQL', 'Microsoft SQL Server', 'Oracle'],
      moodleBranches: ['MOODLE_500_STABLE', 'MOODLE_405_STABLE', 'MOODLE_404_STABLE', 'MOODLE_403_STABLE', 'MOODLE_402_STABLE', 'MOODLE_401_STABLE'],
      logsExpanded: false, // Controls the visibility of the logs section
      logs: [],
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
      // Port availability cache
      portAvailability: {},
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
      // Check if the site name is valid: only lowercase letters, numbers, underscores, and hyphens
      return this.siteName.length === 0 || /^[a-z0-9_-]+$/.test(this.siteName);
    },
    
    formIsValid() {
      // Basic required fields validation
      if (!this.siteName || !this.siteName.trim()) {
        return false;
      }
      
      // Directory must be selected or using default directory
      // Enhance validation to be very strict about directory selection
      if (!this.useDefaultDirectory) {
        if (!this.siteDirectory || !this.siteDirectory.trim()) {
          console.log("Form validation failed: Custom directory not selected");
          return false;
        }
      }
      
      // If using default directory, make sure it's available
      if (this.useDefaultDirectory && !this.defaultDirectory) {
        console.log("Form validation failed: Default directory not available");
        return false;
      }
      
      // Additional validation for the active tab
      if (this.activeTab === 'standard' && !this.moodleBranch) {
        return false;
      }
      
      // If in custom tab with custom repo, must have a branch
      if (this.activeTab === 'custom' && this.customRepoUrl && !this.customBranch) {
        return false;
      }
      
      return true;
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
    
    // Check port availability if ports are already set
    if (this.webPort) {
      this.checkPortAvailability(this.webPort);
    }
    if (this.dbPort) {
      this.checkPortAvailability(this.dbPort);
    }
  },
  watch: {
    // Watch for changes to db port
    dbPort: {
      handler(newVal) {
        this.onDbPortChanged(newVal);
      }
    },
    
    // Watch for changes to web port
    webPort: {
      handler(newVal) {
        this.onWebPortChanged(newVal);
      }
    },
    
    // When DB type changes, clear port cache and check again if needed
    dbType() {
      // Clear the port availability cache when changing db type
      this.portAvailability = {};
      
      if (this.dbPort) {
        // If a port is already set, check its availability with new DB type
        this.onDbPortChanged(this.dbPort);
      }
    }
  },
  methods: {
    // Window control methods
    openSettings() {
      const modal = this.$refs.settingsModal;
      if (modal && typeof modal.openModal === 'function') {
        modal.openModal();
      }
    },
    goBack() {
      this.$router.go(-1);
    },
    goForward() {
      this.$router.go(1);
    },
    reloadPage() {
      window.location.reload();
    },
    minimizeWindow() {
      appWindow.minimize();
    },
    async toggleMaximizeWindow() {
      const isMaximized = await appWindow.isMaximized();
      if (isMaximized) {
        appWindow.unmaximize();
      } else {
        appWindow.maximize();
      }
    },
    closeWindow() {
      appWindow.close();
    },
    // Other existing methods
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
    
    async generateWebPort() {
      // Generate a random port within the web port range
      const min = this.portRanges.web.min;
      const max = this.portRanges.web.max;
      
      // First try to find an available port
      const availablePort = await this.findAvailableWebPort();
      if (availablePort) {
        this.webPort = availablePort;
        return;
      }
      
      // Fallback to random port if finding available port failed
      const randomPort = Math.floor(Math.random() * (max - min + 1)) + min;
      this.webPort = randomPort;
      
      // Check and cache its availability
      this.checkPortAvailability(randomPort);
    },
    
    async generateDbPort() {
      // Generate a random port within the database-specific range
      const dbRange = this.portRanges.db[this.dbType] || { min: 3306, max: 3399 };
      const min = dbRange.min;
      const max = dbRange.max;
      
      // First try to find an available port
      const availablePort = await this.findAvailableDbPort();
      if (availablePort) {
        this.dbPort = availablePort;
        return;
      }
      
      // Fallback to random port if finding available port failed
      const randomPort = Math.floor(Math.random() * (max - min + 1)) + min;
      this.dbPort = randomPort;
      
      // Check and cache its availability
      this.checkPortAvailability(randomPort);
    },
    
    // New functions for finding available ports
    async findAvailableWebPort() {
      const min = this.portRanges.web.min;
      const max = this.portRanges.web.max;
      
      // Try sequential ports first for efficiency
      for (let port = min; port <= max; port++) {
        if (await this.isPortAvailable(port)) {
          return port;
        }
      }
      
      // If no available port found, return null
      return null;
    },
    
    async findAvailableDbPort() {
      const dbRange = this.portRanges.db[this.dbType] || { min: 3306, max: 3399 };
      const min = dbRange.min;
      const max = dbRange.max;
      
      // Try sequential ports first for efficiency
      for (let port = min; port <= max; port++) {
        if (await this.isPortAvailable(port)) {
          return port;
        }
      }
      
      // If no available port found, return null
      return null;
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
        button.innerHTML = '<svg class="w-3 h-3 mr-0.5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 01-1.414-1.414L7.586 10l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" /></svg>Copied!';
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
        
        this.dockerInstalled = dockerInstalled;
        this.dockerRunning = dockerRunning;
        
        if (!dockerInstalled || !dockerRunning) {
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
      // Start the creation process UI
      this.createSiteInProgress = true;
      this.addLogMessage(`Starting creation of site: ${this.siteName}...`);
      
      if (!this.dockerInstalled) {
        this.addLogMessage('Cannot create site: Docker is not installed or not running.');
        this.createSiteInProgress = false;
        return;
      }

      if (!this.siteName) {
        this.addLogMessage('Please enter a site name');
        this.createSiteInProgress = false;
        return;
      }

      if (!this.siteDirectory && !this.useDefaultDirectory) {
        this.addLogMessage('Please select a directory for the site');
        this.createSiteInProgress = false;
        return;
      }
      
      // Enhanced directory validation
      let installDir = "";
      
      if (this.useDefaultDirectory) {
        if (!this.defaultDirectory) {
          this.addLogMessage('Default directory is not available. Please select a custom directory.');
          this.createSiteInProgress = false;
          return;
        }
        installDir = this.defaultDirectory;
      } else {
        if (!this.siteDirectory || !this.siteDirectory.trim()) {
          this.addLogMessage('Please select a valid installation directory');
          this.createSiteInProgress = false;
          return;
        }
        installDir = this.siteDirectory;
      }
      
      // Validate directory doesn't contain problematic paths
      if (installDir.includes("src-tauri") || installDir.includes("node_modules")) {
        this.addLogMessage('The selected directory appears to be inside the application source. Please choose a different location.');
        this.createSiteInProgress = false;
        return;
      }
      
      this.addLogMessage(`Using installation directory: ${installDir}`);

      if (!this.moodleBranch && this.activeTab === 'standard') {
        this.addLogMessage('Please select a Moodle version');
        this.createSiteInProgress = false;
        return;
      }

      // Validate ports if specified
      if (this.webPort && !this.isWebPortInRange) {
        this.addLogMessage(`Web port ${this.webPort} is outside the recommended range (${this.currentWebPortRange})`);
        this.createSiteInProgress = false;
        return;
      }
      
      if (this.dbPort && !this.isDbPortInRange) {
        this.addLogMessage(`DB port ${this.dbPort} is outside the recommended range for ${this.dbType} (${this.currentDbPortRange})`);
        this.createSiteInProgress = false;
        return;
      }

      try {
        // First validate the environment
        this.addLogMessage('Validating environment...');
        await invoke('validate_environment');
        
        // Find available ports if none are specified
        let webPort = this.webPort;
        let dbPort = this.dbPort;
        
        if (!webPort) {
          this.addLogMessage('Finding available web port...');
          webPort = await this.findAvailableWebPort();
          if (!webPort) {
            this.addLogMessage('Failed to find available web port in the range ' + this.currentWebPortRange);
            this.createSiteInProgress = false;
            return;
          }
          this.addLogMessage(`Found available web port: ${webPort}`);
        }
        
        if (!dbPort) {
          this.addLogMessage('Finding available database port...');
          dbPort = await this.findAvailableDbPort();
          if (!dbPort) {
            this.addLogMessage('Failed to find available database port in the range ' + this.currentDbPortRange);
            this.createSiteInProgress = false;
            return;
          }
          this.addLogMessage(`Found available ${this.dbType} database port: ${dbPort}`);
        }
        
        // Compile the settings from all tabs
        const siteConfig = {
          name: this.siteName,
          directory: this.useDefaultDirectory ? this.defaultDirectory : this.siteDirectory,
          webPort: webPort,
          dbPort: dbPort,
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
        
        this.addLogMessage('Site configuration prepared, starting creation process...');
        
        // Emit a custom event to show the creation modal
        const params = {
          siteName: this.siteName,
          phpVersion: siteConfig.phpVersion,
          dbType: siteConfig.dbType.toLowerCase(),
          moodleBranch: siteConfig.moodleBranch || siteConfig.customBranch || 'MOODLE_400_STABLE',
          webPort: siteConfig.webPort,
          dbPort: siteConfig.dbPort,
          directory: siteConfig.directory,
        };
        
        // Show the site creation modal immediately and do not navigate away
        // This allows the user to see the creation process from the start
        eventBus.emitShowSiteCreation(params);
        
        // Do not navigate away automatically
        // The user will be able to return to the dashboard using the buttons in the modal
        // This ensures the modal stays visible through the entire process
        
        // Don't reset the form here - let the user return to it if they cancel
      } catch (error) {
        this.addLogMessage(`Error creating site: ${error}`);
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
      this.customBranch = this.moodleBranch;
    },

    // Advanced tab methods
    addVolumeMount() {
      this.volumeMounts.push({
        source: '',
        target: ''
      });
    },
    
    removeVolumeMount(index) {
      this.volumeMounts.splice(index, 1);
      
      // Always ensure there's at least one empty mount for better UX
      if (this.volumeMounts.length === 0) {
        this.addVolumeMount();
      }
    },
    
    addEnvironmentVariable() {
      this.environmentVariables.push({
        key: '',
        value: ''
      });
    },
    
    removeEnvironmentVariable(index) {
      this.environmentVariables.splice(index, 1);
      
      // Always ensure there's at least one empty env var for better UX
      if (this.environmentVariables.length === 0) {
        this.addEnvironmentVariable();
      }
    },
    
    async detectDockerComposePath() {
      try {
        // Show loading state
        this.dockerConfig.composePath = 'Detecting...';
        
        // Call backend to detect path (using Tauri invoke)
        const composePath = await invoke('detect_docker_compose_path');
        
        if (composePath) {
          this.dockerConfig.composePath = composePath;
        } else {
          this.dockerConfig.composePath = '/usr/local/bin/docker-compose';
          this.addLogMessage('⚠️ Could not auto-detect Docker Compose path, using default');
        }
      } catch (error) {
        console.error('Error detecting Docker Compose path:', error);
        this.dockerConfig.composePath = '/usr/local/bin/docker-compose';
        this.addLogMessage(`⚠️ Error detecting Docker Compose path: ${error.message || error}`);
      }
    },

    // Validate a Git URL
    isValidGitUrl(url) {
      if (!url) return true; // Empty is valid (will use default)
      
      // Basic validation for Git URLs
      const gitUrlPattern = /^(https?:\/\/|git@)([^\s:]+)(:|\/)([\w.-]+)\/([\w.-]+)(\.git)?$/;
      return gitUrlPattern.test(url);
    },

    // Check if a port is available using the Tauri function
    async isPortAvailable(port) {
      if (!port) return false;
      try {
        return await invoke('check_port_available', { port: parseInt(port) });
      } catch (error) {
        console.error('Error checking port availability:', error);
        return false;
      }
    },

    // Method to check and cache port availability
    async checkPortAvailability(port) {
      if (!port) return;
      
      // Convert to number to ensure correct comparison
      const portNumber = parseInt(port);
      
      try {
        // Call the Rust function to check if port is available
        const isAvailable = await invoke('check_port_available', { port: portNumber });
        
        // Cache the result
        this.portAvailability = {
          ...this.portAvailability,
          [portNumber]: isAvailable
        };
        
        return isAvailable;
      } catch (error) {
        console.error('Error checking port availability:', error);
        // Cache the error result
        this.portAvailability = {
          ...this.portAvailability,
          [portNumber]: false
        };
        return false;
      }
    },
    
    // When db port changes, check its availability
    async onDbPortChanged(newPort) {
      if (newPort && this.isDbPortInRange) {
        await this.checkPortAvailability(newPort);
      }
    },
    
    // When web port changes, check its availability
    async onWebPortChanged(newPort) {
      if (newPort && this.isWebPortInRange) {
        await this.checkPortAvailability(newPort);
      }
    },

    // When db type changes, update the db port placeholder and clear the value if out of range
    updateDbType() {
      if (this.dbPort) {
        const port = parseInt(this.dbPort);
        const dbRange = this.portRanges.db[this.dbType] || { min: 0, max: 0 };
        if (port < dbRange.min || port > dbRange.max) {
          // Clear the port if it's out of the new range
          this.dbPort = '';
        } else {
          // If port is still in range, check availability
          this.checkPortAvailability(port);
        }
      }
      
      // Check platform compatibility with the selected database type
      this.checkPlatformCompatibility();
    },
  }
};
</script>

<style scoped>
/* Common Styles */
.dashboard-full {
  display: flex;
  flex-direction: column;
}

.hide-scrollbar {
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;
}

/* App Header */
.app-header {
  background: rgba(17, 24, 39, 0.9);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding: 0.75rem 1rem;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 10;
  transition: all 0.3s ease;
  height: 56px;
  display: flex;
  align-items: center;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  margin: 0 auto;
  padding: 0 0.5rem;
}

.main-content {
  padding-top: 72px;
  flex-grow: 1;
  overflow-y: auto;
}

/* Window Controls */
.titlebar-left-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.window-controls {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-right: 12px;
}

.window-control-button {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  border: none;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: default;
}

.navigation-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.header-nav-button {
  background: rgba(33, 37, 50, 0.95);
  border: none;
  padding: 6px;
  margin: 0 2px;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  color: #FFFFFF;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s ease;
}

.header-nav-button:hover {
  background: rgba(50, 56, 74, 0.95);
}

.chevron {
  font-size: 16px;
  font-weight: bold;
}

/* App Branding */
.app-branding {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-left: 12px;
}

.logo-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background-color: #6366F1;
  border-radius: 6px;
}

.logo-icon {
  width: 18px;
  height: 18px;
}

.title-container {
  display: flex;
  flex-direction: column;
}

.app-title {
  font-size: 1rem;
  font-weight: 600;
  margin: 0;
  letter-spacing: -0.025em;
  color: #FFFFFF;
}

.button-icon {
  width: 20px;
  height: 20px;
}

/* Modern Settings Button */
.modern-settings-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background-color: rgba(31, 41, 55, 0.5);
  color: #F9FAFB;
  border: none;
  transition: all 0.2s ease;
}

.modern-settings-button:hover {
  background-color: rgba(55, 65, 81, 0.7);
  transform: rotate(30deg);
}

/* Create Site Container */
.create-site-container {
  background-color: #111827;
  background-image: radial-gradient(circle at top right, rgba(79, 70, 229, 0.1), transparent 70%),
                    radial-gradient(circle at bottom left, rgba(59, 130, 246, 0.1), transparent 70%);
  border-radius: 20px;
  padding: 28px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.25);
  border: 1px solid rgba(255, 255, 255, 0.07);
}

.site-header {
  margin-bottom: 20px;
}

.icon-container {
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Modern Tabs */
.modern-tabs-container {
  display: flex;
  background-color: rgba(31, 41, 55, 0.3);
  border-radius: 14px;
  padding: 4px;
  margin-bottom: 28px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1), inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.modern-tab-button {
  flex: 1;
  padding: 12px 16px;
  border: none;
  background: transparent;
  color: #9CA3AF;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.25s ease;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.tab-icon {
  font-size: 16px;
  opacity: 0.8;
  transition: all 0.25s ease;
}

.modern-tab-button:hover:not(.active-tab) {
  color: #E5E7EB;
  background-color: rgba(55, 65, 81, 0.3);
}

.modern-tab-button:hover:not(.active-tab) .tab-icon {
  opacity: 1;
  transform: scale(1.1);
}

.active-tab {
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.9), rgba(79, 70, 229, 0.9));
  color: white;
  font-weight: 600;
  box-shadow: 0 2px 8px rgba(79, 70, 229, 0.35);
}

.active-tab .tab-icon {
  opacity: 1;
  transform: scale(1.1);
}

/* Form Content */
.form-content {
  padding: 4px;
}

.config-section {
  background: linear-gradient(to bottom, rgba(31, 41, 55, 0.5), rgba(31, 41, 55, 0.4));
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 14px;
  padding: 18px;
  margin-bottom: 20px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.config-section:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.modern-form-label {
  display: flex;
  align-items: center;
  color: #E5E7EB;
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
}

.label-icon {
  margin-right: 8px;
  font-size: 14px;
  opacity: 0.9;
}

.section-title {
  display: flex;
  align-items: center;
  color: #F3F4F6;
  font-size: 15px;
  font-weight: 600;
  margin-bottom: 12px;
}

.section-icon {
  margin-right: 10px;
  font-size: 16px;
  opacity: 0.9;
}

/* Modern Form Inputs */
.modern-input {
  width: 100%;
  padding: 10px 14px;
  font-size: 14px;
  background-color: rgba(17, 24, 39, 0.7);
  border: 1px solid rgba(75, 85, 99, 0.5);
  border-radius: 10px;
  color: #F9FAFB;
  transition: all 0.25s ease;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

.modern-input:focus {
  border-color: rgba(99, 102, 241, 0.8);
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.25), inset 0 2px 4px rgba(0, 0, 0, 0.05);
  outline: none;
  background-color: rgba(17, 24, 39, 0.6);
}

.modern-input:hover:not(:focus) {
  border-color: rgba(75, 85, 99, 0.8);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.15);
}

.modern-select {
  width: 100%;
  padding: 8px 12px;
  font-size: 14px;
  background-color: rgba(17, 24, 39, 0.7);
  border: 1px solid rgba(75, 85, 99, 0.5);
  border-radius: 8px;
  color: #F9FAFB;
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='white' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 10px center;
  background-size: 16px;
  padding-right: 30px;
  transition: all 0.2s ease;
}

.modern-select:focus {
  border-color: rgba(99, 102, 241, 0.8);
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.25);
  outline: none;
}

.modern-checkbox-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.modern-checkbox-item {
  display: flex;
  align-items: center;
}

.modern-checkbox {
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 4px;
  background-color: rgba(31, 41, 55, 0.7);
  border: 1px solid rgba(107, 114, 128, 0.6);
  position: relative;
  cursor: pointer;
  margin-right: 8px;
}

.modern-checkbox:checked {
  background-color: #6366F1;
  border-color: #6366F1;
}

.modern-checkbox:checked::after {
  content: '';
  position: absolute;
  left: 6px;
  top: 2px;
  width: 5px;
  height: 10px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.modern-checkbox-label {
  color: #E5E7EB;
  font-size: 14px;
  cursor: pointer;
}

/* Message Styles */
.info-message {
  color: #9CA3AF;
  font-size: 12px;
  margin-top: 5px;
}

.error-message {
  color: #F87171;
  font-size: 12px;
  margin-top: 5px;
}

.warning-message {
  color: #FBBF24;
  font-size: 12px;
  margin-top: 5px;
}

.recommendation-message {
  color: #34D399;
  font-size: 12px;
  margin-top: 5px;
}

.highlight {
  color: #6366F1;
  font-weight: 500;
}

/* Toggle Switch */
.toggle-section {
  padding: 8px 12px;
}

.toggle-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: rgba(31, 41, 55, 0.4);
  border-radius: 12px;
}

.toggle-content {
  display: flex;
  align-items: center;
}

.toggle-icon {
  font-size: 18px;
  margin-right: 12px;
}

.toggle-info {
  display: flex;
  flex-direction: column;
}

.toggle-title {
  color: #F3F4F6;
  font-size: 14px;
  font-weight: 500;
}

.toggle-description {
  color: #9CA3AF;
  font-size: 12px;
}

.toggle-switch {
  position: relative;
  min-width: 42px;
  height: 22px;
}

.toggle-switch-label {
  position: absolute;
  width: 42px;
  height: 22px;
  background-color: rgba(31, 41, 55, 0.8);
  border-radius: 50px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.toggle-switch-label::after {
  content: "";
  position: absolute;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background-color: #9CA3AF;
  top: 2px;
  left: 2px;
  transition: all 0.3s ease;
}

input:checked + .toggle-switch-label {
  background-color: #6366F1;
}

input:checked + .toggle-switch-label::after {
  left: 22px;
  background-color: white;
}

/* Volume Mounts and Environment Variables */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.add-item-button {
  display: flex;
  align-items: center;
  gap: 5px;
  background-color: rgba(79, 70, 229, 0.2);
  border: 1px solid rgba(79, 70, 229, 0.5);
  color: #C7D2FE;
  font-size: 13px;
  padding: 5px 10px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.add-item-button:hover {
  background-color: rgba(79, 70, 229, 0.3);
  border-color: rgba(79, 70, 229, 0.7);
}

.add-item-button span {
  font-size: 14px;
  font-weight: bold;
}

.mount-row, .env-var-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.mount-separator, .env-var-separator {
  color: #9CA3AF;
  font-size: 16px;
  font-weight: bold;
}

.remove-item-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  background-color: rgba(220, 38, 38, 0.2);
  border: 1px solid rgba(220, 38, 38, 0.5);
  color: #FCA5A5;
  transition: all 0.2s ease;
}

.remove-item-button:hover {
  background-color: rgba(220, 38, 38, 0.3);
  border-color: rgba(220, 38, 38, 0.7);
}

/* Server Configuration Styles */
.server-config {
  padding-bottom: 12px;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.subsection-title {
  font-size: 14px;
  font-weight: 500;
  color: #E5E7EB;
}

.info-tooltip {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background-color: rgba(75, 85, 99, 0.4);
  color: #F3F4F6;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-style: italic;
  cursor: help;
}

.port-config-row {
  display: flex;
  gap: 8px;
  margin-bottom: 5px;
}

.db-type-container {
  width: 140px;
}

.port-input-container {
  position: relative;
  flex: 1;
  display: flex;
}

.port-input {
  width: 100%;
  padding-right: 40px;
}

.generate-button {
  position: absolute;
  right: 4px;
  top: 50%;
  transform: translateY(-50%);
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #9CA3AF;
  font-size: 16px;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.generate-button:hover {
  color: #F9FAFB;
  background-color: rgba(75, 85, 99, 0.3);
}

.port-status {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  margin-top: 4px;
}

.status-available {
  color: #34D399;
}

.status-error {
  color: #F87171;
}

.status-warning {
  color: #FBBF24;
}

.status-checking {
  color: #60A5FA;
}

.db-type-label {
  color: #9CA3AF;
}

/* Directory Configuration */
.directory-config {
  padding-bottom: 12px;
}

.directory-input-row {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
}

.directory-input {
  background-color: rgba(17, 24, 39, 0.4);
  border-color: rgba(75, 85, 99, 0.3);
  color: #9CA3AF;
}

.browse-button {
  padding: 0 12px;
  background-color: rgba(75, 85, 99, 0.4);
  border: 1px solid rgba(75, 85, 99, 0.5);
  color: #F3F4F6;
  border-radius: 6px;
  font-size: 13px;
  transition: all 0.2s ease;
}

.browse-button:hover {
  background-color: rgba(75, 85, 99, 0.6);
}

.directory-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.directory-path {
  font-size: 12px;
  color: #9CA3AF;
}

.path-value {
  color: #C7D2FE;
  font-family: monospace;
}

.reset-button {
  font-size: 12px;
  color: #93C5FD;
  background: transparent;
  border: none;
  text-decoration: underline;
  cursor: pointer;
}

.reset-button:hover {
  color: #60A5FA;
}

/* Action Buttons */
.action-buttons {
  display: flex;
  gap: 12px;
  margin-top: 24px;
}

.cancel-button {
  padding: 10px 20px;
  background-color: rgba(75, 85, 99, 0.4);
  border: 1px solid rgba(75, 85, 99, 0.6);
  color: #F3F4F6;
  border-radius: 8px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.cancel-button:hover {
  background-color: rgba(75, 85, 99, 0.6);
}

.create-button {
  flex: 1;
  padding: 10px 24px;
  background-color: #6366F1;
  border: none;
  color: white;
  border-radius: 8px;
  font-weight: 600;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
}

.create-button:hover {
  background-color: #4F46E5;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.4);
}

.secondary-button {
  padding: 8px 12px;
  background-color: rgba(75, 85, 99, 0.4);
  border: 1px solid rgba(75, 85, 99, 0.5);
  color: #F3F4F6;
  border-radius: 6px;
  font-size: 13px;
  transition: all 0.2s ease;
}

.secondary-button:hover {
  background-color: rgba(75, 85, 99, 0.6);
}

.action-button {
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 13px;
  transition: all 0.2s ease;
}

/* Form Validation */
.error-input {
  border-color: rgba(239, 68, 68, 0.7);
  box-shadow: 0 0 0 1px rgba(239, 68, 68, 0.3);
}

.success-input {
  border-color: rgba(16, 185, 129, 0.7);
  box-shadow: 0 0 0 1px rgba(16, 185, 129, 0.3);
}

/* Animation for collapsed elements */
.collapse-enter-active,
.collapse-leave-active {
  transition: height 0.3s ease-out;
  overflow: hidden;
}

.collapse-enter-from,
.collapse-leave-to {
  height: 0;
}

/* Loading Spinner */
.disabled-button {
  opacity: 0.5;
  cursor: not-allowed;
}

.disabled-button:hover {
  background-color: #6366F1;
  transform: none;
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
}

.loading-text {
  display: flex;
  align-items: center;
  gap: 8px;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  animation: spin 1s linear infinite;
}

.spinner-track {
  fill: none;
  stroke: rgba(255, 255, 255, 0.3);
  stroke-width: 2;
}

.spinner-circle {
  fill: none;
  stroke: white;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-dasharray: 62.83;
  stroke-dashoffset: 50;
  transform-origin: center;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Tab Transition Animations */
.tab-transition-enter-active,
.tab-transition-leave-active {
  transition: all 0.3s ease;
}

.tab-transition-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.tab-transition-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

/* Enhanced Button Interactions */
.modern-tab-button {
  position: relative;
  overflow: hidden;
}

.modern-tab-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left 0.5s ease;
}

.modern-tab-button:hover::before {
  left: 100%;
}

/* Floating Label Animation */
.modern-input:focus + .floating-label,
.modern-input:not(:placeholder-shown) + .floating-label {
  transform: translateY(-14px) scale(0.85);
  color: #6366F1;
}

.floating-label {
  position: absolute;
  top: 12px;
  left: 14px;
  color: #9CA3AF;
  pointer-events: none;
  transition: all 0.2s ease;
  background-color: rgba(17, 24, 39, 0.9);
  padding: 0 4px;
}

/* Micro-interactions */
.config-section {
  position: relative;
  overflow: hidden;
}

.config-section::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(45deg, transparent, rgba(99, 102, 241, 0.03), transparent);
  transform: translateX(-100%) translateY(-100%) rotate(45deg);
  transition: transform 0.6s ease;
  pointer-events: none;
}

.config-section:hover::before {
  transform: translateX(0) translateY(0) rotate(45deg);
}

/* Port Status Animations */
.status-available,
.status-error,
.status-warning,
.status-checking {
  position: relative;
  font-weight: 500;
}

.status-available::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 6px;
  height: 6px;
  background-color: #34D399;
  border-radius: 50%;
  animation: pulse-success 2s infinite;
}

.status-error::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 6px;
  height: 6px;
  background-color: #F87171;
  border-radius: 50%;
  animation: pulse-error 2s infinite;
}

.status-checking::before {
  content: '';
  position: absolute;
  left: -12px;
  top: 50%;
  transform: translateY(-50%);
  width: 6px;
  height: 6px;
  background-color: #60A5FA;
  border-radius: 50%;
  animation: pulse-checking 1s infinite;
}

@keyframes pulse-success {
  0%, 100% { opacity: 1; transform: translateY(-50%) scale(1); }
  50% { opacity: 0.7; transform: translateY(-50%) scale(1.2); }
}

@keyframes pulse-error {
  0%, 100% { opacity: 1; transform: translateY(-50%) scale(1); }
  50% { opacity: 0.7; transform: translateY(-50%) scale(1.2); }
}

@keyframes pulse-checking {
  0%, 100% { opacity: 0.5; transform: translateY(-50%) scale(0.8); }
  50% { opacity: 1; transform: translateY(-50%) scale(1.2); }
}

/* Enhanced Focus States */
.modern-input:focus,
.modern-select:focus {
  position: relative;
  z-index: 2;
}

.modern-input:focus::after {
  content: '';
  position: absolute;
  top: -2px;
  left: -2px;
  right: -2px;
  bottom: -2px;
  border-radius: 12px;
  background: linear-gradient(45deg, #6366F1, #8B5CF6, #6366F1);
  background-size: 200% 200%;
  animation: gradient-border 3s ease infinite;
  z-index: -1;
}

@keyframes gradient-border {
  0%, 100% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
}

/* Improved Toggle Animations */
.toggle-switch-label {
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

input:checked + .toggle-switch-label::after {
  box-shadow: 0 2px 4px rgba(99, 102, 241, 0.3);
}

/* Button Ripple Effect */
.create-button,
.cancel-button,
.secondary-button {
  position: relative;
  overflow: hidden;
}

.create-button::before,
.cancel-button::before,
.secondary-button::before {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background-color: rgba(255, 255, 255, 0.2);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
}

.create-button:active::before,
.cancel-button:active::before,
.secondary-button:active::before {
  width: 300px;
  height: 300px;
}

/* Enhanced Form Section Animations */
.form-content {
  animation: slideInUp 0.4s ease-out;
}

@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Staggered Animation for Config Sections */
.config-section:nth-child(1) { animation-delay: 0.1s; }
.config-section:nth-child(2) { animation-delay: 0.2s; }
.config-section:nth-child(3) { animation-delay: 0.3s; }
.config-section:nth-child(4) { animation-delay: 0.4s; }
.config-section:nth-child(5) { animation-delay: 0.5s; }

.config-section {
  animation: fadeInUp 0.5s ease-out both;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(15px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Enhanced Checkbox Animations */
.modern-checkbox {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modern-checkbox:checked {
  animation: checkboxPop 0.3s ease-out;
}

@keyframes checkboxPop {
  0% { transform: scale(1); }
  50% { transform: scale(1.15); }
  100% { transform: scale(1); }
}

.modern-checkbox:checked::after {
  animation: checkmarkDraw 0.3s ease-out 0.1s both;
}

@keyframes checkmarkDraw {
  from {
    stroke-dasharray: 0 20;
  }
  to {
    stroke-dasharray: 20 20;
  }
}

/* Improved Tooltip Animations */
.info-tooltip {
  transition: all 0.2s ease;
}

.info-tooltip:hover {
  background-color: rgba(99, 102, 241, 0.8);
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(99, 102, 241, 0.3);
}

/* Icon Glow Effects */
.section-icon,
.label-icon,
.toggle-icon {
  transition: all 0.2s ease;
}

.config-section:hover .section-icon,
.modern-form-label:hover .label-icon {
  filter: drop-shadow(0 0 4px currentColor);
  transform: scale(1.1);
}

/* Enhanced Loading States */
.loading-text {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

/* Responsive Design */
@media (max-width: 768px) {
  .modern-checkbox-grid {
    grid-template-columns: 1fr;
  }
  
  .grid-cols-2 {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
  }
}
</style>