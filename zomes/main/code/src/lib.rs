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
    error::ZomeApiResult,
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    error::HolochainError,
    json::JsonString,
    cas::content::AddressableContent,
};


mod game;
mod game_move;
mod game_state;

use game::Game;
use game_move::{Move, MoveInput};
use game_state::GameState;

fn handle_create_game(opponent: Address, timestamp: u32) -> ZomeApiResult<Address> {
    let new_game = Game {
        player_1: AGENT_ADDRESS.to_string().into(),
        player_2: opponent,
        created_at: timestamp,
    };
    let game_entry = Entry::App(
        "game".into(),
        new_game.into(),
    );
    hdk::commit_entry(&game_entry)
}

fn handle_make_move(game_move: MoveInput) -> ZomeApiResult<()> {
    let base_address = match game::get_moves(&game_move.game)?.last() {
        Some(previous_move) => Entry::App("move".into(), previous_move.into()).address(),
        None => game_move.game.clone(),
    };
    let new_move = Move {
        game: game_move.game,
        author: AGENT_ADDRESS.to_string().into(),
        move_type: game_move.move_type,
        previous_move: base_address.clone(),
    };
    let game_entry = Entry::App(
        "move".into(),
        new_move.into(),
    );
    let move_address = hdk::commit_entry(&game_entry)?;
    hdk::link_entries(&base_address, &move_address, "")?;
    Ok(())
}

fn handle_get_state(game_address: Address) -> ZomeApiResult<GameState> {
    game::get_state(&game_address)
}

fn handle_get_moves(game_address: Address) -> ZomeApiResult<Vec<Move>> {
    game::get_moves(&game_address)
}

define_zome! {
    entries: [
       game::definition(),
       game_move::definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_game: {
            inputs: |opponent: Address, timestamp: u32|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_game
        }
        make_move: {
            inputs: |game_move: MoveInput|,
            outputs: |result: ZomeApiResult<()>|,
            handler: handle_make_move
        }
        get_state: {
            inputs: |game_address: Address|,
            outputs: |result: ZomeApiResult<GameState>|,
            handler: handle_get_state
        }
        get_moves: {
            inputs: |game_address: Address|,
            outputs: |result: ZomeApiResult<Vec<Move>>|,
            handler: handle_get_moves            
        }
    ]

    traits: {
        hc_public [create_game, make_move, get_state, get_moves]
    }
}
