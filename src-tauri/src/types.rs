pub use ndarray::prelude::*;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) ID: u8
}

#[derive(Serialize, Deserialize, Default)]
pub struct Competition {
    pub(crate) players: Vec<Player>,
    pub(crate) games_played: Array2<u8>,
    pub(crate) games_won: Array2<u8>,
    pub(crate) prop: Array2<f64>,
    pub(crate) rankings: Array1<f64>,
    pub(crate) initialised: bool
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