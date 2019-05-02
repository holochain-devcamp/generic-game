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

use crate::game_state::GameState;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Game {
    pub player_1: Address,
    pub player_2: Address,
    pub created_at: u32,
}

pub fn get_latest_move(game_address: &Address) -> Address {
    Address::from("")
}

pub fn get_state(game_address: &Address) -> ZomeApiResult<GameState> {
    Ok(GameState::new())
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
                EntryValidationData::Create{entry, validation_data} => {
                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a game".into())
                }
            }
        }
    )
}
