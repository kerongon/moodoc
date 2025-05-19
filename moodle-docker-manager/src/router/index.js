import { createRouter, createWebHashHistory } from 'vue-router';
import Dashboard from '../components/Dashboard.vue';
import CreateSite from '../components/CreateSite.vue';
import SiteLogs from '../components/SiteLogs.vue';
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
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
