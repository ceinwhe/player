use crate::get_idv3;
use rayon::prelude::*;
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Manager};
use tokio;

pub struct DataBase {
    pool: Pool<Sqlite>,
}

impl DataBase {
    pub async fn new(url: &str) -> Result<DataBase, Box<dyn std::error::Error>> {
        let pool = Pool::<Sqlite>::connect(url).await?;
        Ok(DataBase { pool })
    }
    pub async fn add_info(
        &self,
        music_dir: &str,
        app_handle: &AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cover_dir = app_handle.path().app_data_dir().unwrap().join("cover");
        let music_paths = get_idv3::get_music_path(music_dir)
            .par_iter()
            .filter_map(|music_path| get_idv3::Music::get(music_path,&cover_dir).ok())
            .collect::<Vec<get_idv3::Music>>();

        let mut handles = Vec::new();
        for music in music_paths {
            let pool = self.pool.clone();
            let handle = tokio::spawn(async move {
                if let Err(e) = music.insert(&pool).await {
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
}
