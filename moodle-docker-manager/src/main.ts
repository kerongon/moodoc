import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { createPinia } from 'pinia';
import { useSettingsStore } from './stores/settings';

// Import CSS directly - this helps ensure proper CSS processing
import './index.css';
// Import compact UI styles
import './assets/compact-ui.css';
// Import compact UI fixes
import './assets/compact-ui-fix.css';

// Import icons
import * as PhosphorIcons from '@phosphor-icons/vue';

// Create the app instance
const app = createApp(App);

// Register plugins
const pinia = createPinia();
app.use(router);
app.use(pinia);

// Register global components
// Register all Phosphor icons as global components
Object.entries(PhosphorIcons).forEach(([name, component]) => {
  app.component(name, component);
});

// Mount the app
app.mount('#app');

// Initialize settings and apply dark mode
const settingsStore = useSettingsStore();
settingsStore.loadSettings().then(() => {
  settingsStore.applyDarkMode();
});