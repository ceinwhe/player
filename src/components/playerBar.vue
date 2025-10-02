<template>
  <div class="playerBar">
    <!-- 歌曲信息 -->
    <div class="track-info">
      <img :src="currentTrack.cover" alt="cover" class="cover" />
      <div class="text">
        <span class="title">{{ currentTrack.title }}</span>
        <span class="artist">{{ currentTrack.artist }}</span>
      </div>
    </div>

    <!-- 控制区域 -->
    <div class="controls">
      <button @click="prevTrack" class="btn icon-btn" aria-label="previous">
        <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
          <path d="M17.5 6.5v11l-7-5.5 7-5.5zM9.5 6.5v11h-1v-11h1z" fill="currentColor" />
        </svg>
      </button>

      <button
        @click="togglePlay"
        :class="['btn', 'play-btn', { playing: isPlaying }]"
        aria-label="play-pause"
      >
        <svg v-if="!isPlaying" viewBox="0 0 24 24" width="20" height="20" aria-hidden="true">
          <path d="M6 4l14 8-14 8V4z" fill="currentColor" />
        </svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
          <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" fill="currentColor" />
        </svg>
      </button>

      <button @click="nextTrack" class="btn icon-btn" aria-label="next">
        <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
          <path d="M6.5 6.5v11l7-5.5-7-5.5zM14.5 6.5v11h1v-11h-1z" fill="currentColor" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const isPlaying = ref(false)

const currentTrack = ref({
  title: '心中的日月',
  artist: '王力宏',
  cover: 'https://placehold.co/100x100',
})

const togglePlay = () => {
  isPlaying.value = !isPlaying.value
}
const prevTrack = () => {}
const nextTrack = () => {}
</script>

<style scoped>
.playerBar {
  position: absolute;
  box-sizing: border-box;
  height: 78px;
  width: 86%;
  left: 7%;
  bottom: 30px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 0;
  padding: 10px 18px;
  border-radius: 14px;

  /* Apple Music-like translucent surface */
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.92), rgba(250, 250, 252, 0.9));
  backdrop-filter: blur(10px) saturate(120%);

  border: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif;
}

.track-info {
  display: flex;
  align-items: center;
  flex: 1;
  min-width: 150px;
}

.cover {
  width: 52px;
  height: 52px;
  border-radius: 8px;
  object-fit: cover;
  margin-right: 12px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.18);
  flex-shrink: 0;
}

.text {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.text .title {
  font-weight: 700;
  font-size: 15px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.text .artist {
  font-size: 13px;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.controls {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 0 20px;
}
.controls .btn {
  background: transparent;
  border: none;
  width: 40px;
  height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  font-size: 16px;
  cursor: pointer;
  color: #222;
  transition:
    transform 0.12s ease,
    background 0.12s ease;
}
.controls .btn:hover {
  background: rgba(0, 0, 0, 0.04);
  transform: translateY(-1px);
}
.controls .play-btn {
  width: 46px;
  height: 46px;
  font-size: 18px;
  border-radius: 12px;
  background: rgba(0, 0, 0, 0.04);
}

.controls .play-btn.playing {
  background: linear-gradient(180deg, rgba(83, 215, 161, 0.95), rgba(20, 140, 100, 0.95));
  color: white;
  box-shadow: 0 8px 30px rgba(20, 140, 100, 0.18);
}

</style>
