<template>
  <aside class="navigate">
    <div class="brand">
      <div class="logo">
        <svg width="36" height="36" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
          <rect width="32" height="32" rx="6" fill="#53D7A1" />
          <path
            d="M10 20c0-6 4-11 11-12-2 9-8 13-11 13-0.3 0-0.6 0-1-0.2 0.6-0.2 1.2-0.5 1-0.8z"
            fill="#fff"
            opacity="0.9"
          />
        </svg>
      </div>
      
    </div>

    <nav class="nav-group primary">
      <router-link :to="{ name: 'store' }" class="nav-item" active-class="active">
        <span class="icon">🎵</span>
        <span class="label">曲库</span>
      </router-link>
      <router-link :to="{ name: 'favorite' }" class="nav-item" active-class="active">
        <span class="icon">💖</span>
        <span class="label">喜爱</span>
      </router-link>
      <router-link :to="{ name: 'playback' }" class="nav-item" active-class="active">
        <span class="icon">🕘</span>
        <span class="label">历史</span>
      </router-link>
      <router-link :to="{ name: 'loadPage' }" class="nav-item" active-class="active">
        <span class="icon">🕘</span>
        <span class="label">历史</span>
      </router-link>
    </nav>

    <div class="nav-group playlists">
      <div class="group-title">播放列表</div>
      <div class="playlist-list">
        <a href="#" class="playlist-item">每日推荐</a>
        <a href="#" class="playlist-item">我喜欢的歌</a>
        <a href="#" class="playlist-item">工作播放列表</a>
        <a href="#" class="playlist-item">轻音乐</a>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
</script>

<style scoped>
.navigate {
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.75), rgba(250, 250, 252, 0.75));
  backdrop-filter: blur(10px);
  height: 100%;
  width: 180px;
  flex: 0 0 180px; /* fixed column width */
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 18px 14px;
  box-sizing: border-box;
  border-right: 1px solid rgba(0, 0, 0, 0.04);
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
}
.logo {
  width: 36px;
  height: 36px;
  flex: 0 0 36px;
}

.nav-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.nav-group.primary {
  margin-top: 6px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  border-radius: 10px;
  color: #111;
  text-decoration: none;
  font-size: 14px;
}
.nav-item .icon {
  font-size: 16px;
  width: 20px;
  text-align: center;
}
.nav-item .label {
  flex: 1;
}
.nav-item:hover {
  background: rgba(0, 0, 0, 0.04);
}
.nav-item.active {
  background: linear-gradient(90deg, rgba(83, 215, 161, 0.12), rgba(83, 215, 161, 0.06));
  color: #06684a;
  font-weight: 600;
}

.nav-group.playlists {
  margin-top: 10px;
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
}
.group-title {
  font-size: 12px;
  color: #6b6b6f;
  padding: 6px 8px;
}
.playlist-list {
  overflow: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding-right: 6px;
}
.playlist-item {
  padding: 8px 10px;
  border-radius: 8px;
  color: #333;
  text-decoration: none;
  font-size: 13px;
}
.playlist-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

@media (max-width: 520px) {
  .navigate {
    padding: 12px 8px;
  }
  .nav-item .label {
    display: none;
  }
}
</style>
