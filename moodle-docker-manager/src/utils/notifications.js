import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

/**
 * Utility functions for handling notifications in the application
 */

/**
 * Check if notification permissions are granted
 * Returns true if permissions are granted, false otherwise
 */
export async function checkNotificationPermission() {
  try {
    let permissionGranted = await isPermissionGranted();
    
    // If permission is not granted, request it
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    
    return permissionGranted;
  } catch (error) {
    console.error('Failed to check notification permissions:', error);
    return false;
  }
}

/**
 * Send a notification
 * @param {string} title - The notification title
 * @param {string} body - The notification body text
 * @param {string} icon - Optional path to notification icon
 * @returns {Promise<boolean>} - True if notification was sent, false otherwise
 */
export async function sendAppNotification(title, body, icon = 'icons/icon.png') {
  try {
    const permissionGranted = await checkNotificationPermission();
    
    if (!permissionGranted) {
      console.warn('Notification permission not granted');
      return false;
    }
    
    await sendNotification({
      title,
      body,
      icon
    });
    
    return true;
  } catch (error) {
    console.error('Failed to send notification:', error);
    return false;
  }
}

/**
 * Track recently created sites in localStorage
 * @param {string} siteName - The name of the site that was created
 */
export function trackRecentSite(siteName) {
  try {
    const recentSites = JSON.parse(localStorage.getItem('recentlyCreatedSites') || '{}');
    recentSites[siteName] = Date.now();
    localStorage.setItem('recentlyCreatedSites', JSON.stringify(recentSites));
  } catch (storageErr) {
    console.error("Failed to update recently created sites:", storageErr);
  }
}

/**
 * Clean up old sites from the recently created sites list
 * Removes sites older than the specified time
 * @param {number} maxAgeMs - Maximum age in milliseconds (default: 1 hour)
 */
export function cleanupRecentSites(maxAgeMs = 3600000) {
  try {
    const storedRecent = localStorage.getItem('recentlyCreatedSites');
    if (storedRecent) {
      const recentlyCreatedSites = JSON.parse(storedRecent);
      
      // Clean up sites older than the specified time
      const now = Date.now();
      let hasChanges = false;
      
      Object.keys(recentlyCreatedSites).forEach(siteName => {
        const timestamp = recentlyCreatedSites[siteName];
        if (now - timestamp > maxAgeMs) {
          delete recentlyCreatedSites[siteName];
          hasChanges = true;
        }
      });
      
      if (hasChanges) {
        localStorage.setItem('recentlyCreatedSites', JSON.stringify(recentlyCreatedSites));
      }
    }
  } catch (err) {
    console.error("Failed to clean up recently created sites:", err);
  }
}
