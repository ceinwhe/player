import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Music {
  id: number
  title: string
  artist: string
  album: string
  duration: number
  cover: string
  file_path: string
}

export const useMusicInfoStore = defineStore('musicInfo', () => {
  const musicList = ref<Music[]>([])
  const playingMusic=ref<Music>()
  const menuMusic = ref<Music>()  //用于菜单的歌曲操作
  const showMenu = ref(false)
  const showInfo = ref(false)
  const position = ref({ x: 0, y: 0 })
  const table = ref<string>()
  const isPlaying = ref(false)

  function openContextMenu($event: MouseEvent, music: Music, tableName: string) {
    $event.preventDefault()
    position.value = { x: $event.clientX, y: $event.clientY }
    showMenu.value = true
    menuMusic.value = music
    table.value = tableName
  }

  function closeContextMenu() {
    showMenu.value = false
  }

  async function play(music?: Music, tableName?: string) {
    if (!music || !tableName) return
    playingMusic.value = music
    isPlaying.value = true
    await invoke('play', { invokeId: music.id, invokeTable: tableName })
  }
  async function menuPlay(music?: Music, tableName?: string) {
    if (!music || !tableName) return
    playingMusic.value = music
    isPlaying.value = true
    await invoke('play', { invokeId: music.id, invokeTable: tableName })
  }

  async function addToAnotherTable(music?: Music, fromTable?: string, toTable?: string) {
    if (!music || !fromTable || !toTable) return
    await invoke('add_to_another_table', {
      invokeId: music.id,
      invokeFromTable: fromTable,
      invokeToTable: toTable,
    })
  }

  return {
    showMenu,
    showInfo,
    position,
    table,
    menuMusic,
    playingMusic,
    musicList,
    isPlaying,
    openContextMenu,
    closeContextMenu,
    play,
    menuPlay,
    addToAnotherTable,
  }
})
