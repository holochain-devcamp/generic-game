use hdk::holochain_json_api::{
    error::JsonError, json::JsonString,
};
use hdk::AGENT_ADDRESS;

use crate::game_move::Move;
use crate::game::Game;
use super::{
    MoveType,
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


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState { 
    /**
     * DEVCAMP TODO #3:
     * Implement struct that determines the state of your game
     * 
     * Hint: you can define other helper structs and reuse the Move struct to store the state
     * References: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
     */
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



impl GameState {
    pub fn initial() -> Self {
        /**
         * DEVCAMP TODO #4:
         * Return the initial game state
         * 
         * Hint: 
         * References: 
         */
    }

    pub fn render(&self) -> String {
        /**
         * DEVCAMP TODO #5:
         * Return a string rendering the state of the game, so that the CLI can render it in the terminal
         * 
         * Hint: useful snippets:
         *     let mut disp = "\n".to_string();
         *     disp.push_str("  x  0 1 2 3 4 5 6 7\ny\n");
         * References: 
         */
    }

    pub fn evolve(&self, game: Game, next_move: &Move) -> Self {
        /**
         * DEVCAMP TODO #6:
         * Return the new game state resulting from applying the next_move to the current state of the game
         * You can assume that next_move is valid
         * 
         * Hints: 
         *   - This is similar to a Redux reducer, a function that given a state and a new action,
         *     returns the next state
         *   - You can declare helper functions (eg: get_current_player, format_state)
         *     to help transform your game state into an easier format for this function to read
         * References: 
         */
    }
}
