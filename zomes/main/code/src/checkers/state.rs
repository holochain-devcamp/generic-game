use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    moves::Piece,
    MoveType,
    validation::{Player, get_current_player},
};

pub const BOARD_SIZE: usize = 8;

const WHITE_PIECE: char = '░';
const BLACK_PIECE: char = '▓';
const EMPTY_SPACE: char = ' ';

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub player_1: PlayerState,
    pub player_2: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub pieces: Vec<Piece>,
    pub resigned: bool,
    pub winner: bool,
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
            winner: false,
        };
        let p2 = PlayerState {
            pieces: vec![
                Piece{x: 1, y: 5}, Piece{x: 3, y: 5}, Piece{x: 5, y: 5}, Piece{x: 7, y: 5},
                Piece{x: 0, y: 6}, Piece{x: 2, y: 6}, Piece{x: 4, y: 6}, Piece{x: 6, y: 6},
                Piece{x: 1, y: 7}, Piece{x: 3, y: 7}, Piece{x: 5, y: 7}, Piece{x: 7, y: 7},
            ],
            resigned: false,
            winner: false,
        };
        GameState {
            moves: Vec::new(),
            player_1: p1,
            player_2: p2,
        }
    }

    pub fn render(&self) -> String {
        let mut disp = "\n".to_string();

        if let Some(last_move) = self.moves.last() {
            if last_move.author.to_string() == AGENT_ADDRESS.to_string() {
                disp.push_str("It is your opponents turn \n");
            } else {
                disp.push_str("It is your turn \n");
            }
        } else {
            disp.push_str("Non-creator must make the first move \n");        
        }
        disp.push('\n');

        disp.push_str("  x  0 1 2 3 4 5 6 7\ny\n");
        let board = board_sparse_to_dense(self);
        for y in 0..BOARD_SIZE {
            disp.push_str(&format!("{}   |", y));
            for x in 0..BOARD_SIZE {
                let c = match board[x][y] {
                    1 => WHITE_PIECE,
                    2 => BLACK_PIECE,
                    _ => EMPTY_SPACE,
                };
                disp.push_str(&format!("{}|", c));
            }
            disp.push('\n');
        }
        if self.player_1.resigned {
            disp.push_str(&format!("Game over: Player 1 has resigned!\n"));
        } else if self.player_2.resigned {
            disp.push_str(&format!("Game over: Player 2 has resigned!\n"));
        } else if self.player_1.winner {
            disp.push_str(&format!("Game over: Player 1 is the winner!\n"));
        } else if self.player_2.winner {
            disp.push_str(&format!("Game over: Player 2 is the winner!\n"));
        }
        disp
    }
}

/// takes a current game state and a move and progresses the state
/// assumes that moves are totally valid by this stage
pub fn state_reducer(game: Game, current_state: GameState, next_move: &Move) -> GameState {
    let current_player = get_current_player(&game, &next_move.author).unwrap();
    let mut moves = current_state.moves.clone();
    moves.push(next_move.to_owned());
    
    match &next_move.move_type {
        MoveType::MovePiece{to, from} => {
            let mut board = board_sparse_to_dense(&current_state);
            // make the move by deleting the piece at the from position and adding one at the to position
            board[from.x][from.y] = 0;
            board[to.x][to.y] = match current_player { Player::Player1 => 1, Player::Player2 => 2};

            // TODO: check if any opponent pieces were taken in this move
            

            // TODO: Check if either player has won

            let (player_1_pieces, player_2_pieces) = board_dense_to_sparse(board);

            GameState{
                player_1: PlayerState {
                    pieces: player_1_pieces,
                    ..current_state.player_1
                },
                player_2: PlayerState {
                    pieces: player_2_pieces,
                    ..current_state.player_2
                },
                moves,
                ..current_state
            }
        },
        MoveType::Resign => {
            match current_player {
                Player::Player1 => {
                    GameState{ player_1: PlayerState{resigned: true, ..current_state.player_1}, moves, ..current_state}
                },
                Player::Player2 => {
                    GameState{ player_2: PlayerState{resigned: true, ..current_state.player_2}, moves, ..current_state}
                }
            }
        }
    }
}

/*========================================
=            Helper functions            =
========================================*/

pub fn board_sparse_to_dense(state: &GameState)-> [[u8; 8]; 8] {
    let mut board = [[0u8; 8]; 8];
    state.player_1.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 1;
    });
    state.player_2.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 2;
    });
    board
}

pub fn board_dense_to_sparse(board: [[u8; 8]; 8]) -> (Vec<Piece>, Vec<Piece>) {
    let mut player_1_pieces = Vec::new();
    let mut player_2_pieces = Vec::new();
    board.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, square)| {
            if *square == 1 {
                player_1_pieces.push(Piece{x , y});
            } else if *square == 2 {
                player_2_pieces.push(Piece{x , y});               
            }
        })
    });
    (player_1_pieces, player_2_pieces)
}

/*=====  End of Helper functions  ======*/
