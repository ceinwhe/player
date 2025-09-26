#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      close,
      minimize,
      change_window_state
      ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn close(window: tauri::Window){
  window.close().unwrap();
}
#[tauri::command]
fn minimize(window: tauri::Window){
  window.minimize().unwrap();
}
#[tauri::command]
fn change_window_state(window: tauri::Window){
  if window.is_maximized().unwrap() {
    window.unmaximize().unwrap();
  }else {
      window.maximize().unwrap();
  }
  
}