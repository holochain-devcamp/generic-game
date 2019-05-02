use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData
};

use crate::game_move::Pos;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pieces: Vec<Pos>,
    resigned: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    complete: bool,
    player_1: PlayerState,
    player_2: PlayerState,
}

impl GameState {
    pub fn new() -> Self {
        let p = PlayerState {
            pieces: Vec::new(),
            resigned: false,
        };
        GameState {
            complete: false,
            player_1: p.clone(),
            player_2: p.clone(),
        }
    }
}
