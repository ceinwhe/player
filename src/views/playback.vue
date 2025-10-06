<script setup lang="ts">
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'
// 模拟从数据库获取的数据
interface Music {
  title: string
  artist: string
  album: string
  duration: number
  cover: string
  file_path: string
}
const songs = ref<Music[]>([])
onMounted(async () => {
  songs.value = await invoke<Music[]>('get_music_info', { invokeTable: 'history' })
})

// 格式化时间，秒转分秒
function formatTime(seconds: number): string {
  const min = Math.floor(seconds / 60);       // 分钟
  const sec = seconds % 60;                   // 秒
  return `${min}:${sec.toString().padStart(2, '0')}`; // 秒不足两位补 0
}
console.log(songs.value)
</script>

<template>
  <div class="list">
    <div class="card" v-for="song in songs" :key="song.title">
      <img :src="convertFileSrc(song.cover)" alt="cover" class="cover" />
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
  width: 60px;
  height: 60px;

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
  min-width: 40%;
  max-width: 40%;
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
  min-width: 40%;
  max-width: 40%;
  white-space: nowrap;
}
.duration {
  min-width: 20%;
  max-width: 20%;
  padding-left: calc(12%);
}
</style>
