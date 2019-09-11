use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    MoveType,
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
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        /**
         * DEVCAMP TODO #7:
         * Return Ok() if the self move is valid, or Err("Error message".into()) otherwise
         * 
         * Hint: 
         *   - You can use the standard '?' rust notation to call helper functions and 
         *     return their error messages upwards (akin to 'throw Exception' in other languages)
         *   - Usually you should make global move checks first ('is it the turn of the author of the move?'),
         *     and then match the 'self.move_type' enum and check the validity of each variant
         * References: 
         */
    }
}

