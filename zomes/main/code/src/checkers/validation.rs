use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    MoveType,
    Piece,
    state::{board_sparse_to_dense, BOARD_SIZE, get_current_player, Player}
};


/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */


impl Move {
        /**
         * DEVCAMP TODO #7:
         * Return Ok() if the self move is valid, or Err("Error message".into()) otherwise
         * 
         * Hint: 
         *   - You can use the standard '?' rust notation to call helper functions and 
         *     return their error messages upwards (akin to 'throw Exception' in other languages)
         *   - Usually you should make global move checks first ('is it the turn of the author of the move?'),
         *     and then match the 'self.move_type' enum and check the validity of each variant
         * References: https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
         */
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        // If it is this player's turn
        is_players_turn(self.author.clone(), game.clone(), &game_state)?;

        let current_player = get_current_player(&game, &self.author)?;

        // If the game has not ended
        if 
            game_state.player_1.resigned || 
            game_state.player_1.winner || 
            game_state.player_2.resigned || 
            game_state.player_2.winner {
                return Err(String::from("The game has already ended: no more moves allowed"));
            } 

        // Match move_type

        match &self.move_type {
            MoveType::Resign => Ok(()),
            MoveType::MovePiece { from, to } => {
                from.is_in_bounds()?;
                to.is_in_bounds()?;

                from.is_piece_belonging_to_player(&current_player, &game_state)?;

                to.is_empty(&game_state)?;

                from.can_move_to(&to, &current_player, &game_state)?;

                Ok(())
            }
        }

        // MovePiece
        // From is in bounds
        // To is in bounds
        // The piece belongs the player
        // If there is no piece at the to location

        // Resign
        // Ok(())


    }
}


fn is_players_turn(player: Address, game: Game, game_state: &GameState) -> Result<(), String> {
    let moves = &game_state.moves;
    match moves.last() {
        Some(last_move) => {
            if last_move.author == player {
                Err("It is not this players turn.".into())
            } else {
                Ok(())
            }
        },
        None => {
            // by convention player 2 makes the first move thus accepting the invitation to play
            if game.player_2 == player {
                Ok(())
            } else {
                Err("Player 2 must make the first move.".into())
            }
        },
    }
}

impl Piece {
    fn is_in_bounds(&self) -> Result<(), String> {
        if self.x < BOARD_SIZE 
        && self.y < BOARD_SIZE // no need to check > 0 as usize is always positive
        {
            Ok(())
        } else {
            Err("Position is not in bounds".to_string())
        }
    }

    fn is_piece_belonging_to_player(&self, player: &Player, game_state: &GameState) -> Result<(), String> {
        match board_sparse_to_dense(game_state)[self.x][self.y] {
            0 => Err("There are no pieces at the 'from' location".into()),
            1 => match player { Player::Player1 => Ok(()), _ => Err("Piece at 'from' position belongs to other player".into())},
            2 => match player { Player::Player2 => Ok(()), _ => Err("Piece at 'from' position belongs to other player".into())},
            _ => Err("Board is in an invalid state!".into())
        }
    }

    fn can_move_to(&self, to: &Piece, player: &Player, game_state: &GameState) -> Result<(), String> {

        let jump = match ((to.x as i32 - self.x as i32).abs(), (to.y as i32 - self.y as i32).abs()) {
            (1,1) => false,
            (2,2) => {
                let inbetween_piece = Piece{
                    x: (self.x as i32 + (to.x as i32 - self.x as i32)/2) as usize,
                    y: (self.y as i32 + (to.y as i32 - self.y as i32)/2) as usize,
                };
                hdk::debug(format!("inbetween piece is: {:?}", inbetween_piece))?;
                match inbetween_piece.is_empty(game_state) { // must be jumping over another piece
                    Ok(_) => {
                        return Err("Can only jump over another piece".into())
                    },
                    Err(_) => {
                        // that piece must belong to the other player
                        match player {
                            Player::Player1 => {
                                match inbetween_piece.is_piece_belonging_to_player(&Player::Player2, game_state) {
                                    Ok(_) => true,
                                    Err(_) => return Err("Cannot jump own piece".into())
                                }
                            },
                            Player::Player2 => {
                                match inbetween_piece.is_piece_belonging_to_player(&Player::Player1, game_state) {
                                    Ok(_) => true,
                                    Err(_) => return Err("Cannot jump own piece".into())
                                }
                            },                        
                        }
                    }
                }
            },
            _ => return Err("This move is not diagonal".to_string())
        };

        // are we jumping a piece? If so you can move
        let step = match jump { false => 1, true => 2};

        // pawns can only move:
        // sideways by 1 square (or 2 if jumping)
        if !(to.x == self.x + step || to.x == self.x - step) {
            return Err("Pawns must move diagonally.".into())
        }
        // and foward by 1 according to the player (or 2 if jumping)
        match player {
            Player::Player1 => {
                match to.y == self.y + step {
                    true => Ok(()),
                    false => Err("Pawns cannot move backward".into())
                }
            },
            Player::Player2 => {
                match to.y == self.y - step {
                    true => Ok(()),
                    false => Err("Pawns cannot move backward".into())
                }
            }
        }
    }

    fn is_empty(&self, game_state: &GameState) -> Result<(), String> {
        match board_sparse_to_dense(game_state)[self.x][self.y] == 0 {
            true => Ok(()),
            false => Err("A piece already exists at the 'to' position.".to_string())
        }
    }
}