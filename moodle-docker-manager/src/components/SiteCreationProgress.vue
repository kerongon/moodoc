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
        <button v-else class="done-button" @click="goToDashboard">Back to Dashboard</button>
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
}

.modal-content {
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  padding: 25px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

h2 {
  margin-top: 0;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
}

.progress-container {
  display: flex;
  align-items: center;
  gap: 10px;
}

.progress-bar {
  flex-grow: 1;
  height: 8px;
  background-color: #eee;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background-color: #5755d9;
  transition: width 0.5s ease-in-out;
}

.progress-text {
  min-width: 40px;
  text-align: right;
  font-weight: bold;
}

.time-info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #666;
  margin-top: 5px;
  margin-bottom: 15px;
}

.time-elapsed {
  color: #5755d9;
  font-weight: 500;
}

.time-remaining {
  color: #666;
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

.animation-container {
  height: 120px;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f8f9fa;
  border-radius: 8px;
  overflow: hidden;
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

/* Log accordion styles */
.logs-accordion {
  border: 1px solid #ddd;
  border-radius: 4px;
  overflow: hidden;
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
  border-radius: 4px;
  padding: 10px;
  margin-top: 5px;
  background-color: #f8f9fa;
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
  border-bottom: 1px solid #333;
}

.log-time {
  color: #888;
  white-space: nowrap;
}

.log-message {
  word-break: break-word;
}

/* Button styles */
.button-container {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 10px;
}

.cancel-button {
  background-color: #f44336;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.2s;
}

.cancel-button:hover {
  background-color: #d32f2f;
}

.done-button {
  background-color: #5755d9;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.2s;
}

.done-button:hover {
  background-color: #4644b8;
}
</style>
