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

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Game {
    player_1: Address,
    player_2: Address,
    created_at: u32,
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
