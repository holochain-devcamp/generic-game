use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use crate::game::Game;
use crate::game_move::Move;
use super::{
    GameState,
    moves::Piece,
    MoveType,
    state::{board_sparse_to_dense, BOARD_SIZE},
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
        hdk::debug(format!("{:?}", game_state)).unwrap();
        // let current_player = get_current_player(&game, &self.author)?;
        match &self.move_type {
            MoveType::Place{pos} => {
                is_players_turn(self.author.clone(), game, &game_state)?;
                pos.is_in_bounds()?;
                pos.is_empty(&game_state)?;
                hdk::debug("Validation Success!").unwrap();
                Ok(())            }
        }
    }
}


/*========================================
=            Helper functions            =
========================================*/

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

    fn is_empty(&self, game_state: &GameState) -> Result<(), String> {
        match board_sparse_to_dense(game_state)[self.x][self.y] == 0 {
            true => Ok(()),
            false => Err("A piece already exists at that position.".to_string())
        }
    }
}

/*=====  End of Helper functions  ======*/

