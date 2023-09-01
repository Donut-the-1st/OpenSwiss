pub use ndarray::prelude::*;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Player {
    pub(crate) player_name: String,
    pub(crate) ID: usize,
    pub(crate) score: f64,
    pub(crate) wins: usize,
    pub(crate) losses: usize
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Competition {
    pub(crate) players: Vec<Player>,
    pub(crate) games_played: Array2<usize>,
    pub(crate) games_won: Array2<usize>,
    pub(crate) prop: Array2<f64>,
    pub(crate) rankings: Array1<f64>,
    pub(crate) initialised: bool,
}

impl Default for Competition {
    fn default() -> Self {
        Competition {
            players: vec![],
            games_played: Default::default(),
            games_won: Default::default(),
            prop: Default::default(),
            rankings: Default::default(),
            initialised: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResult {
    pub(crate) player_1_id: usize,
    pub(crate) player_2_id: usize,
    pub(crate) player_1_wins: usize,
    pub(crate) player_2_wins: usize,
}
