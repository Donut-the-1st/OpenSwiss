// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;
mod power;

use types::*;
use std::sync::*;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
async fn add_player(state: tauri::State<'_, Mutex<Competition>>, player_name: String) -> Competition {
  let mut locked_state = state.lock()?;
  locked_state.initialised = false;
  let new_player: Player = Player {
    name: player_name,
    ID: locked_state.players.len() as u8
  };
  locked_state.players.push(new_player);
  locked_state.clone()
}

#[tauri::command]
async fn initialise(state: tauri::State<'_, Mutex<Competition>>) -> Competition {
  let mut locked_state = state.lock()?;
  let num_players:u8 = locked_state.players.len() as u8;
  locked_state.games_won = Array2::<u8>::zeros((num_players, num_players));
  locked_state.prop = Array2::<f64>::zeros((num_players, num_players));
  locked_state.rankings = Array1::<f64>::zeros(num_players);
  locked_state.initialised = true;
  locked_state.clone()
}

fn main() {
  tauri::Builder::default()
      .manage(Mutex::new(Competition { ..Default::default() }))
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
