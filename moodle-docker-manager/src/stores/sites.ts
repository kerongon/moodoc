import { defineStore } from 'pinia';

interface Site {
  id: string;
  name: string;
  phpVersion: string;
  dbType: string;
  moodleBranch: string;
  dbPort: number | null;
  webPort: number | null;
  status: 'running' | 'stopped';
}

export const useSitesStore = defineStore('sites', {
  state: () => ({
    sites: [] as Site[]
  }),
  
  actions: {
    addSite(site: Site) {
      this.sites.push(site);
    },
    
    removeSite(siteId: string) {
      this.sites = this.sites.filter(site => site.id !== siteId);
    },
    
    updateSite(updatedSite: Site) {
      const index = this.sites.findIndex(site => site.id === updatedSite.id);
      if (index !== -1) {
        this.sites[index] = updatedSite;
      }
    },
    
    startSite(siteId: string) {
      const site = this.sites.find(site => site.id === siteId);
      if (site) {
        site.status = 'running';
      }
    },
    
    stopSite(siteId: string) {
      const site = this.sites.find(site => site.id === siteId);
      if (site) {
        site.status = 'stopped';
      }
    },
    /**
     * Return the current list of all sites
     */
    getSites(): Site[] {
      return this.sites;
    }
  }
});

export default useSitesStore;