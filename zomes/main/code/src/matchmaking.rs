use hdk::{
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::{JsonString, default_to_json},
    validation::EntryValidationData,
    cas::content::AddressableContent,
};
use serde::Serialize;
use std::fmt::Debug;

use crate::game::Game;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GameProposal {
    pub agent: Address,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
} 

pub fn handle_create_proposal(message: String) -> ZomeApiResult<Address> {

    // create the data as a struct
    let game_proposal_data = GameProposal { 
        agent: AGENT_ADDRESS.to_string().into(),
        message,
    };
    
    // create an entry
    let entry = Entry::App(
        "game_proposal".into(),
        game_proposal_data.into(),
    );
    
    // commit the entry. '?' means return immedietly on error
    let proposal_address = hdk::commit_entry(&entry)?;
    
    // create an anchor entry and commit.
    // The native type is string so we can skip the first step
    let anchor_entry = Entry::App(
        "anchor".into(),
        "game_proposals".into(),
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    
    // finally link them together
    hdk::link_entries(
        &anchor_address,
        &proposal_address,
        "has_proposal", // the link type, defined on the base entry
        "" // the tag which is not used in this example
    )?;
    
    // return the proposal address
    Ok(proposal_address)
}

pub fn handle_get_proposals() -> ZomeApiResult<Vec<GetResponse<GameProposal>>> {
    // define the anchor entry again and compute its hash
    let anchor_address = Entry::App(
        "anchor".into(),
        "game_proposals".into()
    ).address();
    
    Ok(
        hdk::utils::get_links_and_load_type(
            &anchor_address, 
            Some("has_proposal".into()), // the link type to match
            None
        )?.into_iter().map(|proposal: GameProposal| {
            let address = Entry::App("game_proposal".into(), proposal.clone().into()).address();
            GetResponse{entry: proposal, address}
        }).collect()
    )
}

pub fn handle_accept_proposal(proposal_addr: Address, created_at: u32) -> ZomeApiResult<()> {
    // this will early return error if it doesn't exist
    let proposal: GameProposal = hdk::utils::get_as_type(proposal_addr.clone())?;

    // create the new game
    let game = Game {
        player_1: AGENT_ADDRESS.to_string().into(),
        player_2: proposal.agent,
        created_at,
    };
    let game_entry = Entry::App(
        "game".into(),
        game.into()
    );
    let game_addr = hdk::commit_entry(&game_entry)?;

    // link to the proposal
    hdk::link_entries(
        &proposal_addr,
        &game_addr,
        "from_proposal",
        ""
    )?;
    Ok(())
}

pub fn handle_check_responses(proposal_addr: Address) -> ZomeApiResult<Vec<GetResponse<Game>>> {
    Ok(
        hdk::utils::get_links_and_load_type(&proposal_addr, Some("from_proposal".into()), None)?
        .into_iter().map(|game: Game| {
            let address = Entry::App("game".into(), game.clone().into()).address();
            GetResponse{entry: game, address}
        }).collect()
    )
}

pub fn handle_remove_proposal(proposal_addr: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&proposal_addr)
}

pub fn game_proposal_def() -> ValidatingEntryType {
    entry!(
        // we will need to use this name when creating an entry later
        name: "game_proposal",
        description: "Represents an agent advertizing they wish to play a game at this time",
        // Public sharing means this entry goes to the local chain *and* DHT
        sharing: Sharing::Public, 
        validation_package: || {
            // This defines the data required for the validation callback.
            // In this case it is just the entry data itself
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<GameProposal>| {
            match validation_data {
                // only match if the entry is being created (not modified or deleted)
                EntryValidationData::Create{ entry, validation_data } => {
                    let game_proposal = GameProposal::from(entry);
                    if validation_data.sources().contains(&game_proposal.agent) {
                        Ok(())
                    } else {
                        Err("Cannot author a proposal from another agent".into())
                    }
                    
                },
                EntryValidationData::Delete{..} => {
                    Ok(())
                },
                _ => {
                    Err("Cannot modify, only create and delete".into())
                }
            }
        },
        links: [
            to!(
                "game",
                link_type: "from_proposal",
                validation_package: || { hdk::ValidationPackageDefinition::Entry },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn anchor_def() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "Central known location to link from",
        sharing: Sharing::Public, 
        validation_package: || { hdk::ValidationPackageDefinition::Entry },
        validation: | _validation_data: hdk::EntryValidationData<String>| {
            Ok(())
        },
        links: [
            to!(
                "game_proposal", // this must match exactly the target entry type
                link_type: "has_proposal", // must use this when creating the link
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