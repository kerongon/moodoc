<script>
import { ref, onMounted, onBeforeUnmount, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { useRouter } from 'vue-router';
import { eventBus } from '../utils/event-bus';
import { checkNotificationPermission, sendAppNotification, trackRecentSite } from '../utils/notifications';

export default {
  name: 'SiteCreationProgress',
  props: {
    siteName: {
      type: String,
      default: ''
    },
    phpVersion: {
      type: String,
      default: ''
    },
    dbType: {
      type: String,
      default: ''
    },
    moodleBranch: {
      type: String, 
      default: ''
    },
    webPort: {
      type: Number,
      default: 0
    },
    dbPort: {
      type: Number,
      default: 0
    },
    directory: {
      type: String,
      default: ''
    }
  },
  emits: ['close'],
  setup(props, { emit }) {
    const router = useRouter();
    
    // Status references
    const progress = ref(0);
    const status = ref('Starting creation process...');
    const isCreating = ref(true);
    const logs = ref([]);
    const result = ref('');
    const error = ref(null);
    const showLogs = ref(false);
    const creationSuccess = ref(false);
    const copySuccess = ref(false);
    const notificationPermission = ref(false);

    // Time-based progress tracking
    const startTime = ref(Date.now());
    const estimatedTotalTime = ref(180000); // 3 minutes estimate in milliseconds
    const currentTime = ref(Date.now());
    const timeInterval = ref(null);

    // Animation element references
    const animationPhase = ref('preparing');

    // Computed elapsed time display
    const elapsedTimeDisplay = computed(() => {
      const elapsed = Math.floor((currentTime.value - startTime.value) / 1000);
      const minutes = Math.floor(elapsed / 60);
      const seconds = elapsed % 60;
      return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
    });

    // Computed estimated time remaining
    const estimatedTimeRemaining = computed(() => {
      if (progress.value >= 100) return '0:00';
      if (progress.value <= 0) return '3:00'; // Default to 3 minutes if no progress
      
      const elapsed = currentTime.value - startTime.value;
      // Avoid division by zero or very small numbers
      const progressPercent = Math.max(0.01, progress.value / 100);
      const estimatedTotal = elapsed / progressPercent;
      const remaining = Math.max(0, Math.floor((estimatedTotal - elapsed) / 1000));
      
      const minutes = Math.floor(remaining / 60);
      const seconds = remaining % 60;
      return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
    });

    let unlisten = null;

    // Function to check for notification permissions
    const checkPermissions = async () => {
      const permissionGranted = await checkNotificationPermission();
      notificationPermission.value = permissionGranted;
      console.log("Notification permission:", permissionGranted);
      return permissionGranted;
    };

    // Function to send a notification when site is created
    const sendSiteCreatedNotification = async () => {
      if (!notificationPermission.value) {
        const granted = await checkPermissions();
        if (!granted) return;
      }
      
      try {
        // Create a notification with site information
        const siteUrl = `http://localhost:${props.webPort}`;
        
        const success = await sendAppNotification(
          "Site Creation Complete!",
          `Your Moodle site "${props.siteName}" has been successfully created.\nClick to view site details.`
        );
        
        if (success) {
          console.log("Notification sent for site:", props.siteName);
        }
        
        // Store the site creation timestamp in localStorage to show a visual indicator
        trackRecentSite(props.siteName);
      } catch (err) {
        console.error("Failed to send notification:", err);
      }
    };

    const startSiteCreation = async () => {
      // Check for notification permissions early
      await checkPermissions();
      
      // Listen for site creation progress events
      unlisten = await eventBus.listenForSiteCreationProgress((msg) => {
        logs.value.push({ time: new Date().toLocaleTimeString(), message: msg });
        status.value = msg;
        
        // Update progress and animation phase based on message content
        if (msg.includes('Preparing')) {
          progress.value = 10;
          animationPhase.value = 'preparing';
        } else if (msg.includes('Cloning Moodle')) {
          progress.value = 20;
          animationPhase.value = 'preparing';
        } else if (msg.includes('Starting Docker')) {
          progress.value = 35;
          animationPhase.value = 'container';
        } else if (msg.includes('Waiting for database')) {
          progress.value = 50;
          animationPhase.value = 'database';
        } else if (msg.includes('Installing Moodle')) {
          progress.value = 70;
          animationPhase.value = 'installing';
        } else if (msg.includes('Finalizing')) {
          progress.value = 90;
          animationPhase.value = 'complete';
        }
        
        // Scroll to the bottom of the logs
        if (document.querySelector('.logs-container')) {
          const container = document.querySelector('.logs-container');
          container.scrollTop = container.scrollHeight;
        }
      });

      // Listen for site creation cancelled events
      const cancelListener = await eventBus.listenForSiteCreationCancelled((msg) => {
        logs.value.push({ time: new Date().toLocaleTimeString(), message: msg });
        status.value = 'Creation cancelled';
        isCreating.value = false;
        progress.value = 0;
        
        // Clean up the time interval
        if (timeInterval.value) {
          clearInterval(timeInterval.value);
          timeInterval.value = null;
        }
        
        // Close the dialog after a brief delay
        setTimeout(() => {
          emit('close');
        }, 2000);
      });

      // Store the cancel listener for cleanup
      const originalUnlisten = unlisten;
      unlisten = () => {
        if (originalUnlisten) originalUnlisten();
        if (cancelListener) cancelListener();
      };
      
      // Update time every second
      timeInterval.value = setInterval(() => {
        // Stop updating time if creation is complete or has error
        if (progress.value >= 100 || !isCreating.value) return;
        
        currentTime.value = Date.now();
        
        // Calculate progress based on time if no explicit progress updates
        if (isCreating.value && progress.value < 90) {
          const elapsed = currentTime.value - startTime.value;
          // Ensure we don't divide by zero with a minimum value
          const totalTime = Math.max(1, estimatedTotalTime.value);
          const timeBasedProgress = Math.min(90, Math.max(1, Math.floor((elapsed / totalTime) * 100)));
          
          // Only use time-based progress if it's ahead of our event-based progress
          if (timeBasedProgress > progress.value) {
            progress.value = timeBasedProgress;
          }
        }
        
        // Make sure progress is never NaN
        if (isNaN(progress.value)) {
          progress.value = 1; // Set to minimum value
        }
      }, 1000);
      
      try {
        logs.value.push({ time: new Date().toLocaleTimeString(), message: `Starting creation of site: ${props.siteName}` });
        logs.value.push({ time: new Date().toLocaleTimeString(), message: `PHP Version: ${props.phpVersion}, DB Type: ${props.dbType}` });
        logs.value.push({ time: new Date().toLocaleTimeString(), message: `Moodle Branch: ${props.moodleBranch}` });
        logs.value.push({ time: new Date().toLocaleTimeString(), message: `Ports - Web: ${props.webPort}, DB: ${props.dbPort}` });
        
        result.value = await invoke('create_site', {
          name: props.siteName,
          phpVersion: props.phpVersion,
          dbType: props.dbType,
          moodleBranch: props.moodleBranch,
          webPort: props.webPort,
          dbPort: props.dbPort,
          directory: props.directory,
        });
        
        logs.value.push({ time: new Date().toLocaleTimeString(), message: 'Site creation completed successfully!' });
        status.value = 'Site created successfully!';
        progress.value = 100;
        animationPhase.value = 'complete';
        isCreating.value = false;
        creationSuccess.value = true;
        
        // Send notification when site is created successfully
        await sendSiteCreatedNotification();
      } catch (err) {
        error.value = err;
        logs.value.push({ time: new Date().toLocaleTimeString(), message: `Error: ${err}` });
        status.value = 'Site creation failed.';
        progress.value = 100;
        isCreating.value = false;
        animationPhase.value = 'complete';
      }
    };

    const toggleLogs = () => {
      showLogs.value = !showLogs.value;
    };
    
    const copyLogs = (event) => {
      event.stopPropagation(); // Prevent toggling logs when clicking copy button
      
      if (logs.value.length === 0) return;
      
      // Format logs for clipboard
      const logText = logs.value.map(log => `[${log.time}] ${log.message}`).join('\n');
      
      // Use Clipboard API to copy text
      navigator.clipboard.writeText(logText)
        .then(() => {
          copySuccess.value = true;
          setTimeout(() => {
            copySuccess.value = false;
          }, 2000);
        })
        .catch(err => {
          console.error('Failed to copy logs: ', err);
        });
    };

    // Prevent accidental closing when clicking backdrop
    const handleBackdropClick = (event) => {
      event.stopPropagation();
    };

    const handleCancel = async () => {
      if (isCreating.value) {
        const confirmed = await confirm('Are you sure you want to cancel site creation? This may leave the system in an inconsistent state.', { 
          title: 'Confirm Cancellation',
          type: 'warning'
        });
        
        if (confirmed) {
          try {
            logs.value.push({ time: new Date().toLocaleTimeString(), message: 'Cancelling site creation...' });
            status.value = 'Cancelling...';
            
            // Call the backend to cancel site creation
            await eventBus.cancelSiteCreation(props.siteName);
            
            // Note: The actual cleanup and UI updates will be handled by the 
            // site_creation_cancelled event listener we set up above
          } catch (error) {
            console.error('Failed to cancel site creation:', error);
            logs.value.push({ time: new Date().toLocaleTimeString(), message: `Error cancelling site creation: ${error}` });
            
            // Fallback: just close the dialog
            emit('close');
            router.push('/');
          }
        }
      } else {
        emit('close');
        router.push('/');
      }
    };

    const goToDashboard = () => {
      emit('close');
      router.push('/');
    };
    
    onMounted(() => {
      console.log('SiteCreationProgress mounted with props:', props);
      // Make sure the progress starts at a valid value
      progress.value = 1;
      
      // Emit an initial progress event to ensure we see something right away
      logs.value.push({ time: new Date().toLocaleTimeString(), message: 'Initializing site creation process...' });
      status.value = 'Initializing site creation process...';
      
      // Start the actual site creation process after a very short delay
      // to ensure the modal is fully rendered
      setTimeout(() => {
        startSiteCreation();
      }, 100);
    });
    
    onBeforeUnmount(() => {
      if (unlisten) unlisten();
      if (timeInterval.value) clearInterval(timeInterval.value);
    });

    return {
      progress,
      status,
      isCreating,
      logs,
      result,
      error,
      showLogs,
      creationSuccess,
      animationPhase,
      elapsedTimeDisplay,
      estimatedTimeRemaining,
      toggleLogs,
      handleBackdropClick,
      handleCancel,
      goToDashboard,
      copyLogs,
      copySuccess
    };
  }
};
</script>

<template>
  <div class="modal-backdrop" ref="modalBackdrop" @click.self="handleBackdropClick">
    <div class="modal-content" data-tauri-drag-region>
      <h2>Creating site: {{ siteName }}</h2>
      
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" 
               :style="{ width: `${progress}%` }" 
               :class="{ 'complete': progress >= 100 && !error, 'has-error': error }">
          </div>
        </div>
        <div class="progress-text">{{ progress }}%</div>
      </div>
      
      <div class="time-info">
        <div class="time-elapsed">Elapsed: {{ elapsedTimeDisplay }}</div>
        <div class="time-remaining">Est. remaining: {{ estimatedTimeRemaining }}</div>
      </div>
      
      <div class="status-container">
        <div class="status-text">{{ status }}</div>
        
        <!-- Animation container based on the creation phase -->
        <div class="animation-container">
          <div v-if="animationPhase === 'preparing'" class="animation preparing">
            <div class="gear spinning"></div>
            <div class="document moving"></div>
          </div>
          <div v-else-if="animationPhase === 'container'" class="animation container">
            <div class="docker-container pulsing"></div>
          </div>
          <div v-else-if="animationPhase === 'database'" class="animation database">
            <div class="database-icon blinking"></div>
          </div>
          <div v-else-if="animationPhase === 'installing'" class="animation installing">
            <div class="code-lines"></div>
          </div>
          <div v-else-if="animationPhase === 'complete'" class="animation complete">
            <div class="checkmark" v-if="!error"></div>
            <div class="error-icon" v-else></div>
          </div>
        </div>
      </div>
      
      <!-- Log accordion -->
      <div class="logs-accordion">
        <div class="logs-header" @click="toggleLogs">
          <h3>Installation logs</h3>
          <div class="logs-controls">
            <button 
              class="copy-logs-btn" 
              @click.stop="copyLogs" 
              title="Copy logs to clipboard"
              :disabled="logs.length === 0"
            >
              <span class="copy-icon">📋</span>
              <span v-if="copySuccess" class="copy-success">Copied!</span>
            </button>
            <span class="toggle-icon">{{ showLogs ? '▼' : '▶' }}</span>
          </div>
        </div>
        <div class="logs-container hide-scrollbar" v-if="showLogs" @mousedown.stop>
          <div v-for="(log, index) in logs" :key="index" class="log-entry">
            <span class="log-time">{{ log.time }}</span>
            <span class="log-message">{{ log.message }}</span>
          </div>
          <div v-if="logs.length === 0" class="empty-logs-message">
            Waiting for logs...
          </div>
        </div>
      </div>
      
      <!-- Action buttons -->
      <div class="button-container">
        <button v-if="isCreating" class="cancel-button" @click="handleCancel">Cancel</button>
        <button v-else class="done-button" @click="goToDashboard">BACK TO DASHBOARD</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
  backdrop-filter: blur(3px);
}

.modal-content {
  background-color: #fff;
  border-radius: 12px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2), 0 6px 10px rgba(0, 0, 0, 0.1);
  padding: 28px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  display: flex;
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
  flex-direction: column;
  gap: 20px;
  animation: modal-appear 0.3s ease-out;
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.dark .modal-content {
  background-color: #1f2937;
  color: #f3f4f6;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5), 0 6px 10px rgba(0, 0, 0, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.modal-content::-webkit-scrollbar {
  display: none;  /* Chrome, Safari and Opera */
}

.modal-header {
  margin-bottom: 20px;
  text-align: center;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.dark .modal-header h2 {
  color: #e5e7eb;
}

.progress-container {
  margin-bottom: 20px;
  position: relative;
  display: flex;
  align-items: center;
}

.progress-bar {
  height: 20px;
  background-color: #e2e8f0;
  border-radius: 10px;
  overflow: hidden;
  flex-grow: 1;
  margin-right: 10px;
}

.dark .progress-bar {
  background-color: #4b5563;
}

.progress-fill {
  height: 100%;
  background-color: #5755d9;
  transition: width 0.5s ease-in-out;
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.15) 75%, transparent 75%, transparent);
  background-size: 1rem 1rem;
  animation: progress-animation 1s linear infinite;
}

.progress-fill.complete {
  background-color: #10b981;
  animation: none;
}

.progress-fill.has-error {
  background-color: #ef4444;
  animation: none;
}

.dark .progress-fill {
  background-color: #6366f1;
}

.dark .progress-fill.complete {
  background-color: #10b981;
}

.dark .progress-fill.has-error {
  background-color: #ef4444;
}

@keyframes progress-animation {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: 1rem 0;
  }
}

@keyframes modal-appear {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.progress-text {
  min-width: 50px;
  text-align: right;
  font-weight: bold;
  font-family: monospace;
  font-size: 1rem;
}

.time-info {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;
  font-size: 0.95rem;
}

.status-container {
  display: flex;
  flex-direction: column;
  gap: 15px;
  margin-bottom: 15px;
}

.status-text {
  font-weight: 500;
  text-align: center;
  font-size: 1.1rem;
}

.animation-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
  background-color: #f8fafc;
  border-radius: 8px;
  padding: 15px;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.05);
}

.dark .animation-container {
  background-color: #293548;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
}

.animation {
  width: 80px;
  height: 80px;
  position: relative;
}

/* Animation styling for preparing stage */
.animation.preparing .gear {
  position: absolute;
  width: 40px;
  height: 40px;
  background-color: #6366f1;
  border-radius: 50%;
  top: 20px;
  left: 20px;
}

.animation.preparing .document {
  position: absolute;
  width: 25px;
  height: 35px;
  background-color: white;
  border: 2px solid #6366f1;
  border-radius: 3px;
  top: 22px;
  left: 45px;
}

.spinning {
  animation: spin 2s linear infinite;
}

.moving {
  animation: move 2s ease-in-out infinite;
}

@keyframes move {
  0%, 100% { transform: translateX(-10px); }
  50% { transform: translateX(10px); }
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Animation styling for container stage */
.animation.container .docker-container {
  width: 60px;
  height: 40px;
  background-color: #2563eb;
  border-radius: 4px;
  position: absolute;
  top: 20px;
  left: 10px;
}

.pulsing {
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0% { transform: scale(1); }
  50% { transform: scale(1.1); }
  100% { transform: scale(1); }
}

/* Animation styling for database stage */
.animation.database .database-icon {
  width: 50px;
  height: 50px;
  background-color: #10b981;
  border-radius: 4px;
  position: absolute;
  top: 15px;
  left: 15px;
}

.blinking {
  animation: blink 1s ease-in-out infinite;
}

@keyframes blink {
  0% { opacity: 1; }
  50% { opacity: 0.5; }
  100% { opacity: 1; }
}

/* Animation styling for installing stage */
.animation.installing .code-lines {
  width: 60px;
  height: 40px;
  border-left: 4px solid #f59e0b;
  position: absolute;
  top: 20px;
  left: 10px;
  animation: typing 1s steps(5) infinite;
}

@keyframes typing {
  from { height: 0; }
  to { height: 40px; }
}

/* Animation styling for complete stage */
.animation.complete .checkmark {
  width: 40px;
  height: 20px;
  border-bottom: 8px solid #10b981;
  border-left: 8px solid #10b981;
  position: absolute;
  top: 30px;
  left: 20px;
  transform: rotate(-45deg);
  animation: complete-check 0.5s ease-in-out;
}

.animation.complete .error-icon {
  position: absolute;
  width: 60px;
  height: 60px;
}

.animation.complete .error-icon:before,
.animation.complete .error-icon:after {
  position: absolute;
  content: "";
  width: 8px;
  height: 60px;
  background-color: #ef4444;
  top: 10px;
  left: 26px;
}

.animation.complete .error-icon:before {
  transform: rotate(45deg);
}

.animation.complete .error-icon:after {
  transform: rotate(-45deg);
}

@keyframes complete-check {
  0% { opacity: 0; transform: scale(0) rotate(-45deg); }
  100% { opacity: 1; transform: scale(1) rotate(-45deg); }
}

.time-display {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;
  font-size: 0.95rem;
}

.time-elapsed, .time-remaining {
  display: flex;
  align-items: center;
}

.time-label {
  font-weight: 500;
  margin-right: 10px;
}

.time-value {
  font-family: monospace;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
  background-color: rgba(87, 85, 217, 0.1);
}

.dark .time-value {
  background-color: rgba(99, 102, 241, 0.2);
  color: #e5e7eb;
}

.logs-accordion {
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  overflow: hidden;
}

.dark .logs-accordion {
  border-color: #4b5563;
}

.logs-header {
  background-color: #f1f5f9;
  padding: 12px 15px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dark .logs-header {
  background-color: #374151;
}

.logs-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 500;
}

.logs-controls {
  display: flex;
  align-items: center;
  gap: 12px;
}

.copy-logs-btn {
  display: inline-flex;
  align-items: center;
  background-color: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.85rem;
  color: #5755d9;
  padding: 2px 8px;
  border-radius: 4px;
  position: relative;
}

.copy-logs-btn:hover {
  background-color: rgba(87, 85, 217, 0.1);
}

.copy-logs-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dark .copy-logs-btn {
  color: #818cf8;
}

.dark .copy-logs-btn:hover {
  background-color: rgba(129, 140, 248, 0.2);
}

.copy-icon {
  font-size: 1rem;
}

.copy-success {
  position: absolute;
  bottom: -20px;
  left: 50%;
  transform: translateX(-50%);
  background-color: #10b981;
  color: white;
  font-size: 0.75rem;
  padding: 2px 6px;
  border-radius: 4px;
  white-space: nowrap;
  animation: fadeInOut 2s ease forwards;
}

@keyframes fadeInOut {
  0% { opacity: 0; }
  20% { opacity: 1; }
  80% { opacity: 1; }
  100% { opacity: 0; }
}

.logs-container {
  overflow-y: auto;
  background-color: #f5f5f5;
  border-radius: 0 0 8px 8px;
  padding: 15px;
  max-height: 200px;
  font-family: monospace;
  font-size: 0.9rem;
  white-space: pre-wrap;
  user-select: text;
  -webkit-user-select: text;
  cursor: text;
}

.dark .logs-container {
  background-color: #374151;
  color: #e5e7eb;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.log-entry {
  display: flex;
  margin-bottom: 6px;
  line-height: 1.4;
}

.log-time {
  font-weight: 600;
  color: #64748b;
  margin-right: 10px;
  min-width: 80px;
}

.dark .log-time {
  color: #9ca3af;
}

.log-message {
  flex: 1;
  word-break: break-word;
}

.log-error {
  color: #ef4444;
  font-weight: 500;
}

.dark .log-error {
  color: #f87171;
}

.empty-logs-message {
  color: #94a3b8;
  font-style: italic;
  text-align: center;
  padding: 10px;
}

.dark .empty-logs-message {
  color: #64748b;
}

.button-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.done-button {
  background-color: #5755d9;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 15px 30px 15px;
  font-size: 1rem;
  letter-spacing: 0.7px;
  font-weight: 600;
  text-transform: uppercase;
  cursor: pointer;
  width: auto; 
  min-width: 250px;
  text-align: center;
  justify-content: center;
  line-height: 1.2;
  display: flex;
  align-items: center;
  height: 50px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.done-button:hover:not(.disabled) {
  background-color: #4240c9;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
  transform: translateY(-1px);
}

.done-button.disabled {
  background-color: #9d9dd6;
  cursor: not-allowed;
  opacity: 0.7;
  box-shadow: none;
}

.dark .done-button {
  background-color: #6366f1;
  color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.dark .done-button:hover:not(.disabled) {
  background-color: #4f46e5;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
}

.dark .done-button.disabled {
  background-color: #6366f1;
  opacity: 0.5;
  box-shadow: none;
}

.cancel-button {
  background-color: #94a3b8;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 12px 25px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-button:hover {
  background-color: #64748b;
}

.dark .cancel-button {
  background-color: #4b5563;
}

.dark .cancel-button:hover {
  background-color: #374151;
}

/* Import light mode fixes */
@import '../assets/site-creation-progress-light-mode-fix.css';
</style>
