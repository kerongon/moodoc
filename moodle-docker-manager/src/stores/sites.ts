import { reactive } from 'vue';

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

const state = reactive({
  sites: [] as Site[],
});

const addSite = (site: Site) => {
  state.sites.push(site);
};

const removeSite = (siteId: string) => {
  state.sites = state.sites.filter(site => site.id !== siteId);
};

const updateSite = (updatedSite: Site) => {
  const index = state.sites.findIndex(site => site.id === updatedSite.id);
  if (index !== -1) {
    state.sites[index] = updatedSite;
  }
};

const startSite = (siteId: string) => {
  const site = state.sites.find(site => site.id === siteId);
  if (site) {
    site.status = 'running';
  }
};

const stopSite = (siteId: string) => {
  const site = state.sites.find(site => site.id === siteId);
  if (site) {
    site.status = 'stopped';
  }
};

export default {
  state,
  addSite,
  removeSite,
  updateSite,
  startSite,
  stopSite,
};