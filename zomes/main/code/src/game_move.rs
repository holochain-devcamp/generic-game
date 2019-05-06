use hdk::{
    entry_definition::ValidatingEntryType,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData
};

use crate::game_state::GameState;
use crate::game::{Game, get_game_local_chain, get_state_local_chain};

const BOARD_SIZE: usize = 8;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Piece {
	pub x: usize,
	pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct MoveInput {
	pub game: Address,
	pub move_type: MoveType,
	pub timestamp: u32,
}


// this is specific to Checkers
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub enum MoveType {
	MovePiece {
		from: Piece,
		to: Piece,
	},
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Move {
	pub game: Address,
	pub author: Address,
	pub move_type: MoveType,
	pub previous_move: Address,
	pub timestamp: u32,
}

impl Move {
	fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
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


pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "move",
        description: "A move by an agent in an game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },

        validation: | validation_data: hdk::EntryValidationData<Move>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data} => {
                	let local_chain = validation_data.package.source_chain_entries
                		.ok_or("Could not retrieve source chain")?;
                	hdk::debug(format!("{:?}", local_chain))?;
                	// load the game and game state
                	let new_move = Move::from(entry);
                	let mut state = get_state_local_chain(local_chain.clone(), &new_move.game)
                		.map_err(|_| "Could not load state during validation")?;
                	let game = get_game_local_chain(local_chain, &new_move.game)
                	    .map_err(|_| "Could not load game during validation")?;

                	// THIS IS A HACK! Find a better solution to the problem of unsorted, possibly duplicate calls to validate
                	state.moves.remove_item(&new_move);
                    
                    new_move.is_valid(game, state)
                },
                _ => {
                    Err("Cannot modify or delete a move".into())
                }
            }
        },

        links: [
        	from!(
                "game",
                tag: "",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
        	from!(
                "move",
                tag: "",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
