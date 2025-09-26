<template>
  <main class="main">
    <navigate></navigate>
    <div class="content">
      <top></top>
      <div class="view">
        <router-view class="router-view" />
        <playerBar></playerBar>
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import Navigate from '@/components/navigate.vue'
import Top from '@/components/top.vue'
import PlayerBar from '@/components/playerBar.vue'
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

