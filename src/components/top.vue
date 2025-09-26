<template>
  <header class="top">
    <div class="left">
      <div class="logo">Player</div>
      <div class="search-wrap">
        <input type="text" class="search" placeholder="搜索歌曲、歌手或专辑" />
      </div>
    </div>

    <div class="buttons">
      <button @click="minimize" class="icon-btn" aria-label="minimize">▁</button>
      <button @click="change_window_state" class="icon-btn" aria-label="toggle">▢</button>
      <button @click="close" class="icon-btn" aria-label="close">✕</button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

function close() {
  invoke('close')
}
function minimize() {
  invoke('minimize')
}
function change_window_state() {
  invoke('change_window_state')
}
</script>

<style>
.top {
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  -webkit-app-region: drag;
}

.left {
  display: flex;
  align-items: center;
  min-width: 80px;
}
.logo {
  font-weight: 600;
  font-size: 16px;
  color: #111;
  margin-right: 40px;
}

.search-wrap {
  width: 420px;
  max-width: 70%;
}
.search {
  width: 100%;
  height: 36px;
  border-radius: 10px;
  border: 1px solid rgba(0, 0, 0, 0.08);
  padding: 6px 12px;
  outline: none;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  -webkit-app-region: no-drag;
}

.icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: none;
  background: transparent;
  cursor: pointer;
  color: #222;
}
.icon-btn:hover {
  background: rgba(0, 0, 0, 0.04);
}
</style>
