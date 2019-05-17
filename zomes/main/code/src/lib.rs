#![feature(try_from, vec_remove_item)]
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
};

// This is where you would import your own game State, MoveType and state_reducer
// mod checkers;
// pub use checkers::{
//     GameState,
//     MoveType,
//     state_reducer,
// };

mod tictactoe;
pub use tictactoe::{
    GameState,
    MoveType,
    state_reducer,
};


mod game;
mod game_move;

use game::Game;
use game_move::{Move, MoveInput};

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

fn handle_make_move(new_move: MoveInput) -> ZomeApiResult<()> {
    // get all the moves from the DHT by following the hash chain
    let moves = game::get_moves(&new_move.game)?;

    // commit the latest move to local chain to allow validation of the next move (if one exists)
    let base_address = match moves.last() {
        Some(last_move) => {
            let new_move_entry = Entry::App("move".into(), last_move.into());
            hdk::commit_entry(&new_move_entry)?
        }
        None => { // no moves have been made so commit the Game
            let game = game::get_game(&new_move.game)?;
            let game_entry = Entry::App("game".into(), game.into());
            hdk::commit_entry(&game_entry)?
        }
    };

    let new_move = Move {
        game: new_move.game,
        author: AGENT_ADDRESS.to_string().into(),
        move_type: new_move.move_type,
        previous_move: base_address.clone(),
        timestamp: new_move.timestamp,
    };
    let move_entry = Entry::App(
        "move".into(),
        new_move.into(),
    );
    let move_address = hdk::commit_entry(&move_entry)?;
    hdk::link_entries(&base_address, &move_address, "")?;
    Ok(())
}

fn handle_get_state(game_address: Address) -> ZomeApiResult<GameState> {
    game::get_state(&game_address)
}

fn handle_render_state(game_address: Address) -> ZomeApiResult<String> {
    Ok(handle_get_state(game_address)?.render())
}

fn handle_get_valid_moves() -> ZomeApiResult<Vec<MoveType>> {
    Ok(MoveType::describe())
}

fn handle_whoami() -> ZomeApiResult<Address> {
    Ok(AGENT_ADDRESS.to_string().into())
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
        render_state: {
            inputs: |game_address: Address|,
            outputs: |result: ZomeApiResult<String>|,
            handler: handle_render_state
        }
        valid_moves: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<MoveType>>|,
            handler: handle_get_valid_moves
        }
        whoami: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_whoami
        }
    ]

    traits: {
        hc_public [create_game, make_move, get_state, render_state, valid_moves, whoami]
    }
}
