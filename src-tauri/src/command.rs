use tauri::{ AppHandle};
use tauri_plugin_store::StoreBuilder;
use crate::{database, music_play};
use crate::music_play::{PLAYER};
use crate::music_info::Music;

///添加音乐信息到数据库
#[tauri::command]
pub async fn add_music_info_to_db(invoke_music_dir:String,app_handle: AppHandle,) {
    let url= "sqlite://../music.db";
    let database=database::DataBase::new(url).await.expect("database初始化失败");
    database.add_info(&invoke_music_dir,&app_handle).await.expect("添加音乐信息失败");
}

///检查是否已经扫描过音乐文件
///返回true表示已经扫描过，返回false表示没有扫描过
#[tauri::command]
pub async fn check_songs(app_handle: AppHandle)-> bool {
    let store = StoreBuilder::new(&app_handle, "settings.json")
        .build()
        .unwrap();

    match store.get("songs_check") {
        Some(_) => {
            true
        }
        None => {
            store.set("songs_check", "has_checked");
            store.save().unwrap();
            false
        }
    }
}



#[tauri::command]
pub async fn play(invoke_id:i32,invoke_table:String){
    let database=database::DataBase::default().await.expect("database初始化失败");
    music_play::play(invoke_id,invoke_table,database).await;
    // PLAYER.play(invoke_id,invoke_table,database).await.expect("播放失败");
}


///获取音乐信息结构体
#[tauri::command]
pub async fn get_music_info(invoke_table:String)->Vec<Music>{
    let url= "sqlite://../music.db";
    let database=database::DataBase::new(url).await.expect("database初始化失败");
    let music=database.get_music_info(&invoke_table).await.expect("获取音乐信息失败");
    music
}
///添加音乐到另一个表
#[tauri::command]
pub async fn add_to_another_table(invoke_id:i32, invoke_from_table:String, invoke_to_table:String){
    let database=database::DataBase::default().await.expect("database初始化失败");
    database.add_to_another_table(invoke_id,&invoke_from_table,&invoke_to_table).await.expect("添加音乐到另一个表失败");
}
/// 窗口控制命令
#[tauri::command]
pub fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
pub fn minimize(window: tauri::Window) {
    window.minimize().unwrap();
}
#[tauri::command]
pub fn toggle(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}