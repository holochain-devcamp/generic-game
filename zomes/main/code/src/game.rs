use std::convert::TryFrom;
use hdk::{
    utils,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult, ZomeApiError},
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData,
    entry::Entry,
    cas::content::AddressableContent,
};

use crate::game_move::Move;
use crate::{GameState, state_reducer};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Game {
    pub player_1: Address,
    pub player_2: Address,
    pub created_at: u32,
}

/// Traverse the linked list rooted at a game to find all the moves
pub fn get_moves(game_address: &Address) -> ZomeApiResult<Vec<Move>> {
    match hdk::get_links(game_address, "")?.addresses().into_iter().next() {
        Some(first_move) => {
            let mut move_addresses = vec![first_move];
            let mut more = true;
            while more {
                more = match hdk::get_links(move_addresses.last().unwrap(), "")?.addresses().into_iter().next() {
                    Some(addr) => {
                        move_addresses.push(addr.clone());
                        true
                    },
                    None => {
                        false
                    },
                }
            }
            let moves: Vec<Move> = move_addresses.iter().map(|addr| {
                let move_entry = hdk::get_entry(addr).unwrap().unwrap();
                if let Entry::App(_, move_struct) = move_entry {
                    Move::try_from(move_struct).expect("Entry at address is type other than Move")
                } else {
                    panic!("Not an app entry!")
                }
            }).collect();
            Ok(moves)
        },
        None => {
            Ok(Vec::new())
        }
    }
}

pub fn get_state(game_address: &Address) -> ZomeApiResult<GameState> {
    let moves = get_moves(game_address)?;
    let new_state = moves.iter().fold(GameState::initial(), state_reducer);
    Ok(new_state)
}

pub fn get_game(game_address: &Address) -> ZomeApiResult<Game> {
    utils::get_as_type(game_address.to_owned())
}

pub fn get_game_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<Game> {
    local_chain
        .iter()
        .filter(|entry| {
            entry.address() == game_address.to_owned()
        })
        .filter_map(|entry| {
            if let Entry::App(_, entry_data) = entry {
                Some(Game::try_from(entry_data.clone()).unwrap())
            } else {
                None
            }
        })
        .next()
        .ok_or(ZomeApiError::HashNotFound)
}

pub fn get_moves_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<Vec<Move>> {
    Ok(local_chain
        .iter()
        .filter_map(|entry| {
            if let Entry::App(entry_type, entry_data) = entry {
                if entry_type.to_string() == "move" {
                    Some(Move::try_from(entry_data.clone()).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|game_move| {
            game_move.game == game_address.to_owned()
        })
        .rev()
        .collect())
}

pub fn get_state_local_chain(local_chain: Vec<Entry>, game_address: &Address) -> ZomeApiResult<GameState> {
    let moves = get_moves_local_chain(local_chain, game_address)?;
    let new_state = moves.iter().fold(GameState::initial(), state_reducer);
    Ok(new_state)
}


pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "game",
        description: "Represents an occurence of a game between several agents",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<Game>| {
            match validation_data {
                EntryValidationData::Create{entry: _, validation_data: _} => {
                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a game".into())
                }
            }
        }
    )
}
