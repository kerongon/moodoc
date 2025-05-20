import { createRouter, createWebHashHistory } from 'vue-router';
import Dashboard from '../components/Dashboard.vue';
import CreateSite from '../components/CreateSite.vue';
import SiteLogs from '../components/SiteLogs.vue';
import SiteBackups from '../components/SiteBackups.vue';
import UpdateSite from '../components/UpdateSite.vue';
import SettingsView from '../views/SettingsView.vue';

const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: Dashboard
  },
  {
    path: '/create',
    name: 'CreateSite',
    component: CreateSite
  },
  {
    path: '/logs/:siteName',
    name: 'SiteLogs',
    component: SiteLogs,
    props: true
  },
  {
    path: '/backups/:siteName',
    name: 'SiteBackups',
    component: SiteBackups,
    props: true
  },
  {
    path: '/update/:siteName',
    name: 'UpdateSite',
    component: UpdateSite,
    props: true
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  }
  ,
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
