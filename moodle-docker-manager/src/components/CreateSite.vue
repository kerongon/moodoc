<template>
  <div class="dashboard-full hide-scrollbar bg-gradient-to-br from-white to-gray-100 dark:from-gray-900 dark:to-gray-800 min-h-screen w-full p-0">
    <!-- Modern macOS-style header (copied from Dashboard, no New Site button) -->
    <header class="app-header" data-tauri-drag-region>
      <div class="header-content">
        <div class="titlebar-left-controls">
          <!-- Window Controls -->
          <div class="window-controls">
            <button @click="closeWindow" class="window-control-button close-button" title="Close">
              <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                <circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1" />
              </svg>
              <svg class="icon-hover" width="8" height="8" viewBox="0 0 24 24">
                <path stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
            <button @click="minimizeWindow" class="window-control-button minimize-button" title="Minimize">
              <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                <circle cx="6" cy="6" r="5.5" fill="#FFBD2E" stroke="#DEA123" stroke-width="1" />
              </svg>
              <svg class="icon-hover" width="8" height="2" viewBox="0 0 12 12">
                <rect x="2" y="5" width="8" height="2" fill="white" />
              </svg>
            </button>
            <button @click="toggleMaximizeWindow" class="window-control-button maximize-button" title="Maximize">
              <svg class="icon-default" width="12" height="12" viewBox="0 0 12 12">
                <circle cx="6" cy="6" r="5.5" fill="#27C93F" stroke="#1AAB29" stroke-width="1" />
              </svg>
              <PhArrowsOut class="icon-hover w-2 h-2 text-white" weight="fill" />
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
            <svg class="logo-icon text-white" width="24" height="24" viewBox="0 0 1024 1024" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect width="1024" height="1024" rx="224" fill="#7066C6"/>
              <g id="icon-elements">
                <!-- M Letter: Scaled and positioned to be 450px wide, 300px tall. Top at y=277 -->
                <path transform="translate(287, 227) scale(0.78125)" d="M0 64H134.4L288 344.3L441.6 64H576V448H480V204.3L326.4 448H249.6L96 204.3V448H0V64Z" fill="white"/>
                <!-- Server Rack: 450px wide, 120px tall. Top at y=627 -->
                <rect x="287" y="627" width="450" height="120" rx="20" fill="white"/>
                <line x1="307" y1="663" x2="717" y2="663" stroke="#5A52A3" stroke-width="10" stroke-linecap="round"/>
                <line x1="307" y1="703" x2="717" y2="703" stroke="#5A52A3" stroke-width="10" stroke-linecap="round"/>
              </g>
            </svg>
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
    <div class="main-content px-8 pb-8 hide-scrollbar">
      <div class="create-site-container">
        <div class="site-header flex items-center mb-6">
          <div class="icon-container bg-gradient-to-r from-indigo-600 to-blue-500 p-2.5 rounded-xl mr-4 shadow-lg shadow-indigo-500/20">
            <PhPlus class="text-white w-6 h-6" weight="fill" />
          </div>
          <div>
            <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">Create a New Moodle Site</h2>
            <p class="text-sm text-gray-500 dark:text-gray-400">Configure and set up a new Moodle development environment</p>
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
    <form @submit.prevent="createSite" class="space-y-4">
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
          <div class="grid-container">
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
        <div class="space-y-5">
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
            <div class="grid grid-cols-2 gap-4">
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
        <div class="space-y-5">
          <!-- Docker Configuration -->
          <div class="config-section">
            <h3 class="section-title">
              <div class="section-icon">🐳</div>
              Docker Configuration
            </h3>
            <div class="grid grid-cols-2 gap-4">
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
            <div class="grid grid-cols-2 gap-4">
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
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useSettingsStore } from '../stores/settings';
import { computed, watch } from 'vue';
import { PhList, PhGear, PhArrowClockwise, PhArrowsOut } from '@phosphor-icons/vue';
import { eventBus } from '../utils/event-bus';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import DockerStatus from './DockerStatus.vue';
import SettingsModal from './SettingsModal.vue';
import { MoodleService } from '../services/moodleService';
const appWindow = getCurrentWebviewWindow()

export default {
  components: {
    DockerStatus,
    SettingsModal,
    PhList,
    PhGear,
    PhArrowClockwise,
    PhArrowsOut
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
      phpVersions: MoodleService.getAvailablePhpVersions(),
      dbTypes: MoodleService.getAvailableDbTypes(),
      moodleBranches: MoodleService.getAvailableBranches(),
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
      // Site creation state
      createSiteInProgress: false,
      // Port availability cache
      portAvailability: {},
      // Custom settings for the Custom tab
      customRepoUrl: '',
      customBranch: '',
      dbPrefix: 'mdl_',
      siteConfig: MoodleService.getDefaultSiteConfig(),
      adminAccount: MoodleService.getDefaultAdminAccount(),
      // New data properties for Advanced tab
      dockerConfig: MoodleService.getDefaultDockerConfig(),
      phpConfig: MoodleService.getDefaultPhpConfig(),
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
    },
    
    // Watch for PHP version changes to check compatibility
    phpVersion() {
      this.isPhpVersionCompatible = MoodleService.isPhpVersionCompatible(this.phpVersion, this.moodleBranch);
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
      const versionInfo = MoodleService.updatePhpVersionForBranch(this.moodleBranch);
      this.recommendedPhpVersion = versionInfo.recommendedVersion;
      this.phpVersion = versionInfo.recommendedVersion;
      // Check compatibility with the current/new PHP version
      this.isPhpVersionCompatible = MoodleService.isPhpVersionCompatible(this.phpVersion, this.moodleBranch);
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

<style>
@import '../assets/create-site.css';
@import '../assets/create-site-compact.css'; /* Add this new import for fixes */
@import '../assets/create-site-light-mode-fix.css'; /* Import light mode fixes */

.window-control-button {
  position: relative;
  width: 12px;
  height: 12px;
  padding: 0;
}
.window-control-button svg.icon-hover {
  display: none;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}
.window-control-button:hover svg.icon-hover {
  display: block;
}
</style>