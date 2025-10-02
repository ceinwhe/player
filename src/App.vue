<template>
  <main class="main">
    <nav-page></nav-page>
    <div class="content">
      <top-page/>
      <div class="view">
        <router-view class="router-view" />
        <playerBar v-if="showPlayBar"/>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import NavPage from '@/components/navPage.vue'
import TopPage from '@/components/topPage.vue'
import PlayerBar from '@/components/playerBar.vue'
import { computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'

const router = useRouter()
const showPlayBar = computed(()=>(
  router.currentRoute.value.path != '/loadPage'
))

onMounted(async () => {
  if (await invoke('check_songs')) {
    await router.push('/storePage')
  } else {
    await router.push('/loadPage')
  }
})
</script>

<style scoped>
.main {
  position: fixed;
  display: flex;
  overflow: hidden;
  flex-direction: row;
  background: #ffffff;
  padding: 0;
  margin: 0;
  height: 100%;
  width: 100%;
}

.content {
  display: flex;
  flex: 1;
  flex-direction: column;
  background: #ffffff;
  padding: 0;
  margin: 0;
}

.view {
  position: relative;
  flex: 1;
}

.router-view {
  display: flex;
}
</style>

