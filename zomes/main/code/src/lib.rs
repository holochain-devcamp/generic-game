#![feature(vec_remove_item, proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

extern crate hdk_proc_macros;
use hdk_proc_macros::zome;

use hdk::{
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::{
            Address,
            AddressableContent,
        },
    },
    holochain_core_types::{
        entry::Entry,
    },
};

/**
 * DEVCAMP TODO #8:
 * Import your own game State, MoveType and state_reducer
 *
 * Example: 
 *    mod tictactoe;
 *    pub use tictactoe::{
 *       GameState,
 *       MoveType,
 *    };
 * 
 */


mod game;
mod game_move;
mod matchmaking;

use game::Game;
use game_move::{Move, MoveInput};
use matchmaking::{GameProposal, GetResponse};

#[zome]
pub mod main {

    #[init]
    pub fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    /*=========================================
    =            Entry Definitions            =
    =========================================*/

    #[entry_def]
    fn game_entry_def() -> ValidatingEntryType {
        game::definition()
    }

    #[entry_def]
    fn game_move_entry_def() -> ValidatingEntryType {
        game_move::definition()
    }

    #[entry_def]
    fn game_proposal_def() -> ValidatingEntryType {
        matchmaking::game_proposal_def()
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        matchmaking::anchor_def()
    }

    /*=====  End of Entry Definitions  ======*/


    /*======================================
    =            Zome functions            =
    ======================================*/

    #[zome_fn("hc_public")]
    fn create_game(opponent: Address, timestamp: u32) -> ZomeApiResult<Address> {
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

    #[zome_fn("hc_public")]
    fn make_move(new_move: MoveInput) -> ZomeApiResult<()> {
        // get all the moves from the DHT by following the hash chain
        let published_moves = game::get_moves(&new_move.game)?;

        // get all moves in this agents local chain
        let chain_moves = hdk::query("move".into(), 0, 0)?;

        // Update this agents local chain to match the game state
        let base_address = match published_moves.clone().last() {
            Some(last_move) => { // add any moves NOT found in the DHT to this agents local chain
                for _move in &published_moves {
                    let move_entry = Entry::App("move".into(), _move.into());
                    if !chain_moves.contains(&move_entry.address()) {
                        hdk::commit_entry(&move_entry)?;
                    }
                }
                Entry::App("move".into(), last_move.into()).address()

            }
            None => { // no moves have been made so commit the Game to local chain
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

        match published_moves.last() {
            Some(_) => {
                // base is a move
                hdk::link_entries(&base_address, &move_address, "move->move", "")?;
            }
            None => {
                // base is a game
                hdk::link_entries(&base_address, &move_address, "game->move", "")?;
            }
        }

        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_game_hash(opponent: Address, timestamp: u32) -> ZomeApiResult<Address> {
        let new_game = Game {
            player_1: opponent,
            player_2: AGENT_ADDRESS.to_string().into(),
            created_at: timestamp,
        };
        Ok(Entry::App(
            "game".into(),
            new_game.into(),
        ).address())
    }

    #[zome_fn("hc_public")]
    fn get_state(game_address: Address) -> ZomeApiResult<GameState> {
        game::get_state(&game_address)
    }

    #[zome_fn("hc_public")]
    fn render_state(game_address: Address) -> ZomeApiResult<String> {
        Ok(game::get_state(&game_address)?.render())
    }

    #[zome_fn("hc_public")]
    fn get_valid_moves() -> ZomeApiResult<Vec<MoveType>> {
        Ok(MoveType::describe())
    }

    #[zome_fn("hc_public")]
    fn whoami() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.to_string().into())
    }

    #[zome_fn("hc_public")]
    fn create_proposal(message: String) -> ZomeApiResult<Address> {
        matchmaking::handle_create_proposal(message)
    }

    #[zome_fn("hc_public")]
    fn get_proposals() -> ZomeApiResult<Vec<GetResponse<GameProposal>>> {
        matchmaking::handle_get_proposals()
    }

    #[zome_fn("hc_public")]
    fn accept_proposal(proposal_addr: Address, created_at: u32) -> ZomeApiResult<Address> {
        matchmaking::handle_accept_proposal(proposal_addr, created_at)
    }

    #[zome_fn("hc_public")]
    fn check_responses(proposal_addr: Address) -> ZomeApiResult<Vec<GetResponse<Game>>> {
        matchmaking::handle_check_responses(proposal_addr)
    }

    #[zome_fn("hc_public")]
    fn remove_proposal(proposal_addr: Address) -> ZomeApiResult<Address> {
        matchmaking::handle_remove_proposal(proposal_addr)
    }
    /*=====  End of Zome functions  ======*/
}
