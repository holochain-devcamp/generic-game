use hdk::holochain_core_types::{
    cas::content::Address,
};

use crate::game::Game;
use crate::game_move::Move;
use crate::checkers::{
    BOARD_SIZE,
    GameState,
    moves::Piece,
    MoveType,
};

/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 * 
 * This function  takes the current game and the game state (which includes all the existing moves) 
 * and determines if a new candidate move is valid.
 * 
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these are visible from the testing code.
 *
 */


impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        match &self.move_type {
            MoveType::MovePiece{from, to} => {
                is_players_turn(self.author.clone(), game, game_state)?;
                from.is_in_bounds()?;
                to.is_in_bounds()?;
                Ok(())
            }
        }
    }

}


/*========================================
=            Helper functions            =
========================================*/

fn is_players_turn(player: Address, game: Game, game_state: GameState) -> Result<(), String> {
    let moves = game_state.moves;
    match moves.last() {
        Some(last_move) => {
            if last_move.author == player {
                Err("It is not this players turn".into())
            } else {
                Ok(())
            }
        },
        None => {
            // by convention player 2 makes the first move thus accepting the invitation to play
            if game.player_2 == player {
                Ok(())
            } else {
                Err("Player 2 must make the first move".into())
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
}

/*=====  End of Helper functions  ======*/

