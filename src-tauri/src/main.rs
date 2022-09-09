#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::prelude::*;

#[tauri::command]
fn save_dst(filename: String, json:String) {
  let mut file = File::create(filename).expect("file creation failed");
  file.write_all(json.as_bytes()).expect("writing json failed");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save_dst])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
