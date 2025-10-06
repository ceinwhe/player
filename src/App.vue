<script setup lang="ts">
import NavPage from '@/components/navPage.vue'
import TopPage from '@/components/topPage.vue'
import PlayerBar from '@/components/playerBar.vue'
import ContextMenu from '@/components/ContextMenu.vue'
import { computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'

const router = useRouter()
const showPlayBar = computed(() => router.currentRoute.value.path != '/loadPage')

onMounted(async () => {
  if (await invoke('check_songs')) {
    await router.push('/storePage')
  } else {
    await router.push('/loadPage')
  }
})
</script>

<template>
  <div class="app">
    <main class="main">
      <nav-page />
      <div class="content">
        <top-page class="top" />
        <router-view class="view" />
      </div>
    </main>
    <player-bar v-if="showPlayBar" />
    <ContextMenu />
  </div>
</template>

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
  flex: 1;
}
</style>
