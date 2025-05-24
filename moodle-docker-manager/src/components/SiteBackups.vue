<template>
  <div>
    <div class="dashboard-full hide-scrollbar bg-white dark:bg-gray-900 min-h-screen w-full p-0">
      <!-- Modern macOS-style header (similar to CreateSite.vue, no New Site button) -->
      <header class="app-header" data-tauri-drag-region>
        <div class="header-content">
          <div class="titlebar-left-controls">
            <!-- Window Controls -->
            <div class="window-controls">
              <button @click="closeWindow" class="window-control-button close-button" title="Close">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FF5F57" stroke="#E0443E" stroke-width="1"></circle></svg>
              </button>
              <button @click="minimizeWindow" class="window-control-button minimize-button" title="Minimize">
                <svg width="12" height="12" viewBox="0 0 12 12"><circle cx="6" cy="6" r="5.5" fill="#FFBD2E" stroke="#DEA123" stroke-width="1"></circle></svg>
              </button>
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
              <PhList class="logo-icon" weight="fill" />
            </div>
            <div class="title-container">
              <h1 class="app-title">Moodle Docker Manager</h1>
            </div>
          </div>
          <!-- Header actions -->
          <div style="display: flex; align-items: center; gap: 0.75rem;">
            <DockerStatus />
            
            <router-link to="/create" title="Create New Site" class="modern-new-site-button">
              <PhPlus class="button-icon" weight="fill" />
              <span>New Site</span>
            </router-link>
            
            <button @click="openSettings" title="Settings" class="modern-settings-button">
              <PhGear class="button-icon" />
            </button>
          </div>
        </div>
      </header>
      
      <div class="main-content px-2 pb-2 hide-scrollbar">
        <!-- Breadcrumbs -->
        <div class="flex py-0.5 mb-1">
          <nav class="text-xs text-gray-600 dark:text-gray-400">
            <router-link to="/" class="hover:underline">Home</router-link>
            <span class="mx-1">›</span>
            <span>Site Backups</span>
          </nav>
        </div>
        
        <div class="site-backups bg-gradient-to-br from-gray-800 to-gray-900 dark:from-gray-900 dark:to-black rounded-lg shadow-lg border border-gray-700 dark:border-gray-800 w-full overflow-hidden">
          
          <!-- Header - Ultra compact style with actions -->
          <div class="page-header flex items-center justify-between p-2 border-b border-gray-700 dark:border-gray-800">
            <div class="header-info flex items-center gap-2">
              <div class="site-icon bg-gradient-to-br from-green-500 to-green-600 rounded-lg p-1.5 shadow-lg">
                <PhArchiveBox class="text-white w-4 h-4" weight="fill" />
              </div>
              <div class="header-text">
                <h2 class="text-base font-medium text-white">Site Backups: {{ siteName }}</h2>
                <p class="text-2xs text-gray-300">Manage backups for this Moodle site • Last activity: {{ formatRelativeTime(lastActivityTimestamp) }}</p>
              </div>
            </div>
            
            <div class="header-actions flex gap-1">
              <button 
                @click="createBackup" 
                :disabled="isProcessing" 
                class="ultra-compact-button flex items-center gap-1 bg-gradient-to-r from-green-500 to-green-600 text-white px-2 py-1 rounded text-2xs font-medium hover:from-green-600 hover:to-green-700 transition-all duration-200 shadow-sm disabled:opacity-50 disabled:cursor-not-allowed">
                <PhPlus class="w-3 h-3" />
                Create Backup
              </button>
              
              <button 
                @click="scheduleBackups" 
                :disabled="isProcessing" 
                class="ultra-compact-button flex items-center gap-1 bg-gray-700 text-gray-200 px-2 py-1 rounded text-2xs font-medium hover:bg-gray-600 transition-all duration-200 border border-gray-600 disabled:opacity-50 disabled:cursor-not-allowed">
                <PhClock class="w-3 h-3" />
                Schedule
              </button>
              
              <button 
                @click="openBackupsFolder" 
                :disabled="isProcessing" 
                class="ultra-compact-button flex items-center gap-1 bg-gray-700 text-gray-200 px-2 py-1 rounded text-2xs font-medium hover:bg-gray-600 transition-all duration-200 border border-gray-600 disabled:opacity-50 disabled:cursor-not-allowed">
                <PhFolderOpen class="w-3 h-3" />
                Open Folder
              </button>
              
              <button 
                @click="refreshBackups" 
                :disabled="isProcessing || loading" 
                class="ultra-compact-button flex items-center gap-1 bg-gray-700 text-gray-200 px-2 py-1 rounded text-2xs font-medium hover:bg-gray-600 transition-all duration-200 border border-gray-600 disabled:opacity-50 disabled:cursor-not-allowed">
                <PhArrowClockwise class="w-3 h-3" :class="{ 'animate-spin': loading }" />
                {{ loading ? 'Refreshing...' : 'Refresh' }}
              </button>
            </div>
          </div>

          <!-- Success message notification -->
          <div v-if="showSuccessMessage" class="mx-6 mt-4 p-3 bg-green-100 border border-green-200 rounded-md">
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <svg class="h-5 w-5 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <div class="ml-3">
                <p class="text-sm text-green-800">{{ successMessage }}</p>
              </div>
              <div class="ml-auto pl-3">
                <button @click="showSuccessMessage = false" class="inline-flex text-green-700 hover:text-green-800">
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- Stats Cards Grid - More compact design -->
          <div class="dashboard-grid grid grid-cols-3 gap-3 px-3 pt-3 pb-2">
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2.5 transition-all duration-300 hover:border-blue-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1.5">
                <div class="stat-icon bg-gradient-to-br from-blue-500 to-blue-600 rounded p-1 text-white">
                  <PhDatabase class="w-3.5 h-3.5" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-xs">Site Size</div>
              </div>
              <div class="stat-value text-lg font-bold text-white">{{ siteSize.total }}</div>
              <div class="stat-desc text-xs text-gray-400">
                Database: {{ siteSize.database }} • Files: {{ siteSize.files }}
                <br v-if="siteSize.moodledata !== '0 B' && siteSize.moodledata !== '0 MB'">
                <span v-if="siteSize.moodledata !== '0 B' && siteSize.moodledata !== '0 MB'">Data: {{ siteSize.moodledata }} • Code: {{ siteSize.moodle }}</span>
              </div>
            </div>
            
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2.5 transition-all duration-300 hover:border-red-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1.5">
                <div class="stat-icon bg-gradient-to-br from-red-500 to-red-600 rounded p-1 text-white">
                  <PhClock class="w-3.5 h-3.5" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-xs">Last Backup</div>
              </div>
              <div v-if="backups.length === 0" class="stat-value text-lg font-bold text-white">Never</div>
              <div v-else class="stat-value text-lg font-bold text-white">{{ formatDate(backups[0].timestamp).split(',')[0] }}</div>
              <div v-if="backups.length === 0" class="stat-desc text-xs text-gray-400">No backups created yet</div>
              <div v-else class="stat-desc text-xs text-gray-400">
                {{ formatDate(backups[0].timestamp).split(',')[1] || '' }}
                <br>{{ backups.length }} backups available
              </div>
            </div>
            
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2.5 transition-all duration-300 hover:border-green-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1.5">
                <div class="stat-icon bg-gradient-to-br from-green-500 to-green-600 rounded p-1 text-white">
                  <PhHardDrive class="w-3.5 h-3.5" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-xs">Storage Available</div>
              </div>
              <div class="stat-value text-lg font-bold text-white">{{ storageInfo.available }}</div>
              <div class="stat-desc text-xs text-gray-400 mb-2">{{ storageInfo.percentAvailable }}% available space</div>
              <div class="progress-bar h-1.5 w-full bg-gray-700 rounded-full overflow-hidden">
                <div class="progress-fill h-full bg-gradient-to-r from-green-500 to-green-400" :style="{ width: `${100 - storageInfo.percentUsed}%` }"></div>
              </div>
            </div>
          </div>

          <div v-if="loading" class="loading text-sm text-center py-10">
            <div class="flex items-center justify-center space-x-2">
              <div class="h-3 w-3 bg-blue-500 rounded-full animate-bounce" style="animation-delay: 0.0s"></div>
              <div class="h-3 w-3 bg-blue-500 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
              <div class="h-3 w-3 bg-blue-500 rounded-full animate-bounce" style="animation-delay: 0.4s"></div>
            </div>
            <p class="text-gray-400 mt-3">{{ loadingMessage }}</p>
          </div>
          
          <div v-else-if="error" class="mx-6 my-4">
            <div class="error text-xs text-red-400 p-4 bg-red-900/20 border border-red-700/30 rounded-md overflow-auto max-h-60" v-html="error"></div>
          </div>
          
          <div v-else class="main-content grid grid-cols-3 gap-2 p-2">
            <!-- Main Content Section (2/3 width) -->
            <div class="backup-section col-span-2 bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2">
              <!-- Empty state when no backups -->
              <div v-if="backups.length === 0" class="empty-state text-center">
                <div class="empty-icon mx-auto mb-2 bg-gray-700/50 rounded-xl flex items-center justify-center w-16 h-16">
                  <PhPackage class="w-8 h-8 text-gray-500" weight="fill" />
                </div>
                <h2 class="empty-title text-sm font-semibold text-white mb-1">Ready to Create Your First Backup?</h2>
                <p class="empty-desc text-2xs text-gray-400 mx-auto max-w-md mb-2">
                  Protect your Moodle site with regular backups. A backup includes your courses, user data, 
                  settings, and uploaded files. Estimated backup time: ~3 minutes.
                </p>
                
                <div class="onboarding-steps grid grid-cols-2 gap-2 mb-3">
                  <div class="step-card bg-gray-700/30 border border-gray-600/30 rounded-lg p-2">
                    <div class="step-number bg-gradient-to-r from-blue-500 to-blue-600 text-white w-5 h-5 rounded-full flex items-center justify-center text-2xs font-medium mb-1">1</div>
                    <div class="step-title text-white text-2xs font-medium mb-0.5">Quick Backup</div>
                    <div class="step-desc text-gray-400 text-2xs">Create an immediate backup with default settings</div>
                  </div>
                  
                  <div class="step-card bg-gray-700/30 border border-gray-600/30 rounded-lg p-2">
                    <div class="step-number bg-gradient-to-r from-blue-500 to-blue-600 text-white w-5 h-5 rounded-full flex items-center justify-center text-2xs font-medium mb-1">2</div>
                    <div class="step-title text-white text-2xs font-medium mb-0.5">Custom Backup</div>
                    <div class="step-desc text-gray-400 text-2xs">Choose what to include and backup location</div>
                  </div>
                  
                  <div class="step-card bg-gray-700/30 border border-gray-600/30 rounded-lg p-2">
                    <div class="step-number bg-gradient-to-r from-blue-500 to-blue-600 text-white w-5 h-5 rounded-full flex items-center justify-center text-2xs font-medium mb-1">3</div>
                    <div class="step-title text-white text-2xs font-medium mb-0.5">Schedule Backups</div>
                    <div class="step-desc text-gray-400 text-2xs">Set up automatic daily or weekly backups</div>
                  </div>
                  
                  <div class="step-card bg-gray-700/30 border border-gray-600/30 rounded-lg p-2">
                    <div class="step-number bg-gradient-to-r from-blue-500 to-blue-600 text-white w-5 h-5 rounded-full flex items-center justify-center text-2xs font-medium mb-1">4</div>
                    <div class="step-title text-white text-2xs font-medium mb-0.5">Test Restore</div>
                    <div class="step-desc text-gray-400 text-2xs">Verify your backup works by testing restore</div>
                  </div>
                </div>
                
                <button 
                  @click="createBackup" 
                  :disabled="isProcessing" 
                  class="compact-button bg-gradient-to-r from-green-500 to-green-600 text-white px-3 py-1 rounded-md text-2xs font-medium hover:from-green-600 hover:to-green-700 transition-all duration-200 shadow-md flex items-center gap-1.5 mx-auto disabled:opacity-50 disabled:cursor-not-allowed">
                  <PhRocketLaunch class="w-3 h-3" />
                  Create Your First Backup
                </button>
              </div>
              
              <!-- Backups list when backups exist -->
              <div v-else>
                <h3 class="text-xs font-medium text-white mb-2">Backup History</h3>
                
                <div class="overflow-x-auto">
                  <table class="min-w-full divide-y divide-gray-700">
                    <thead class="bg-gray-800">
                      <tr>
                        <th scope="col" class="px-2 py-1 text-2xs font-medium text-gray-400 text-left">Date</th>
                        <th scope="col" class="px-2 py-1 text-2xs font-medium text-gray-400 text-left">Moodle Version</th>
                        <th scope="col" class="px-2 py-1 text-2xs font-medium text-gray-400 text-left">PHP Version</th>
                        <th scope="col" class="px-2 py-1 text-2xs font-medium text-gray-400 text-left">Database</th>
                        <th scope="col" class="px-2 py-1 text-2xs font-medium text-gray-400 text-left">Actions</th>
                      </tr>
                    </thead>
                    <tbody class="bg-gray-800/30 divide-y divide-gray-700">
                      <tr v-for="backup in backups" :key="backup.timestamp" class="hover:bg-gray-700/40 transition-colors duration-150">
                        <td class="px-2 py-1 text-2xs text-gray-200">
                          {{ formatDate(backup.timestamp) }}
                          <span v-if="backup.timestamp && isNaN(new Date(backup.timestamp).getTime())" class="text-red-300 text-opacity-70 ml-1">
                            <!-- (Raw: {{ backup.timestamp }}) -->
                          </span>
                        </td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.moodle_branch }}</td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.php_version }}</td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.db_type }}</td>
                        <td class="px-2 py-1 text-2xs">
                          <div class="flex space-x-1">
                            <button 
                              @click="restoreBackup(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-blue-600 hover:bg-blue-700 text-white p-1 rounded transition-colors duration-150 w-7 h-6 flex items-center justify-center"
                              title="Restore this backup">
                              <PhArrowClockwise class="w-3 h-3" />
                            </button>
                            <button 
                              @click="openInFinder(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-green-600 hover:bg-green-700 text-white p-1 rounded transition-colors duration-150 w-7 h-6 flex items-center justify-center"
                              title="Show in Finder">
                              <PhFolderOpen class="w-3 h-3" />
                            </button>
                            <button 
                              @click="confirmDelete(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-red-600 hover:bg-red-700 text-white p-1 rounded transition-colors duration-150 w-7 h-6 flex items-center justify-center"
                              title="Delete this backup">
                              <div v-if="isDeletingBackup && deletingBackupTimestamp === backup.timestamp" class="custom-spinner w-3 h-3"></div>
                              <PhTrash v-else class="w-3 h-3" />
                            </button>
                          </div>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
            
            <!-- Sidebar Section (1/3 width) -->
            <div class="sidebar flex flex-col gap-3">
              <!-- Settings Card -->
              <div class="settings-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2">
                <h3 class="card-title text-2xs font-medium text-white mb-1 flex items-center gap-1">
                  <PhGear class="w-3 h-3 text-gray-400" />
                  Backup Settings
                </h3>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Auto-backup</div>
                    <div class="setting-value text-2xs text-gray-400">Disabled</div>
                  </div>
                </div>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Retention</div>
                    <div class="setting-value text-2xs text-gray-400">Keep 7 backups</div>
                  </div>
                </div>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Include user files</div>
                    <div class="setting-value text-2xs text-gray-400">Yes</div>
                  </div>
                </div>
                <div class="setting-item py-1">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Compress backups</div>
                    <div class="setting-value text-2xs text-gray-400">Yes</div>
                  </div>
                </div>
                <button @click="configSettings" class="w-full mt-1 ultra-compact-button bg-gray-700 text-gray-200 px-1 py-0.5 rounded text-2xs hover:bg-gray-600 transition-all duration-200 border border-gray-600">
                  Configure Settings
                </button>
              </div>
              
              <!-- Site Activity Card -->
              <div class="settings-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2">
                <h3 class="card-title text-2xs font-medium text-white mb-1 flex items-center gap-1">
                  <PhChartLine class="w-3 h-3 text-gray-400" />
                  Site Activity
                </h3>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Last backup</div>
                    <div class="setting-value text-2xs text-gray-400">
                      {{ backups.length ? formatRelativeTime(lastActivityTimestamp) : 'Never' }}
                    </div>
                  </div>
                </div>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Backup count</div>
                    <div class="setting-value text-2xs text-gray-400">{{ backups.length }} backups</div>
                  </div>
                </div>
                <div class="setting-item py-1">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Last refresh</div>
                    <div class="setting-value text-2xs text-gray-400">{{ formatRelativeTime(lastRefreshTimestamp) }}</div>
                  </div>
                </div>
              </div>
              
              <!-- Help Card -->
              <div class="help-card bg-gradient-to-br from-blue-900/20 to-indigo-900/20 backdrop-blur-sm border border-blue-700/30 rounded-lg p-2">
                <h3 class="card-title text-2xs font-medium text-white mb-0.5 flex items-center gap-1">
                  <PhQuestion class="w-3 h-3 text-blue-400" />
                  Need Help?
                </h3>
                <p class="text-2xs text-gray-300 mb-1">Learn more about backing up your Moodle site</p>
                
                <div class="grid grid-cols-2 gap-x-2 gap-y-1">
                  <a href="javascript:void(0);" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhBook class="w-2.5 h-2.5" />
                    Backup Guide
                  </a>
                  <a href="javascript:void(0);" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhArrowClockwise class="w-2.5 h-2.5" />
                    How to Restore
                  </a>
                  <a href="javascript:void(0);" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhClock class="w-2.5 h-2.5" />
                    Scheduling Backups
                  </a>
                  <a href="javascript:void(0);" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhWrench class="w-2.5 h-2.5" />
                    Troubleshooting
                  </a>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Footer Actions -->
          <div class="actions p-1 border-t border-gray-700">
            <button @click="$router.push('/')" class="ultra-compact-button bg-gray-700 text-gray-200 px-1.5 py-0.5 rounded text-2xs font-medium hover:bg-gray-600 transition-all duration-200 border border-gray-600 flex items-center gap-1">
              <PhArrowLeft class="w-2 h-2" />
              Back to Dashboard
            </button>
          </div>
        </div> <!-- end of .site-backups -->
      </div> <!-- end of .main-content -->
    </div> <!-- end of dashboard-full -->
    
    <!-- Restore Progress Overlay -->
    <div v-if="showRestoreProgress" class="fixed inset-0 flex items-center justify-center z-50 bg-black bg-opacity-70">
      <div class="bg-gray-800 border border-gray-700 rounded-xl shadow-2xl p-6 w-96 mx-4">
        <div class="mb-4">
          <h3 class="text-lg font-medium text-white">Restoring Backup</h3>
          <p class="text-sm text-gray-300 mt-2">
            {{ restoreMessage }}
          </p>
        </div>
        
        <div class="mt-4">
          <div class="w-full bg-gray-700 rounded-full h-2.5">
            <div class="bg-blue-600 h-2.5 rounded-full" :style="{ width: restoreProgress + '%' }"></div>
          </div>
          <div class="flex justify-between mt-1">
            <span class="text-2xs text-gray-400">{{ Math.round(restoreProgress) }}%</span>
            <span class="text-2xs text-gray-400">{{ restoreStatus }}</span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Restore Confirmation Modal -->
    <div v-if="showRestoreModal" class="fixed inset-0 flex items-center justify-center z-50 bg-black bg-opacity-70">
      <div class="bg-gray-800 border border-gray-700 rounded-xl shadow-2xl p-6 w-96 mx-4">
        <div class="mb-4">
          <h3 class="text-lg font-medium text-white">Confirm Restore</h3>
          <p class="text-sm text-gray-300 mt-2">
            Are you sure you want to restore this backup? This will overwrite the current site content and database.
          </p>
        </div>
        <div class="text-sm text-red-400 font-medium mb-4 flex items-center gap-2">
          <PhWarningCircle class="w-5 h-5" />
          <p>This action cannot be undone.</p>
        </div>
        <div class="flex justify-end space-x-3 mt-6">
          <button 
            @click="showRestoreModal = false" 
            class="btn-secondary bg-gray-700 text-gray-200 px-4 py-2 rounded-lg text-sm font-medium hover:bg-gray-600 transition-colors duration-150 border border-gray-600 shadow-sm flex items-center justify-center">
            <span>Cancel</span>
          </button>
          <button 
            @click="confirmRestore" 
            :disabled="isProcessing" 
            class="btn-primary bg-gradient-to-r from-green-500 to-green-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:from-green-600 hover:to-green-700 transition-colors duration-150 shadow-sm disabled:opacity-50 flex items-center justify-center">
            <span>{{ isProcessing ? 'Restoring...' : 'Restore Backup' }}</span>
          </button>
        </div>
      </div>
    </div>
    
    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 flex items-center justify-center z-50 bg-black bg-opacity-70">
      <div class="bg-gray-800 border border-gray-700 rounded-xl shadow-2xl p-6 w-96 mx-4">
        <div class="mb-4">
          <h3 class="text-lg font-medium text-white">Confirm Delete</h3>
          <p class="text-sm text-gray-300 mt-2">
            Are you sure you want to delete this backup from {{ formatDate(confirmingDeleteTimestamp) }}?
          </p>
        </div>
        <div class="text-sm text-red-400 font-medium mb-4 flex items-center gap-2">
          <PhWarningCircle class="w-5 h-5" />
          <p>This action cannot be undone.</p>
        </div>
        <div class="flex justify-end space-x-3 mt-6">
          <button 
            @click="showDeleteConfirm = false" 
            class="btn-secondary bg-gray-700 text-gray-200 px-4 py-2 rounded-lg text-sm font-medium hover:bg-gray-600 transition-colors duration-150 border border-gray-600 shadow-sm flex items-center justify-center">
            <span>Cancel</span>
          </button>
          <button 
            @click="deleteBackup" 
            :disabled="isProcessing" 
            class="btn-danger bg-gradient-to-r from-red-600 to-red-700 text-white px-4 py-2 rounded-lg text-sm font-medium hover:from-red-700 hover:to-red-800 transition-colors duration-150 shadow-sm disabled:opacity-50 flex items-center justify-center">
            <span>{{ isProcessing ? 'Deleting...' : 'Delete Backup' }}</span>
          </button>
        </div>
      </div>
    </div>
    
    <SettingsModal ref="settingsModal" />
  </div>
</template>

<script>
import DockerStatus from './DockerStatus.vue';
import SettingsModal from './SettingsModal.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import { 
  PhArchiveBox, 
  PhPlus, 
  PhArrowClockwise, 
  PhFolderNotch, 
  PhFolderOpen, 
  PhTrash, 
  PhGear, 
  PhList,
  PhDatabase, 
  PhClock, 
  PhHardDrive, 
  PhPackage, 
  PhRocketLaunch,
  PhChartLine, 
  PhQuestion, 
  PhBook, 
  PhWrench, 
  PhArrowLeft,
  PhWarningCircle 
} from '@phosphor-icons/vue';

export default {
  name: 'SiteBackups',
  components: {
    DockerStatus,
    SettingsModal,
    PhArchiveBox,
    PhPlus,
    PhArrowClockwise,
    PhFolderNotch,
    PhFolderOpen,
    PhTrash,
    PhGear,
    PhList,
    PhDatabase,
    PhClock,
    PhHardDrive,
    PhPackage,
    PhRocketLaunch,
    PhChartLine,
    PhQuestion,
    PhBook,
    PhWrench,
    PhArrowLeft,
    PhWarningCircle
  },
  props: {
    siteName: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      backups: [],
      loading: false,
      isProcessing: false,
      error: null,
      showRestoreModal: false,
      selectedBackupTimestamp: null,
      confirmingDeleteTimestamp: null,
      showDeleteConfirm: false,
      loadingMessage: 'Loading backups...',
      showSuccessMessage: false,
      successMessage: '',
      lastActivityTimestamp: null,
      lastRefreshTimestamp: null,
      showRestoreProgress: false,
      restoreProgress: 0,
      restoreStatus: 'starting',
      restoreMessage: '',
      isDeletingBackup: false,
      deletingBackupTimestamp: null,
      siteSize: {
        total: '0 MB',
        database: '0 MB',
        files: '0 MB',
        moodle: '0 MB',
        moodledata: '0 MB',
        other: '0 MB'
      },
      storageInfo: {
        available: '0 GB',
        total: '0 GB',
        percentAvailable: 0,
        percentUsed: 0
      },
      lastActivityTimestamp: Date.now(),
      lastRefreshTimestamp: Date.now()
    };
  },
  mounted() {
    // this.fetchSiteSize();
    // this.fetchStorageInfo();
    this.refreshBackups().then(() => {
      // Initialize lastActivityTimestamp with most recent backup time if available
      this.updateActivityTimestamp();

      // If no backups exist, set to current time
      if (!this.lastActivityTimestamp) {
        this.lastActivityTimestamp = Date.now();
      }

      // Initialize refresh timestamp
      this.lastRefreshTimestamp = Date.now();
    });
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
    
    // Enhanced refresh method to reload all data
    async refreshBackups() {
      this.loading = true;
      this.error = null;
      this.loadingMessage = 'Refreshing data...';
      
      try {
        // Refresh backups list
        this.backups = await invoke('list_site_backups', { 
          name: this.siteName 
        });
        
        // Update activity timestamp based on most recent backup
        this.updateActivityTimestamp();
        
        // Update refresh timestamp
        this.lastRefreshTimestamp = Date.now();
        
        // Also refresh site size and storage info
        await Promise.all([
          this.fetchSiteSize(),
          this.fetchStorageInfo()
        ]);
        
        // Update last refresh timestamp (but not activity timestamp)
        this.lastRefreshTimestamp = Date.now();
        
      } catch (err) {
        this.error = `Failed to fetch data: ${err}`;
        console.error(err);
      } finally {
        this.loading = false;
      }
    },
    
    async createBackup() {
      this.isProcessing = true;
      this.loading = true;
      this.loadingMessage = 'Creating backup... This might take a few minutes depending on your site size';
      this.error = null;
      
      try {
        const result = await invoke('backup_site', { 
          name: this.siteName 
        });
        
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
        // Update activity timestamp based on the new backup's actual timestamp
        this.updateActivityTimestamp();
      } catch (err) {
        const errorMsg = String(err);
        console.error(`Backup error:`, errorMsg);
        
        // Format error message for display
        if (errorMsg.includes("Error output:") || errorMsg.includes("Standard output:")) {
          this.error = this.formatErrorMessage(errorMsg);
        } else {
          this.error = `Failed to create backup: ${errorMsg}`;
        }
        this.loading = false;
      } finally {
        this.isProcessing = false;
      }
    },
    
    restoreBackup(timestamp) {
      this.selectedBackupTimestamp = timestamp;
      this.showRestoreModal = true;
    },
    
    async confirmRestore() {
      if (!this.selectedBackupTimestamp) return;
      
      this.showRestoreModal = false;
      this.isProcessing = true;
      this.loading = true;
      this.loadingMessage = 'Restoring backup... This might take a few minutes';
      this.error = null;
      this.restoreProgress = 0;
      this.restoreStatus = 'starting';
      this.restoreMessage = 'Preparing to restore backup...';
      this.showRestoreProgress = true;
      
      try {
        // Set up a listener for restore progress events
        const unlisten = await listen('restore_progress', (event) => {
          const data = event.payload;
          if (data.siteName === this.siteName) {
            this.restoreStatus = data.status;
            this.restoreMessage = data.message;
            this.restoreProgress = data.progress * 100; // Convert to percentage
            
            if (data.status === 'completed' || data.status === 'error') {
              // When complete/error, we can clean up the listener
              unlisten();
              
              if (data.status === 'completed') {
                this.showSuccessMessage = true;
                this.successMessage = `Site '${this.siteName}' restored successfully from backup ${this.selectedBackupTimestamp}`;
                // Don't auto-hide for restore operations - let user close it manually
              } else {
                this.error = `Failed to restore backup: ${data.message}`;
              }

              // Hide progress display after a short delay
              setTimeout(() => {
                this.showRestoreProgress = false;
                this.isProcessing = false;
                this.loading = false;
              }, 2000);
            }
          }
        });
        
        // Start the async restore process
        await invoke('restore_site_backup_async', { 
          name: this.siteName,
          backupTimestamp: this.selectedBackupTimestamp
        });
        
        // For restore, we can update activity timestamp to current time since it's an action
        this.lastActivityTimestamp = Date.now();
      } catch (err) {
        this.error = `Failed to restore backup: ${err}`;
        console.error(err);
        this.showRestoreProgress = false;
        this.isProcessing = false;
        this.loading = false;
      }
        this.loading = false;
        this.selectedBackupTimestamp = null;
      },
    
    confirmDelete(timestamp) {
      this.confirmingDeleteTimestamp = timestamp;
      this.showDeleteConfirm = true;
    },
    
    async deleteBackup() {
      if (!this.confirmingDeleteTimestamp) return;
      
      this.showDeleteConfirm = false;
      this.isProcessing = true;
      this.loading = true;
      this.loadingMessage = 'Deleting backup...';
      this.error = null;
      
      // Set delete-specific loading state
      this.isDeletingBackup = true;
      this.deletingBackupTimestamp = this.confirmingDeleteTimestamp;
      
      // Add minimum delay to ensure spinner visibility
      const startTime = Date.now();
      const minDelay = 800; // Minimum 800ms to see the spinner
      
      try {
        const result = await invoke('delete_site_backup', { 
          name: this.siteName,
          backupTimestamp: this.confirmingDeleteTimestamp
        });
        
        // Ensure minimum delay has passed
        const elapsed = Date.now() - startTime;
        if (elapsed < minDelay) {
          await new Promise(resolve => setTimeout(resolve, minDelay - elapsed));
        }
        
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
        
        // Update activity timestamp based on the remaining backups
        this.updateActivityTimestamp();
      } catch (err) {
        this.error = `Failed to delete backup: ${err}`;
        console.error(err);
      } finally {
        this.isProcessing = false;
        this.loading = false;
        this.isDeletingBackup = false;
        this.deletingBackupTimestamp = null;
        this.confirmingDeleteTimestamp = null;
      }
    },
    
    async openBackupsFolder() {
      try {
        await invoke('open_backup_location', { name: this.siteName });
      } catch (err) {
        this.error = `Failed to open backups folder: ${err}`;
        console.error(err);
      }
    },
    
    async openInFinder(timestamp) {
      try {
        await invoke('open_backup_location', { 
          name: this.siteName,
          backup_timestamp: timestamp
        });
      } catch (err) {
        this.error = `Failed to open backup in Finder: ${err}`;
        console.error(err);
      }
    },
    
    // Helper method to update activity timestamp based on most recent backup
    updateActivityTimestamp() {
      if (this.backups.length > 0 && this.backups[0].timestamp) {
        try {
          if (typeof this.backups[0].timestamp === 'string' && 
              this.backups[0].timestamp.match(/^\d{8}_\d{6}$/)) {
            // Parse YYYYMMDD_HHMMSS format
            const timestamp = this.backups[0].timestamp;
            const year = parseInt(timestamp.substring(0, 4));
            const month = parseInt(timestamp.substring(4, 6)) - 1; // Month is 0-indexed in JS
            const day = parseInt(timestamp.substring(6, 8));
            const hour = parseInt(timestamp.substring(9, 11));
            const minute = parseInt(timestamp.substring(11, 13));
            const second = parseInt(timestamp.substring(13, 15));
            
            const date = new Date(year, month, day, hour, minute, second);
            
            if (!isNaN(date.getTime())) {
              this.lastActivityTimestamp = date.getTime();
            }
          } else {
            // For standard timestamp format
            const ts = parseInt(this.backups[0].timestamp);
            if (!isNaN(ts)) {
              this.lastActivityTimestamp = ts < 10000000000 ? ts * 1000 : ts;
            }
          }
        } catch (e) {
          console.error("Failed to parse backup timestamp:", e);
        }
      }
    },
    
    formatDate(timestamp) {
      // Check if timestamp is valid before formatting
      if (!timestamp) {
        return "Unknown date";
      }
      
      try {
        // Check if the timestamp is in format YYYYMMDD_HHMMSS
        if (typeof timestamp === 'string' && timestamp.match(/^\d{8}_\d{6}$/)) {
          // Extract date parts: YYYYMMDD_HHMMSS
          const year = parseInt(timestamp.substring(0, 4));
          const month = parseInt(timestamp.substring(4, 6)) - 1; // Month is 0-indexed in JS
          const day = parseInt(timestamp.substring(6, 8));
          const hour = parseInt(timestamp.substring(9, 11));
          const minute = parseInt(timestamp.substring(11, 13));
          const second = parseInt(timestamp.substring(13, 15));
          
          // Create a UTC date object since backup timestamps are likely in UTC
          const date = new Date(Date.UTC(year, month, day, hour, minute, second));
          
          // Check if date is valid
          if (isNaN(date.getTime())) {
            return "Invalid date format";
          }
          
          // Format the date using locale settings (will automatically convert to local time)
          return date.toLocaleString(undefined, {
            year: 'numeric',
            month: '2-digit', 
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            timeZoneName: 'short'
          });
        } else {
          // For normal timestamps, use the previous logic
          const ts = typeof timestamp === 'string' ? parseInt(timestamp, 10) : timestamp;
          const date = new Date(ts < 10000000000 ? ts * 1000 : ts);
          
          // Check if date is valid
          if (isNaN(date.getTime())) {
            return "Invalid date format";
          }
          
          return date.toLocaleString(undefined, {
            year: 'numeric',
            month: '2-digit', 
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            timeZoneName: 'short'
          });
        }
      } catch (e) {
        console.error("Date formatting error:", e, timestamp);
        return "Date error";
      }
    },
    
    formatRelativeTime(timestamp) {
      if (!timestamp) return 'Never';
      
      const now = new Date();
      const date = new Date(timestamp);
      const diffMs = now - date;
      
      // Convert to seconds
      const diffSec = Math.floor(diffMs / 1000);
      
      // Less than a minute
      if (diffSec < 60) {
        return 'Just now';
      }
      
      // Minutes
      const diffMin = Math.floor(diffSec / 60);
      if (diffMin < 60) {
        return diffMin === 1 ? '1 minute ago' : `${diffMin} minutes ago`;
      }
      
      // Hours
      const diffHour = Math.floor(diffMin / 60);
      if (diffHour < 24) {
        return diffHour === 1 ? '1 hour ago' : `${diffHour} hours ago`;
      }
      
      // Days
      const diffDay = Math.floor(diffHour / 24);
      if (diffDay < 30) {
        return diffDay === 1 ? 'Yesterday' : `${diffDay} days ago`;
      }
      
      // Months
      const diffMonth = Math.floor(diffDay / 30);
      if (diffMonth < 12) {
        return diffMonth === 1 ? '1 month ago' : `${diffMonth} months ago`;
      }
      
      // Years
      const diffYear = Math.floor(diffMonth / 12);
      return diffYear === 1 ? '1 year ago' : `${diffYear} years ago`;
    },
    
    formatErrorMessage(errorMsg) {
      // Format the error message with HTML line breaks
      return errorMsg.replace(/\n/g, '<br>');
    },
    
    async fetchSiteSize() {
      try {
        const sizeData = await invoke('get_site_size', { 
          name: this.siteName 
        });
        
        if (sizeData) {
          this.siteSize = {
            total: this.formatSize(sizeData.total_size || 0),
            database: this.formatSize(sizeData.db_size || 0),
            files: this.formatSize(sizeData.files_size || 0),
            // Store additional details for potential future use
            moodle: this.formatSize(sizeData.moodle_size || 0),
            moodledata: this.formatSize(sizeData.moodledata_size || 0),
            other: this.formatSize(sizeData.other_files_size || 0)
          };
        }
      } catch (err) {
        console.error('Failed to fetch site size:', err);
        // Don't show error to user, just use default values
      }
    },
    
    formatSize(size) {
      // Convert bytes to human readable format
      if (size === 0) return '0 B';
      
      const units = ['B', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(size) / Math.log(1024));
      
      return (size / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0) + ' ' + units[i];
    },
    
    async fetchStorageInfo() {
      try {
        const storageData = await invoke('get_storage_info');
        
        if (storageData) {
          this.storageInfo = {
            available: this.formatSize(storageData.available || 0),
            total: this.formatSize(storageData.total || 0),
            percentAvailable: Math.round((storageData.available / storageData.total) * 100) || 0,
            percentUsed: Math.round(((storageData.total - storageData.available) / storageData.total) * 100) || 0
          };
        }
      } catch (err) {
        console.error('Failed to fetch storage information:', err);
        // Don't show error to user, just use default values
      }
    },
    
    // New methods for this UI
    scheduleBackups() {
      // To be implemented
      alert('Schedule backups functionality coming soon');
    },
    
    configSettings() {
      // To be implemented
      alert('Configure backup settings functionality coming soon');
    },
    
    // Helper method to update activity timestamp based on most recent backup
    updateActivityTimestamp() {
      // Update activity timestamp based on the most recent backup
      if (this.backups.length > 0 && this.backups[0].timestamp) {
        try {
          if (typeof this.backups[0].timestamp === 'string' && 
              this.backups[0].timestamp.match(/^\d{8}_\d{6}$/)) {
            // Parse YYYYMMDD_HHMMSS format
            const timestamp = this.backups[0].timestamp;
            const year = parseInt(timestamp.substring(0, 4));
            const month = parseInt(timestamp.substring(4, 6)) - 1; // Month is 0-indexed in JS
            const day = parseInt(timestamp.substring(6, 8));
            const hour = parseInt(timestamp.substring(9, 11));
            const minute = parseInt(timestamp.substring(11, 13));
            const second = parseInt(timestamp.substring(13, 15));
            
            // Create a UTC date object since backup timestamps are likely in UTC
            const date = new Date(Date.UTC(year, month, day, hour, minute, second));
            
            if (!isNaN(date.getTime())) {
              this.lastActivityTimestamp = date.getTime();
            } else {
              this.lastActivityTimestamp = Date.now();
            }
          } else {
            // For standard timestamp format
            const ts = parseInt(this.backups[0].timestamp);
            if (!isNaN(ts)) {
              this.lastActivityTimestamp = ts < 10000000000 ? ts * 1000 : ts;
            } else {
              this.lastActivityTimestamp = Date.now();
            }
          }
        } catch (e) {
          console.error("Failed to parse backup timestamp:", e);
          this.lastActivityTimestamp = Date.now();
        }
      } else {
        // No backups, use current time
        this.lastActivityTimestamp = Date.now();
      }
    },
    
    // Format relative time for display
    formatRelativeTime(timestamp) {
      if (!timestamp) return 'Never';
      
      const now = Date.now();
      const diff = now - timestamp;
      
      if (diff < 60000) { // Less than 1 minute
        return 'Just now';
      } else if (diff < 3600000) { // Less than 1 hour
        const minutes = Math.floor(diff / 60000);
        return `${minutes} minute${minutes === 1 ? '' : 's'} ago`;
      } else if (diff < 86400000) { // Less than 24 hours
        const hours = Math.floor(diff / 3600000);
        return `${hours} hour${hours === 1 ? '' : 's'} ago`;
      } else if (diff < 2592000000) { // Less than 30 days
        const days = Math.floor(diff / 86400000);
        return `${days} day${days === 1 ? '' : 's'} ago`;
      } else if (diff < 31536000000) { // Less than 1 year
        const months = Math.floor(diff / 2592000000);
        return `${months} month${months === 1 ? '' : 's'} ago`;
      } else {
        const years = Math.floor(diff / 31536000000);
        return `${years} year${years === 1 ? '' : 's'} ago`;
      }
    }
  }
};
</script>

<style scoped>
/* Add any additional styles needed for the component */
.stat-card {
  transform: translateY(0);
  transition: all 0.3s ease;
}
.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

/* Progress bar animation */
@keyframes pulse {
  0% { opacity: 0.6; }
  50% { opacity: 1; }
  100% { opacity: 0.6; }
}

/* Custom spinner for delete button */
.custom-spinner {
  width: 12px;
  height: 12px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top: 2px solid white;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.bg-blue-600 {
  transition: width 0.5s ease-in-out;
  animation: pulse 2s infinite;
}

.btn-icon {
  transition: all 0.2s ease;
  margin: 0 1px; /* Add some horizontal spacing between buttons */
}
.btn-icon:hover {
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2); /* Add a subtle shadow on hover */
}

.backdrop-blur-sm {
  backdrop-filter: blur(8px);
}

.progress-fill {
  transition: width 1s ease;
}

/* Modal button styles */
.btn-primary, .btn-danger, .btn-secondary {
  min-width: 120px;
  transition: all 0.2s ease;
  height: 36px;
  line-height: 1;
}
.btn-primary:hover, .btn-danger:hover, .btn-secondary:hover {
  transform: translateY(-1px);
}
.btn-primary span, .btn-danger span, .btn-secondary span {
  display: inline-block;
}
</style>
