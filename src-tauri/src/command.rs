use tauri::{AppHandle};
use tauri_plugin_store::StoreBuilder;
use crate::database;



#[tauri::command]
pub async fn add_music_info_to_db(invoke_music_dir:String,app_handle: AppHandle,) {
    let url= "sqlite:./music.db";
    let database=database::DataBase::new(url).await.expect("database初始化失败");
    database.add_info(&invoke_music_dir,&app_handle).await.expect("添加音乐信息失败");
}

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
pub fn close(window: tauri::Window) {
    window.close().unwrap();
}
#[tauri::command]
pub fn minimize(window: tauri::Window) {
    window.minimize().unwrap();
}
#[tauri::command]
pub fn change_window_state(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}