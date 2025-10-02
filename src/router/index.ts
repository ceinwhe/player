import { createRouter, createWebHistory } from 'vue-router'
import favorite from '@/views/favorite.vue'
import PlayBack from '@/views/playBack.vue'
import StorePage from '@/views/storePage.vue'
import LoadPage from '@/views/loadPage.vue'


const routes = [
  { path: '/storePage', component: StorePage, name: 'store' },
  { path: '/favorite', component: favorite, name: 'favorite' },
  { path: '/playBack', component: PlayBack, name: 'playback' },
  { path: '/loadPage', component: LoadPage, name: 'loadPage' },
  { path: '/', redirect: '/storePage' },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})



export default router
