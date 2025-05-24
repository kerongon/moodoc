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
                <p class="text-2xs text-gray-300">Manage backups for this Moodle site • Last activity: 2 hours ago</p>
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
                :disabled="isProcessing" 
                class="ultra-compact-button flex items-center gap-1 bg-gray-700 text-gray-200 px-2 py-1 rounded text-2xs font-medium hover:bg-gray-600 transition-all duration-200 border border-gray-600 disabled:opacity-50 disabled:cursor-not-allowed">
                <PhArrowClockwise class="w-3 h-3" />
                Refresh
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
                <button @click="showSuccessMessage = false" class="inline-flex text-green-500 hover:text-green-700">
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- Stats Cards Grid - More compact design -->
          <div class="dashboard-grid grid grid-cols-3 gap-3 p-3">
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2 transition-all duration-300 hover:border-blue-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1">
                <div class="stat-icon bg-gradient-to-br from-blue-500 to-blue-600 rounded p-1 text-white">
                  <PhDatabase class="w-3 h-3" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-2xs">Site Size</div>
              </div>
              <div class="stat-value text-lg font-bold text-white">847 MB</div>
              <div class="stat-desc text-2xs text-gray-400">Database: 23MB • Files: 824MB</div>
            </div>
            
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2 transition-all duration-300 hover:border-red-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1">
                <div class="stat-icon bg-gradient-to-br from-red-500 to-red-600 rounded p-1 text-white">
                  <PhClock class="w-3 h-3" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-2xs">Last Backup</div>
              </div>
              <div v-if="backups.length === 0" class="stat-value text-lg font-bold text-white">Never</div>
              <div v-else class="stat-value text-lg font-bold text-white">{{ formatDate(backups[0].timestamp).split(' ')[0] }}</div>
              <div v-if="backups.length === 0" class="stat-desc text-2xs text-gray-400">No backups created yet</div>
              <div v-else class="stat-desc text-2xs text-gray-400">{{ backups.length }} backups available</div>
            </div>
            
            <div class="stat-card bg-gray-800/60 backdrop-blur-sm border border-gray-700 rounded-lg p-2 transition-all duration-300 hover:border-green-500/30 hover:translate-y-[-2px]">
              <div class="stat-header flex items-center gap-1.5 mb-1">
                <div class="stat-icon bg-gradient-to-br from-green-500 to-green-600 rounded p-1 text-white">
                  <PhHardDrive class="w-3 h-3" weight="fill" />
                </div>
                <div class="stat-title text-gray-400 text-2xs">Storage Available</div>
              </div>
              <div class="stat-value text-lg font-bold text-white">15.2 GB</div>
              <div class="stat-desc text-2xs text-gray-400 mb-1">85% available space</div>
              <div class="progress-bar h-1 w-full bg-gray-700 rounded-full overflow-hidden">
                <div class="progress-fill h-full bg-gradient-to-r from-green-500 to-green-400" style="width: 15%;"></div>
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
                        </td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.moodle_branch }}</td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.php_version }}</td>
                        <td class="px-2 py-1 text-2xs text-gray-200">{{ backup.db_type }}</td>
                        <td class="px-2 py-1 text-2xs">
                          <div class="flex space-x-1">
                            <button 
                              @click="restoreBackup(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-blue-600 hover:bg-blue-700 text-white p-0.5 rounded transition-colors duration-150"
                              title="Restore this backup">
                              <PhArrowClockwise class="w-2.5 h-2.5" />
                            </button>
                            <button 
                              @click="openInFinder(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-green-600 hover:bg-green-700 text-white p-0.5 rounded transition-colors duration-150"
                              title="Show in Finder">
                              <PhFolderOpen class="w-2.5 h-2.5" />
                            </button>
                            <button 
                              @click="confirmDelete(backup.timestamp)" 
                              :disabled="isProcessing" 
                              class="btn-icon bg-red-600 hover:bg-red-700 text-white p-0.5 rounded transition-colors duration-150"
                              title="Delete this backup">
                              <PhTrash class="w-2.5 h-2.5" />
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
                    <div class="setting-label text-2xs text-gray-200">Active users</div>
                    <div class="setting-value text-2xs text-gray-400">247 users</div>
                  </div>
                </div>
                <div class="setting-item py-1 border-b border-gray-700">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Total courses</div>
                    <div class="setting-value text-2xs text-gray-400">18 courses</div>
                  </div>
                </div>
                <div class="setting-item py-1">
                  <div class="flex justify-between items-center">
                    <div class="setting-label text-2xs text-gray-200">Last modified</div>
                    <div class="setting-value text-2xs text-gray-400">2 hours ago</div>
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
                  <a href="#" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhBook class="w-2.5 h-2.5" />
                    Backup Guide
                  </a>
                  <a href="#" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhArrowClockwise class="w-2.5 h-2.5" />
                    How to Restore
                  </a>
                  <a href="#" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
                    <PhClock class="w-2.5 h-2.5" />
                    Scheduling Backups
                  </a>
                  <a href="#" class="text-blue-400 hover:text-blue-300 text-2xs flex items-center gap-1.5">
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
        <div class="flex justify-end space-x-3">
          <button 
            @click="showRestoreModal = false" 
            class="btn-secondary bg-gray-700 text-gray-200 px-4 py-2 rounded-lg text-sm hover:bg-gray-600 transition-colors duration-150 border border-gray-600">
            Cancel
          </button>
          <button 
            @click="confirmRestore" 
            :disabled="isProcessing" 
            class="btn-danger bg-gradient-to-r from-red-600 to-red-700 text-white px-4 py-2 rounded-lg text-sm hover:from-red-700 hover:to-red-800 transition-colors duration-150 disabled:opacity-50">
            {{ isProcessing ? 'Restoring...' : 'Restore Backup' }}
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
        <div class="flex justify-end space-x-3">
          <button 
            @click="showDeleteConfirm = false" 
            class="btn-secondary bg-gray-700 text-gray-200 px-4 py-2 rounded-lg text-sm hover:bg-gray-600 transition-colors duration-150 border border-gray-600">
            Cancel
          </button>
          <button 
            @click="deleteBackup" 
            :disabled="isProcessing" 
            class="btn-danger bg-gradient-to-r from-red-600 to-red-700 text-white px-4 py-2 rounded-lg text-sm hover:from-red-700 hover:to-red-800 transition-colors duration-150 disabled:opacity-50">
            {{ isProcessing ? 'Deleting...' : 'Delete Backup' }}
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
    };
  },
  mounted() {
    this.refreshBackups();
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
    
    // Existing backup methods
    async refreshBackups() {
      this.loading = true;
      this.error = null;
      this.loadingMessage = 'Loading backups...';
      
      try {
        this.backups = await invoke('list_site_backups', { 
          name: this.siteName 
        });
      } catch (err) {
        this.error = `Failed to fetch backups: ${err}`;
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
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
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
      
      try {
        const result = await invoke('restore_backup', { 
          siteName: this.siteName,
          timestamp: this.selectedBackupTimestamp
        });
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
      } catch (err) {
        this.error = `Failed to restore backup: ${err}`;
        console.error(err);
      } finally {
        this.isProcessing = false;
        this.loading = false;
        this.selectedBackupTimestamp = null;
      }
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
      
      try {
        const result = await invoke('delete_backup', { 
          siteName: this.siteName,
          timestamp: this.confirmingDeleteTimestamp
        });
        
        console.log(result);
        this.showSuccessMessage = true;
        this.successMessage = result;
        setTimeout(() => {
          this.showSuccessMessage = false;
        }, 5000);
        await this.refreshBackups();
      } catch (err) {
        this.error = `Failed to delete backup: ${err}`;
        console.error(err);
      } finally {
        this.isProcessing = false;
        this.loading = false;
        this.confirmingDeleteTimestamp = null;
      }
    },
    
    async openBackupsFolder() {
      try {
        await invoke('open_site_backups_folder', { name: this.siteName });
      } catch (err) {
        this.error = `Failed to open backups folder: ${err}`;
        console.error(err);
      }
    },
    
    async openInFinder(timestamp) {
      try {
        await invoke('open_backup_in_finder', { 
          siteName: this.siteName,
          timestamp: timestamp
        });
      } catch (err) {
        this.error = `Failed to open backup in Finder: ${err}`;
        console.error(err);
      }
    },
    
    formatDate(timestamp) {
      return new Date(timestamp * 1000).toLocaleString();
    },
    
    formatErrorMessage(errorMsg) {
      // Format the error message with HTML line breaks
      return errorMsg.replace(/\n/g, '<br>');
    },
    
    // New methods for this UI
    scheduleBackups() {
      // To be implemented
      alert('Schedule backups functionality coming soon');
    },
    
    configSettings() {
      // To be implemented
      alert('Configure backup settings functionality coming soon');
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

.btn-icon {
  transition: all 0.2s ease;
}
.btn-icon:hover {
  transform: translateY(-1px);
}

.backdrop-blur-sm {
  backdrop-filter: blur(8px);
}

.progress-fill {
  transition: width 1s ease;
}
</style>
