// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use app_lib::music_play::PLAYER;

fn main() {
    //全局初始化play线程
    Lazy::force(&PLAYER);
    app_lib::run();
}
