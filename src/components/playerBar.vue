<template>
  <div class="playerBar">
    <!-- 歌曲信息 -->
    <div class="track-info">
      <img :src="currentTrack.cover" alt="cover" class="cover" :class="{ playing: isPlaying }" />
      <div class="text">
        <span class="title">{{ currentTrack.title }}</span>
        <span class="artist">{{ currentTrack.artist }}</span>
      </div>
    </div>

    <!-- 控制区域 -->
    <div class="controls">
      <button @click="prevTrack" class="btn icon-btn" aria-label="previous">
        <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true">
          <path d="M6.5 6.5v11l7-5.5-7-5.5zM14.5 6.5v11h1v-11h-1z" fill="currentColor" />
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
          <path d="M17.5 6.5v11l-7-5.5 7-5.5zM9.5 6.5v11h-1v-11h1z" fill="currentColor" />
        </svg>
      </button>
    </div>

    <!-- 进度条与时间 -->
    <div class="progress">
      <div class="time left">{{ formatTime(currentTime) }}</div>
      <input
        type="range"
        min="0"
        :max="duration"
        v-model="currentTime"
        @input="onSeek"
        :style="progressStyle"
      />
      <div class="time right">{{ formatTime(duration) }}</div>
    </div>

    <div class="extras">
      <button
        @click="toggleMute"
        class="btn icon-btn volume-btn"
        :aria-pressed="isMuted"
        aria-label="mute"
      >
        <svg v-if="!isMuted" viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
          <path d="M5 9v6h4l5 4V5L9 9H5z" fill="currentColor" />
        </svg>
        <svg v-else viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
          <path
            d="M16.5 12c0-1.77-.77-3.36-2-4.44v8.88c1.23-1.08 2-2.67 2-4.44zM19 12c0 2.89-1.46 5.44-3.71 6.93l1.43 1.43C19.07 18.69 21 15.53 21 12s-1.93-6.69-4.28-8.36l-1.43 1.43C17.54 6.56 19 9.11 19 12zM4 9v6h4l5 4V5L8 9H4z"
            fill="currentColor"
          />
        </svg>
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const isPlaying = ref(false)
const currentTime = ref(0)
const duration = ref(240)

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

function formatTime(secRef: number | { value: number }) {
  const s = typeof secRef === 'number' ? secRef : secRef.value
  const m = Math.floor(s / 60)
  const sec = Math.floor(s % 60)
  return `${m}:${sec.toString().padStart(2, '0')}`
}

function onSeek(e: Event) {
  const target = e.target as HTMLInputElement
  currentTime.value = Number(target.value)
}

import { computed } from 'vue'
const progressStyle = computed(() => {
  const pct = Math.max(0, Math.min(100, (currentTime.value / Math.max(1, duration.value)) * 100))
  return {
    background: `linear-gradient(90deg, rgba(83,215,161,0.95) 0%, rgba(83,215,161,0.95) ${pct}%, rgba(200,200,200,0.25) ${pct}%, rgba(200,200,200,0.25) 100%)`,
  }
})

const isMuted = ref(false)
function toggleMute() {
  isMuted.value = !isMuted.value
}
</script>

<style scoped>
.playerBar {
  position: absolute;
  bottom: 30px;
  width: 86%;
  left: calc(7% - 24px);
  height: 64px; /* slightly taller */
  display: flex;
  align-items: center;
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

.progress {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  max-width: 420px;
}
.progress .time {
  font-size: 12px;
  color: #666;
  width: 40px;
  text-align: center;
}
.progress input[type='range'] {
  width: 100%;
  appearance: none;
  height: 6px;
  border-radius: 6px;
  background: linear-gradient(90deg, rgba(83, 215, 161, 0.9) 0%, rgba(200, 200, 200, 0.25) 100%);
  outline: none;
  cursor: pointer;
}
.progress input[type='range']::-webkit-slider-runnable-track {
  height: 6px;
  border-radius: 6px;
}
.progress input[type='range']::-webkit-slider-thumb {
  appearance: none;
  margin-top: -5px; /* center the thumb */
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: white;
  border: 3px solid rgba(83, 215, 161, 0.95);
  box-shadow: 0 6px 18px rgba(83, 215, 161, 0.16);
  transition: transform 0.12s;
}
.progress input[type='range']::-webkit-slider-thumb:hover {
  transform: scale(1.08);
}

.extras {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 12px;
}
.volume-btn {
  width: 36px;
  height: 36px;
  border-radius: 8px;
}
.volume-btn svg {
  opacity: 0.95;
}

.cover.playing {
  animation: cover-rotate 6s linear infinite;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.18);
}
@keyframes cover-rotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
