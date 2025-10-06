use crate::music_info;
use crate::music_info::{Music, MusicTag};
use rayon::prelude::*;
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Manager};
use tokio;

#[derive(Clone)]
pub struct DataBase {
    pool: Pool<Sqlite>,
}

impl DataBase {
    ///初始化DataBase
    pub async fn new(url: &str) -> Result<DataBase, sqlx::Error> {
        let pool = Pool::<Sqlite>::connect(url).await?;
        Ok(DataBase { pool })
    }
    pub async fn default() -> Result<DataBase, sqlx::Error> {
        let url = "sqlite://../music.db";
        let pool = Pool::<Sqlite>::connect(url).await?;
        Ok(DataBase { pool })
    }

    ///添加音乐信息到数据库
    pub async fn add_info(
        &self,
        music_dir: &str,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cover_dir = app_handle.path().app_data_dir().unwrap().join("cover");
        let music_paths = music_info::get_music_path(music_dir)
            .par_iter()
            .filter_map(|music_path| MusicTag::get(music_path, &cover_dir).ok())
            .collect::<Vec<MusicTag>>();

        let mut handles = Vec::new();
        for music in music_paths {
            let pool = self.pool.clone();
            let handle = tokio::spawn(async move {
                if let Err(e) = music.insert_store(&pool).await {
                    eprintln!("Failed to insert music: {}", e);
                }
            });
            handles.push(handle);
        }
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        Ok(())
    }

    ///从数据库中获取音乐信息
    pub async fn get_music_info(
        &self,
        table: &str,
    ) -> Result<Vec<Music>, Box<dyn std::error::Error>> {
        let music = sqlx::query_as::<_, Music>(&format!("SELECT * FROM {}", table))
            .fetch_all(&self.pool)
            .await?;
        Ok(music)
    }

    ///根据id获取音乐路径
    pub async fn get_path_by_id(&self, id: i32, table: &str) -> Result<String, sqlx::Error> {
        let sql_order = format!("SELECT file_path FROM {} WHERE id = ?", table);
        let path = sqlx::query_scalar::<_, String>(&sql_order)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(path)
    }

    ///随机获取音乐路径
    pub async fn get_path_random(&self, table: &str) -> Result<String, sqlx::Error> {
        let sql_random = format!(
            "SELECT file_path FROM {table} ORDER BY RANDOM() LIMIT 1",
            table = table
        );
        let next_path = sqlx::query_scalar::<_, String>(&sql_random)
            .fetch_one(&self.pool)
            .await?;

        Ok(next_path)
    }

    ///从一个表中添加到另一个表
    pub async fn add_to_another_table(
        &self,
        id: i32,
        from_table: &str,
        to_table: &str,
    ) -> Result<(), sqlx::Error> {
        // 1. 从源表获取完整信息
        let music: MusicTag =
            sqlx::query_as::<_, MusicTag>(&format!("SELECT * FROM {} WHERE id = ?", from_table))
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        // 2. 插入到目标表
        sqlx::query(
            &format!(
                "INSERT INTO {} ( artist, title, album, duration, cover, file_path) VALUES (?, ?, ?, ?, ?, ?)",
                to_table
            )
        )
            .bind(music.artist)
            .bind(music.title)
            .bind(music.album)
            .bind(music.duration)
            .bind(music.cover)
            .bind(music.file_path)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
