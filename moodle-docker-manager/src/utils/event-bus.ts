import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';

/**
 * Provides a global event bus for site creation events
 */
class EventBus {
  /**
   * Emit an event to show site creation progress
   * @param params Site creation parameters
   */
  async emitShowSiteCreation(params: {
    siteName: string;
    phpVersion: string;
    dbType: string;
    moodleBranch: string;
    webPort: number;
    dbPort: number;
    directory: string;
  }): Promise<void> {
    await emit('show_site_creation', params);
  }

  /**
   * Emit a site creation progress event
   * @param message Progress message
   */
  async emitSiteCreationProgress(message: string): Promise<void> {
    await emit('site_creation_progress', message);
  }

  /**
   * Listen for site creation events
   * @param callback Function to call when event is received
   * @returns Unlisten function
   */
  async listenForSiteCreation(callback: (params: any) => void): Promise<() => void> {
    return await listen('show_site_creation', (event) => {
      callback(event.payload);
    });
  }

  /**
   * Listen for site creation progress events
   * @param callback Function to call when event is received
   * @returns Unlisten function
   */
  async listenForSiteCreationProgress(callback: (message: string) => void): Promise<() => void> {
    return await listen('site_creation_progress', (event) => {
      callback(event.payload);
    });
  }

  /**
   * Listen for site creation cancelled events
   * @param callback Function to call when event is received
   * @returns Unlisten function
   */
  async listenForSiteCreationCancelled(callback: (message: string) => void): Promise<() => void> {
    return await listen('site_creation_cancelled', (event) => {
      callback(event.payload as string);
    });
  }

  /**
   * Cancel site creation
   * @param siteName Site name to cancel
   */
  async cancelSiteCreation(siteName: string): Promise<void> {
    try {
      await invoke('cancel_site_creation', { name: siteName });
      await emit('site_creation_cancelled', { siteName });
    } catch (error) {
      console.error('Failed to cancel site creation:', error);
    }
  }
}

export const eventBus = new EventBus();
