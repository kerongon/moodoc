<script>
import { ref, onMounted, onBeforeUnmount, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { confirm } from '@tauri-apps/api/dialog';
import { useRouter } from 'vue-router';
import { eventBus } from '../utils/event-bus';

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
    const showLogs = ref(true);
    const creationSuccess = ref(false);

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

    const startSiteCreation = async () => {
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
      
      // Update time every second
      timeInterval.value = setInterval(() => {
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
          // Here you would ideally implement cleanup logic
          logs.value.push({ time: new Date().toLocaleTimeString(), message: 'Site creation cancelled by user.' });
          emit('close');
          router.push('/');
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
      goToDashboard
    };
  }
};
</script>

<template>
  <div class="modal-backdrop" ref="modalBackdrop" @click.self="handleBackdropClick">
    <div class="modal-content">
      <h2>Creating site: {{ siteName }}</h2>
      
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: `${progress}%` }"></div>
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
          <span class="toggle-icon">{{ showLogs ? '▼' : '▶' }}</span>
        </div>
        <div class="logs-container" v-if="showLogs" style="max-height: 200px; overflow-y: auto;">
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
  background-color: rgba(0, 0, 0, 0.7); /* Darker background for better visibility */
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999; /* Maximum z-index to ensure it's above all other elements */
  backdrop-filter: blur(3px); /* Add subtle blur effect to background */
  transition: all 0.3s ease;
}

.dark .modal-backdrop {
  background-color: rgba(0, 0, 0, 0.8); /* Even darker in dark mode */
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

h2 {
  margin-top: 0;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
}

.dark h2 {
  border-bottom: 1px solid #374151;
  color: #f9fafb;
}

.progress-container {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 5px;
}

.progress-bar {
  flex-grow: 1;
  height: 10px;
  background-color: #eee;
  border-radius: 5px;
  overflow: hidden;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
}

.dark .progress-bar {
  background-color: #374151;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.3);
}

.progress-fill {
  height: 100%;
  background-color: #5755d9;
  transition: width 0.5s ease-in-out;
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.15) 75%, transparent 75%, transparent);
  background-size: 1rem 1rem;
  animation: progress-animation 1s linear infinite;
}

.dark .progress-fill {
  background-color: #6366f1; /* Slightly brighter indigo in dark mode */
}

.progress-text {
  min-width: 50px;
  text-align: right;
  font-weight: bold;
  font-size: 1.1em;
}

.time-info {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: #666;
  margin-top: 8px;
  margin-bottom: 15px;
  padding: 0 5px;
  font-family: monospace;
}

.time-elapsed {
  color: #5755d9;
  font-weight: 600;
  background-color: rgba(87, 85, 217, 0.1);
  padding: 3px 8px;
  border-radius: 4px;
}

.dark .time-elapsed {
  color: #a5b4fc; /* Even brighter indigo for elapsed time in dark mode */
  background-color: rgba(99, 102, 241, 0.2);
}

.time-remaining {
  color: #666;
  background-color: rgba(0, 0, 0, 0.05);
  padding: 3px 8px;
  border-radius: 4px;
}

.dark .time-remaining {
  color: #d1d5db;
  background-color: rgba(255, 255, 255, 0.05);
}

.status-container {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.status-text {
  font-size: 16px;
  text-align: center;
  min-height: 24px;
}

.dark .status-text {
  color: #e5e7eb;
}

.animation-container {
  height: 120px;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f8f9fa;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.05);
  position: relative;
  transition: all 0.3s ease-in-out;
}

.dark .animation-container {
  background-color: #111827;
  border: 1px solid #374151;
}

/* Animation styles */
.animation {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.preparing .gear {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  border: 6px dashed #5755d9;
  animation: spin 3s linear infinite;
}

.preparing .document {
  position: absolute;
  width: 30px;
  height: 40px;
  background-color: #fff;
  border: 2px solid #5755d9;
  animation: moveLeftRight 2s ease-in-out infinite;
}

.container .docker-container {
  width: 80px;
  height: 60px;
  background-color: #5755d9;
  border-radius: 6px;
  position: relative;
  animation: pulse 2s ease-in-out infinite;
}

.container .docker-container::before {
  content: "";
  position: absolute;
  width: 20px;
  height: 20px;
  background-color: #fff;
  border-radius: 50%;
  top: 20px;
  left: 15px;
}

.database .database-icon {
  width: 60px;
  height: 40px;
  background-color: #5755d9;
  border-radius: 4px;
  position: relative;
  animation: blink 1s ease-in-out infinite;
}

.database .database-icon::before {
  content: "";
  position: absolute;
  width: 50px;
  height: 5px;
  background-color: #fff;
  top: 10px;
  left: 5px;
}

.database .database-icon::after {
  content: "";
  position: absolute;
  width: 50px;
  height: 5px;
  background-color: #fff;
  bottom: 10px;
  left: 5px;
}

.installing .code-lines {
  width: 80px;
  height: 60px;
  background-color: #f8f9fa;
  border: 2px solid #5755d9;
  position: relative;
}

.dark .installing .code-lines {
  background-color: #1f2937;
  border: 2px solid #6366f1;
}

.installing .code-lines::before,
.installing .code-lines::after {
  content: "";
  position: absolute;
  left: 10px;
  height: 2px;
  background-color: #5755d9;
  animation: typingCode 2s ease-in-out infinite;
}

.installing .code-lines::before {
  top: 15px;
  width: 60%;
}

.installing .code-lines::after {
  top: 30px;
  width: 40%;
}

.complete .checkmark {
  width: 60px;
  height: 60px;
  background-color: #4caf50;
  border-radius: 50%;
  position: relative;
}

.complete .checkmark::before {
  content: "";
  position: absolute;
  width: 30px;
  height: 15px;
  border-left: 4px solid white;
  border-bottom: 4px solid white;
  transform: rotate(-45deg);
  top: 20px;
  left: 15px;
}

.complete .error-icon {
  width: 60px;
  height: 60px;
  background-color: #f44336;
  border-radius: 50%;
  position: relative;
}

.complete .error-icon::before,
.complete .error-icon::after {
  content: "";
  position: absolute;
  width: 4px;
  height: 30px;
  background-color: white;
  top: 15px;
  left: 28px;
}

.complete .error-icon::before {
  transform: rotate(45deg);
}

.complete .error-icon::after {
  transform: rotate(-45deg);
}

/* Animation keyframes */
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes moveLeftRight {
  0%, 100% { transform: translateX(-20px); }
  50% { transform: translateX(20px); }
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes typingCode {
  0%, 100% { width: 30%; }
  50% { width: 70%; }
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

@keyframes progress-animation {
  0% {
    background-position: 1rem 0;
  }
  100% {
    background-position: 0 0;
  }
}

/* Log accordion styles */
.logs-accordion {
  border: 1px solid #ddd;
  border-radius: 6px;
  overflow: hidden;
  transition: all 0.3s ease;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.05);
}

.logs-header {
  background-color: #f5f5f5;
  padding: 10px 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  user-select: none;
}

.dark .logs-header {
  background-color: #1f2937;
  color: #f9fafb;
  border-bottom: 1px solid #374151;
}

.logs-header h3 {
  margin: 0;
  font-size: 16px;
}

.toggle-icon {
  font-size: 14px;
}

.logs-container {
  max-height: 200px;
  overflow-y: auto;
  border: 1px solid #eee;
  border-radius: 6px;
  padding: 10px;
  margin-top: 5px;
  background-color: #f8f9fa;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.05);
  font-family: monospace;
  font-size: 0.9em;
}

.dark .logs-container {
  background-color: #111827;
  border-color: #374151;
}

.empty-logs-message {
  padding: 10px;
  color: #888;
  font-style: italic;
  text-align: center;
}

.log-entry {
  padding: 2px 0;
  display: flex;
  gap: 10px;
  border-bottom: 1px solid #ccc;
}

.dark .log-entry {
  border-bottom: 1px solid #374151;
}

.log-time {
  color: #888;
  white-space: nowrap;
}

.dark .log-time {
  color: #9ca3af;
}

.log-message {
  word-break: break-word;
}

.dark .log-message {
  color: #e5e7eb;
}

.dark .empty-logs-message {
  color: #9ca3af;
}

/* Button styles */
.button-container {
  display: flex;
  justify-content: center; /* Center buttons */
  gap: 10px;
  margin-top: 20px;
  width: 100%;
}

.cancel-button {
  background-color: #f44336;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 6px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  text-transform: uppercase;
  font-size: 0.9em;
  letter-spacing: 0.5px;
}

.dark .cancel-button {
  background-color: #ef4444;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.cancel-button:hover {
  background-color: #d32f2f;
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.dark .cancel-button:hover {
  background-color: #dc2626;
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.4);
}

.done-button {
  background-color: #5755d9;
  color: white;
  border: none;
  padding: 12px 30px;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500; /* Slightly lighter weight */
  transition: all 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  text-transform: uppercase; /* Removed for dashboard button */
  font-size: 1rem; /* Larger font size */
  letter-spacing: 0.5px;
  width: 60%; /* Make button wider but not full width */
  text-align: center;
}

.dark .done-button {
  background-color: #5755d9; /* Match the color from the screenshot */
  color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  border-radius: 8px;
  padding: 15px 30px 15px; /* Equal padding top and bottom */
  font-size: 1rem;
  letter-spacing: 0.7px;
  font-weight: 600;
  text-transform: uppercase;
  width: auto; 
  min-width: 250px;
  text-align: center;
  justify-content: center;
  line-height: 1.2; /* Slightly increased line height */
  display: flex;
  align-items: center;
  height: 50px; /* Fixed height to ensure proper centering */
}

.done-button:hover {
  background-color: #4644b8;
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.dark .done-button:hover {
  background-color: #4f46e5;
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.4);
  padding: 15px 30px 15px; /* Maintain the same padding on hover */
  height: 50px; /* Maintain the same height on hover */
}
</style>
