import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Music{
  id: number
  title: string
  artist: string
  album: string
  duration: number
  cover: string
  file_path: string
}

export const useMusicInfoStore = defineStore('musicInfo',()=>{

  const musicList = ref<Music[]>([])
  const currentMusic = ref<Music>()




  return { currentMusic, musicList}
})