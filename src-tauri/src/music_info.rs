use lofty::picture::MimeType;
use lofty::prelude::*;
use lofty::{picture, read_from_path};
use sqlx::pool::Pool;
use sqlx::Sqlite;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Music{
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: u32,
    pub cover: String,
    pub file_path: String,
}


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MusicTag {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: u32,
    pub cover: String,
    pub file_path: String,
}

impl MusicTag {

    ///从音乐文件中提取标签信息
    ///如果没有标签信息，则使用默认值"Unknown"
    /// 如果有封面图片，则将其保存到指定目录，并返回保存路径
    /// 如果没有封面图片，则返回空字符串
    /// 返回一个MusicTag结构体
    pub fn get(
        music_path: &PathBuf,
        cover_dir: &PathBuf,
    ) -> Result<MusicTag, Box<dyn std::error::Error>> {
        let tagged_file = read_from_path(music_path)?;
        let idv3 = tagged_file.primary_tag();

        let (artist, title, album, cover) = match idv3 {
            Some(tag) => (
                tag.artist()
                    .map(|s| s.to_string())
                    .unwrap_or("Unknown".to_string()),
                tag.title()
                    .map(|s| s.to_string())
                    .unwrap_or("Unknown".to_string()),
                tag.album()
                    .map(|s| s.to_string())
                    .unwrap_or("Unknown".to_string()),
                tag.pictures()
                    .iter()
                    .find(|p| p.pic_type() == picture::PictureType::CoverFront),
            ),
            None => (
                "Unknown".to_string(),
                "Unknown".to_string(),
                "Unknown".to_string(),
                None,
            ),
        };

        let cover_path;
        match cover {
            Some(cover) => {
                let ext = match cover.mime_type() {
                    Some(ty) => match ty {
                        MimeType::Png => "png",
                        MimeType::Jpeg => "jpg",
                        MimeType::Gif => "gif",
                        MimeType::Bmp => "bmp",
                        MimeType::Tiff => "tiff",
                        _ => "bin",
                    },
                    None => "bin",
                };
                let cover_data = cover.data();
                // let save_dir=PathBuf::from(save_dir);
                cover_path = cover_dir.join(format!("{}-{}.{}", title, artist, ext));

                fs::write(&cover_path, cover_data)?;
            }
            None => {
                cover_path = PathBuf::from("");
            }
        }

        Ok(MusicTag {
            artist,
            title,
            album,
            duration: tagged_file.properties().duration().as_secs() as u32,
            cover: cover_path.to_str().unwrap_or("").to_string(),
            file_path: music_path.to_str().unwrap_or("").to_string(),
        })
    }

    ///将音乐信息插入数据库
    ///如果已经存在，则忽略
    ///使用INSERT OR IGNORE语句
    ///返回Result<(), sqlx::Error>
    pub async fn insert_store(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT OR IGNORE INTO store (artist, title, album, duration, cover, file_path)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            self.artist,
            self.title,
            self.album,
            self.duration,
            self.cover,
            self.file_path
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

///遍历得到音乐文件路径
pub fn get_music_path(dir_path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }
    files
}
