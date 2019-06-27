use hdk::{
    holochain_json_api::{
        error::JsonError, json::JsonString,
    },
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    moves::Piece,
    MoveType,
    validation::{Player, get_current_player},
};

pub const BOARD_SIZE: usize = 3;
pub const PLAYER_1_MARK: char = 'O';
pub const PLAYER_2_MARK: char = 'X';  //player 2 / Xs go first
pub const EMPTY_SPACE: char = ' ';

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

impl PlayerState {
    pub fn initial() -> Self {
        PlayerState {
            pieces: Vec::new(),
            resigned: false,
            winner: false,
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        GameState {
            moves: Vec::new(),
            player_1: PlayerState::initial(),
            player_2: PlayerState::initial(),
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
        disp.push_str("  x  0 1 2\ny\n");
        let board = board_sparse_to_dense(self);
        for y in 0..BOARD_SIZE {
            disp.push_str(&format!("{}   |", y));
            for x in 0..BOARD_SIZE {
                let c = match board[x][y] {
                    1 => PLAYER_1_MARK,
                    2 => PLAYER_2_MARK,
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

    match &next_move.move_type {
        MoveType::Place{pos} => {
            let mut board = board_sparse_to_dense(&current_state);
            let mut moves = current_state.moves;
            moves.push(next_move.to_owned());

            // make the move by adding a new piece at the position
            board[pos.x][pos.y] = match current_player { Player::Player1 => 1, Player::Player2 => 2};

            // check if this resulted in a player victory
            let mut diag_down = 0;
            let mut diag_up = 0;
            let mut across = [0; 3];
            let mut down = [0; 3];
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    let delta = match board[x][y] {1 => 1, 2 => -1, _ => 0};
                    down[x] += delta;
                    across[y] += delta;
                    // diag down e.g. \
                    if x == y {
                        diag_down += delta;
                    } //diag up  e.g. /
                    else if x == (BOARD_SIZE - 1 - y) {
                        diag_up += delta;
                    }
                }
            }
            let player_1_victory = across.iter().any(|e| *e == (BOARD_SIZE as i32)) 
                                || down.iter().any(|e| *e == (BOARD_SIZE as i32));
                                || diag_down == (BOARD_SIZE as i32);
                                || diag_up == (BOARD_SIZE as i32);

            let player_2_victory = across.iter().any(|e| *e == (-1*BOARD_SIZE as i32)) 
                                || down.iter().any(|e| *e == (-1*BOARD_SIZE as i32));
                                || diag_down == (-1*BOARD_SIZE as i32);
                                || diag_up == (-1*BOARD_SIZE as i32);

            let (player_1_pieces, player_2_pieces) = board_dense_to_sparse(board);

            GameState{
                player_1: PlayerState {
                    pieces: player_1_pieces,
                    resigned: false,
                    winner: player_1_victory,
                },
                player_2: PlayerState {
                    pieces: player_2_pieces,
                    resigned: false,
                    winner: player_2_victory,
                },
                moves,
                ..current_state
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
