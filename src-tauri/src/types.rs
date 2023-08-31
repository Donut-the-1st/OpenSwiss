pub use ndarray::prelude::*;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    name: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct Competition {
    pub(crate) players: Vec<Player>,
    games_played: Array2<u8>,
    games_won: Array2<u8>,
    prop: Array2<f64>,
    rankings: Array1<f64>,
    initialised: bool
}

impl Default for Competition {
    fn default() -> Self {
        Competition{
            players: vec![],
            games_played: Default::default(),
            games_won: Default::default(),
            prop: Default::default(),
            rankings: Default::default(),
            initialised: false
        }
    }
}