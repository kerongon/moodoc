<template>
  <div class="site-list-container bg-white dark:bg-gray-900 w-full text-gray-900 dark:text-gray-100 flex flex-col">
    <!-- Header - macOS style (more compact) -->
    <div class="flex items-center justify-between mb-6 px-4 pt-4">
      <div class="flex items-center">
        <div class="bg-indigo-600 rounded-lg p-3 mr-4 shadow-md" style="background-color: #4f46e5 !important;">
          <PhSquare class="text-white w-6 h-6" weight="fill" style="color: white !important;" />
        </div>
        <div>
          <h2 class="text-xl font-bold text-gray-800 dark:text-white">Configured Sites</h2>
          <p class="text-sm text-gray-500 dark:text-gray-400">Manage your Moodle development environments</p>
        </div>
      </div>

      <div class="flex items-center space-x-3">
        <!-- search box -->
        <div v-if="sites.length > 0" class="relative">
          <input
            type="search"
            v-model="searchQuery"
            placeholder="Search sites..."
            aria-label="Search sites"
            class="p-2 pl-3 pr-10 border border-gray-300 dark:border-gray-600 rounded-md text-sm dark:bg-gray-700 dark:text-gray-200 dark:placeholder-gray-400 focus:ring-blue-500 focus:border-blue-500 w-64"
          />
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
            <PhMagnifyingGlass class="w-4 h-4 text-gray-400 dark:text-gray-500" />
          </div>
        </div>
        <!-- Add new site button -->  
        <router-link 
          to="/create" 
          class="inline-flex items-center justify-center px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 transition-all duration-150 text-sm"
        >
          <PhPlus class="w-5 h-5 mr-2 -ml-1" weight="bold" />
          New Site
        </router-link>
      </div>
    </div>

    <div class="site-list flex-1 overflow-x-auto px-4">
      <div v-if="loading" class="loading text-xs p-4 text-center">
        Loading sites...
      </div>
      <div v-else-if="error" class="error text-xs p-4 text-center text-red-500">
        {{ error }}
      </div>
      <div v-else-if="sites.length === 0" 
           class="flex-1 flex flex-col items-center justify-center p-6 text-center">
        <div class="bg-white dark:bg-slate-800 p-8 sm:p-12 rounded-xl shadow-2xl border border-gray-200 dark:border-slate-700 max-w-md mx-auto">
          <div class="bg-indigo-500 dark:bg-indigo-600 p-4 rounded-lg inline-block mb-6 shadow-lg">
            <PhCheckSquare class="w-12 h-12 text-white" weight="light" />
          </div>
          <h3 class="text-xl font-semibold mb-2 text-gray-800 dark:text-gray-50">No sites have been created yet</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-8">
            Get started by creating your first Moodle development site. You can configure multiple environments and manage them all from here.
          </p>
          <router-link 
            to="/create" 
            class="inline-flex items-center justify-center px-6 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg shadow-md hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:focus:ring-offset-slate-800 transition-all duration-150 text-sm"
          >
            <PhPlus class="w-5 h-5 mr-2 -ml-1" weight="bold" />
            Create a Site
          </router-link>
        </div>
      </div>
      <div v-else class="table-container w-full">
        <div class="bg-white dark:bg-gray-800 rounded-lg overflow-hidden shadow-lg border border-gray-200 dark:border-gray-700">
          <div class="moodle-table-header grid grid-cols-custom-table-layout items-center px-4 py-3 text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700">
            <div class="moodle-header-cell col-span-2 flex items-center"
                  @click="sort('name')"
                  :aria-sort="sortKey==='name'? sortOrder:'none'"
                  tabindex="0">
              Site Name
              <span v-if="sortKey === 'name'" class="ml-1">
                <PhCaretUp v-if="sortOrder === 'asc'" class="inline-block w-3 h-3" />
                <PhCaretDown v-else class="inline-block w-3 h-3" />
              </span>
            </div>
            <div class="moodle-header-cell flex items-center"
                  @click="sort('version_combined')"
                  :aria-sort="sortKey==='version_combined'? sortOrder:'none'"
                  tabindex="0">
              Version
              <span v-if="sortKey === 'version_combined'" class="ml-1">
                <PhCaretUp v-if="sortOrder === 'asc'" class="inline-block w-3 h-3" />
                <PhCaretDown v-else class="inline-block w-3 h-3" />
              </span>
            </div>
            <div class="moodle-header-cell flex items-center"
                  @click="sort('db_type')"
                  :aria-sort="sortKey==='db_type'? sortOrder:'none'"
                  tabindex="0">
              Database
              <span v-if="sortKey === 'db_type'" class="ml-1">
                <PhCaretUp v-if="sortOrder === 'asc'" class="inline-block w-3 h-3" />
                <PhCaretDown v-else class="inline-block w-3 h-3" />
              </span>
            </div>
            <div class="moodle-header-cell flex items-center"
                  @click="sort('status')"
                  :aria-sort="sortKey==='status'? sortOrder:'none'"
                  tabindex="0">
              Status
              <span v-if="sortKey === 'status'" class="ml-1">
                <PhCaretUp v-if="sortOrder === 'asc'" class="inline-block w-3 h-3" />
                <PhCaretDown v-else class="inline-block w-3 h-3" />
              </span>
            </div>
            <div class="moodle-header-cell flex items-center">Resources</div>
            <div class="moodle-header-cell flex items-center col-span-2">Actions</div>
          </div>
          <div v-for="(site, index) in paginatedSites" :key="site.name" 
               class="moodle-table-row grid grid-cols-custom-table-layout items-center px-4 py-4 border-b border-gray-200 dark:border-gray-700"
               :class="[
                  { 'bg-white dark:bg-gray-800': index % 2 === 0, 'bg-gray-50 dark:bg-[#2c334b]': index % 2 !== 0 }, // Alternating row colors
                  { 'opacity-50 pointer-events-none': activeActionSite===site.name },
                ]">
            <div class="moodle-data-cell col-span-2 flex items-center">
              <div class="flex items-center">
                <div class="w-8 h-8 rounded-md bg-orange-500 flex items-center justify-center text-white font-semibold text-sm mr-3">
                  {{ site.name.charAt(0).toUpperCase() }}
                </div>
                <div>
                  <span class="site-name-container text-sm font-semibold text-gray-900 dark:text-white">
                    {{ site.name }}
                  </span>
                  <a v-if="site.web_port"
                    :href="`http://localhost:${site.web_port}`"
                    :title="`http://localhost:${site.web_port}`"
                    target="_blank"
                    rel="noopener"
                    class="block text-xs text-gray-500 dark:text-gray-400 hover:text-blue-500 dark:hover:text-blue-400 flex items-center">
                    localhost:{{site.web_port}}
                    <PhLinkSimple class="inline w-3 h-3 ml-1 text-blue-500 dark:text-blue-400 group-hover:text-blue-600 dark:group-hover:text-blue-300" />
                  </a>
                </div>
                <span v-if="activeActionSite === site.name" class="ml-2 text-xs text-blue-600 dark:text-blue-400 animate-pulse">
                  processing...
                </span>
              </div>
            </div>
            <div class="moodle-data-cell text-sm text-gray-700 dark:text-gray-300">
              <span class="px-2 py-1 rounded-full bg-blue-100 dark:bg-blue-600 text-blue-700 dark:text-blue-100 text-xs">
                {{ formatVersions(site) }}
              </span>
            </div>
            <div class="moodle-data-cell text-sm text-gray-700 dark:text-gray-300">
              <span class="px-2 py-1 rounded-full bg-green-100 dark:bg-green-600 text-green-700 dark:text-green-100 text-xs">
                {{ site.db_type }}
              </span>
            </div>
            <div class="moodle-data-cell">
              <div class="flex items-center">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border" :class="getStatusClasses(site.status)">
                  <span class="inline-block w-2 h-2 rounded-full mr-1.5" :class="getStatusIndicatorClasses(site.status)"></span>
                  {{ site.status }}
                </span>
                <span v-if="isRecentlyCreated(site.name)" 
                      class="ml-1.5 text-xs new-badge bg-green-100 text-green-700 dark:bg-green-700 dark:text-green-200 py-0.5 px-1.5 rounded-full"
                      :title="`Created at ${getCreationTime(site.name)}`">
                  NEW
                </span>
                <button v-if="site.status === 'error'" @click="checkDetailedStatus(site.name)" 
                        class="ml-1.5 text-red-500 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300" 
                        title="Check detailed error information">
                  <PhWarningCircle class="w-4 h-4" />
                </button>
              </div>
              <div v-if="site.status === 'running' && site.uptime" class="text-xs text-gray-500 dark:text-gray-500 mt-0.5">
                {{ site.uptime }}
              </div>
               <div v-else-if="site.status === 'stopped'" class="text-xs text-gray-500 dark:text-gray-500 mt-0.5">
                -
              </div>
            </div>
            <div class="moodle-data-cell text-sm text-gray-700 dark:text-gray-300 flex items-center space-x-1.5">
              <button
                  @click.prevent="openDirectory(site.path)"
                  :title="site.path"
                  type="button"
                  class="action-icon-button bg-teal-500 hover:bg-teal-600 text-white"
                >
                  <PhFolder class="w-4 h-4" />
                </button>
                <button
                  @click.prevent="openInEditor(site.path)"
                  :title="`Open in editor`"
                  type="button"
                  class="action-icon-button bg-purple-500 hover:bg-purple-600 text-white"
                >
                  <PhCode class="w-4 h-4" />
                </button>
            </div>
            <div class="moodle-data-cell col-span-2">
              <div class="flex items-center space-x-1.5">
                <button v-if="site.status !== 'running'"
                    @click="startSite(site.name)"
                    class="action-icon-button bg-green-500 hover:bg-green-600 text-white"
                    :disabled="activeActionSite === site.name"
                    title="Start">
                  <template v-if="activeActionSite === site.name">
                    <div class="h-4 w-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  </template>
                  <template v-else>
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                      <path d="M5 3v18l15-9L5 3z" />
                    </svg>
                  </template>
                </button>
                <button v-else
                    @click="stopSite(site.name)"
                    class="action-icon-button bg-red-500 hover:bg-red-600 text-white"
                    :disabled="activeActionSite === site.name"
                    title="Stop">
                  <template v-if="activeActionSite === site.name">
                    <div class="h-4 w-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  </template>
                  <template v-else>
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                      <rect x="6" y="6" width="12" height="12" rx="2" />
                    </svg>
                  </template>
                </button>
                <button @click="viewLogs(site.name)" class="action-icon-button bg-blue-500 hover:bg-blue-600 text-white" title="Logs">
                  <PhFileText class="w-4 h-4" weight="fill" />
                </button>
                <button @click="viewBackups(site.name)" class="action-icon-button bg-gray-500 hover:bg-gray-600 text-white" title="Backups">
                  <PhArchiveBox class="w-4 h-4" />
                </button>
                <button
                    @click="removeSite(site.name)"
                    class="action-icon-button bg-red-500 text-white hover:bg-red-600"
                    :disabled="isRemovingSite && removingSiteName === site.name"
                    title="Remove"
                >
                  <template v-if="isRemovingSite && removingSiteName === site.name">
                    <div class="h-4 w-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  </template>
                  <template v-else>
                    <PhTrash class="w-4 h-4" />
                  </template>
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="table-footer bg-gray-50 dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 px-4 py-3 flex justify-between items-center rounded-b-lg">
          <div class="text-sm text-gray-600 dark:text-gray-400">
            Showing {{ paginatedSites.length }} of {{ sites.length }} sites
          </div>
          <div class="pagination-container flex items-center space-x-1">
            <button
              @click="currentPage = Math.max(currentPage-1,1)"
              :disabled="currentPage===1"
              class="px-3 py-1 text-sm text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >Previous</button>
            <button
              v-for="page in totalPages"
              :key="page"
              @click="currentPage = page"
              :class="[
                'px-3 py-1 text-sm border rounded-md',
                currentPage===page 
                  ? 'bg-blue-600 text-white border-blue-600' 
                  : 'text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700'
              ]"
              aria-current="page"
            >{{ page }}</button>
            <button
              @click="currentPage = Math.min(currentPage+1,totalPages)"
              :disabled="currentPage===totalPages"
              class="px-3 py-1 text-sm text-gray-700 dark:text-gray-300 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-100 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >Next</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Success notification shown after starting a site -->
    <transition name="fade">
      <div v-if="successMessage" class="mx-4 mt-4 p-3 rounded border border-green-400 bg-green-100 text-green-800 dark:border-green-600 dark:bg-green-200 dark:text-green-900 text-center">
        {{ successMessage }}
      </div>
    </transition>
    <!-- Stop notification shown after stopping a site -->
    <transition name="fade">
      <div v-if="stopMessage" class="mx-4 mt-4 p-3 rounded border border-red-400 bg-red-100 text-red-800 dark:border-red-600 dark:bg-red-200 dark:text-red-900 text-center">
        {{ stopMessage }}
      </div>
    </transition>

    <!-- Error Modal -->
    <div v-if="showErrorModal" class="fixed inset-0 z-50 overflow-y-auto hide-scrollbar">
      <div class="flex items-center justify-center min-h-screen px-4">
        <!-- Modal backdrop -->
        <div class="fixed inset-0 bg-black/50 transition-opacity" @click="closeErrorModal"></div>
        
        <!-- Modal panel -->
        <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full">
          <!-- Modal content -->
          <div class="p-6">
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center">
                <div class="bg-red-600 rounded-lg p-1.5 mr-3 shadow-sm">
                  <svg class="text-white w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                  </svg>
                </div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Error</h2>
              </div>
              <button @click="closeErrorModal" class="text-gray-400 hover:text-gray-500">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <!-- Error content -->
            <div class="text-sm text-gray-800 dark:text-gray-200 overflow-auto max-h-96">
              <div v-html="errorContent"></div>
            </div>
            
            <!-- Footer -->
            <div class="mt-6 flex justify-end">
              <button @click="closeErrorModal" class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">Close</button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Update Site Modal -->
    <UpdateSite 
      :show="updateModalVisible"
      :siteName="selectedSiteName"
      @close="hideUpdateModal"
      @updated="handleSiteUpdated" 
    />
    
    <!-- Diagnostic Modal -->
    <DiagnosticModal 
      :show="diagnosticModalVisible"
      @close="hideDiagnosticModal"
    />
    
    <!-- Confirmation Modal for Site Removal -->
    <div v-if="showRemoveModal" class="fixed inset-0 z-50 overflow-y-auto hide-scrollbar">
      <div class="flex items-center justify-center min-h-screen px-4">
        <!-- Modal backdrop -->
        <div class="fixed inset-0 bg-black/50 transition-opacity" @click="cancelRemove"></div>
        
        <!-- Modal panel -->
        <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <!-- Modal content -->
          <div class="p-6">
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center">
                <div class="bg-red-600 rounded-lg p-1.5 mr-3 shadow-sm">
                  <PhTrash class="text-white w-6 h-6" weight="fill" />
                </div>
                <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Remove Site</h2>
              </div>
              <button @click="cancelRemove" class="text-gray-400 hover:text-gray-500">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <!-- Confirmation message -->
            <div class="mb-4 text-sm text-gray-800 dark:text-gray-200">
              <p>Are you sure you want to remove the site <span class="font-semibold">{{ siteToRemove }}</span>?</p>
              <div class="mt-3 bg-blue-50 dark:bg-blue-900/20 p-3 rounded-md">
                <div class="flex items-center">
                  <input type="checkbox" id="removeContainers" v-model="removeContainers" class="h-4 w-4 text-blue-600 rounded border-gray-300">
                  <label for="removeContainers" class="ml-2 font-medium">Clean up Docker resources</label>
                </div>
                <p class="mt-1 text-xs text-blue-600 dark:text-blue-400 ml-6">
                  This will stop and remove all Docker containers, volumes, networks, and associated images for this site.
                </p>
              </div>
            </div>
            
            <!-- Warning message -->
            <div class="bg-amber-50 dark:bg-amber-900/20 p-3 rounded-md mb-4">
              <p class="text-xs text-amber-600 dark:text-amber-400">
                <span class="font-medium">Warning: </span>
                This action cannot be undone. Please make sure you have any necessary backups.
              </p>
            </div>
            
            <!-- Footer -->
            <div class="mt-4 flex justify-end space-x-2">
              <button @click="cancelRemove" class="flex items-center justify-center px-3 py-1.5 bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-500 text-sm min-w-[80px]">
                Cancel
              </button>
              <button @click="confirmRemove" class="flex items-center justify-center px-3 py-1.5 bg-red-600 text-white rounded-md hover:bg-red-700 text-sm min-w-[100px]">
                Remove Site
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Site Removal Progress Modal -->
    <div v-if="isRemovingSite" class="fixed inset-0 z-50 overflow-y-auto hide-scrollbar">
      <div class="flex items-center justify-center min-h-screen px-4">
        <!-- Modal backdrop -->
        <div class="fixed inset-0 bg-black/50 transition-opacity"></div>
        
        <!-- Modal panel -->
        <div class="relative bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full">
          <!-- Modal content -->
          <div class="p-6">
            <!-- Header -->
            <div class="flex items-center justify-between mb-6">
              <div class="flex items-center">
                <div class="bg-red-600 rounded-lg p-1.5 mr-3 shadow-sm">
                  <PhTrash class="text-white w-6 h-6" weight="fill" />
                </div>
                <div>
                  <h2 class="text-base font-medium text-gray-900 dark:text-gray-100">Removing Site</h2>
                  <p class="text-xs text-gray-500 dark:text-gray-400">{{ removingSiteName }}</p>
                </div>
              </div>
            </div>
            
            <!-- Progress bar -->
            <div class="mb-4">
              <div class="h-2 w-full bg-gray-200 dark:bg-gray-700 rounded-full">
                <div 
                  class="h-2 bg-red-500 dark:bg-red-600 rounded-full transition-all duration-300"
                  :style="{ width: `${removalProgress}%` }">
                </div>
              </div>
              <p class="text-xs text-center mt-2 text-gray-600 dark:text-gray-400">{{ removalStatus }}</p>
              <p class="text-xs text-center mt-1 text-gray-500 dark:text-gray-500">{{ removalProgress }}% complete</p>
            </div>
            
            <!-- Loading animation -->
            <div class="flex items-center justify-center space-x-2 mb-4">
              <div class="h-3 w-3 bg-red-500 rounded-full animate-bounce" style="animation-delay: 0.0s"></div>
              <div class="h-3 w-3 bg-red-500 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
              <div class="h-3 w-3 bg-red-500 rounded-full animate-bounce" style="animation-delay: 0.4s"></div>
            </div>
            
            <!-- Note -->
            <div class="bg-amber-50 dark:bg-amber-900/20 p-3 rounded-md">
              <p class="text-xs text-amber-600 dark:text-amber-400">
                <span class="font-medium">Please wait:</span> Site removal is in progress. This may take a few minutes.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div> 
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ref, computed, onMounted, onBeforeUnmount, watch } from 'vue';
import { useRouter } from 'vue-router';
import { PhList, PhArchiveBox, PhArrowClockwise, PhTrash, PhTerminal, PhRecycle, PhLinkSimple, PhFolder, PhCode, PhMagnifyingGlass, PhFileText, PhCaretUp, PhCaretDown, PhCheckSquare, PhPlus, PhSquare, PhWrench, PhWarningCircle } from '@phosphor-icons/vue';
import UpdateSite from './UpdateSite.vue';
import DiagnosticModal from './DiagnosticModal.vue';
import { Command } from '@tauri-apps/plugin-shell';
import { cleanupRecentSites } from '../utils/notifications';
import { useSettingsStore } from '../stores/settings';

export default {
  name: 'SiteList',
  components: {
    PhList,
    PhArchiveBox,
    PhArrowClockwise,
    PhTrash,
    PhTerminal,
    PhRecycle,
    PhLinkSimple,
    PhFolder,
    PhCode,
    UpdateSite,
    DiagnosticModal,
    PhMagnifyingGlass,
    PhFileText,
    PhCaretUp,
    PhCaretDown,
    PhCheckSquare,
    PhPlus,
    PhSquare,
    PhWrench,
    PhWarningCircle
  },
  emits: ['action-start', 'action-end'],
  setup(props, { expose, emit }) {
    const sites = ref([]);
    const loading = ref(true);
    const activeActionSite = ref(null);
    const error = ref(null);
    const router = useRouter();
    const settingsStore = useSettingsStore();
    const updateModalVisible = ref(false);
    const selectedSiteName = ref(null);
    const showErrorModal = ref(false);
    const errorContent = ref('');
    const showRemoveModal = ref(false);
    const siteToRemove = ref('');
    const removeContainers = ref(true);
    const diagnosticModalVisible = ref(false);

    // Site removal progress tracking
    const isRemovingSite = ref(false);
    const removalProgress = ref(0);
    const removalStatus = ref('');
    const removingSiteName = ref('');

    // new for search/sort/pagination
    const searchQuery = ref('');
    const sortKey = ref('');
    const sortOrder = ref('asc');
    const currentPage = ref(1);
    const perPage = 5;

    // Recently created sites tracking
    const recentlyCreatedSites = ref({});
    
    // Success notification message
    const successMessage = ref('');
    // Stop notification message
    const stopMessage = ref('');

    // Check for recently created sites
    const loadRecentlyCreatedSites = () => {
      try {
        // First, clean up old entries
        cleanupRecentSites();
        
        // Then, load the freshly cleaned list
        const storedRecent = localStorage.getItem('recentlyCreatedSites');
        if (storedRecent) {
          recentlyCreatedSites.value = JSON.parse(storedRecent);
        }
      } catch (err) {
        console.error("Failed to load recently created sites:", err);
      }
    };
    
    // Check if a site was recently created (within last hour)
    const isRecentlyCreated = (siteName) => {
      return recentlyCreatedSites.value[siteName] !== undefined;
    };
    
    // Get when a site was created (for tooltip)
    const getCreationTime = (siteName) => {
      const timestamp = recentlyCreatedSites.value[siteName];
      if (!timestamp) return '';
      
      const date = new Date(timestamp);
      return date.toLocaleTimeString();
    };

    // Initial load of site list
    const loadSites = async () => {
      loading.value = true;
      error.value = null;
      try {
        // fetch full list including statuses
        sites.value = await invoke('list_sites');
      } catch (err) {
        console.error('Failed to load sites:', err);
        error.value = `Failed to load sites: ${err}`;
      } finally {
        loading.value = false;
      }
    };

    // Show error modal
    const showError = (title, errorMessage) => {
      const formattedError = errorMessage
        .replace(/\n/g, '<br>')
        .replace(/Error output:/g, '<strong class="text-red-500">Error output:</strong>')
        .replace(/Standard output:/g, '<strong class="text-blue-500">Standard output:</strong>');
      
      errorContent.value = `<div class="font-medium mb-2">${title}</div><div>${formattedError}</div>`;
      showErrorModal.value = true;
    };

    // Close error modal
    const closeErrorModal = () => {
      showErrorModal.value = false;
      errorContent.value = '';
    };

    // Refresh only statuses without replacing list
    const refreshStatuses = async () => {
      try {
        const updated = await invoke('list_sites');
        updated.forEach(u => {
          const existing = sites.value.find(s => s.name === u.name);
          if (existing && existing.status !== u.status) {
            existing.status = u.status;
          }
        });
      } catch (err) {
        console.error('Failed to refresh site statuses:', err);
      }
    };
    // Start a site
    const startSite = async (name) => {
      // Set active action and emit event
      activeActionSite.value = name;
      emit('action-start', name);
      
      // Minimum time to show spinner (for visual feedback)
      const minSpinnerTime = Date.now() + 1000; // At least 1 second
      
      try {
        // invoke backend to start
        await invoke('start_site', { name });
        // Poll status until running or timeout
        for (let i = 0; i < 20; i++) {
          await refreshStatuses();
          const siteObj = sites.value.find(s => s.name === name);
          if (siteObj && siteObj.status === 'running') break;
          await new Promise(r => setTimeout(r, 500));
        }
        // Show success notification
        successMessage.value = `Site '${name}' started successfully`;
        setTimeout(() => { successMessage.value = ''; }, 5000);
      } catch (err) {
        console.error(`Failed to start site ${name}:`, err);
        showError(`Failed to start site: ${name}`, String(err));
      } finally {
        // Ensure spinner shows for minimum time for better UX
        const remainingTime = minSpinnerTime - Date.now();
        if (remainingTime > 0) {
          await new Promise(r => setTimeout(r, remainingTime));
        }
        emit('action-end', name);
        activeActionSite.value = null;
      }
    };

    // Stop a site
    const stopSite = async (name) => {
      activeActionSite.value = name;
      emit('action-start', name);
      
      try {
        // Start the async stop operation in the background immediately
        setTimeout(async () => {
          try {
            await invoke('stop_site', { name });
            // Poll status until stopped or timeout
            for (let i = 0; i < 20; i++) {
              await refreshStatuses();
              const siteObj = sites.value.find(s => s.name === name);
              if (siteObj && siteObj.status !== 'running') break;
              await new Promise(r => setTimeout(r, 500));
            }
            // Show stop notification
            stopMessage.value = `Site '${name}' stopped successfully`;
            setTimeout(() => { stopMessage.value = ''; }, 5000);
            // Clear spinner after operation completes
            emit('action-end', name);
            activeActionSite.value = null;
          } catch (err) {
            console.error(`Failed to stop site ${name}:`, err);
            showError(`Failed to stop site: ${name}`, String(err));
            // Clear spinner on error
            emit('action-end', name);
            activeActionSite.value = null;
          }
        }, 0);
        
        // Return immediately to avoid blocking the UI
      } catch (err) {
        console.error(`Failed to initiate stop for site ${name}:`, err);
        showError(`Failed to stop site: ${name}`, String(err));
        emit('action-end', name);
        activeActionSite.value = null;
      }
    };

    // Show remove site modal
    const removeSite = (name) => {
      siteToRemove.value = name;
      removeContainers.value = true; // Default to true
      showRemoveModal.value = true;
    };
    
    // Cancel remove operation
    const cancelRemove = () => {
      showRemoveModal.value = false;
      siteToRemove.value = '';
    };
    
    // Confirm and execute site removal
    const confirmRemove = async () => {
      try {
        // Setup removal progress tracking
        isRemovingSite.value = true;
        removalProgress.value = 0;
        removalStatus.value = 'Preparing to remove site...';
        removingSiteName.value = siteToRemove.value;
        showRemoveModal.value = false;
        activeActionSite.value = siteToRemove.value;

        // Call the backend remove_site function with progress tracking
        await invoke('remove_site', {
          name: siteToRemove.value,
          removeContainers: removeContainers.value
        });

        // Reload sites after successful removal
        await loadSites();
        
        console.log(`Site ${siteToRemove.value} removed successfully`);
      } catch (err) {
        console.error(`Failed to remove site ${siteToRemove.value}:`, err);
        showError(`Failed to remove site: ${siteToRemove.value}`, String(err));
      } finally {
        // Reset removal progress state
        isRemovingSite.value = false;
        removalProgress.value = 0;
        removalStatus.value = '';
        removingSiteName.value = '';
        activeActionSite.value = null;
        siteToRemove.value = '';
      }
    };

    // View logs for a site
    const viewLogs = (name) => {
      // Open logs directly in a new window using the window manager
      import('../utils/window-manager').then(({ openLogsWindow }) => {
        openLogsWindow(name, 'webserver');
      });
    };
    
    // View backups for a site
    const viewBackups = (name) => {
      router.push({ name: 'SiteBackups', params: { siteName: name } });
    };
    
    // Update site modal controls
    const showUpdateModal = (siteName) => {
      selectedSiteName.value = siteName;
      updateModalVisible.value = true;
    };
    
    const hideUpdateModal = () => {
      updateModalVisible.value = false;
      selectedSiteName.value = null;
    };
    
    const handleSiteUpdated = () => {
      loadSites();
      hideUpdateModal();
    };

    // Diagnostic modal controls
    const showDiagnosticModal = () => {
      diagnosticModalVisible.value = true;
    };
    
    const hideDiagnosticModal = () => {
      diagnosticModalVisible.value = false;
    };
    
    // Check detailed status for a specific site
    const checkDetailedStatus = async (siteName) => {
      try {
        console.log(`Checking detailed status for site: ${siteName}`);
        const result = await invoke('check_site_status_detailed', { name: siteName });
        console.log('Detailed status result:', result);
        
        // Format the detailed error information
        let errorDetails = `<h3 class="font-semibold mb-2">Site: ${siteName}</h3>`;
        
        if (result.status !== 'running') {
          errorDetails += `<p class="mb-2"><strong>Status:</strong> ${result.status}</p>`;
        }
        
        if (result.error_details && result.error_details.length > 0) {
          errorDetails += '<p class="mb-2"><strong>Error Details:</strong></p>';
          errorDetails += '<ul class="list-disc list-inside space-y-1">';
          result.error_details.forEach(error => {
            errorDetails += `<li class="text-sm">${error}</li>`;
          });
          errorDetails += '</ul>';
        }
        
        if (result.container_states) {
          errorDetails += '<p class="mt-3 mb-2"><strong>Container States:</strong></p>';
          errorDetails += '<div class="space-y-1">';
          Object.entries(result.container_states).forEach(([container, state]) => {
            const stateColor = state === 'running' ? 'text-green-600' : 'text-red-600';
            errorDetails += `<div class="text-sm"><span class="font-medium">${container}:</span> <span class="${stateColor}">${state}</span></div>`;
          });
          errorDetails += '</div>';
        }
        
        if (result.directory_checks) {
          errorDetails += '<p class="mt-3 mb-2"><strong>Directory Checks:</strong></p>';
          errorDetails += '<div class="space-y-1">';
          result.directory_checks.forEach(check => {
            const statusColor = check.exists ? 'text-green-600' : 'text-red-600';
            errorDetails += `<div class="text-sm"><span class="font-medium">${check.path}:</span> <span class="${statusColor}">${check.exists ? 'exists' : 'missing'}</span></div>`;
          });
          errorDetails += '</div>';
        }
        
        showError('Detailed Site Status', errorDetails);
      } catch (err) {
        console.error(`Failed to check detailed status for ${siteName}:`, err);
        showError('Status Check Failed', `Failed to get detailed status for ${siteName}: ${err}`);
      }
    };

    // Status styling methods
    const getStatusClasses = (status) => {
      switch (status) {
        case 'running':
          return 'bg-green-100 text-green-800 border-green-500 dark:bg-green-900 dark:text-green-300 dark:border-green-500';
        case 'error':
          return 'bg-red-100 text-red-800 border-red-500 dark:bg-red-900 dark:text-red-300 dark:border-red-500';
        case 'stopped':
          return 'bg-red-100 text-red-800 border-red-500 dark:bg-red-900 dark:text-red-300 dark:border-red-500';
        default: // For 'starting', 'stopping', or any other status
          return 'bg-yellow-100 text-yellow-800 border-yellow-500 dark:bg-yellow-900 dark:text-yellow-300 dark:border-yellow-500';
      }
    };

    const getStatusIndicatorClasses = (status) => {
      switch (status) {
        case 'running':
          return 'bg-green-500';
        case 'error':
          return 'bg-red-500';
        case 'stopped':
          return 'bg-red-500'; // Changed from gray to red
        default:
          return 'bg-yellow-500';
      }
    };

    // Format long paths for display
    const formatPath = (path) => {
      if (!path) return 'N/A';
      
      // If path is too long, truncate it
      const maxLength = 30;
      if (path.length <= maxLength) return path;
      
      // Extract directory name and show ellipsis
      const parts = path.split('/');
      const filename = parts.pop();
      return `.../${parts.slice(-2).join('/')}/${filename}`;
    };
    
    // Extract Moodle version from branch name
    const extractMoodleVersion = (branchName) => {
      if (!branchName) return 'N/A';
      
      // Common format: MOODLE_402_STABLE or MOODLE_311_STABLE
      const parts = branchName.split('_');
      if (parts.length >= 2) {
        // Check if the second part is numeric
        const versionPart = parts[1];
        if (/^\d+$/.test(versionPart)) {
          return versionPart;
        }
      }
      
      // Fallback for other formats
      return branchName;
    };
    
    // Format versions display as "MoodleVer / PHPVer"
    const formatVersions = (site) => {
      const moodleVer = extractMoodleVersion(site.moodle_branch);
      const phpVer = site.php_version;
      
      return `${moodleVer} / ${phpVer}`;
    };

    // Open local directory in system file explorer via backend
    const openDirectory = async (path) => {
      try {
        await invoke('open_site_folder', { path });
      } catch (e) {
        console.error('Failed to open directory via backend:', e);
      }
    };

    // Open in selected code editor
    const openInEditor = async (path) => {
      try {
        // retrieve editor executable from settings store
        const editorPath = settingsStore.editorPath;
        if (!editorPath) {
          return showError(
            'No editor configured',
            'Please set your code editor executable path in the application settings.'
          );
        }
        
        // Extract the application name from the path (between last / and .app)
        let appName = editorPath;
        const match = editorPath.match(/\/([^\/]+)\.app\/?$/);
        if (match && match[1]) {
          appName = match[1];
        }
        
        // Use Tauri's backend function to open the editor
        // This assumes you'll create a Rust function called open_in_editor on the backend
        await invoke('open_in_editor', { 
          editorName: appName,
          path: path 
        });
      } catch (e) {
        console.error('Failed to open in editor:', e);
        showError('Open in Editor failed', String(e));
      }
    };

    // Computed properties and methods for search, sort, and pagination
    const filteredSites = computed(() =>
      sites.value.filter(s => {
        const q = searchQuery.value.toLowerCase();
        // Include moodle_branch in search fields and also search in the formatted version string
        return ['name','php_version','moodle_branch','db_type','path','status']
          .some(k => String(s[k]||'').toLowerCase().includes(q)) || 
          formatVersions(s).toLowerCase().includes(q);
      })
    );

    const sortedSites = computed(() => {
      const arr = [...filteredSites.value];
      if (!sortKey.value) return arr;
      
      return arr.sort((a,b) => {
        // Special case for combined version sorting
        if (sortKey.value === 'version_combined') {
          // First sort by Moodle version, then by PHP version
          const moodleVerA = extractMoodleVersion(a.moodle_branch);
          const moodleVerB = extractMoodleVersion(b.moodle_branch);
          
          // Try to convert to numbers for numeric comparison
          const numA = parseInt(moodleVerA);
          const numB = parseInt(moodleVerB);
          
          if (!isNaN(numA) && !isNaN(numB)) {
            // Primary sort by Moodle version (numeric)
            if (numA !== numB) {
              return sortOrder.value === 'asc' ? numA - numB : numB - numA;
            }
          } else if (moodleVerA !== moodleVerB) {
            // Primary sort by Moodle version (string)
            return sortOrder.value === 'asc' ? 
              moodleVerA.localeCompare(moodleVerB) : 
              moodleVerB.localeCompare(moodleVerA);
          }
          
          // Secondary sort by PHP version
          return sortOrder.value === 'asc' ? 
            a.php_version.localeCompare(b.php_version) : 
            b.php_version.localeCompare(a.php_version);
        }
        
        // Default sort for other columns
        const A = String(a[sortKey.value]||'').toLowerCase();
        const B = String(b[sortKey.value]||'').toLowerCase();
        return sortOrder.value === 'asc' ? A.localeCompare(B) : B.localeCompare(A);
      });
    });

    const totalPages = computed(() =>
      Math.ceil(sortedSites.value.length / perPage)
    );

    const paginatedSites = computed(() => {
      const start = (currentPage.value - 1) * perPage;
      return sortedSites.value.slice(start, start + perPage);
    });

    function sort(column) {
      if (sortKey.value === column) {
        sortOrder.value = sortOrder.value==='asc' ? 'desc' : 'asc';
      } else {
        sortKey.value = column;
        sortOrder.value = 'asc';
      }
      currentPage.value = 1;
    }

    let refreshTimer;
    let progressUnlistener;
    let completeUnlistener;
    
    onMounted(async () => {
      // Load settings first to ensure editor path is available
      await settingsStore.loadSettings();
      
      loadSites();
      // Poll for status changes every 5 seconds without full re-render
      refreshTimer = setInterval(refreshStatuses, 5000);
      // Load recently created sites from local storage
      loadRecentlyCreatedSites();
      
      // Listen for site removal progress events
      progressUnlistener = await listen('site_removal_progress', (event) => {
        console.log('Site removal progress:', event.payload);
        if (isRemovingSite.value) {
          const { site, percent, step } = event.payload;
          // Ensure this event is for the site we're removing
          if (site === removingSiteName.value) {
            removalProgress.value = percent;
            
            // Update status message based on step
            switch (step) {
              case 'validating':
                removalStatus.value = 'Validating site...';
                break;
              case 'stopping_containers':
                removalStatus.value = 'Stopping containers...';
                break;
              case 'cleaning_networks':
                removalStatus.value = 'Cleaning up networks...';
                break;
              case 'containers_removed':
                removalStatus.value = 'Containers removed successfully';
                break;
              case 'removing_files':
                removalStatus.value = 'Removing site files...';
                break;
              case 'updating_config':
                removalStatus.value = 'Updating configuration...';
                break;
              case 'completed':
                removalStatus.value = 'Site removal completed';
                break;
              default:
                removalStatus.value = `Processing: ${step}`;
            }
          }
        }
      });
      
      // Listen for site removal completion events
      completeUnlistener = await listen('site_removed', (event) => {
        console.log('Site removal complete:', event.payload);
        if (isRemovingSite.value) {
          // Reset removal state
          isRemovingSite.value = false;
          removalProgress.value = 0;
          removalStatus.value = '';
          removingSiteName.value = '';
          activeActionSite.value = null;
          siteToRemove.value = '';
        }
      });
    });
    
    onBeforeUnmount(() => {
      clearInterval(refreshTimer);
      if (progressUnlistener) progressUnlistener();
      if (completeUnlistener) completeUnlistener();
    });
    
    // Expose the loadSites method to parent components
    expose({ loadSites });

    return {
      sites,
      loading,
      activeActionSite,
      error,
      startSite,
      stopSite,
      removeSite,
      viewLogs,
      viewBackups,
      showUpdateModal,
      hideUpdateModal,
      handleSiteUpdated,
      formatPath,
      extractMoodleVersion,
      formatVersions,
      openDirectory,
      openInEditor,
      updateModalVisible,
      selectedSiteName,
      showErrorModal,
      errorContent,
      closeErrorModal,
      // Diagnostic modal
      diagnosticModalVisible,
      showDiagnosticModal,
      hideDiagnosticModal,
      checkDetailedStatus,
      getStatusClasses,
      getStatusIndicatorClasses,
      // Recently created sites highlight
      isRecentlyCreated,
      getCreationTime,
      // New removal modal properties and methods
      showRemoveModal,
      siteToRemove,
      removeContainers,
      cancelRemove,
      confirmRemove,
      // Site removal progress tracking
      isRemovingSite,
      removalProgress,
      removalStatus,
      removingSiteName,
      // Search, sort, and pagination properties
      searchQuery,
      sortKey, sortOrder, sort,
      currentPage, totalPages, paginatedSites,
      // Recently created sites
      recentlyCreatedSites,
      isRecentlyCreated,
      getCreationTime,
      // Success message
      successMessage,
      stopMessage
    };
  }
};
</script>

<style scoped>
.moodle-header-cell {
  cursor: pointer;
}

.moodle-table-row {
  transition: background-color 0.2s ease-in-out;
}

.moodle-table-row:hover {
  background-color: #f0f0f0; /* Lighter gray for light mode hover */
}

.dark .moodle-table-row:hover {
  background-color: #2c334b; /* Slightly lighter on hover for dark theme */
}

/* Ensure the overall container has a dark background if it's not already set by a parent */
.dark .site-list-container {
  background-color: #1a202c; /* A common dark background color */
}

/* Custom grid layout for the table */
.grid-cols-custom-table-layout {
  grid-template-columns: repeat(2, minmax(0, 1fr)) repeat(3, minmax(0, 0.75fr)) minmax(0, 1fr) repeat(2, minmax(0, 0.75fr)); 
  /* Site Name (2fr), Version (0.75fr), DB (0.75fr), Status (0.75fr), Resources (1fr), Actions (1.5fr) */
}

.bg-gray-750 {
  background-color: #2c334b; /* Use the hover color for alternating rows */
}
</style>
<!-- Global light mode fixes for SiteList -->
<style src="../assets/site-list-light-mode-fix.css"></style>
