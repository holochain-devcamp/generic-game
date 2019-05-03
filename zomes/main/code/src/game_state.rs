use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

use crate::game_move::Piece;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub pieces: Vec<Piece>,
    pub resigned: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub complete: bool,
    pub player_1: PlayerState,
    pub player_2: PlayerState,
}

impl GameState {
    pub fn initial() -> Self {
        let p1 = PlayerState {
            pieces: vec![
                Piece{x: 0, y: 0}, Piece{x: 2, y: 0}, Piece{x: 4, y: 0}, Piece{x: 6, y: 0},
                Piece{x: 1, y: 1}, Piece{x: 3, y: 1}, Piece{x: 5, y: 1}, Piece{x: 7, y: 1},
                Piece{x: 0, y: 2}, Piece{x: 2, y: 2}, Piece{x: 4, y: 2}, Piece{x: 6, y: 2},
            ],
            resigned: false,
        };
        let p2 = PlayerState {
            pieces: vec![
                Piece{x: 1, y: 5}, Piece{x: 3, y: 5}, Piece{x: 5, y: 5}, Piece{x: 7, y: 5},
                Piece{x: 0, y: 6}, Piece{x: 2, y: 6}, Piece{x: 4, y: 6}, Piece{x: 6, y: 6},
                Piece{x: 1, y: 7}, Piece{x: 3, y: 7}, Piece{x: 5, y: 7}, Piece{x: 7, y: 7},
            ],
            resigned: false,
        };
        GameState {
            complete: false,
            player_1: p1,
            player_2: p2,
        }
    }
}
