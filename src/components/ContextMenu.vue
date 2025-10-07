<script setup lang="ts">
import { useMusicInfoStore } from '@/stores/musicInfo'

const musicInfoStore = useMusicInfoStore()
</script>

<template>
  <div
    class="menu"
    v-show="musicInfoStore.showMenu"
    :style="{ top: musicInfoStore.position.y + 'px', left: musicInfoStore.position.x + 'px' }"
    @mouseleave="musicInfoStore.closeContextMenu()"
    @click.stop
  >
    <ul class="list">
      <li class="option" @click="musicInfoStore.menuPlay(musicInfoStore.menuMusic, musicInfoStore.table)">播放</li>
      <li class="option">下一首播放</li>
      <li class="option" @click="musicInfoStore.addToAnotherTable(musicInfoStore.menuMusic, musicInfoStore.table, 'favorite')">添加至喜爱</li>
    </ul>
  </div>
</template>

<style scoped>
.menu {
  position: fixed;
  z-index: 2;
  border-radius: 10px;
  padding: 0;
  background: #e5e7eb;
}
.list {
  display: flex;
  flex-direction: column;
  padding-left: 0;
  margin: 0;
}
.option {
  list-style: none;
  padding: 10px 20px;
  margin: 0;
}
.option:hover {
  cursor: pointer;
  background: #fafafa;
}
</style>
