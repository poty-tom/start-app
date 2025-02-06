import { createRouter, createWebHistory } from 'vue-router'
import Home from './components/Home.vue';

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Home,
    },
    {
      path: '/explorer',
      component: () => import('./components/Explorer.vue'),
    },
    {
      path: '/database',
      component: () => import('./components/Database.vue'),
    },
    {
      path: '/movecat',
      component: () => import('./components/MoveCat.vue'),
    }
  ],
})
