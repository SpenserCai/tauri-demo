/*
 * @Author: SpenserCai
 * @Date: 2024-12-13 16:39:18
 * @version: 
 * @LastEditors: SpenserCai
 * @LastEditTime: 2024-12-16 20:58:12
 * @Description: file content
 */
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod tools;
use tools::bilibili_downloader::Bilibili;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_bilibili_real_url(url: &str) -> String {
    let mut bilibili = Bilibili::new(
        url.to_string(),
        "https://api.bilibili.com/x/player/wbi/playurl".to_string(),
        // "https://api.bilibili.com/x/player/playurl".to_string(),
        "https://api.bilibili.com/x/web-interface/view?bvid=".to_string(),
    );
    let _ = bilibili.get_cid();
    let _ = bilibili.get_real_url();
    bilibili.real_url
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_bilibili_real_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
