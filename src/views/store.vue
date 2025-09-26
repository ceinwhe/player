<template>
  <div class="store">
    <h2>商店 / 曲库（占位）</h2>
    <h2>商店 / 曲库（占位）</h2>
    <h2>商店 / 曲库（占位）</h2>
    <h2>商店 / 曲库（占位）</h2>
  </div>
</template>

<script setup lang="ts"></script>

<style>
.store {
  background: #ffffff;
}

.list {
  display: flex;
  padding: 22px;
  border-bottom: 1.3px solid #f0f0f0;
}
.list:last-child {
  border-bottom: none;
}

.song-title {
  flex: 1.5;
  font-size: 17px;
  font-weight: 500;
  min-width: 0;
  color: #1d1d1f;
  margin: 0 0 4px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.song-artist,
.song-album {
  flex: 1;
  min-width: 0;
  font-size: 14px;
  color: #6e6e73;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.song-duration {
  flex: 1;
  text-align: right;
  color: #6e6e73;
  font-size: 14px;
}
</style>
