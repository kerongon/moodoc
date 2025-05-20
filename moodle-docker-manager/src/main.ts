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

// Define the ThemeManager type
interface ThemeManager {
  applyTheme(mode: string): boolean;
  refreshTheme(): void;
}

// Extend Window interface
declare global {
  interface Window {
    ThemeManager: ThemeManager;
  }
}

// Create a global theme manager
window.ThemeManager = {
  // Force apply the theme
  applyTheme(mode: string) {
    const html = document.documentElement;
    const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)').matches;

    // Determine if dark mode should be applied
    const useDarkMode =
      mode === 'dark' ||
      (mode === 'system' && prefersDarkScheme);

    // Apply or remove the dark class
    if (useDarkMode) {
      html.classList.add('dark');
      document.body.setAttribute('data-theme', 'dark');
    } else {
      html.classList.remove('dark');
      document.body.setAttribute('data-theme', 'light');
    }

    // Force reset all tailwind dark classes if needed
    if (!useDarkMode) {
      // Target common Tailwind dark mode classes and force reset them
      document.querySelectorAll('[class*="dark:bg-"]').forEach(el => {
        // Remove the inline style if it was set
        (el as HTMLElement).style.removeProperty('background-color');
      });

      document.querySelectorAll('[class*="dark:text-"]').forEach(el => {
        // Remove the inline style if it was set
        (el as HTMLElement).style.removeProperty('color');
      });

      document.querySelectorAll('[class*="dark:border-"]').forEach(el => {
        // Remove the inline style if it was set
        (el as HTMLElement).style.removeProperty('border-color');
      });
    }

    // Explicitly set colors to ensure proper theme application
    if (useDarkMode) {
      document.body.style.setProperty('background-color', '#111827', 'important');
      document.body.style.setProperty('color', '#f9fafb', 'important');
    } else {
      document.body.style.setProperty('background-color', '#ffffff', 'important');
      document.body.style.setProperty('color', '#111827', 'important');
    }

    // Save to localStorage for persistence
    localStorage.setItem('theme', mode);
    
    // Dispatch a custom event that components can listen for
    window.dispatchEvent(new CustomEvent('themechange', { detail: { darkMode: useDarkMode } }));
    
    console.log(`Theme applied: ${useDarkMode ? 'dark' : 'light'} mode (${mode})`);
    return useDarkMode;
  },
  
  // Method to do a hard refresh of the theme (can be called from DevTools if needed)
  refreshTheme() {
    const mode = localStorage.getItem('theme') || 'system';
    
    // Apply CSS to force redraw
    document.body.style.display = 'none';
    
    // Force a DOM repaint
    void document.body.offsetHeight;
    
    // Re-apply theme
    this.applyTheme(mode);
    
    // Restore display
    document.body.style.display = '';
    
    console.log('Theme forcefully refreshed');
    
    // Update all CSS custom properties just in case
    if (mode === 'light' || (mode === 'system' && !window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.style.setProperty('--bg-color', '#ffffff');
      document.documentElement.style.setProperty('--text-color', '#111827');
      document.documentElement.style.setProperty('--border-color', '#e5e7eb');
    } else {
      document.documentElement.style.setProperty('--bg-color', '#111827');
      document.documentElement.style.setProperty('--text-color', '#f9fafb');
      document.documentElement.style.setProperty('--border-color', '#374151');
    }
  }
};

// Apply theme from localStorage first (before rendering)
const savedTheme = localStorage.getItem('theme') || 'system';
// @ts-ignore - Using window.ThemeManager
window.ThemeManager.applyTheme(savedTheme);

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

// Initialize settings
const settingsStore = useSettingsStore();
settingsStore.loadSettings().then(() => {
  // Apply theme again after settings load (in case settings differ from localStorage)
  window.ThemeManager.applyTheme(settingsStore.darkMode);

  // Set up a system preference change listener
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', () => {
    if (settingsStore.darkMode === 'system') {
      // @ts-ignore - Using window.ThemeManager
      window.ThemeManager.applyTheme('system');
    }
  });
});