<script setup lang="ts">
import { computed, ref } from 'vue'
import { useMusicInfoStore } from '@/stores/musicInfo.ts'
import { invoke } from '@tauri-apps/api/core'

const MusicStore = useMusicInfoStore()
const currentMusic = computed(()=>MusicStore.currentMusic)

const show = ref(true)

async function play() {
  if (currentMusic.value?.id){
    await invoke('play', { invokeId: currentMusic.value.id, invokeTable: 'store' })
  }else {
    // 当前没有选中的音乐，无法播放
  }

}

async function addToAnotherTable() {
  if (currentMusic.value?.id){
    await invoke('add_to_another_table', {
      invokeId: currentMusic.value.id,
      invokeFromTable: 'store',
      invokeToTable: 'favorite',
    })
  }else {
    // 当前没有选中的音乐，无法添加
  }

}

</script>

<template>
  <div class="menu" v-show="show">
    <ul class="list">
      <li class="option" @click="play()">播放</li>
      <li class="option">下一首播放</li>
      <li class="option" @click="addToAnotherTable()">添加至喜爱</li>
    </ul>
  </div>
</template>

<style scoped>
.menu{
  position: fixed;

  border-radius: 10px;
  padding: 0;
  left: 50px;
  top: 50px;
  background: #E5E7EB;
}
.list{
  display: flex;
  flex-direction: column;
  padding-left: 0;
  margin: 0;
}
.option{
  list-style: none;
  padding: 10px 20px;
  margin: 0;
}
.option:hover{
  cursor: pointer;
  background: #FAFAFA;
}
</style>
