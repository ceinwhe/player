<script setup lang="ts">
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
import { useMusicInfoStore } from '@/stores/musicInfo.ts'
import type { Music } from '@/stores/musicInfo.ts'

const tableName = 'history'
const musicInfoStore = useMusicInfoStore()

// 模拟从数据库获取的数据
const songs = ref<Music[]>([])
onMounted(async () => {
  songs.value = await invoke<Music[]>('get_music_info', { invokeTable: tableName })
  musicInfoStore.musicList = songs.value
})

// 格式化时间，秒转分秒
function formatTime(seconds: number): string {
  const min = Math.floor(seconds / 60) // 分钟
  const sec = seconds % 60 // 秒
  return `${min}:${sec.toString().padStart(2, '0')}` // 秒不足两位补 0
}
//检测cover路径是否存在,不存在则替换为默认图片
function checkCoverPath(cover: string): string {
  if (!cover) {
    return '/assets/album.png' // 替换为默认图片路径
  } else {
    return convertFileSrc(cover)
  }
}

</script>

<template>
  <div class="list">
    <div
      class="card"
      v-for="song in songs"
      :key="song.id"
      @click="musicInfoStore.play( song, tableName)"
      @contextmenu="musicInfoStore.openContextMenu($event, song, tableName)"
    >
      <img :src="checkCoverPath(song.cover)" alt="cover" class="cover" />
      <div class="info">
        <div class="artist-title">
          <p class="artist">{{ song.title }}</p>
          <p class="title">{{ song.artist }}</p>
        </div>
        <div class="album">{{ song.album }}</div>
        <div class="duration">{{ formatTime(song.duration) }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.list {
  display: flex;
  flex-direction: column;
  overflow-y: scroll;
  scrollbar-width: none;
  padding-bottom: 110px;
}

.card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
  width: 100%;
  padding: 5px 0;
  cursor: pointer;
}
.card:nth-child(odd) {
  background: #fafafa;
}
.card:hover {
  background: #f5f5f7;
}

.cover {
  width: 50px;
  height: 50px;
  object-fit: cover;
  margin-right: 10px;
  box-shadow: 0 2px 8px 0 rgba(60, 60, 67, 0.1);
}

.info {
  flex: 1;
  display: flex;
  flex-direction: row;
  align-items: center;
}
.artist-title {
  width: 40%;
  display: flex;
  flex-direction: column;
  margin: 0;
  padding: 0;
}
.artist {
  font-size: 0.8rem;
  margin: 0;
  padding: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis; /* 超出部分显示省略号 */
}
.title {
  font-size: 0.8rem;
  color: #666;
  margin: 0;
  padding: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis; /* 超出部分显示省略号 */
}
.album {
  width: 40%;
  white-space: nowrap;
  margin-right: 12%;
}
</style>
