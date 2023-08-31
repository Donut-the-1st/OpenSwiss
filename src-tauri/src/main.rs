// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;
mod power;
mod argsort;



use core::result::*;
use argsort::*;
use types::*;
use power::{rs_power};
use std::sync::*;

// begin code from Simon Hyll
// https://hackmd.io/@simonhyll/r1O1reUr2#Shortly-about-errors
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("the mutex was poisoned")]
  PoisonError(String),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
  fn from(err: std::sync::PoisonError<T>) -> Self {
    Error::PoisonError(err.to_string())
  }
}
// end code from Simon Hyll


#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
async fn add_player(state: tauri::State<'_, Mutex<Competition>>, player_name: String) -> Result<(), Error> {
  let mut locked_state = state.lock()?;
  locked_state.initialised = false;
  let new_player: Player = Player {
    name: player_name,
    ID: u8::from(locked_state.players.len())
  };
  locked_state.players.push(new_player);
  Ok(())
}

#[tauri::command]
async fn initialise(state: tauri::State<'_, Mutex<Competition>>) -> Result<(), Error> {
  let mut locked_state = state.lock()?;
  let num_players:u8 = u8::from(locked_state.players.len());
  locked_state.games_won = Array2::<u8>::zeros((num_players, num_players));
  locked_state.prop = Array2::<f64>::zeros((num_players, num_players));
  locked_state.rankings = Array1::<f64>::zeros(num_players);
  locked_state.initialised = true;
  Ok(())
}

#[tauri::command]
async fn add_result(state: tauri::State<'_, Mutex<Competition>>, result: String) -> Result<(), Error> {
  //Check if we can conform result to a result before we lock the thread
  let result: Result = serde_json::from_str(&result)?;
  let mut locked_state = state.lock()?;
  let p1_p2 = [result.player_1_id, result.player_2_id];
  let p2_p1 = [result.player_2_id, result.player_1_id];
  locked_state.games_won[p1_p2] += result.player_1_wins;
  locked_state.games_won[p2_p1] += result.player_2_wins;
  locked_state.games_played[p1_p2] += result.player_1_wins + result.player_2_wins;
  locked_state.games_played[p2_p1] += result.player_1_wins + result.player_2_wins;
  locked_state.prop[p1_p2] = f64::from(locked_state.games_won[p1_p2])/f64::from(locked_state.games_played[p1_p2]);
  locked_state.prop[p2_p1] = f64::from(locked_state.games_won[p2_p1])/f64::from(locked_state.games_played[p2_p1]);
  Ok(())
}

#[tauri::command]
async fn update_results(state: tauri::State<'_, Mutex<Competition>>) -> Result<(), Error> {
  let mut locked_state = state.lock()?;
  let (new_rankings, _) = rs_power(locked_state.prop.view(), 1e-2);
  locked_state.rankings.assign(&new_rankings);
  Ok(())
}

#[tauri::command]
async fn get_placements(state: tauri::State<'_, Mutex<Competition>>) -> Result<String, Error> {

}

fn main() {
  tauri::Builder::default()
      .manage(Mutex::new(Competition { ..Default::default() }))
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
