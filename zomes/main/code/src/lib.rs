#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

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


mod game;
mod game_move;
mod game_state;

use game::Game;
use game_move::Move;
use game_state::GameState;

fn handle_create_game(opponent: Address, timestamp: i32) -> ZomeApiResult<Address> {
    Ok(Address::from("yep"))
}

fn handle_make_move(game_move: Move) -> ZomeApiResult<GameState> {
    Ok(GameState::new())
}

fn handle_get_state(game_address: Address) -> ZomeApiResult<GameState> {
    Ok(GameState::new())
}

define_zome! {
    entries: [
       game::definition(),
       game_move::definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_game: {
            inputs: |opponent: Address, timestamp: i32|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_game
        }
        make_move: {
            inputs: |game_move: Move|,
            outputs: |result: ZomeApiResult<GameState>|,
            handler: handle_make_move
        }
        get_state: {
            inputs: |game_address: Address|,
            outputs: |result: ZomeApiResult<GameState>|,
            handler: handle_get_state
        }

]

    traits: {
        hc_public [create_game, make_move, get_state]
    }
}
