use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};
use hdk::holochain_persistence_api::{
    cas::content::Address,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    MoveType,
    Piece
    // Usually you would import structs and types defined in the 'moves' file
};

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 * 
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */

pub const BOARD_SIZE: usize = 8;

const WHITE_PIECE: char = '░';
const BLACK_PIECE: char = '▓';
const EMPTY_SPACE: char = ' ';

    /**
     * DEVCAMP TODO #3:
     * Implement struct that determines the state of your game
     * 
     * Hint: you can define other helper structs and reuse the Move struct to store the state
     * References: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
     */
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub player_1: PlayerState,
    pub player_2: PlayerState
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub pieces: Vec<Piece>,
    pub resigned: bool,
    pub winner: bool
}

/**
 * Example of a possible GameState
 * 
 * #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
 * pub struct GameState {
 *     pub moves: Vec<Move>,
 *     pub player_1: PlayerState,
 *     pub player_2: PlayerState,
 * }
 * 
 * #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
 * pub struct PlayerState {
 *     pub pieces: Vec<Piece>,
 *     pub resigned: bool,
 *     pub winner: bool,
 * }
 */
/* 
fn foo() -> String {
    let game_state = GameState::initial();
    let string = game_state.render();

    // GOOD
    game_state.get_moves();
    // BAD DOG
    GameState::get_moves();

    string
}
 */

impl GameState {
        /**
         * DEVCAMP TODO #4:
         * Return the initial game state
         * 
         * Hint: this function will always return a static value, that 
         * Example: 
         *     GameState {
         *         moves: Vec::new(),
         *         player_1: PlayerState {
         *             pieces: Vec::new(),
         *             resigned: false,
         *             winner: false,
         *         },
         *         player_2: PlayerState {
         *             pieces: Vec::new(),
         *             resigned: false,
         *             winner: false,
         *         },
         *     }
         */
    pub fn initial() -> Self {
        let player1_state = PlayerState {
            pieces: vec![
                Piece{x: 0, y: 0}, Piece{x: 2, y: 0}, Piece{x: 4, y: 0}, Piece{x: 6, y: 0},
                Piece{x: 1, y: 1}, Piece{x: 3, y: 1}, Piece{x: 5, y: 1}, Piece{x: 7, y: 1},
                Piece{x: 0, y: 2}, Piece{x: 2, y: 2}, Piece{x: 4, y: 2}, Piece{x: 6, y: 2},
            ],
            resigned: false,
            winner: false
        };

        let player2_state = PlayerState {
            pieces: vec![
                Piece{x: 1, y: 5}, Piece{x: 3, y: 5}, Piece{x: 5, y: 5}, Piece{x: 7, y: 5},
                Piece{x: 0, y: 6}, Piece{x: 2, y: 6}, Piece{x: 4, y: 6}, Piece{x: 6, y: 6},
                Piece{x: 1, y: 7}, Piece{x: 3, y: 7}, Piece{x: 5, y: 7}, Piece{x: 7, y: 7},
            ],
            resigned: false,
            winner: false
        };
        
        GameState {
            moves: Vec::new(),
            player_1: player1_state,
            player_2: player2_state
        }
    }

        /**
         * DEVCAMP TODO #5:
         * Return a string rendering the state of the game, so that the CLI can render it in the terminal
         * 
         * Hint: useful snippets:
         *     let mut disp = "\n".to_string();
         *     disp.push_str("  x  0 1 2 3 4 5 6 7\n");
         * References: https://doc.rust-lang.org/rust-by-example/std/str.html
         */
    pub fn render(&self) -> String {
        let board_dense = board_sparse_to_dense(self);
        let mut disp = "\n".to_string();

        let maybe_move: Option<&Move> = self.moves.last();

        match maybe_move {
            None => {
                disp.push_str("Non-creator must make the first move \n");        
            },
            Some(last_move) => {
                if last_move.author == AGENT_ADDRESS.clone() {
                    disp.push_str("It's your oponent's turn \n");        
                } else {
                    disp.push_str("It's your turn \n");        
                }
            }
        }

        disp.push_str("  x  0 1 2 3 4 5 6 7\n");
        disp.push_str("y\n");

        for y in 0..8 {
            disp.push_str(&format!("{}   |", y));

            for x in 0..8 {
                let piece = board_dense[x][y];

                match piece {
                    0 => disp.push_str(&format!("{}|", EMPTY_SPACE)),
                    1 => disp.push_str(&format!("{}|", WHITE_PIECE)),
                    2 => disp.push_str(&format!("{}|", BLACK_PIECE)),
                    _ => {}
                }
            }

            disp.push_str("\n");
        }

        if self.player_1.resigned {
            disp.push_str("Player 1 has resigned");
        } else if self.player_2.resigned {
            disp.push_str("Player 2 has resigned");
        } else if self.player_1.winner {
            disp.push_str("Player 1 has won");
        } else if self.player_2.winner {
            disp.push_str("Player 2 has won");
        }

        disp
    }

        /**
         * DEVCAMP TODO #6:
         * Return the new game state resulting from applying the next_move to the current state of the game
         * You can assume that next_move is valid
         * 
         * Hints: 
         *   - This is similar to a Redux reducer, a function that given a state and a new action,
         *     returns the next state
         *   - It can be really helpful to use the helper functions below to transform your state before and after
         *     you apply the next_move changes to it. For example, the function could be structured like this:
         *       * Transform GameState to a matrix of all pieces
         *       * Apply move to the matrix
         *       * Transform matrix to GameState
         * References: https://www.joshmcguigan.com/blog/array-initialization-rust/
         */
    pub fn evolve(&self, game: Game, next_move: &Move) -> Self {
        let current_player = get_current_player(&game, &next_move.author).unwrap();
        let mut moves = self.clone().moves.clone();
        moves.push(next_move.to_owned());
        
        match &next_move.move_type {
            MoveType::MovePiece{to, from} => {
                let mut board = board_sparse_to_dense(&self.clone());
                // make the move by deleting the piece at the from position and adding one at the to position
                board[from.x][from.y] = 0;
                board[to.x][to.y] = match current_player { Player::Player1 => 1, Player::Player2 => 2};

                // Delete any hopped pieces this move
                if ((to.x as i32 - from.x as i32).abs(), (to.y as i32 - from.y as i32).abs()) == (2,2) {
                    board
                        [(from.x as i32 + (to.x as i32 - from.x as i32)/2) as usize] 
                        [(from.y as i32 + (to.y as i32 - from.y as i32)/2) as usize]
                     = 0;
                }

                // TODO: Check if either player has won

                let (player_1_pieces, player_2_pieces) = board_dense_to_sparse(board);

                GameState{
                    player_1: PlayerState {
                        pieces: player_1_pieces,
                        ..self.clone().player_1
                    },
                    player_2: PlayerState {
                        pieces: player_2_pieces,
                        ..self.clone().player_2
                    },
                    moves,
                    ..self.clone()
                }
            },
            MoveType::Resign => {
                match current_player {
                    Player::Player1 => {
                        GameState {
                            player_1: PlayerState{ resigned: true, ..self.clone().player_1 }, 
                            player_2: PlayerState { winner: true, ..self.player_2.clone() },
                            moves,
                            ..self.clone()
                        }
                    },
                    Player::Player2 => {
                        GameState { 
                            player_2: PlayerState{ resigned: true, ..self.clone().player_2 },
                            player_1: PlayerState{ winner: true, ..self.clone().player_1 },
                            moves,
                            ..self.clone()
                        }
                    }
                }
            }
        }
    }
}

/*========================================
=            Helper functions            =
*/
// This function transforms the game state from a vector of pieces to a matrix where each
// position represents the location of the piece

// Visual representation:
// From: { player1: [{x: 0, y: 4}, {x: 6, y: 3}], player2: [{x: 6, y: 4}, {x: 5, y: 6}]
// To: 
// [
//    [0, 0, 0, 0, 1, 0, 0, 0],
//    [0, 0, 0, 0, 0, 0, 0, 0],
//    [0, 0, 0, 0, 0, 0, 0, 0],
//    [0, 0, 0, 0, 0, 0, 1, 0],
//    [0, 0, 0, 0, 0, 0, 0, 0],
//    [0, 0, 0, 0, 0, 0, 2, 0],
//    [0, 0, 0, 0, 2, 0, 0, 0],
//    [0, 0, 0, 0, 0, 0, 0, 0],
//]


pub enum Player{
    Player1,
    Player2,
}

pub fn get_current_player(game: &Game, player_addr: &Address) -> Result<Player, String> {
    match (player_addr == &game.player_1, player_addr == &game.player_2) {
        (true, true) => return Err("Player cannot play themselves".into()),
        (true, false) => Ok(Player::Player1),
        (false, true) => Ok(Player::Player2),
        (false, false) => return Err("Player is not part of this game!".into()),
    }
}

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

// This function is the inverse of the function above,
// takes a matrix in the same format of above and transforms it to a tuple of two vectors of pieces,
// that can directly be saved in the entry

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