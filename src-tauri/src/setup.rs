use sqlx::{Pool, Sqlite};
use std::fs;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::{ StoreBuilder};

pub async fn setup(app_handle: &AppHandle) -> Result<(), sqlx::Error> {
    //连接数据库
    let store = StoreBuilder::new(app_handle, "settings.json")
        .build()
        .unwrap();
    let database_url = "sqlite:./music.db";
    let pool = Pool::<Sqlite>::connect(database_url).await?;


    //监测数据库表，cover文件夹是否存在
    if let None = store.get("initialized") {
        store.set("initialized", true);
        store.save().unwrap();
        initialize_database(pool).await?;
        if let Err(e) = initialize_fs(&app_handle) {
            eprintln!("初始化文件系统失败: {}", e);
        }
    }

    Ok(())
}

async fn initialize_database(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS store (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                duration INTEGER,
                cover TEXT,
                file_path TEXT NOT NULL UNIQUE
            );
            "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                duration INTEGER,
                cover TEXT,
                file_path TEXT NOT NULL UNIQUE
            );
            "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                duration INTEGER,
                cover TEXT,
                file_path TEXT NOT NULL UNIQUE
            );
            "#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}

fn initialize_fs(app_handle: &AppHandle) -> Result<(), String> {
    // 获取 %APPDATA% 路径
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取 app_data_dir 失败: {}", e))?;
    // 拼接 player 目录
    let cover_dir = app_data_dir.join("cover");

    // 创建目录（递归创建）
    if let Err(e) = fs::create_dir_all(&cover_dir) {
        return Err(format!("创建文件夹失败: {}", e));
    }

    Ok(())
}
