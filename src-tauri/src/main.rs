// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;
use types::*;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn add_player(state: tauri::State<Competition>, player: String) {
  let player: Player = serde_json::from_str(&player).unwrap();
  state.players.push(player);
  #[cfg(debug_assertions)]
  {
    println!("{:?}", player);
  }
}

fn main() {
  tauri::Builder::default()
      .manage(Competition { ..Default::default() })
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
