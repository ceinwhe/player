use lofty::picture::MimeType;
use lofty::prelude::*;
use lofty::{picture, read_from_path};
use sqlx::pool::Pool;
use sqlx::Sqlite;
use std::fs;
use std::path::PathBuf;


pub struct Music {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: u32,
    pub cover: String,
    pub file_path: String,
}

impl Music {
    pub fn get(
        music_path: &PathBuf,
        cover_dir: &PathBuf,
    ) -> Result<Music, Box<dyn std::error::Error>> {
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
                cover_path = cover_dir.join(format!("{}.{}", artist, ext));

                fs::write(&cover_path, cover_data)?;
            }
            None => {
                cover_path = PathBuf::from("");
            }
        }

        Ok(Music {
            artist,
            title,
            album,
            duration: tagged_file.properties().duration().as_secs() as u32,
            cover: cover_path.to_str().unwrap_or("").to_string(),
            file_path: music_path.to_str().unwrap_or("").to_string(),
        })
    }

    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO store (artist, title, album, duration, cover, file_path)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,

        )
            .bind(&self.artist)
            .bind(&self.title)
            .bind(&self.album)
            .bind(self.duration as i64)
            .bind(&self.cover)
            .bind(&self.file_path)
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
