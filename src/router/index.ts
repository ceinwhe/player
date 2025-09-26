import { createRouter, createWebHistory } from 'vue-router'
import favorite from '@/views/favorite.vue'
import playback from '@/views/playback.vue'
import store from '@/views/store.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {path:'/favorite',component:favorite,name:'favorite'},
    {path:'/playback',component:playback,name:'playback'},
    {path:'/store',component:store,name:'store'},
    {path:'/',redirect:'/store'}
  ]
})



export default router

